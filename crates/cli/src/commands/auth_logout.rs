use anyhow::Result;
use clap::Args;
use log::info;
use serde::Serialize;

use crate::updater::Updater;

#[derive(Args, Debug, Serialize)]
pub struct LogoutArgs {}

pub(crate) async fn run_logout(_arg: LogoutArgs) -> Result<()> {
    let mut updater = Updater::from_current_bin().await?;

    updater.delete_token().await?;
    info!("You are now logged out!");

    Ok(())
}
