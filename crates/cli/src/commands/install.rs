use anyhow::Result;
use clap::Args;
use log::info;
use serde::Serialize;

use crate::updater::{SupportedApp, Updater};

#[derive(Args, Debug, Serialize)]
pub struct InstallArgs {
    /// Look for updates and install them
    #[clap(long = "update")]
    update: bool,
    /// Specify a specific app to install
    #[clap(long = "app")]
    app: Option<SupportedApp>,
}

pub(crate) async fn run_install(arg: InstallArgs) -> Result<()> {
    let should_update = arg.update;
    let mut updater = Updater::from_current_bin().await?;

    info!(
        "Targeting {} as install directory",
        updater.install_path.display()
    );

    if let Some(app) = arg.app {
        match updater.is_app_installed(app)? {
            true => match should_update {
                true => {
                    info!("{} already present, installing latest", app);
                    updater.install_latest(app).await?;
                }
                false => info!("{} already present, skipping", app),
            },
            false => {
                info!("{} not present, installing", app);
                updater.install_latest(app).await?;
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
                    updater.install_latest(app).await?;
                }
                false => info!("{} already present, skipping", app),
            },
            false => {
                if app.is_default_app() {
                    info!("{} not present, installing", app);
                    updater.install_latest(app).await?;
                } else {
                    info!("{app} not present, skipping, run with --app {app} to install",);
                }
            }
        }
    }

    Ok(())
}
