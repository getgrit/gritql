use anyhow::Result;
use clap::Args;
use colored::Colorize;
use log::info;
use serde::Serialize;

use crate::updater::Updater;

#[derive(Args, Debug, Serialize)]
pub struct RefreshAuthArgs {}

pub(crate) async fn run_refresh_auth(_arg: RefreshAuthArgs) -> Result<()> {
    let mut updater = Updater::from_current_bin().await?;

    let auth = updater.refresh_auth().await?;

    if let Some(username) = auth.get_user_name()? {
        info!(
            "Hello {}, your token has been refreshed.",
            username.yellow()
        );
    } else {
        info!("Hello, your token has been refreshed.");
    }

    Ok(())
}
