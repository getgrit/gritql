use anyhow::Result;
use clap::Args;
use colored::Colorize;
use log::debug;
use log::info;
use marzano_auth::env::get_app_url;
use marzano_auth::env::get_graphql_api_url;
use marzano_auth::env::get_grit_api_url;
use marzano_auth::env::ENV_VAR_GRAPHQL_API_URL;
use marzano_auth::env::ENV_VAR_GRIT_API_URL;
use marzano_auth::env::ENV_VAR_GRIT_APP_URL;
use marzano_gritmodule::config::init_config_from_path;
use marzano_gritmodule::fetcher::KeepFetcherKind;
use marzano_gritmodule::searcher::find_grit_modules_dir;
use serde::Serialize;

use crate::updater::{SupportedApp, Updater};

#[derive(Args, Debug, Serialize)]
pub struct DoctorArgs {}

pub(crate) async fn run_doctor(_arg: DoctorArgs) -> Result<()> {
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;
    info!("{}", "Client environment".bold());
    info!("  OS: {}", target_os.yellow());
    info!("  Architecture: {}", target_arch.yellow());

    let mut updater = Updater::from_current_bin().await?;

    info!("{}", "Configuration".bold());

    let manifest_path = &updater.manifest_path;
    info!(
        "  Expected location: {}",
        format!("{}", manifest_path.display()).underline().yellow()
    );

    let cwd = std::env::current_dir()?;
    let config = init_config_from_path::<KeepFetcherKind>(cwd.clone(), false).await?;
    info!("  Config: {}", format!("{}", config).underline().yellow());

    let mod_dir = find_grit_modules_dir(cwd.clone()).await;
    match mod_dir {
        Ok(mod_dir) => {
            info!(
                "  Existing Grit modules dir: {}",
                format!("{}", mod_dir.display()).underline().yellow()
            );
        }
        Err(e) => {
            info!("  Grit modules dir not found: {}", e);
            let initialized = init_config_from_path::<KeepFetcherKind>(cwd.clone(), false).await?;
            info!("  Initialized config: {}", initialized);
        }
    }

    let configs = vec![
        (get_grit_api_url(), "Grit API URL", ENV_VAR_GRIT_API_URL),
        (
            get_graphql_api_url(),
            "Grit GraphQL API URL",
            ENV_VAR_GRAPHQL_API_URL,
        ),
        (get_app_url(), "Grit App URL", ENV_VAR_GRIT_APP_URL),
    ];
    for (value, name, env_var) in configs {
        info!(
            "  {}: {} (override by setting {})",
            name,
            value.yellow().underline(),
            env_var.blue()
        );
    }

    info!("{}", "Authentication".bold());

    let auth = updater.get_auth();
    match auth {
        Some(auth) => {
            debug!("  Auth token: {}", auth.access_token.to_string().yellow());
            info!(
                "  Auth user id: {}",
                (auth.get_user_id()?).to_string().yellow()
            );

            if let Some(username) = auth.get_user_name()? {
                info!("  Auth user name: {}", username.yellow());
            }

            if auth.is_expired()? {
                info!(
                    "  Auth token expired: {}.",
                    format!("{}", auth.get_expiry()?).red().bold()
                );
            } else {
                info!(
                    "  Auth token expiration: {}.",
                    format!("{}", auth.get_expiry()?).green()
                );
            }
        }
        None => {
            info!(
                "  {}",
                "You are not authenticated."
                    .to_string()
                    .bright_blue()
                    .bold()
            );
        }
    }

    info!("{}", "Installed binaries".bold());

    let existing_manifests = updater.binaries.clone();
    for app in Updater::get_apps() {
        let name = SupportedApp::get_base_name(&app);
        if !existing_manifests.contains_key(&name) {
            info!(
                "  {}: not installed. Run {} to install missing binaries",
                format!("{}", app).bold(),
                "grit install".bold().blue()
            );
            continue;
        }
        let manifest = existing_manifests.get(&name).unwrap();
        if manifest.version.is_none() {
            if let Some(supported) = SupportedApp::from_all_app(manifest.name.clone()) {
                updater.sync_manifest_version(supported).await?;
            }
        }
    }

    for app_manifest in updater.binaries.values() {
        info!(
            "  {}: {} (release {})",
            format!("{}", app_manifest.name).bold(),
            app_manifest
                .version
                .as_ref()
                .unwrap_or(&"unknown".to_string())
                .to_string()
                .green(),
            app_manifest
                .version
                .as_ref()
                .unwrap_or(&"unknown".to_string())
                .to_string()
                .green()
        );
    }

    Ok(())
}
