use anyhow::{bail, Result};
use console::style;
use log::debug;
use marzano_gritmodule::searcher::find_grit_dir_from;
use marzano_messenger::{emit::Messager, workflows::PackagedWorkflowOutcome};
use serde::Serialize;
use serde_json::to_string;
use std::path::PathBuf;
use tempfile::NamedTempFile;

use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

use crate::updater::{SupportedApp, Updater};

// Sync with cli/src/worker.ts
#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct WorkflowSettings {
    pub workflow_id: String,
    pub cwd: PathBuf,
    pub continue_on_failure: bool,
    pub paths: Vec<PathBuf>,
    pub grit_dir: Option<String>,
    pub verbose: bool,
    pub workflow_entrypoint: Option<String>,
    pub payload: Vec<serde_json::Value>,
}

#[derive(Debug)]
pub struct WorkflowInputs {
    // If this is a custom workflow, this will be the path to the entrypoint
    pub workflow_entrypoint: String,
    // Input paths, might include unresolved globs
    pub paths: Vec<PathBuf>,
    // Payload
    pub payload: Vec<serde_json::Value>,
    // Verbose
    pub verbose: bool,
}

pub async fn run_bin_workflow<M>(
    emitter: M,
    arg: WorkflowInputs,
) -> Result<(M, PackagedWorkflowOutcome)>
where
    M: Messager,
{
    let cwd = std::env::current_dir()?;

    let workflow_id =
        std::env::var("GRIT_EXECUTION_ID").unwrap_or_else(|_| Uuid::new_v4().to_string());
    let marzano_bin = std::env::current_exe()?;

    let mut updater = Updater::from_current_bin().await?;

    let runner_path = updater
        .get_app_bin_and_install(SupportedApp::WorkflowRunner)
        .await?;
    let grit_dir = find_grit_dir_from(cwd.clone()).await;
    let settings = WorkflowSettings {
        workflow_id: workflow_id.clone(),
        cwd,
        continue_on_failure: false,
        paths: arg.paths,
        grit_dir,
        verbose: arg.verbose,
        workflow_entrypoint: Some(arg.workflow_entrypoint),
        payload: arg.payload,
    };

    let tempfile = NamedTempFile::new()?;
    let tempfile_path = tempfile.path().to_owned();

    let serialized = to_string(&settings).expect("Failed to serialize");
    fs::write(&tempfile_path, &serialized).await?;

    debug!(
        "Wrote workflow settings: {}",
        &tempfile_path.to_string_lossy()
    );

    let grit_token = match updater.get_valid_auth() {
        Ok(token) => token.access_token,
        Err(_) => {
            bail!(
                "No valid authentication token found, please run {}",
                style("grit auth login").bold().red()
            );
        }
    };

    let mut child = Command::new(runner_path)
        .arg(tempfile_path.to_string_lossy().to_string())
        .env("GRIT_MARZANO_PATH", marzano_bin)
        .env("GRIT_AUTH_TOKEN", grit_token)
        .arg("--file")
        .arg(&tempfile_path)
        .kill_on_drop(true)
        .spawn()
        .expect("Failed to start worker");

    let status = child.wait().await?;

    // TODO: pass along outcome message
    if status.success() {
        Ok((
            emitter,
            PackagedWorkflowOutcome {
                message: Some("Workflow completed successfully".to_string()),
                success: true,
                data: None,
            },
        ))
    } else {
        Ok((
            emitter,
            PackagedWorkflowOutcome {
                message: Some("Workflow failed".to_string()),
                success: false,
                data: None,
            },
        ))
    }
}
