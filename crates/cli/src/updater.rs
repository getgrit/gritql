use anyhow::bail;
use anyhow::{Context, Result};
use axoupdater::{AxoUpdater, ReleaseSource, ReleaseSourceType, Version};
use chrono::{DateTime, NaiveDateTime, Utc};
use colored::Colorize;
use futures_util::StreamExt;
use indicatif::ProgressBar;
use log::info;
use marzano_auth::info::AuthInfo;
use marzano_gritmodule::config::REPO_CONFIG_DIR_NAME;
use marzano_util::runtime::{ExecutionContext, LanguageModelAPI};
use reqwest::redirect::Policy;
use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;
use std::fmt;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::fs as async_fs;
use tokio::fs::File;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::process::Command as AsyncCommand;
use tokio::sync::Mutex;
use uuid::Uuid;

use crate::utils::{get_client_arch, get_client_os};
use marzano_auth::env::{get_env_auth, get_grit_api_url};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum AllApp {
    Marzano,
    Cli,
    Timekeeper, // Our copy of Temporalite
    // legacy
    Engine,
    Yeast,
    WorkflowRunner,
    // Server CLI
    Gouda,
}

impl fmt::Display for AllApp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AllApp::Marzano => write!(f, "marzano"),
            AllApp::Cli => write!(f, "cli"),
            AllApp::Timekeeper => write!(f, "timekeeper"),
            AllApp::Engine => write!(f, "engine"),
            AllApp::Yeast => write!(f, "yeast"),
            AllApp::WorkflowRunner => write!(f, "workflow_runner"),
            AllApp::Gouda => write!(f, "gouda"),
        }
    }
}

impl AllApp {
    fn from_supported_app(app: SupportedApp) -> Self {
        match app {
            SupportedApp::Marzano => AllApp::Marzano,
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => AllApp::WorkflowRunner,
            SupportedApp::Gouda => AllApp::Gouda,
        }
    }
}

// Allowed modern apps
#[derive(Debug, Clone, Copy, Serialize, Deserialize, clap::ValueEnum)]
pub enum SupportedApp {
    Marzano,
    Gouda,
    #[cfg(feature = "workflows_v2")]
    WorkflowRunner,
}

impl SupportedApp {
    // Apps to install for a default "install" command
    pub fn is_default_app(&self) -> bool {
        matches!(self, SupportedApp::Marzano)
    }

    pub fn get_base_name(&self) -> String {
        match self {
            SupportedApp::Marzano => "marzano".to_string(),
            SupportedApp::Gouda => "gouda".to_string(),
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => "workflow_runner".to_string(),
        }
    }

    fn get_env_name(&self) -> String {
        match self {
            SupportedApp::Marzano => "GRIT_MARZANO_PATH".to_string(),
            SupportedApp::Gouda => "GRIT_GOUDA_PATH".to_string(),
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => "GRIT_WORKFLOW_RUNNER".to_string(),
        }
    }

    fn get_bin_name(&self) -> String {
        format!("{}-{}", self.get_base_name(), get_client_os())
    }

    fn get_fallback_bin_name(&self) -> String {
        self.get_base_name().to_string()
    }

    fn get_file_name(&self, os: &str, arch: &str) -> String {
        let base_name = self.get_base_name();
        format!("{}-{}-{}", base_name, os, arch)
    }

    pub fn from_all_app(app: AllApp) -> Option<Self> {
        match app {
            AllApp::Marzano => Some(SupportedApp::Marzano),
            AllApp::Gouda => Some(SupportedApp::Gouda),
            #[cfg(feature = "workflows_v2")]
            AllApp::WorkflowRunner => Some(SupportedApp::WorkflowRunner),
            _ => None,
        }
    }
}

impl fmt::Display for SupportedApp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_base_name())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppManifest {
    pub name: AllApp,
    pub release: Option<String>,
    pub version: Option<String>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
struct Manifest {
    binaries: HashMap<String, AppManifest>,
    #[cfg(feature = "updater")]
    last_checked_update: Option<NaiveDateTime>,
    installation_id: Option<Uuid>,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

async fn read_manifest(manifest_path: &PathBuf) -> Result<Manifest> {
    let mut manifest_file = File::open(manifest_path).await?;
    let mut manifest_content = String::new();
    manifest_file.read_to_string(&mut manifest_content).await?;
    let manifest: Manifest = serde_json::from_str(&manifest_content)?;
    Ok(manifest)
}

const MANIFEST_FILE: &str = "manifests.json";
const KEYGEN_API: &str = "https://api.keygen.sh/";
const KEYGEN_ACCOUNT: &str = "custodian-dev";

#[derive(Debug)]
pub struct Updater {
    pub manifest_path: PathBuf,
    pub install_path: PathBuf,
    bin_path: PathBuf,
    pub global_grit_path: PathBuf,
    pub binaries: HashMap<String, AppManifest>,
    #[cfg(feature = "updater")]
    last_checked_update: Option<NaiveDateTime>,
    pub installation_id: Uuid,
    access_token: Option<String>,
    refresh_token: Option<String>,
}

impl Updater {
    #[tracing::instrument]
    pub async fn from_current_bin() -> Result<Self> {
        let current_bin = std::env::current_exe()?;
        let install_path = current_bin
            .parent()
            .context("Could not get bin path")?
            .parent()
            .context("Could not get install path")?
            .to_path_buf();
        let updater = Updater::from_install_path(install_path).await?;
        Ok(updater)
    }

    async fn from_install_path(install_path: PathBuf) -> Result<Self> {
        let manifest_path = install_path.join(MANIFEST_FILE);

        if let Ok(manifest) = read_manifest(&manifest_path).await {
            return Ok(Self {
                manifest_path,
                bin_path: install_path.join("bin"),
                global_grit_path: install_path.join(REPO_CONFIG_DIR_NAME),
                install_path,
                binaries: manifest.binaries,
                #[cfg(feature = "updater")]
                last_checked_update: manifest.last_checked_update,
                installation_id: manifest.installation_id.unwrap_or_else(Uuid::new_v4),
                access_token: manifest.access_token,
                refresh_token: manifest.refresh_token,
            });
        }

        let bin_path = install_path.join("bin");
        // Make sure it exists
        async_fs::create_dir_all(&bin_path).await?;
        let global_grit_path = install_path.join(REPO_CONFIG_DIR_NAME);
        let updater = Self {
            manifest_path: install_path.join(MANIFEST_FILE),
            install_path,
            bin_path,
            global_grit_path,
            binaries: HashMap::new(),
            #[cfg(feature = "updater")]
            last_checked_update: None,
            installation_id: Uuid::new_v4(),
            access_token: None,
            refresh_token: None,
        };
        Ok(updater)
    }

    // Apps to check for updates
    pub fn get_apps() -> Vec<SupportedApp> {
        vec![
            SupportedApp::Marzano,
            SupportedApp::Gouda,
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner,
        ]
    }

    async fn _from_manifest(manifest_path: PathBuf) -> Result<Self> {
        match manifest_path.parent() {
            Some(path) => Updater::from_install_path(path.into()).await,
            None => bail!(
                "Could not get install path as parent of manifest: {}",
                manifest_path.display()
            ),
        }
    }

    pub async fn check_for_update(&mut self) -> Result<bool> {
        if self.binaries.is_empty() {
            return Ok(false);
        }

        let now = chrono::Utc::now();
        #[cfg(feature = "updater")]
        {
            if let Some(last_checked_update) = self.last_checked_update {
                let utc_now = now.naive_utc();
                let time_since_last_checked_update = utc_now - last_checked_update;
                if time_since_last_checked_update.num_hours() < 24 {
                    return Ok(false);
                }
            }

            self.last_checked_update = Some(chrono::Utc::now().naive_utc());
        }
        self.dump().await?;

        for binary in self.binaries.values() {
            let version = match binary.version.as_ref() {
                Some(version) => version,
                None => {
                    continue;
                }
            };
            let epoch_timestamp = version.split('.').last().unwrap();
            let timestamp = epoch_timestamp.parse::<i64>()? * 1000;
            let millis_now = now.timestamp_millis();
            let time_since_last_update = millis_now - timestamp;
            if time_since_last_update < 24 * 60 * 60 * 1000 {
                return Ok(false);
            }
        }

        let found_updates = Arc::new(Mutex::new(vec![]));

        // Start tasks for each app
        let mut tasks = vec![];
        for app in Self::get_apps() {
            let found_updates = found_updates.clone();
            let app_string = app.get_base_name();
            let current_binary = self.binaries.get(&app_string).cloned();
            tasks.push(tokio::spawn(async move {
                {
                    let version = match check_release(app, &current_binary).await {
                        Ok(v) => v,
                        Err(_) => return,
                    };

                    if let Some(version) = version {
                        let mut found = found_updates.lock().await;
                        found.push((app, version));
                    }
                }
            }));
        }

        for task in tasks {
            let _ = task.await;
        }
        let found_updates = found_updates.lock().await;
        if !found_updates.is_empty() {
            let alert = "Updates available for the following binaries:\n".blue();
            info!("{}", alert);
            for (app, version) in &*found_updates {
                let prompt = format!("{}: {}", app.get_base_name(), version).blue();
                info!("{}", prompt);
            }
            let update_prompt = "\nRun grit install --update to update.\n\n".blue();
            info!("{}", update_prompt);
        }
        Ok(!found_updates.is_empty())
    }

    pub async fn install_latest_axo(&mut self, app: SupportedApp) -> Result<()> {
        let mut updater = AxoUpdater::new_for(&app.get_base_name());
        // Disable axo installers' verbose output with info on
        // where the tool is installed
        updater.disable_installer_output();

        // Set "always update" to match install_latest_internal,
        // and because this is preceded by an "is this outdated" check.
        updater.always_update(true);

        // This can be autodetected if grit was itself installed via an axo
        // installer in the past, but specifying this source manually is
        // necessary if no cargo-dist-style install receipt exists.
        updater.set_release_source(ReleaseSource {
            release_type: ReleaseSourceType::GitHub,
            owner: "getgrit".to_owned(),
            name: "gritql".to_owned(),
            app_name: app.get_base_name(),
        });
        updater.configure_version_specifier(axoupdater::UpdateRequest::LatestMaybePrerelease);
        // add bin/ since axoupdater wants to know where bins go
        updater.set_install_dir(&self.install_path.join("bin").to_string_lossy());
        match updater.run().await {
            Ok(result) => {
                if let Some(outcome) = result {
                    self.set_app_version(
                        app,
                        outcome.new_version.to_string(),
                        outcome.new_version_tag,
                    )?;
                    self.dump().await?;
                // This path is primarily hit if no releases exist, or
                // if `always_update(false)` is set and there isn't a newer
                // version available.
                } else {
                    info!("New version of {app} not installed");
                }
            }
            Err(e) => {
                return Err(e.into());
            }
        }

        Ok(())
    }

    pub async fn install_latest_internal(
        &mut self,
        app: SupportedApp,
        os: Option<&str>,
        arch: Option<&str>,
    ) -> Result<()> {
        // Look for updates
        let (download_url, info_url) = get_release_url(app, os, arch).await?;

        info!("Starting download");
        // Download the artifact
        let downloader = self.download_artifact(app, download_url);
        let manifest_fetcher = fetch_manifest(&info_url, app);
        let (downloaded, manifest) = tokio::try_join!(downloader, manifest_fetcher)?;

        // Unzip the artifact
        self.unpack_artifact(app, downloaded.to_owned()).await?;

        // Clean up the temp artifact
        async_fs::remove_file(&downloaded).await?;

        self.set_app_version(app, manifest.version.unwrap(), manifest.release.unwrap())?;
        self.dump().await?;

        Ok(())
    }

    pub async fn install_latest(
        &mut self,
        app: SupportedApp,
        os: Option<&str>,
        arch: Option<&str>,
    ) -> Result<()> {
        match app {
            SupportedApp::Marzano | SupportedApp::Gouda => self.install_latest_axo(app).await,
            _ => self.install_latest_internal(app, os, arch).await,
        }
    }

    pub fn get_context(&self) -> Result<ExecutionContext> {
        let auth = self.get_auth();
        let context = if let Some(auth) = auth {
            if !auth.is_expired()? {
                let api = LanguageModelAPI {
                    base_endpoint: get_grit_api_url(),
                    bearer_token: auth.access_token,
                    can_cache: true,
                };
                ExecutionContext::default().with_llm_api(api)
            } else {
                ExecutionContext::default()
            }
        } else {
            ExecutionContext::default()
        };
        Ok(context)
    }

    /// Dump the manifest to the manifest file
    pub async fn dump(&self) -> Result<()> {
        let mut manifest_file = File::create(&self.manifest_path).await?;
        let manifest = Manifest {
            binaries: self.binaries.clone(),
            #[cfg(feature = "updater")]
            last_checked_update: self.last_checked_update,
            installation_id: Some(self.installation_id),
            access_token: self.access_token.clone(),
            refresh_token: self.refresh_token.clone(),
        };
        let manifest_string = serde_json::to_string_pretty(&manifest)?;
        manifest_file.write_all(manifest_string.as_bytes()).await?;
        Ok(())
    }

    /// Save a new auth token to the manifest
    pub async fn save_token(&mut self, auth: &AuthInfo) -> Result<()> {
        self.access_token = Some(auth.access_token.clone());
        if auth.refresh_token.is_some() {
            self.refresh_token = auth.refresh_token.clone();
        }
        self.dump().await?;
        Ok(())
    }

    /// Delete the auth token from the manifest, if present
    pub async fn delete_token(&mut self) -> Result<()> {
        if self.access_token.is_none() {
            bail!("You are not authenticated.");
        }
        self.access_token = None;
        self.refresh_token = None;
        self.dump().await?;
        Ok(())
    }

    /// Retrieve auth info from the manifest, if available
    pub fn get_auth(&self) -> Option<AuthInfo> {
        let auth = get_env_auth(false);
        if let Some(auth) = auth {
            return Some(auth);
        }
        if let Some(token) = &self.access_token {
            let mut info = AuthInfo::new(token.to_string());
            if let Some(refresh_token) = &self.refresh_token {
                info.refresh_token = Some(refresh_token.to_string());
            }
            return Some(info);
        }
        None
    }

    pub async fn refresh_auth(&mut self) -> Result<AuthInfo> {
        let Some(auth) = self.get_auth() else {
            bail!("Not authenticated");
        };

        let pg = ProgressBar::new_spinner();
        pg.set_message("Refreshing auth...");
        let refreshed_auth = marzano_auth::auth0::refresh_token(&auth).await?;
        self.save_token(&refreshed_auth).await?;

        pg.finish_and_clear();
        Ok(refreshed_auth)
    }

    /// Get a valid auth token, refreshing if necessary
    pub async fn get_valid_auth(&mut self) -> Result<AuthInfo> {
        let auth = self.get_auth();
        let Some(auth) = auth else {
            bail!("Not authenticated, please run `grit auth login` to authenticate.");
        };
        if auth.is_expired()? {
            let refreshed = self.refresh_auth().await?;
            return Ok(refreshed);
        }
        Ok(auth)
    }

    async fn download_artifact(&self, app: SupportedApp, artifact_url: String) -> Result<PathBuf> {
        let target_path = self.bin_path.join(format!("{}-temp", app.get_bin_name()));

        match reqwest::get(&artifact_url).await {
            Ok(response) => {
                let mut file = async_fs::File::create(&target_path).await?;
                let mut bytes_stream = response.bytes_stream();
                while let Some(chunk) = bytes_stream.next().await {
                    tokio::io::copy(&mut chunk?.as_ref(), &mut file).await?;
                }
            }
            Err(e) => {
                bail!("Failed to download artifact: {:?}", e);
            }
        }

        Ok(target_path)
    }

    async fn unpack_artifact(&self, app: SupportedApp, packed_path: PathBuf) -> Result<()> {
        let unpacked_dir = self.bin_path.join(format!("{}-bin", app.get_bin_name()));
        // Create the subdir
        async_fs::create_dir_all(&unpacked_dir).await?;

        info!(
            "Unpacking from {} to {}",
            packed_path.display(),
            unpacked_dir.display()
        );

        let output = AsyncCommand::new("tar")
            .arg("-xzf")
            .arg(packed_path)
            .arg("-C")
            .arg(&unpacked_dir)
            .output()
            .await?;

        if !output.status.success() {
            bail!("Failed to unpack files: {:?}", output);
        }

        let target_path = self.get_app_bin(&app)?;
        if async_fs::rename(unpacked_dir.join(app.get_bin_name()), &target_path)
            .await
            .is_err()
        {
            if let Err(e) =
                async_fs::rename(unpacked_dir.join(app.get_fallback_bin_name()), &target_path).await
            {
                bail!("Failed to move files: {:?}", e);
            }
        }

        // Make the file executable
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            let target_file = fs_err::File::open(&target_path)?;
            let mut perms = target_file.metadata()?.permissions();
            perms.set_mode(0o744);
            if let Err(e) = target_file.set_permissions(perms) {
                bail!(
                    "Failed to make {} executable: {:?}",
                    target_path.display(),
                    e
                );
            }

            info!("Successfully made {} executable", target_path.display());
        }

        async_fs::remove_dir_all(&unpacked_dir).await?;

        Ok(())
    }

    fn _get_app_manifest(&self, app: SupportedApp) -> Result<&AppManifest> {
        let app_string = app.get_base_name();
        let app_manifest = self
            .binaries
            .get(app_string.as_str())
            .context(format!("Could not find manifest for app: {}", app_string))?;
        Ok(app_manifest)
    }

    fn _get_app_version(&self, app_name: SupportedApp) -> Result<String> {
        let app_manifest = self._get_app_manifest(app_name)?;
        let version = app_manifest
            .version
            .as_ref()
            .context("Missing version string")?;
        Ok(version.to_string())
    }

    fn get_env_bin(&self, app_name: &SupportedApp) -> Result<Option<PathBuf>> {
        let env_name = app_name.get_env_name();
        let bin_path = std::env::var(env_name).ok();
        if let Some(bin_path) = bin_path {
            let bin_path = PathBuf::from(bin_path);
            return Ok(Some(bin_path));
        }
        Ok(None)
    }

    fn get_app_bin(&self, app_name: &SupportedApp) -> Result<PathBuf> {
        if let Some(bin_path) = self.get_env_bin(app_name)? {
            return Ok(bin_path);
        }
        let bin_name = app_name.get_base_name();
        let bin_path = self.bin_path.join(bin_name);
        #[cfg(windows)]
        let bin_path = bin_path.with_extension("exe");
        Ok(bin_path)
    }

    pub fn is_app_installed(&self, app_name: SupportedApp) -> Result<bool> {
        let bin_path = self.get_app_bin(&app_name)?;
        Ok(bin_path.exists())
    }

    pub async fn get_app_bin_and_install(&mut self, app: SupportedApp) -> Result<PathBuf> {
        // If the path is overridden, skip checking install
        if let Some(bin_path) = self.get_env_bin(&app)? {
            return Ok(bin_path);
        }
        let bin_path = self.get_app_bin(&app)?;
        if bin_path.exists() {
            return Ok(bin_path);
        }
        bail!(
            "Please set the {} environment variable to the path of the {} binary",
            app.get_env_name(),
            app
        );
    }

    pub async fn sync_manifest_version(&mut self, app: SupportedApp) -> Result<Option<String>> {
        let app_string = app.get_base_name();
        let release = match self.binaries.get(&app_string) {
            Some(app_manifest) => app_manifest.release.clone(),
            None => None,
        };
        if release.is_none() {
            return Ok(None);
        }
        let info_url = release_details_relative_url(release.as_ref().unwrap());
        let manifest = fetch_manifest(&info_url, app).await?;
        if manifest.version.is_none() || manifest.release.is_none() {
            return Ok(None);
        }
        let version = manifest.version.clone();
        self.set_app_version(app, manifest.version.unwrap(), manifest.release.unwrap())?;
        self.dump().await?;
        Ok(version)
    }

    fn set_app_version(
        &mut self,
        app: SupportedApp,
        version: String,
        release: String,
    ) -> Result<()> {
        let app_string = app.get_base_name();
        let app_manifest = AppManifest {
            name: AllApp::from_supported_app(app),
            release: Some(release),
            version: Some(version),
        };
        self.binaries.insert(app_string, app_manifest);
        Ok(())
    }

    /// Get the release date of the app, based on the release ID
    fn _get_app_release_date(&self, app_name: SupportedApp) -> Result<DateTime<Utc>> {
        let app_manifest = self._get_app_manifest(app_name)?;
        let version_string = app_manifest
            .version
            .as_ref()
            .context("Missing version string")?
            .split('.');
        // The date is the last part
        let timestamp = version_string.last().context("Missing timestamp")?;
        // Convert the unix timestamp to a date
        let date = DateTime::from_timestamp_millis(timestamp.parse::<i64>()?)
            .context("Could not parse timestamp")?;
        Ok(date)
    }
}

async fn get_release_url(
    app_name: SupportedApp,
    os: Option<&str>,
    arch: Option<&str>,
) -> Result<(String, String)> {
    let client = reqwest::Client::builder()
        .redirect(Policy::none())
        .build()?;

    let filename = app_name.get_file_name(
        os.unwrap_or(get_client_os()),
        arch.unwrap_or(get_client_arch()),
    );

    let url = format!(
        "{}/v1/accounts/{}/artifacts/{}",
        KEYGEN_API, KEYGEN_ACCOUNT, filename
    );
    info!("Fetching release URL from: {}", url);
    let res = client.get(&url).send().await?.text().await?;

    // Parse as JSON
    let json_data: serde_json::Value = serde_json::from_str(&res)?;

    let latest_release_download_url = if let Some(artifact_data) = json_data["data"]
        .get("links")
        .and_then(|links| links.get("redirect"))
    {
        let artifact_url = artifact_data
            .as_str()
            .expect("Download URL should be a string");
        artifact_url
    } else {
        bail!("Could not find artifact download URL");
    };

    let latest_release_info_url = if let Some(artifact_data) = json_data["data"]
        .get("relationships")
        .and_then(|relationships| relationships.get("release"))
        .and_then(|release| release.get("links"))
        .and_then(|links| links.get("related"))
    {
        let artifact_url = artifact_data
            .as_str()
            .expect("Release info URL should be a string");
        artifact_url
    } else {
        bail!("Could not find release info URL");
    };

    Ok((
        latest_release_download_url.to_string(),
        latest_release_info_url.to_string(),
    ))
}

pub async fn check_release(
    app: SupportedApp,
    current_binary: &Option<AppManifest>,
) -> Result<Option<String>> {
    match app {
        SupportedApp::Marzano | SupportedApp::Gouda => check_release_axo(app, current_binary).await,
        _ => check_release_internal(app, current_binary).await,
    }
}

pub async fn check_release_internal(
    app: SupportedApp,
    current_binary: &Option<AppManifest>,
) -> Result<Option<String>> {
    let (_, info_url) = match get_release_url(app, None, None).await {
        Ok(urls) => urls,
        Err(_) => return Ok(None),
    };
    let manifest = match fetch_manifest(&info_url, app).await {
        Ok(manifest) => manifest,
        Err(_) => return Ok(None),
    };
    if let Some(current_manifest) = current_binary {
        if manifest.release != current_manifest.release && manifest.release.is_some() {
            Ok(manifest.version)
        } else {
            Ok(None)
        }
    } else {
        Ok(None)
    }
}

pub async fn check_release_axo(
    app: SupportedApp,
    current_binary: &Option<AppManifest>,
) -> Result<Option<String>> {
    let mut updater = AxoUpdater::new_for(&app.get_base_name());
    // Avoids a look up to cargo-dist's install receipt, which may not exist yet;
    // we want to fetch this data from the current manifest if at all possible
    if let Some(current_manifest) = current_binary {
        if let Some(current) = &current_manifest.version {
            updater.set_current_version(Version::parse(current)?)?;
        }
    }
    updater.set_release_source(ReleaseSource {
        release_type: ReleaseSourceType::GitHub,
        owner: "getgrit".to_owned(),
        name: "gritql".to_owned(),
        app_name: app.get_base_name(),
    });
    updater.configure_version_specifier(axoupdater::UpdateRequest::LatestMaybePrerelease);
    let update_needed = updater.is_update_needed().await?;
    if !update_needed {
        return Ok(None);
    }
    let new_version = updater.query_new_version().await?.map(|v| v.to_string());
    Ok(new_version)
}

async fn fetch_manifest(relative_url: &str, app: SupportedApp) -> Result<AppManifest> {
    let client = reqwest::Client::builder().build()?;
    let url = format!("{}{}", KEYGEN_API, relative_url);
    let res = client.get(url).send().await?.text().await?;
    let json_data: serde_json::Value = serde_json::from_str(&res).unwrap();

    let version = if let Some(version) = json_data["data"]
        .get("attributes")
        .and_then(|attributes| attributes.get("version"))
    {
        version.as_str().unwrap().to_string()
    } else {
        bail!("Could not find version");
    };

    let release = if let Some(id) = json_data["data"].get("id") {
        id.as_str().unwrap().to_string()
    } else {
        bail!("Could not find release");
    };

    Ok(AppManifest {
        name: AllApp::from_supported_app(app),
        release: Some(release),
        version: Some(version),
    })
}

fn release_details_relative_url(release: &str) -> String {
    format!("/v1/accounts/{}/releases/{}", KEYGEN_ACCOUNT, release)
}

#[cfg(test)]
mod tests {
    use fs_err::create_dir_all;

    use anyhow::Result;
    use chrono::NaiveDate;
    use tempfile::tempdir;
    use trim_margin::MarginTrimmable;

    use super::*;

    #[tokio::test]
    async fn test_filenames() -> Result<()> {
        let marzano = SupportedApp::Marzano;

        assert_eq!(
            marzano.get_file_name("macos", "arm64"),
            "marzano-macos-arm64"
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_basic_updater() -> Result<()> {
        let temp_dir = tempdir()?;

        let old_manifest = r#"
        | {
        |    "installPath":"/Users/morgante/.nvm/versions/node/v16.19.0/",
        |    "binaries":{
        |        "marzano":{"name":"marzano","version":"0.1.0-alpha.1689744085325","release":"02c4911d-1b38-41e0-b350-57de7b850744"},
        |        "yeast":{"name":"yeast","version":"0.0.1-alpha.1687311119164","release":"744cc867-ae03-497f-b82a-ee6a4a57e90e"},
        |        "engine":{"name":"engine","version":"0.0.1-alpha.1687311092626","release":"1a24e0a9-c118-4522-a797-0a17f514be6c"},
        |        "cli":{"name":"cli","version":"0.15.5-alpha.1689138129000","release":"f401cc02-a183-4058-afb8-0cd81e4035d1"}
        |    }
        | }"#.trim_margin().unwrap();

        // Create the manifest file and write the old manifest to it
        let manifest_path = temp_dir.path().join(MANIFEST_FILE);
        let mut manifest_file = File::create(manifest_path.clone()).await?;
        info!("Wrote manifest to: {}", manifest_path.display());
        manifest_file.write_all(old_manifest.as_bytes()).await?;
        info!("Old manifest: {}", old_manifest);

        // Now open the manifest file from the Updater
        let updater = Updater::_from_manifest(manifest_path).await?;
        assert_eq!(updater.binaries.len(), 4); // there are 4 binaries in the manifest

        // Get the version of the marzano binary
        let marzano_version = updater._get_app_version(SupportedApp::Marzano)?;
        assert_eq!(marzano_version, "0.1.0-alpha.1689744085325");

        // Get the release date of the cli binary
        let marzano_release_date = updater._get_app_release_date(SupportedApp::Marzano)?;
        assert_eq!(
            marzano_release_date,
            DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2023, 7, 19)
                    .unwrap()
                    .and_hms_milli_opt(5, 21, 25, 325)
                    .unwrap(),
                Utc
            ),
        );

        Ok(())
    }

    #[tokio::test]
    async fn test_empty_updater() -> Result<()> {
        let temp_dir = tempdir().unwrap();

        let mut updater = Updater::from_install_path(temp_dir.path().to_path_buf()).await?;

        // Set the marzano version
        updater.set_app_version(
            SupportedApp::Marzano,
            "0.0.1-alpha.1687311119164".to_string(),
            "744cc867-ae03-497f-b82a-ee6a4a57e90e".to_string(),
        )?;

        // Dump it
        updater.dump().await?;

        // Read the manifest file
        let manifest_path = temp_dir.path().join(MANIFEST_FILE);
        let new_updater = Updater::_from_manifest(manifest_path).await?;
        assert_eq!(new_updater.binaries.len(), 1);

        // Get the version of the marzano binary
        let marzano_version = new_updater._get_app_version(SupportedApp::Marzano)?;
        assert_eq!(marzano_version, "0.0.1-alpha.1687311119164");

        Ok(())
    }

    #[tokio::test]
    async fn does_not_indicate_update_when_version_is_unknown() -> Result<()> {
        let app = SupportedApp::Marzano;
        let (_, info_url) = get_release_url(app, None, None).await?;
        let manifest = fetch_manifest(&info_url, app).await?;

        let temp_manifest_path = tempdir().unwrap().path().join(MANIFEST_FILE);
        create_dir_all(temp_manifest_path.parent().unwrap())?;
        let mut manifest_file = File::create(&temp_manifest_path).await?;
        let manifest_string = format!(
            r#"{{
  "binaries": {{
    "marzano": {{
      "name": "marzano",
      "release": "{}",
      "version": null
    }}
  }},
  "installationId": "9a151548-26ee-45bd-a793-8b3d8d7f0f33"
}}"#,
            manifest.release.unwrap()
        );
        manifest_file.write_all(manifest_string.as_bytes()).await?;

        let mut updater = Updater::_from_manifest(temp_manifest_path).await?;
        let has_update = updater.check_for_update().await?;

        assert!(!has_update);

        Ok(())
    }

    #[tokio::test]
    #[ignore = "This test is too platform-specific"]
    async fn test_updates() -> Result<()> {
        let temp_dir = tempdir().unwrap();

        let mut updater = Updater::from_install_path(temp_dir.path().to_path_buf()).await?;

        // Set the existing version
        updater.set_app_version(
            SupportedApp::Marzano,
            "0.0.1-alpha.1687311119164".to_string(),
            "744cc867-ae03-497f-b82a-ee6a4a57e90e".to_string(),
        )?;

        updater
            .install_latest(SupportedApp::Marzano, None, None)
            .await?;

        let manifest = async_fs::read_to_string(updater.manifest_path.clone()).await?;

        let parsed_manifest = serde_json::from_str::<serde_json::Value>(&manifest)?;

        let marzano_name = parsed_manifest["binaries"]["marzano"]["name"]
            .as_str()
            .expect("install_path binaries marzano name should be a string");
        assert_eq!(marzano_name, "marzano");

        Ok(())
    }
}
