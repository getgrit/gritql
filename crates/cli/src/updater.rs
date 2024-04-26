use anyhow::bail;
use anyhow::{Context, Result};
use chrono::{DateTime, NaiveDateTime, Utc};
use colored::Colorize;
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
use marzano_auth::env::{get_grit_api_url, ENV_VAR_GRIT_AUTH_TOKEN};

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
            SupportedApp::Cli => AllApp::Cli,
            SupportedApp::Timekeeper => AllApp::Timekeeper,
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => AllApp::WorkflowRunner,
            SupportedApp::Gouda => AllApp::Gouda,
        }
    }
}

// Allowed modern apps
#[derive(Debug, Clone, Copy, Serialize, Deserialize, clap::ValueEnum)]
pub enum SupportedApp {
    Timekeeper,
    Marzano,
    Cli,
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
            SupportedApp::Timekeeper => "timekeeper".to_string(),
            SupportedApp::Marzano => "marzano".to_string(),
            SupportedApp::Cli => "cli".to_string(),
            SupportedApp::Gouda => "gouda".to_string(),
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => "workflow_runner".to_string(),
        }
    }

    fn get_env_name(&self) -> String {
        match self {
            SupportedApp::Timekeeper => "GRIT_TIMEKEEPER_PATH".to_string(),
            SupportedApp::Marzano => "GRIT_MARZANO_PATH".to_string(),
            SupportedApp::Cli => "GRIT_CLI_PATH".to_string(),
            SupportedApp::Gouda => "GRIT_GOUDA_PATH".to_string(),
            #[cfg(feature = "workflows_v2")]
            SupportedApp::WorkflowRunner => "GRIT_WORKFLOW_RUNNER".to_string(),
        }
    }

    fn get_bin_name(&self) -> String {
        match self {
            SupportedApp::Timekeeper => "temporalite".to_string(),
            _ => format!("{}-{}", self.get_base_name(), get_client_os()),
        }
    }

    fn get_fallback_bin_name(&self) -> String {
        match self {
            SupportedApp::Timekeeper => "temporalite".to_string(),
            _ => self.get_base_name().to_string(),
        }
    }

    fn get_file_name(&self, os: &str, arch: &str) -> String {
        let base_name = self.get_base_name();
        format!("{}-{}-{}", base_name, os, arch)
    }

    pub fn from_all_app(app: AllApp) -> Option<Self> {
        match app {
            AllApp::Marzano => Some(SupportedApp::Marzano),
            AllApp::Cli => Some(SupportedApp::Cli),
            AllApp::Timekeeper => Some(SupportedApp::Timekeeper),
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
}

#[allow(dead_code)]
impl Updater {
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
        };
        Ok(updater)
    }

    // Apps to check for updates
    pub fn get_apps() -> Vec<SupportedApp> {
        vec![
            SupportedApp::Marzano,
            SupportedApp::Cli,
            SupportedApp::Timekeeper,
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

    pub fn get_log_file(&self, app: SupportedApp) -> Result<std::fs::File> {
        let log_path = self
            .bin_path
            .parent()
            .unwrap()
            .join(format!("{}.log", app.get_bin_name()));
        let log_file = std::fs::File::create(log_path).unwrap();
        Ok(log_file)
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
                let (_, info_url) = match get_release_url(app, None, None).await {
                    Ok(urls) => urls,
                    Err(_) => return,
                };
                let manifest = match fetch_manifest(&info_url, app).await {
                    Ok(manifest) => manifest,
                    Err(_) => return,
                };
                if let Some(current_manifest) = current_binary {
                    if manifest.release != current_manifest.release && manifest.release.is_some() {
                        let mut found = found_updates.lock().await;
                        found.push((app, manifest.version.unwrap()));
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

    pub async fn install_latest(
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
        self.unpack_artifact(app, downloaded).await?;

        self.set_app_version(app, manifest.version.unwrap(), manifest.release.unwrap())?;
        self.dump().await?;

        Ok(())
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
        };
        let manifest_string = serde_json::to_string_pretty(&manifest)?;
        manifest_file.write_all(manifest_string.as_bytes()).await?;
        Ok(())
    }

    /// Save a new auth token to the manifest
    pub async fn save_token(&mut self, token: &str) -> Result<()> {
        self.access_token = Some(token.to_string());
        self.dump().await?;
        Ok(())
    }

    /// Delete the auth token from the manifest, if present
    pub async fn delete_token(&mut self) -> Result<()> {
        if self.access_token.is_none() {
            bail!("You are not authenticated.");
        }
        self.access_token = None;
        self.dump().await?;
        Ok(())
    }

    /// Retrieve auth info from the manifest, if available
    pub fn get_auth(&self) -> Option<AuthInfo> {
        let env_token = std::env::var(ENV_VAR_GRIT_AUTH_TOKEN).ok();
        if let Some(token) = env_token {
            return Some(AuthInfo::new(token.to_string()));
        }
        if let Some(token) = &self.access_token {
            return Some(AuthInfo::new(token.to_string()));
        }
        None
    }

    pub fn get_valid_auth(&self) -> Result<AuthInfo> {
        let auth = self.get_auth();
        if let Some(auth) = auth {
            if auth.is_expired()? {
                bail!("Auth token expired");
            }
            return Ok(auth);
        }
        bail!("Not authenticated");
    }

    async fn download_artifact(&self, app: SupportedApp, artifact_url: String) -> Result<PathBuf> {
        let target_path = self.bin_path.join(format!("{}-temp", app.get_bin_name()));

        match reqwest::get(&artifact_url).await {
            Ok(response) => {
                let contents = response.bytes().await?.to_vec();
                async_fs::write(&target_path, contents).await?;
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

            let target_file = std::fs::File::open(&target_path)?;
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
        Ok(bin_path)
    }

    pub fn is_app_installed(&self, app_name: SupportedApp) -> Result<bool> {
        let bin_path = self.get_app_bin(&app_name)?;
        Ok(bin_path.exists())
    }

    pub async fn get_app_bin_and_install(&mut self, app: SupportedApp) -> Result<PathBuf> {
        // If the path is overridden, skip checking install
        if let Some(bin_path) = self.get_env_bin(&app)? {
            info!("Using {} from: {}", app, bin_path.display());
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
    use std::fs::create_dir_all;

    use anyhow::Result;
    use chrono::NaiveDate;
    use tempfile::tempdir;
    use trim_margin::MarginTrimmable;

    use super::*;

    #[tokio::test]
    async fn test_filenames() -> Result<()> {
        let marzano = SupportedApp::Marzano;
        let cli = SupportedApp::Cli;

        assert_eq!(
            marzano.get_file_name("macos", "arm64"),
            "marzano-macos-arm64"
        );
        assert_eq!(cli.get_file_name("macos", "arm64"), "cli-macos-arm64");

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
        let cli_release_date = updater._get_app_release_date(SupportedApp::Cli)?;
        assert_eq!(
            cli_release_date,
            DateTime::<Utc>::from_naive_utc_and_offset(
                NaiveDate::from_ymd_opt(2023, 7, 12)
                    .unwrap()
                    .and_hms_opt(5, 2, 9)
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
