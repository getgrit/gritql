use crate::flags::GlobalFormatFlags;
use crate::{flags::OutputFormat, messenger_variant::create_emitter};
use marzano_messenger::emit::FlushableMessenger;

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
    pub(crate) input: Option<String>,
    #[clap(
        long,
        help_heading = "Workflow options",
        help = "Run the workflow remotely on Grit Cloud"
    )]
    pub(crate) remote: bool,
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
    flags: &GlobalFormatFlags,
    min_level: marzano_messenger::emit::VisibilityLevels,
) -> Result<()> {
    use crate::error::GoodError;

    let input = arg.get_payload()?;

    let format = OutputFormat::from(flags);
    let mut emitter = create_emitter(
        &format,
        marzano_messenger::output_mode::OutputMode::default(),
        None,
        false,
        None,
        None,
        min_level,
    )
    .await?;

    emitter.start_workflow()?;

    let mut emitter = run_bin_workflow(
        emitter,
        WorkflowInputs {
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
    if let Some(workflow_status) = emitter.get_workflow_status()? {
        if !workflow_status.success {
            anyhow::bail!(GoodError::new());
        }
    } else {
        anyhow::bail!("Final workflow status not found");
    }

    Ok(())
}
