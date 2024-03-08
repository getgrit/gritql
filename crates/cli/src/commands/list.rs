use anyhow::Result;
use clap::Args;
use marzano_core::pattern::api::EnforcementLevel;
use serde::Serialize;

use crate::{
    flags::GlobalFormatFlags,
    lister::list_applyables,
    resolver::{resolve_from_cwd, Source},
};

#[derive(Args, Debug, Serialize)]
pub struct ListArgs {
    /// List only at or above an enforcement level.
    #[clap(long = "level")]
    pub level: Option<EnforcementLevel>,
    /// List items from a specific source.
    #[clap(long = "source", default_value = "all", value_enum)]
    pub source: Source,
}

pub async fn run_list_all(arg: &ListArgs, parent: &GlobalFormatFlags) -> Result<()> {
    let (resolved, curr_repo) = resolve_from_cwd(&arg.source).await?;
    list_applyables(true, true, resolved, arg.level.clone(), parent, curr_repo).await
}
