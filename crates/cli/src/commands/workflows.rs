use clap::{Parser, Subcommand};
use serde::Serialize;

use super::{workflows_list::WorkflowsListArgs, workflows_watch::WorkflowWatchArgs, workflows_upload::WorkflowsUploadArgs};

#[derive(Parser, Debug, Serialize)]
pub struct Workflows {
    #[structopt(subcommand)]
    pub workflows_commands: WorkflowCommands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum WorkflowCommands {
    /// List all available workflows
    List(WorkflowsListArgs),
    /// Watch an existing workflow
    #[clap(hide = true)]
    Watch(WorkflowWatchArgs),
    /// Upload a workflow
    Upload(WorkflowsUploadArgs),
}
