use anyhow::bail;
use anyhow::Result;
use clap::Args;
use log::info;
use serde::Serialize;

use crate::updater::Updater;

#[derive(Args, Debug, Serialize)]
pub struct GetTokenArgs {}

pub(crate) async fn run_get_token(_arg: GetTokenArgs) -> Result<()> {
    let updater = Updater::from_current_bin().await?;

    let auth = updater.get_auth();
    match auth {
        Some(auth) => {
            if auth.is_expired()? {
                bail!(
                    "Auth token expired: {}. Run grit auth login to refresh.",
                    auth.get_expiry()?
                );
            }
            info!("{}", auth.access_token);
        }
        None => {
            bail!("You are not authenticated. Run grit auth login to authenticate.");
        }
    }

    Ok(())
}
