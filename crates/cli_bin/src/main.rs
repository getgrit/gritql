use marzano_cli::commands::run_command_with_tracing;
use marzano_cli::error::GoodError;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run_command_with_tracing().await
}
