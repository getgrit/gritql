use anyhow::{bail, Result};
use clap::Args;
use serde::Serialize;

use crate::{
    flags::GlobalFormatFlags,
    lister::list_applyables,
    resolver::{resolve_from_cwd, Source},
};

#[derive(Args, Debug, Serialize)]
pub struct WorkflowsListArgs {}

pub async fn run_list_workflows(
    _arg: &WorkflowsListArgs,
    parent: &GlobalFormatFlags,
) -> Result<()> {
    if parent.json || parent.jsonl {
        return Err(GritPatternError::new("JSON output not supported for workflows"));
    }
    let (_resolved, curr_repo) = resolve_from_cwd(&Source::All).await?;

    list_applyables(true, true, vec![], None, parent, curr_repo).await
}
