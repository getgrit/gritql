use anyhow::{bail, Result};
use clap::Args;
use serde::Serialize;
use std::path::PathBuf;

use crate::flags::GlobalFormatFlags;

use crate::commands::apply_migration::{run_apply_migration, ApplyMigrationArgs};
use crate::workflows::fetch_remote_workflow;
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
    let workflow_info = fetch_remote_workflow(
        "https://storage.googleapis.com/grit-workflows-dev-workflow_definitions/upload_workflow.js",
        None,
    )
    .await?;

    let apply_migration_args = ApplyMigrationArgs {
        input: Some(format!(r#"{{"workflow": "{}"}}"#, workflow_path.display())),
        ..Default::default()
    };

    let execution_id =
        std::env::var("GRIT_EXECUTION_ID").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());

    let result = run_apply_migration(
        workflow_info,
        vec![],
        None,
        apply_migration_args,
        parent,
        VisibilityLevels::default(),
        execution_id,
    )
    .await?;

    if let Some(data) = result.data.and_then(|v| v.get("url").cloned()) {
        if let Some(data_str) = data.as_str() {
            if let Some(last_part) = data_str.split('/').last() {
                println!("Uploaded Workflow ID: {}", last_part);
                return Ok(());
            }
        }
    }

    bail!("Failed to upload workflow: URL not returned")
}
