use marzano_cli::commands::run_command_with_tracing;

use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    run_command_with_tracing().await
}
