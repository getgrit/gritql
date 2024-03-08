use anyhow::Result;
use clap::Args;
use console::style;
use log::debug;
use log::info;
use marzano_auth::auth0::start_auth;
use serde::Serialize;

use crate::updater::Updater;

#[derive(Args, Debug, Serialize)]
pub struct LoginArgs {}

pub(crate) async fn run_login(_arg: LoginArgs) -> Result<()> {
    let mut updater = Updater::from_current_bin().await?;

    let mut session = start_auth().await?;

    // Prompt the user to open the URL in their browser
    info!("Authenticating with the Grit API...");
    println!(
        "Your one-time code is: {}\n",
        style(session.user_code()).bold().yellow()
    );
    println!("Please open the following URL in your browser to authenticate:");
    println!("{}", style(session.verify_url()).cyan());

    // Wait for the user to complete the login process
    session.poll().await?;

    updater.save_token(session.token()?).await?;

    println!("You are now logged in!");

    debug!("Token is: {}", session.token()?);

    Ok(())
}
