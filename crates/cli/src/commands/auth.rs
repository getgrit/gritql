use clap::{Parser, Subcommand};
use serde::Serialize;

use super::auth_login::LoginArgs;
use super::auth_logout::LogoutArgs;
use super::auth_refresh::RefreshAuthArgs;
use super::auth_token::GetTokenArgs;

#[derive(Parser, Debug, Serialize)]
pub struct Auth {
    #[structopt(subcommand)]
    pub auth_commands: AuthCommands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum AuthCommands {
    /// Log in with grit.io
    Login(LoginArgs),
    /// Remove your grit.io credentials
    Logout(LogoutArgs),
    /// Get your grit.io token
    #[clap(aliases = &["print-token"])]
    GetToken(GetTokenArgs),
    /// Refresh your grit.io auth (this will also happen automatically when your token expires)
    Refresh(RefreshAuthArgs),
}
