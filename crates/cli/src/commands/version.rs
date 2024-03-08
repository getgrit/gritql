use anyhow::Result;
use clap::Args;
use log::info;
use serde::Serialize;

use crate::updater::Updater;

#[derive(Args, Debug, Serialize)]
pub struct VersionArgs {}

pub(crate) async fn run_version(_arg: VersionArgs) -> Result<()> {
    let updater = Updater::from_current_bin().await?;

    for app_manifest in updater.binaries.into_values() {
        info!(
            "{}: {} (release {})",
            app_manifest.name,
            app_manifest.version.unwrap_or("unknown".to_string()),
            app_manifest.release.unwrap_or("unknown".to_string())
        );
    }

    Ok(())
}
