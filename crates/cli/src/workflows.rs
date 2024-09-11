use crate::updater::{SupportedApp, Updater};
use anyhow::Result;
use console::style;
use log::debug;
use marzano_auth::env::{get_grit_api_url, ENV_VAR_GRIT_API_URL, ENV_VAR_GRIT_AUTH_TOKEN};
use marzano_auth::info::AuthInfo;
use marzano_core::api::AnalysisLogLevel;
use marzano_gritmodule::config::REPO_CONFIG_DIR_NAME;
use marzano_gritmodule::searcher::WorkflowInfo;
use marzano_gritmodule::{fetcher::LocalRepo, searcher::find_grit_dir_from};
use marzano_messenger::workflows::WorkflowMessenger;
use marzano_messenger::{emit::Messager, workflows::PackagedWorkflowOutcome};
use marzano_util::diff::FileDiff;
use serde::Serialize;
use serde_json::to_string;
use std::path::{Path, PathBuf};
use std::process::{ChildStderr, ChildStdout, Stdio};
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
pub async fn run_bin_workflow<M>(emitter: &'static mut M, mut arg: WorkflowInputs) -> Result<()>
where
    M: Messager + WorkflowMessenger + Send + 'static,
{
    let cwd = std::env::current_dir()?;

    let workflow_id = arg.execution_id.clone();
    let marzano_bin = std::env::current_exe()?;

    let mut updater = Updater::from_current_bin().await?;
    let repo = LocalRepo::from_dir(&cwd).await;

    #[cfg(feature = "workflow_server")]
    let (server_addr, handle, shutdown_tx) = {
        let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
        let socket = tokio::net::TcpListener::bind("0.0.0.0:0").await?;
        let server_addr = format!("http://{}", socket.local_addr()?).to_string();
        let handle = grit_cloud_client::spawn_server_tasks(emitter, shutdown_rx, socket);
        log::info!("Started local server at {}", server_addr);
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

    let auth = updater.get_valid_auth().await.map_err(|_| {
        anyhow::anyhow!(
            "No valid authentication token found, please run {}",
            style("grit auth login").bold().red()
        )
    })?;

    if let Some(username) = auth.get_user_name()? {
        if !arg.input.contains_key(GRIT_VCS_USER_NAME) {
            arg.input
                .insert(GRIT_VCS_USER_NAME.to_string(), username.into());
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

    let mut child = Command::new(runner_path);
    child
        .arg(tempfile_path.to_string_lossy().to_string())
        .env("GRIT_MARZANO_PATH", marzano_bin);

    #[cfg(feature = "workflow_server")]
    child.env(marzano_auth::env::ENV_VAR_GRIT_LOCAL_SERVER, &server_addr);

    let mut final_child = child
        .env(ENV_VAR_GRIT_API_URL, get_grit_api_url())
        .env(ENV_VAR_GRIT_AUTH_TOKEN, auth.access_token)
        .env(ENV_GRIT_WORKSPACE_ROOT, root)
        .arg("--file")
        .arg(&tempfile_path)
        .kill_on_drop(true)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to start worker");

    let stdout = final_child.stdout.take().expect("Failed to get stdout");
    let stderr = final_child.stderr.take().expect("Failed to get stderr");

    let stdout_reader = BufReader::new(stdout).lines();
    let stderr_reader = BufReader::new(stderr).lines();

    // let stdout_handler = tokio::spawn(async move {
    //     let mut stdout_reader = stdout_reader;
    //     while let Some(line) = stdout_reader.next_line().await.unwrap() {
    //         let log = marzano_messenger::SimpleLogMessage {
    //             message: line,
    //             level: AnalysisLogLevel::Info,
    //             meta: None,
    //             step_id: None,
    //         };
    //         emitter_mut.emit_log(&log);
    //     }
    // });

    // tokio::select! {
    //     res = stdout_reader.for_each(|line| async {
    //         if let Ok(line) = line {
    //             println!("stdout: {}", line);
    //         }
    //     }) => {
    //         res?;
    //     }
    //     res = stderr_reader.for_each(|line| async {
    //         if let Ok(line) = line {
    //             println!("stderr: {}", line);
    //         }
    //     }) => {
    //         res?;
    //     }
    // }

    let status = final_child.wait().await?;

    // Stop the embedded server
    #[cfg(feature = "workflow_server")]
    {
        shutdown_tx.send(()).unwrap();
        handle.await?
    };

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
    emitter.finish_workflow(&fallback_outcome)?;

    Ok(emitter)
}

#[cfg(feature = "remote_workflows")]
pub async fn run_remote_workflow(
    workflow_name: String,
    args: crate::commands::apply_migration::ApplyMigrationArgs,
    ranges: Option<Vec<FileDiff>>,
    flags: &crate::flags::GlobalFormatFlags,
) -> Result<()> {
    use colored::Colorize;
    use indicatif::{ProgressBar, ProgressDrawTarget, ProgressStyle};
    use marzano_gritmodule::fetcher::ModuleRepo;
    use std::time::Duration;

    let mut updater = Updater::from_current_bin().await?;
    let cwd = std::env::current_dir()?;

    let pb = ProgressBar::with_draw_target(Some(0), ProgressDrawTarget::stderr());
    pb.set_style(ProgressStyle::with_template(
        "{spinner}{prefix:.bold.dim} {wide_msg:.bold.dim}",
    )?);
    pb.set_message("Authenticating with Grit Cloud");
    pb.enable_steady_tick(Duration::from_millis(60));

    let auth = updater.get_valid_auth().await?;

    pb.set_message("Launching workflow on Grit Cloud");

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

    let settings =
        grit_cloud_client::RemoteWorkflowSettings::new(workflow_name, &repo, input.into());
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

async fn fetch_remote_workflow(
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

    let base_search_string = format!(
        "{}/workflows/{}.ts",
        REPO_CONFIG_DIR_NAME, workflow_path_or_name
    );
    let bundled_search_string = format!(
        "{}/workflows/{}/index.ts",
        REPO_CONFIG_DIR_NAME, workflow_path_or_name
    );
    let workflow_file = marzano_gritmodule::searcher::search(
        dir,
        &[base_search_string, bundled_search_string],
        None,
    )
    .await;
    workflow_file.map(|path| WorkflowInfo::new(PathBuf::from(path)))
}
