use anyhow::{anyhow, bail, Result};
use clap::{Args, Subcommand};
use indicatif::MultiProgress;
use marzano_gritmodule::config::GritPatternTestInfo;
use marzano_gritmodule::fetcher::KeepFetcherKind;
use marzano_gritmodule::patterns_directory::PatternsDirectory;
use marzano_gritmodule::searcher::find_grit_modules_dir;
use marzano_messenger::emit::ApplyDetails;
use serde::{Deserialize, Serialize};
use std::io::{stdin, Read};
use std::path::Path;
use std::path::PathBuf;

use crate::analytics::{track_event_line};
use crate::flags::GlobalFormatFlags;
use crate::lister::list_applyables;
use crate::resolver::{get_grit_files_from, resolve_from, Source};
use crate::utils::is_pattern_name;


use super::super::analytics::{AnalyticsArgs};
use super::apply_pattern::{run_apply_pattern, ApplyPatternArgs};
use super::check::{run_check, CheckArg};
use super::init::{init_config_from_cwd, init_global_grit_modules};
use super::list::ListArgs;
use super::parse::{run_parse, ParseInput};
use super::patterns::PatternsTestArgs;
use super::patterns_test::get_marzano_pattern_test_results;

#[derive(Deserialize)]
struct PlumbingApplyInput {
    pub pattern_body: String,
    pub paths: Vec<PathBuf>,
    pub root_path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct PlumbingCheckInput {
    pub paths: Vec<PathBuf>,
    pub root_path: Option<PathBuf>,
}

#[derive(Deserialize)]
struct PlumbingPatternsListInput {
    pub grit_dir: PathBuf,
}

#[derive(Args, Debug, Serialize)]
pub struct SharedPlumbingArgs {
    /// The path to the input file, if unspecified, stdin is used
    #[clap(long = "input")]
    input: Option<PathBuf>,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum PlumbingArgs {
    /// Run `apply` from input
    Apply {
        #[command(flatten)]
        apply_pattern_args: ApplyPatternArgs,
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
    },
    /// Run `parse` via stdin
    Parse {
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
    },
    /// Send an analytics event via stdin
    Analytics {
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
        #[command(flatten)]
        args: AnalyticsArgs,
    },
    /// Run `check` via stdin
    Check {
        #[command(flatten)]
        args: CheckArg,
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
    },
    List {
        #[command(flatten)]
        args: ListArgs,
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
    },
    /// Run `patterns test` via stdin
    Test {
        #[command(flatten)]
        shared_args: SharedPlumbingArgs,
    },
}

fn read_input(shared_args: &SharedPlumbingArgs) -> Result<String> {
    let buffer = if let Some(input) = &shared_args.input {
        std::fs::read_to_string(input)?
    } else {
        let mut buffer = String::new();
        stdin().read_to_string(&mut buffer)?;
        buffer
    };
    Ok(buffer)
}

fn ensure_trailing_slash(root_path: &Path) -> PathBuf {
    let mut path_str = root_path.to_str().unwrap_or_default().to_string();
    if !path_str.ends_with('/') {
        path_str.push('/');
    }
    PathBuf::from(path_str)
}

pub(crate) async fn run_plumbing(
    args: PlumbingArgs,
    multi: MultiProgress,
    details: &mut ApplyDetails,
    parent: GlobalFormatFlags,
) -> Result<()> {
    match args {
        PlumbingArgs::Apply {
            apply_pattern_args,
            shared_args,
        } => {
            let buffer = read_input(&shared_args)?;
            let input: PlumbingApplyInput = serde_json::from_str::<PlumbingApplyInput>(&buffer).map_err(|e| {
                anyhow!(
                    "Failed to parse input JSON: {}. Ensure that input matches schema \
                    {{ pattern_body: string; pattern_libs: {{ [string]: string }}; paths: string[]; }}",
                    e
                )
            })?;
            let grit_files = if input.paths.is_empty() {
                PatternsDirectory::new()
            } else {
                let path = PathBuf::from(input.paths.first().unwrap());
                init_config_from_cwd::<KeepFetcherKind>(path.clone(), false).await?;
                get_grit_files_from(Some(path)).await?
            };
            let raw_name = input.pattern_body.trim_end_matches("()");
            let pattern_libs = grit_files.get_pattern_libraries(raw_name)?;
            let body = if is_pattern_name(&input.pattern_body) && !input.pattern_body.ends_with(')')
            {
                format!("{}()", input.pattern_body)
            } else {
                input.pattern_body
            };
            run_apply_pattern(
                body,
                input.paths,
                apply_pattern_args,
                multi,
                details,
                Some(pattern_libs.library()),
                Some(pattern_libs.language()),
                parent.into(),
                input.root_path.map(|p| ensure_trailing_slash(&p)),
            )
            .await
        }
        PlumbingArgs::Parse { shared_args } => {
            let buffer = read_input(&shared_args)?;
            let input = serde_json::from_str::<ParseInput>(&buffer).map_err(|e| {
                anyhow!(
                    "Failed to parse input JSON: {}. Ensure that input matches schema \
                    {{ pattern_body: string; paths: string[]; }}",
                    e
                )
            })?;
            let pattern_body = input.pattern_body.clone();
            run_parse(input.into(), parent, Some(pattern_body)).await
        }
        PlumbingArgs::Analytics { args, shared_args } => {
            let buffer = read_input(&shared_args)?;
            for line in buffer.lines() {
                let result = track_event_line(
                    line,
                    args.command.clone(),
                    args.args.clone(),
                    args.installation_id,
                    args.user_id.clone(),
                )
                .await;
                if let Err(e) = result {
                    eprintln!("Error when processing {}: {:#}", line, e);
                }
            }

            Ok(())
        }
        PlumbingArgs::Check { args, shared_args } => {
            let buffer = read_input(&shared_args)?;
            let input = serde_json::from_str::<PlumbingCheckInput>(&buffer).map_err(|e| {
                anyhow!(
                    "Failed to parse input JSON: {}. Ensure that input matches schema \
                    {{ paths: string[]; }}",
                    e
                )
            })?;
            if input.paths.is_empty() {
                return Ok(());
            }
            init_global_grit_modules::<KeepFetcherKind>().await?;
            let combined_args = CheckArg {
                paths: input.paths,
                ..args
            };
            run_check(
                combined_args,
                &parent,
                multi,
                true,
                input.root_path.map(|p| ensure_trailing_slash(&p)),
            )
            .await
        }
        PlumbingArgs::List { args, shared_args } => {
            let buffer = read_input(&shared_args)?;
            let input =
                serde_json::from_str::<PlumbingPatternsListInput>(&buffer).map_err(|e| {
                    anyhow!(
                        "Failed to parse input JSON: {}. Ensure that input matches schema \
                    {{ grit_dir: string; }}",
                        e
                    )
                })?;
            let grit_parent = match input.grit_dir.parent() {
                Some(parent) => parent,
                None => return Ok(()),
            };

            let (resolved, curr_repo) =
                resolve_from(grit_parent.to_path_buf(), &Source::All).await?;

            if resolved.is_empty() {
                let existing = find_grit_modules_dir(grit_parent.to_path_buf()).await?;
                if !existing.exists() {
                    bail!(
                    "No grit modules found in {}. Run `grit init` to initialize a grit project.",
                    grit_parent.to_string_lossy());
                } else {
                    bail!("No patterns found.");
                }
            }

            list_applyables(false, false, resolved, args.level, &parent, curr_repo).await
        }
        PlumbingArgs::Test { shared_args } => {
            let buffer = read_input(&shared_args)?;
            let patterns =
                serde_json::from_str::<Vec<GritPatternTestInfo>>(&buffer).map_err(|e| {
                    anyhow!(
                        "Failed to parse input JSON: {}. Ensure that input has correct schema. This command is
                        compatible with the output of `grit patterns list` --json`",
                        e
                    )
                })?;

            let libs = get_grit_files_from(None).await?;
            get_marzano_pattern_test_results(
                patterns,
                &libs,
                PatternsTestArgs {
                    update: false,
                    verbose: false,
                    filter: None,
                    exclude: vec![],
                },
            )
            .await
        }
    }
}
