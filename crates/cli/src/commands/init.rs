use marzano_gritmodule::fetcher::GritModuleFetcher;
use std::{env, fmt, io::ErrorKind, path::PathBuf, str::FromStr};

use tracing::instrument;

use anyhow::{bail, Context, Result};
use clap::Args;
use colored::Colorize;
use log::info;
use marzano_gritmodule::{
    config::REPO_CONFIG_DIR_NAME,
    fetcher::{CleanFetcherKind, FetcherType, ModuleRepo},
    installer::install_default_stdlib,
    resolver::fetch_modules,
    searcher::{
        find_git_dir_from, find_global_grit_dir, find_global_grit_modules_dir, find_grit_dir_from,
    },
};
use serde::Serialize;
use tokio::{fs, io::AsyncWriteExt};

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
