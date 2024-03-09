use anyhow::Result;
use clap::Args;
use log::info;
use serde::Serialize;

use crate::{
    updater::{SupportedApp, Updater},
    utils::{get_client_arch, get_client_os, Architecture, OperatingSystem},
};

#[derive(Args, Debug, Serialize)]
pub struct InstallArgs {
    /// Look for updates and install them
    #[clap(long = "update")]
    update: bool,
    /// Specify a specific app to install
    #[clap(long = "app")]
    app: Option<SupportedApp>,
    /// Override the architecture to install
    #[clap(long = "arch", hide = true)]
    arch: Option<Architecture>,
    /// Override the OS to install for
    #[clap(long = "os", hide = true)]
    os: Option<OperatingSystem>,
}

pub(crate) async fn run_install(arg: InstallArgs) -> Result<()> {
    let should_update = arg.update;
    let mut updater = Updater::from_current_bin().await?;

    let arch = arg
        .arch
        .map_or_else(|| get_client_arch().to_string(), |arch| arch.to_string());
    let os = arg
        .os
        .map_or_else(|| get_client_os().to_string(), |os| format!("{}", &os));

    info!(
        "Targeting {} as install directory",
        updater.install_path.display()
    );

    if let Some(app) = arg.app {
        match updater.is_app_installed(app)? {
            true => match should_update {
                true => {
                    info!("{} already present, installing latest", app);
                    updater.install_latest(app, Some(&os), Some(&arch)).await?;
                }
                false => info!("{} already present, skipping", app),
            },
            false => {
                info!("{} not present, installing", app);
                updater.install_latest(app, Some(&os), Some(&arch)).await?;
            }
        }
        // TODO: output *only* the installed binary path to stdout
        return Ok(());
    }

    for app in Updater::get_apps() {
        match updater.is_app_installed(app)? {
            true => match should_update {
                true => {
                    info!("{} already present, installing latest", app);
                    updater.install_latest(app, Some(&os), Some(&arch)).await?;
                }
                false => info!("{} already present, skipping", app),
            },
            false => {
                if app.is_default_app() {
                    info!("{} not present, installing", app);
                    updater.install_latest(app, Some(&os), Some(&arch)).await?;
                } else {
                    info!("{app} not present, skipping, run with --app {app} to install",);
                }
            }
        }
    }

    Ok(())
}
