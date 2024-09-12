use marzano_messenger::emit::FlushableMessenger;

#[cfg(not(feature = "workflows_v2"))]
use anyhow::bail;
use anyhow::Result;
use clap::Args;
use marzano_gritmodule::searcher::WorkflowInfo;
use marzano_messenger::emit::Messager;
use marzano_messenger::workflows::PackagedWorkflowOutcome;

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
    pub(crate) input: Option<String>,
    #[clap(
        long,
        help_heading = "Workflow options",
        help = "Run the workflow remotely on Grit Cloud",
        requires = "workflow_id"
    )]
    pub(crate) remote: bool,
    /// Workflow ID to set, only applicable when running remotely
    #[clap(long, help_heading = "Workflow options", requires = "remote")]
    pub(crate) workflow_id: Option<String>,
    /// Watch the workflow for updates (only applicable when running remotely)
    #[clap(long, help_heading = "Workflow options")]
    pub(crate) watch: bool,
    /// Print verbose output
    #[clap(long)]
    pub(crate) verbose: bool,
}

impl ApplyMigrationArgs {
    /// Extracts the payload from the input if provided, otherwise returns an empty map
    pub fn get_payload(&self) -> Result<serde_json::Map<String, serde_json::Value>> {
        let map = match &self.input {
            Some(i) => serde_json::from_str::<serde_json::Map<String, serde_json::Value>>(i)?,
            None => serde_json::Map::new(),
        };
        Ok(map)
    }
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
    ranges: Option<Vec<marzano_util::diff::FileDiff>>,
    arg: ApplyMigrationArgs,
    mut emitter: crate::messenger_variant::MessengerVariant<'static>,
    execution_id: String,
) -> Result<PackagedWorkflowOutcome> {
    use crate::error::GoodError;

    let input = arg.get_payload()?;

    emitter.start_workflow()?;

    let mut emitter = run_bin_workflow(
        emitter,
        WorkflowInputs {
            execution_id,
            verbose: arg.verbose,
            workflow_entrypoint: workflow.entrypoint().into(),
            paths,
            input,
            ranges,
        },
    )
    .await?;

    emitter.flush().await?;

    // Get the final workflow status from the emitter
    let Some(workflow_status) = emitter.get_workflow_status()? else {
        anyhow::bail!("Final workflow status not found");
    };

    if !workflow_status.success {
        anyhow::bail!(GoodError::new());
    }

    Ok(workflow_status.clone())
}
