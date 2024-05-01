use crate::flags::GlobalFormatFlags;
use crate::{flags::OutputFormat, messenger_variant::create_emitter};

#[cfg(not(feature = "workflows_v2"))]
use anyhow::bail;
use anyhow::Result;
use clap::Args;
use marzano_gritmodule::searcher::WorkflowInfo;
use marzano_messenger::emit::Messager;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[cfg(feature = "workflows_v2")]
use crate::workflows::{run_bin_workflow, WorkflowInputs};

#[derive(Args, Default, Debug, Serialize)]
pub struct ApplyMigrationArgs {
    #[clap(
        long,
        help_heading = "Workflow options",
        help = "JSON input parameter to pass to the workflow"
    )]
    input: Option<String>,
    #[clap(
        long,
        help_heading = "Workflow options",
        help = "Run the workflow remotely on Grit Cloud"
    )]
    pub(crate) remote: bool,
    /// Print verbose output
    #[clap(long)]
    verbose: bool,
}

#[derive(Serialize, Deserialize)]
pub struct WorkflowSettings {}

#[cfg(not(feature = "workflows_v2"))]
pub(crate) async fn run_apply_migration(
    workflow: WorkflowInfo,
    _paths: Vec<PathBuf>,
    _arg: ApplyMigrationArgs,
) -> Result<()> {
    bail!("Packaged workflows are currently not available through the CLI. You can run this migration through the Grit App at https://app.grit.io.");
}

#[cfg(feature = "workflows_v2")]
pub(crate) async fn run_apply_migration(
    workflow: WorkflowInfo,
    paths: Vec<PathBuf>,
    arg: ApplyMigrationArgs,
    flags: &GlobalFormatFlags,
) -> Result<()> {
    let input = match &arg.input {
        Some(i) => serde_json::from_str::<serde_json::Value>(i)?,
        None => serde_json::json!({}),
    };

    let format = OutputFormat::from(flags);
    let mut emitter = create_emitter(
        &format,
        marzano_messenger::output_mode::OutputMode::default(),
        None,
        false,
        None,
        None,
    )
    .await?;

    emitter.start_workflow()?;

    let (mut emitter, outcome) = run_bin_workflow(
        emitter,
        WorkflowInputs {
            verbose: arg.verbose,
            workflow_entrypoint: workflow.entrypoint().into(),
            paths,
            payload: vec![serde_json::to_value(input)?],
        },
    )
    .await?;

    emitter.finish_workflow(&outcome)?;
    emitter.flush().await?;

    match outcome.success {
        true => {
            log::info!(
                "{}",
                outcome
                    .message
                    .unwrap_or("Workflow completed successfully".to_string())
            );
            Ok(())
        }
        false => anyhow::bail!(outcome.message.unwrap_or("Workflow failed".to_string())),
    }
}
