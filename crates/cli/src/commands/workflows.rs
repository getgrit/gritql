use clap::{Parser, Subcommand};
use serde::Serialize;

use super::{workflows_list::WorkflowsListArgs, workflows_view::WorkflowViewArgs};

#[derive(Parser, Debug, Serialize)]
pub struct Workflows {
    #[structopt(subcommand)]
    pub workflows_commands: WorkflowCommands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum WorkflowCommands {
    /// List all available workflows
    List(WorkflowsListArgs),
    /// View logs from a workflow
    View(WorkflowViewArgs),
}
