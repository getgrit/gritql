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
use marzano_gritmodule::fetcher::KeepFetcherKind;
use marzano_gritmodule::searcher::find_grit_modules_dir;
use serde::Serialize;

use crate::{
    commands::init::init_config_from_cwd,
    updater::{SupportedApp, Updater},
};

#[derive(Args, Debug, Serialize)]
pub struct DoctorArgs {}

pub(crate) async fn run_doctor(_arg: DoctorArgs) -> Result<()> {
    let target_os = std::env::consts::OS;
    let target_arch = std::env::consts::ARCH;
    info!("Running on {}/{}", target_os, target_arch);

    let mut updater = Updater::from_current_bin().await?;

    let manifest_path = &updater.manifest_path;
    info!("Manifest file expected at {}", manifest_path.display());

    let cwd = std::env::current_dir()?;
    let config = init_config_from_cwd::<KeepFetcherKind>(cwd.clone(), false).await?;
    info!("Config: {}", config);

    let mod_dir = find_grit_modules_dir(cwd.clone()).await;
    match mod_dir {
        Ok(mod_dir) => {
            info!("Existing Grit modules dir: {}", mod_dir.display());
        }
        Err(e) => {
            info!("Grit modules dir not found: {}", e);
            let initialized = init_config_from_cwd::<KeepFetcherKind>(cwd.clone(), false).await?;
            info!("Initialized config: {}", initialized);
        }
    }

    let auth = updater.get_auth();
    match auth {
        Some(auth) => {
            debug!("Auth token: {}", auth.access_token);
            info!("Auth user id: {}", auth.get_user_id()?);

            if auth.is_expired()? {
                info!("Auth token expired: {}.", auth.get_expiry()?);
            } else {
                info!("Auth token expiration: {}.", auth.get_expiry()?);
            }
        }
        None => {
            info!("You are not authenticated.");
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
            "{}: {} (override by setting {})",
            name,
            value.blue().underline(),
            env_var.underline()
        );
    }

    let existing_manifests = updater.binaries.clone();
    for app in Updater::get_apps() {
        let name = SupportedApp::get_base_name(&app);
        if !existing_manifests.contains_key(&name) {
            let prompt = "Run grit install to install missing binaries.".blue();
            info!("{}: not installed. {}", app, prompt);
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
            "{}: {} (release {})",
            app_manifest.name,
            app_manifest
                .version
                .as_ref()
                .unwrap_or(&"unknown".to_string()),
            app_manifest
                .version
                .as_ref()
                .unwrap_or(&"unknown".to_string())
        );
    }

    Ok(())
}
