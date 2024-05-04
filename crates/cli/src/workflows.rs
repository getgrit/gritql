use anyhow::{bail, Result};
use console::style;
use grit_util::FileRange;
use log::debug;
use marzano_auth::env::ENV_VAR_GRIT_AUTH_TOKEN;
use marzano_gritmodule::{fetcher::LocalRepo, searcher::find_grit_dir_from};
use marzano_messenger::{emit::Messager, workflows::PackagedWorkflowOutcome};
use serde::Serialize;
use serde_json::to_string;
use std::path::PathBuf;
use tempfile::NamedTempFile;

use tokio::fs;
use tokio::process::Command;
use uuid::Uuid;

use crate::updater::{SupportedApp, Updater};

pub static GRIT_REPO_URL_NAME: &str = "grit_repo_url";
pub static GRIT_REPO_BRANCH_NAME: &str = "grit_repo_branch";
pub static GRIT_TARGET_RANGES: &str = "grit_target_ranges";

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
    /// Ranges to target, if any
    pub ranges: Option<Vec<FileRange>>,
    // Input paths, might include unresolved globs
    pub paths: Vec<PathBuf>,
    // Input
    pub input: serde_json::Map<String, serde_json::Value>,
    // Verbose
    pub verbose: bool,
}

pub async fn run_bin_workflow<M>(
    emitter: M,
    mut arg: WorkflowInputs,
) -> Result<(M, PackagedWorkflowOutcome)>
where
    M: Messager,
{
    let cwd = std::env::current_dir()?;

    let workflow_id =
        std::env::var("GRIT_EXECUTION_ID").unwrap_or_else(|_| Uuid::new_v4().to_string());
    let marzano_bin = std::env::current_exe()?;

    let mut updater = Updater::from_current_bin().await?;
    let repo = LocalRepo::from_dir(&cwd).await;

    if let Some(repo) = &repo {
        if !arg.input.contains_key(GRIT_REPO_URL_NAME) {
            if let Some(url) = repo.remote() {
                arg.input.insert(GRIT_REPO_URL_NAME.to_string(), url.into());
            }
        }
        if !arg.input.contains_key(GRIT_REPO_BRANCH_NAME) {
            if let Some(branch) = repo.branch() {
                arg.input
                    .insert(GRIT_REPO_BRANCH_NAME.to_string(), branch.into());
            }
        }
    }

    if let Some(ranges) = arg.ranges {
        arg.input.insert(
            GRIT_TARGET_RANGES.to_string(),
            serde_json::to_value(ranges)?,
        );
    }

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
        payload: vec![serde_json::Value::Object(arg.input)],
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
        .env(ENV_VAR_GRIT_AUTH_TOKEN, grit_token)
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

pub fn display_workflow_outcome(outcome: PackagedWorkflowOutcome) -> Result<()> {
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

#[cfg(feature = "remote_workflows")]
pub async fn run_remote_workflow(workflow_name: String) -> Result<()> {
    use marzano_gritmodule::fetcher::ModuleRepo;
    let mut updater = Updater::from_current_bin().await?;
    let cwd = std::env::current_dir()?;

    let auth = updater.get_valid_auth()?;

    let repo = ModuleRepo::from_dir(&cwd).await;
    let settings = grit_cloud_client::RemoteWorkflowSettings::new(workflow_name, &repo);
    let outcome = grit_cloud_client::run_remote_workflow(settings, &auth).await?;
    display_workflow_outcome(outcome)?;
    return Ok(());
}
