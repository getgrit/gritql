use crate::updater::{SupportedApp, Updater};
use anyhow::Result;
use console::style;
use log::{debug, info};
use marzano_auth::env::{get_grit_api_url, ENV_VAR_GRIT_API_URL, ENV_VAR_GRIT_AUTH_TOKEN};
use marzano_auth::info::AuthInfo;
use marzano_gritmodule::config::REPO_CONFIG_DIR_NAME;
use marzano_gritmodule::searcher::WorkflowInfo;
use marzano_gritmodule::{fetcher::LocalRepo, searcher::find_grit_dir_from};
use marzano_messenger::workflows::WorkflowMessenger;
use marzano_messenger::{emit::Messager, workflows::PackagedWorkflowOutcome};
use marzano_util::diff::FileDiff;
use serde::Serialize;
use serde_json::to_string;
use std::path::{Path, PathBuf};
use std::process::Stdio;
use tempfile::NamedTempFile;
use tokio::fs;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command;

pub static GRIT_REPO_URL_NAME: &str = "grit_repo_url";
pub static GRIT_REPO_BRANCH_NAME: &str = "grit_branch";
pub static GRIT_TARGET_RANGES: &str = "grit_target_ranges";
pub static GRIT_VCS_USER_NAME: &str = "grit_vcs_username";
pub static ENV_GRIT_WORKSPACE_ROOT: &str = "GRIT_WORKSPACE_ROOT";

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
    /// The workflow execution ID
    pub execution_id: String,
    // If this is a custom workflow, this will be the path to the entrypoint
    pub workflow_entrypoint: String,
    /// Ranges to target, if any
    pub ranges: Option<Vec<FileDiff>>,
    // Input paths, might include unresolved globs
    pub paths: Vec<PathBuf>,
    // Input
    pub input: serde_json::Map<String, serde_json::Value>,
    // Verbose
    pub verbose: bool,
}

#[allow(unused_mut)]
pub async fn run_bin_workflow<M>(mut emitter: M, mut arg: WorkflowInputs) -> Result<M>
where
    M: Messager + WorkflowMessenger + Send + Clone + 'static,
{
    let cwd = std::env::current_dir()?;

    let workflow_id = arg.execution_id.clone();
    let marzano_bin = std::env::current_exe()?;

    let mut updater = Updater::from_current_bin().await?;
    let repo = LocalRepo::from_dir(&cwd).await;

    #[cfg(feature = "workflow_server")]
    let (server_addr, handle, shutdown_tx) = {
        let server_emitter = emitter.clone();
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let socket = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
        let server_addr = format!("http://{}", socket.local_addr()?).to_string();
        let handle = grit_cloud_client::spawn_server_tasks(server_emitter, shutdown_rx, socket);
        log::debug!("Started local server at {}", server_addr);
        (server_addr, handle, shutdown_tx)
    };

    let root = std::env::var(ENV_GRIT_WORKSPACE_ROOT).unwrap_or_else(|_| {
        repo.as_ref().and_then(|r| r.root().ok()).map_or_else(
            || cwd.to_string_lossy().into_owned(),
            |r| r.to_string_lossy().into_owned(),
        )
    });

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

    let auth = updater.get_valid_auth().await;
    if auth.is_err() {
        log::warn!(
            "No valid authentication token found, please run {}",
            style("grit auth login").bold().red()
        );
    }

    if let Ok(ref auth) = auth {
        if let Some(username) = auth.get_user_name()? {
            if !arg.input.contains_key(GRIT_VCS_USER_NAME) {
                arg.input
                    .insert(GRIT_VCS_USER_NAME.to_string(), username.into());
            }
        }
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
        grit_dir: grit_dir.map(|p| p.to_string_lossy().to_string()),
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

    let mut child = Command::new(runner_path);
    child
        .arg(tempfile_path.to_string_lossy().to_string())
        .env("GRIT_MARZANO_PATH", marzano_bin);

    #[cfg(feature = "workflow_server")]
    child.env(marzano_auth::env::ENV_VAR_GRIT_LOCAL_SERVER, &server_addr);

    let mut final_child = child
        .env(ENV_VAR_GRIT_API_URL, get_grit_api_url())
        .env(ENV_GRIT_WORKSPACE_ROOT, root)
        .arg("--file")
        .arg(&tempfile_path)
        .kill_on_drop(true);

    let mut final_child = if let Ok(auth) = auth {
        final_child.env(ENV_VAR_GRIT_AUTH_TOKEN, auth.access_token)
    } else {
        final_child
    };
    let mut final_child = final_child
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start worker");

    let stdout = final_child.stdout.take().expect("Failed to get stdout");
    let stderr = final_child.stderr.take().expect("Failed to get stderr");

    let mut stdout_reader = BufReader::new(stdout).lines();
    let mut stderr_reader = BufReader::new(stderr).lines();

    let mut stdout_emitter = emitter.clone();
    let stdout_handle = tokio::spawn(async move {
        while let Some(line) = stdout_reader.next_line().await.unwrap() {
            let log = marzano_messenger::SimpleLogMessage {
                level: marzano_core::api::AnalysisLogLevel::Info,
                step_id: None,
                message: line,
                meta: Some(std::collections::HashMap::from([(
                    "source".to_string(),
                    serde_json::Value::String("stdout".to_string()),
                )])),
            };
            if let Err(e) = stdout_emitter.emit_log(&log) {
                log::error!("Error emitting log: {}", e);
            }
        }
    });
    let mut stderr_emitter = emitter.clone();
    let stderr_handle = tokio::spawn(async move {
        while let Some(line) = stderr_reader.next_line().await.unwrap() {
            let log = marzano_messenger::SimpleLogMessage {
                level: marzano_core::api::AnalysisLogLevel::Error,
                step_id: None,
                message: line,
                meta: Some(std::collections::HashMap::from([(
                    "source".to_string(),
                    serde_json::Value::String("stderr".to_string()),
                )])),
            };
            if let Err(e) = stderr_emitter.emit_log(&log) {
                log::error!("Error emitting log: {}", e);
            }
        }
    });

    let status = final_child.wait().await?;

    // Stop the embedded server
    #[cfg(feature = "workflow_server")]
    let mut emitter = {
        shutdown_tx.send(()).unwrap();
        handle.await?
    };

    // Wait for the stdout and stderr readers to finish
    stdout_handle.await?;
    stderr_handle.await?;

    // Note the workflow may have already emitted its own conclusion - this is a fallback
    let fallback_outcome = if status.success() {
        PackagedWorkflowOutcome {
            message: None,
            outcome: None,
            success: true,
            data: None,
        }
    } else {
        PackagedWorkflowOutcome {
            message: None,
            outcome: None,
            success: false,
            data: None,
        }
    };
    emitter.finish_workflow(&fallback_outcome).await?;

    Ok(emitter)
}

#[cfg(feature = "remote_workflows")]
pub async fn run_remote_workflow(
    pattern_or_workflow: String,
    args: crate::commands::apply_migration::ApplyMigrationArgs,
    ranges: Option<Vec<FileDiff>>,
    flags: &crate::flags::GlobalFormatFlags,
    root_progress: indicatif::MultiProgress,
) -> Result<()> {
    use colored::Colorize;
    use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
    use marzano_gritmodule::fetcher::ModuleRepo;
    use std::time::Duration;

    use crate::commands::workflows_upload::{run_upload_workflow, WorkflowUploadArgs};

    let mut updater = Updater::from_current_bin().await?;
    let cwd = std::env::current_dir()?;

    let pb = ProgressBar::with_draw_target(Some(0), ProgressDrawTarget::stderr());
    let pb = root_progress.add(pb);
    pb.set_style(ProgressStyle::with_template(
        "{spinner}{prefix:.bold.dim} {wide_msg:.bold.dim}",
    )?);
    pb.set_message("Authenticating with Grit Cloud");
    pb.enable_steady_tick(Duration::from_millis(60));

    let auth = updater.get_valid_auth().await?;

    let repo = ModuleRepo::from_dir(&cwd).await;
    let mut input = args.get_payload()?;

    if let Some(username) = auth.get_user_name()? {
        if !input.contains_key(GRIT_VCS_USER_NAME) {
            input.insert(GRIT_VCS_USER_NAME.to_string(), username.into());
        }
    }

    if let Some(ranges) = ranges {
        if !input.contains_key(GRIT_TARGET_RANGES) {
            input.insert(
                GRIT_TARGET_RANGES.to_string(),
                serde_json::to_value(ranges)?,
            );
        }
    }

    let Some(workflow_id) = args.workflow_id else {
        anyhow::bail!("No workflow ID provided");
    };

    pb.set_message(format!(
        "Uploading {} workflow to Grit Cloud",
        pattern_or_workflow
    ));
    let upload_args = WorkflowUploadArgs {
        workflow_path: pattern_or_workflow,
        workflow_id: workflow_id.to_string(),
    };
    let artifact_download_url = run_upload_workflow(&upload_args, &flags)
        .await
        .map_err(|e| {
            pb.set_message("Failed to upload workflow");
            pb.finish();
            e
        })?;
    input.insert("definition".to_string(), artifact_download_url.into());

    pb.set_message("Starting workflow");

    let settings =
        grit_cloud_client::RemoteWorkflowSettings::new(&workflow_id, &repo, input.into());

    let result = grit_cloud_client::run_remote_workflow(settings, &auth).await?;

    pb.finish_and_clear();

    log::info!(
        "Workflow started at: {}",
        result.url().bright_blue().underline()
    );

    if args.watch {
        // Wait 1 seconds for the workflow to start
        tokio::time::sleep(Duration::from_secs(1)).await;

        let auth = updater.get_valid_auth().await?;

        let format = crate::flags::OutputFormat::from(flags);
        let emitter = crate::messenger_variant::create_emitter(
            &format,
            marzano_messenger::output_mode::OutputMode::default(),
            None,
            false,
            None,
            None,
            marzano_messenger::emit::VisibilityLevels::default(),
        )
        .await?;

        grit_cloud_client::watch_workflow(&result.execution_id, &auth, emitter).await?;

        log::info!(
            "Run this to watch this workflow again:\n  {}",
            format!("grit workflows watch {}", &result.execution_id)
                .bright_yellow()
                .bold(),
        );
    }

    Ok(())
}

pub async fn fetch_remote_workflow(
    workflow_path_or_name: &str,
    auth: Option<AuthInfo>,
) -> Result<WorkflowInfo> {
    let temp_dir = tempfile::tempdir()?;
    // Note: into_path is important here to prevent the temp_dir from being dropped
    let temp_file_path = temp_dir.into_path().join("downloaded_workflow.ts");
    let client = reqwest::Client::new();
    let mut request = client.get(workflow_path_or_name);
    if let Some(auth_info) = auth {
        // Only inject auth if URL is from localhost or grit.io
        if workflow_path_or_name.contains("localhost") || workflow_path_or_name.contains("grit.io")
        {
            request = request.header(
                "Authorization",
                format!("Bearer {}", auth_info.access_token),
            );
        }
    }
    let response = request.send().await?;

    // Verify status code
    if !response.status().is_success() {
        return Err(anyhow::anyhow!(
            "Failed to fetch remote workflow: {}
    {}",
            response.status(),
            response.text().await?
        ));
    }

    let content = response.text().await?;
    fs_err::write(&temp_file_path, content)?;
    Ok(WorkflowInfo::new(temp_file_path))
}

/// Find a workflow file by its name or path, from a given current directory.
///
/// This handles fetching from:
/// - Remote URLs (starting with https)
/// - Explicit workflow paths, anywhere on the local filesystem
/// - Named workflows (ex. "hello" will find `.grit/workflows/hello.ts` in the current directory)
/// - Named workflows from user config (ex. "hello" will find `~/.grit/workflows/hello.ts`)
pub async fn find_workflow_file_from(
    dir: PathBuf,
    workflow_path_or_name: &str,
    auth: Option<AuthInfo>,
) -> Option<WorkflowInfo> {
    if workflow_path_or_name.starts_with("http://") || workflow_path_or_name.starts_with("https://")
    {
        match fetch_remote_workflow(workflow_path_or_name, auth).await {
            Ok(info) => return Some(info),
            Err(e) => {
                log::error!("Failed to fetch remote workflow: {}", e);
            }
        }
    }

    if workflow_path_or_name.ends_with(".js") || workflow_path_or_name.ends_with(".ts") {
        let workflow_file_path = if Path::new(workflow_path_or_name).is_absolute() {
            PathBuf::from(workflow_path_or_name)
        } else {
            dir.join(workflow_path_or_name)
        };
        if fs::metadata(&workflow_file_path).await.is_ok() {
            return Some(WorkflowInfo::new(workflow_file_path));
        }
    }

    let paths = get_workflow_paths(dir.join(REPO_CONFIG_DIR_NAME), workflow_path_or_name)
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect::<Vec<String>>();

    let workflow_file = marzano_gritmodule::searcher::search(dir, &paths, None).await;
    if let Some(path) = workflow_file {
        return Some(WorkflowInfo::new(path));
    }

    if let Some(user_grit) = marzano_gritmodule::resolver::find_user_grit_dir() {
        let paths = get_workflow_paths(user_grit, workflow_path_or_name);

        for path in paths {
            if fs::metadata(&path).await.is_ok() {
                return Some(WorkflowInfo::new(path));
            }
        }
    }
    None
}

fn get_workflow_paths(grit_dir: PathBuf, workflow_path_or_name: &str) -> Vec<PathBuf> {
    let mut base_search_string = grit_dir.clone();
    base_search_string.push("workflows");
    base_search_string.push(format!("{}.ts", workflow_path_or_name));

    let mut bundled_search_string = grit_dir;
    bundled_search_string.push("workflows");
    bundled_search_string.push(workflow_path_or_name);
    bundled_search_string.push("index.ts");

    vec![base_search_string, bundled_search_string]
}
