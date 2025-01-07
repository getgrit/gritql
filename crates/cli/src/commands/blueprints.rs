use crate::commands::apply_migration::{run_apply_migration, ApplyMigrationArgs};
use crate::flags::{GlobalFormatFlags, OutputFormat};
use crate::messenger_variant::create_emitter;
use crate::workflows::fetch_remote_workflow;
use anyhow::{bail, Result};
use clap::{Parser, Subcommand};

use marzano_messenger::emit::VisibilityLevels;
use serde::Serialize;
use serde_json::json;

#[derive(Parser, Debug, Serialize)]
pub struct Blueprints {
    #[structopt(subcommand)]
    pub blueprint_commands: BlueprintCommands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum BlueprintCommands {
    /// List available blueprints
    List(ListArgs),
    /// Pull a blueprint by workflow ID
    Pull(PullArgs),
    /// Push a blueprint by workflow ID
    Push(PushArgs),
}

#[derive(Parser, Debug, Serialize)]
pub struct ListArgs {}

async fn run_blueprint_workflow(
    workflow_name: &str,
    input: Option<serde_json::Value>,
    parent: &GlobalFormatFlags,
) -> Result<()> {
    if parent.json || parent.jsonl {
        bail!("JSON output not supported for blueprints");
    }

    let workflow_info = fetch_remote_workflow(
        &format!("https://storage.googleapis.com/grit-workflows-prod-workflow_definitions/{workflow_name}.js"),
        None,
    ).await?;

    let apply_migration_args = ApplyMigrationArgs {
        input: input.map(|v| v.to_string()),
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

    run_apply_migration(
        workflow_info,
        vec![],
        None,
        apply_migration_args,
        emitter,
        execution_id,
    )
    .await?;

    Ok(())
}

impl ListArgs {
    pub async fn run(&self, parent: &GlobalFormatFlags) -> Result<()> {
        run_blueprint_workflow("blueprints/list", None, parent).await
    }
}

#[derive(Parser, Debug, Serialize)]
pub struct PullArgs {
    /// The workflow ID of the blueprint to pull
    #[clap(long, alias = "id")]
    workflow_id: String,

    /// Force pull even if the blueprint already exists
    #[clap(long, short = 'f')]
    force: bool,

    /// File to save the blueprint to (defaults to blueprint.md)
    #[clap(long, default_value = "blueprint.md")]
    file: String,
}

impl PullArgs {
    pub async fn run(&self, parent: &GlobalFormatFlags) -> Result<()> {
        let input = json!({
            "workflow_id": self.workflow_id,
            "force": self.force,
            "path": self.file,
        });
        run_blueprint_workflow("blueprints/download", Some(input), parent).await
    }
}

#[derive(Parser, Debug, Serialize)]
pub struct PushArgs {
    /// The workflow ID of the blueprint to push
    #[clap(long, alias = "id")]
    workflow_id: String,

    /// File containing the blueprint (defaults to blueprint.md)
    #[clap(long, default_value = "blueprint.md")]
    file: String,
}

impl PushArgs {
    pub async fn run(&self, parent: &GlobalFormatFlags) -> Result<()> {
        let input = json!({
            "workflow_id": self.workflow_id,
            "path": self.file,
        });
        run_blueprint_workflow("blueprints/upload", Some(input), parent).await
    }
}
