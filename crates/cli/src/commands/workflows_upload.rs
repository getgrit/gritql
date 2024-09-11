use anyhow::{bail, Result};
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use crate::flags::GlobalFormatFlags;

use crate::commands::apply_migration::{run_apply_migration, ApplyMigrationArgs};
// use crate::workflows::fetch_remote_workflow;
use marzano_gritmodule::searcher::WorkflowInfo;
use marzano_messenger::emit::VisibilityLevels;

#[derive(Args, Debug, Serialize)]
pub struct WorkflowsUploadArgs {
    #[clap(index = 1)]
    workflow_path: String,
}

pub async fn run_upload_workflows(
    _arg: &WorkflowsUploadArgs,
    parent: &GlobalFormatFlags,
) -> Result<()> {
    if parent.json || parent.jsonl {
        bail!("JSON output not supported for workflows");
    }

    let workflow_path = PathBuf::from(&_arg.workflow_path);
    // let workflow_info = fetch_remote_workflow(
    //     "https://storage.googleapis.com/grit-workflows-dev-workflow_definitions/upload_workflow.js",
    //     None,
    // )
    // .await?;
    // println!("Workflow infosss: {:?}", workflow_info);
    println!("Workflow path: {:?}", workflow_path);

    let apply_migration_args = ApplyMigrationArgs {
        // input: Some(format!(r#"{{"workflow": "{}"}}"#, workflow_path.display())),
        input: Some(format!(r#"{{"query": "{}"}}"#, workflow_path.display())),
        ..Default::default()
    };

    let workflow_info = WorkflowInfo::new(PathBuf::from(
        // "/Users/jfuentes/Projects/leetcode/rewriter/.grit/workflows/upload_workflow.ts",
        "/Users/jfuentes/Projects/leetcode/rewriter/.grit/workflows/test/hello.ts",
    ));

    let result = run_apply_migration(
        workflow_info,
        vec![],
        None,
        apply_migration_args,
        parent,
        VisibilityLevels::default(),
    )
    .await?;
    println!("Result: {:?}", result);
    Ok(())
}
