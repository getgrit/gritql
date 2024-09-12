use anyhow::{bail, Result};
use clap::Args;
use console::style;
use serde::Serialize;
use std::path::PathBuf;

use crate::flags::{GlobalFormatFlags, OutputFormat};

use crate::commands::apply_migration::{run_apply_migration, ApplyMigrationArgs};
use crate::messenger_variant::create_emitter;
use crate::workflows::fetch_remote_workflow;
use marzano_messenger::emit::VisibilityLevels;

#[derive(Args, Debug, Serialize)]
pub struct WorkflowsUploadArgs {
    #[clap(index = 1)]
    pub workflow_path: String,
    #[clap(index = 2)]
    pub workflow_id: String,
}

pub async fn run_upload_workflows(
    _arg: &WorkflowsUploadArgs,
    parent: &GlobalFormatFlags,
) -> Result<String> {
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
        input: Some(format!(
            r#"{{"workflow": "{}", "workflow_id": "{}" }}"#,
            workflow_path.display(),
            _arg.workflow_id
        )),
        ..Default::default()
    };

    let execution_id =
        std::env::var("GRIT_EXECUTION_ID").unwrap_or_else(|_| uuid::Uuid::new_v4().to_string());

    let format = OutputFormat::from(parent);
    let emitter = create_emitter(
        &format,
        marzano_messenger::output_mode::OutputMode::default(),
        None,
        false,
        None,
        None,
        VisibilityLevels::default(),
    )
    .await?;
    let result = run_apply_migration(
        workflow_info,
        vec![],
        None,
        apply_migration_args,
        emitter,
        execution_id,
    )
    .await?;

    if let Some(data) = result.data.and_then(|v| v.get("download").cloned()) {
        if let Some(download_url) = data.as_str() {
            log::info!("Download URL: {}\n", style(download_url).bold().blue());
            return Ok(download_url.to_string());
        }
    }

    bail!("Failed to upload workflow: URL not returned")
}
