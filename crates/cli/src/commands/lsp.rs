use anyhow::Result;
use clap::Args;
use marzano_lsp::server::start_language_server;
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
pub struct LspArgs {
    #[clap(long)]
    stdio: bool,
}

pub(crate) async fn run_lsp(_arg: LspArgs) -> Result<()> {
    start_language_server().await?;

    Ok(())
}
