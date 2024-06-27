use marzano_gritmodule::{
    config::{init_config_from_cwd, init_global_grit_modules},
};




use anyhow::{Result};
use clap::Args;


use marzano_gritmodule::{
    fetcher::{CleanFetcherKind},
};
use serde::Serialize;


#[derive(Args, Debug, Serialize)]
pub struct InitArgs {
    /// Update global grit modules
    #[clap(long = "global", default_value = "false")]
    global: bool,
}

pub(crate) async fn run_init(arg: InitArgs) -> Result<()> {
    if arg.global {
        init_global_grit_modules::<CleanFetcherKind>(None).await?;
    } else {
        let cwd = std::env::current_dir()?;
        init_config_from_cwd::<CleanFetcherKind>(cwd, true).await?;
    }

    Ok(())
}
