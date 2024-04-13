use anyhow::Result;
use std::{
    collections::HashMap,
    sync::atomic::{AtomicI32, Ordering},
};

use clap::ValueEnum;
use colored::Colorize;
use dialoguer::Input;
use indicatif::ProgressBar;
use log::info;
use marzano_core::{
    api::{derive_log_level, is_match, AnalysisLogLevel, MatchResult},
    fs::apply_rewrite,
};
use marzano_language::target_language::TargetLanguage;
use serde::{Deserialize, Serialize};

use crate::{format::format_result, workflows::PackagedWorkflowOutcome};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ApplyDetails {
    pub matched: i32,
    pub rewritten: i32,
    pub named_pattern: Option<String>,
}

pub trait Messager: Send + Sync {
    // Write a message to the output
    fn emit(&mut self, message: &MatchResult, min_level: &VisibilityLevels) -> anyhow::Result<()> {
        if get_visibility(message) >= *min_level {
            self.raw_emit(message)
        } else {
            Ok(())
        }
    }

    // Handle a group of execution results
    #[allow(clippy::too_many_arguments)]
    fn handle_results(
        &mut self,
        execution_result: Vec<MatchResult>,
        details: &mut ApplyDetails,
        dry_run: bool,
        min_level: &VisibilityLevels,
        should_format: bool,
        interactive: &mut bool,
        pg: Option<&ProgressBar>,
        processed: Option<&AtomicI32>,
        mut parse_errors: Option<&mut HashMap<String, usize>>,
        language: &TargetLanguage,
    ) -> bool {
        for r in execution_result.into_iter() {
            if is_match(&r) {
                details.matched += 1;
            }
            if let MatchResult::Rewrite(_) = r {
                details.rewritten += 1;
            }

            if let MatchResult::DoneFile(_) = r {
                if let Some(pg) = pg {
                    pg.inc(1);
                }
                if let Some(processed) = processed {
                    processed.fetch_add(1, Ordering::SeqCst);
                }
            }

            if let Some(pg) = pg {
                if let Some(name) = r.file_name() {
                    pg.set_message(name.to_owned());
                }
            }

            // TODO: log levels and messages should be refactored in the future
            if let Some(parse_errors) = &mut parse_errors {
                if let MatchResult::AnalysisLog(log) = &r {
                    let is_error = matches!(&r, MatchResult::AnalysisLog(log) if derive_log_level(log) >= AnalysisLogLevel::Error);
                    if is_error {
                        let entry = parse_errors.entry(log.file.to_owned());
                        let value = entry.or_insert(0);
                        // We keep track of this so as to only raise a specific kind of error once, for user experience
                        *value += 1;
                        if *value > 1 {
                            continue;
                        }
                    }
                }
            }

            self.emit(&r, min_level).unwrap();

            if !dry_run {
                if is_match(&r) {
                    let file_name = r.file_name().unwrap();
                    if *interactive {
                        let (prefix, question, valid_chars, actions) =
                            if let MatchResult::Match(_) = r {
                                (
                                    "Found a match in",
                                    "Acknowledge this change",
                                    vec!["y", "s", "a", "q"],
                                    "[(y)es,(s)uppress,(a)ccept all,(q)uit]",
                                )
                            } else {
                                (
                                    "Found a rewrite in",
                                    "Apply this change",
                                    vec!["y", "n", "s", "a", "q"],
                                    "[(y)es,(n)o,(s)uppress,(a)ccept all,(q)uit]",
                                )
                            };

                        if let Some(pg) = pg {
                            pg.set_prefix(prefix)
                        } else {
                            info!("{}", format!("{prefix} {file_name}").dimmed().bold());
                        }
                        let actions_bold = actions.bold().blue();
                        let selection = Input::new()
                            .with_prompt(format!("{question} {actions_bold}"))
                            .validate_with(|input: &String| -> Result<(), String> {
                                if valid_chars.contains(
                                    &input
                                        .chars()
                                        .next()
                                        .unwrap_or('_')
                                        .to_lowercase()
                                        .to_string()
                                        .as_str(),
                                ) {
                                    Ok(())
                                } else {
                                    Err(format!("Not a valid choice in {actions:}"))
                                }
                            })
                            .interact_text()
                            .unwrap();
                        if let Some(pg) = pg {
                            pg.set_prefix("Analyzing")
                        }
                        match selection.trim().to_lowercase().as_str() {
                            "y" => {
                                self.track_accept(&r).unwrap();
                            }
                            "n" => {
                                self.track_reject(&r).unwrap();
                                continue;
                            }
                            "s" => {
                                self.track_supress(&r).unwrap();
                                let suppress_rewrite = r
                                    .get_rewrite_to_suppress(
                                        language,
                                        details.named_pattern.as_deref(),
                                    )
                                    .unwrap();
                                apply_rewrite(&suppress_rewrite).unwrap();
                                continue;
                            }
                            "a" => {
                                self.track_accept(&r).unwrap();
                                *interactive = false;
                            }
                            "q" => {
                                self.track_reject(&r).unwrap();
                                *interactive = false;
                                return false;
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        self.track_accept(&r).unwrap();
                    }
                }
                apply_rewrite(&r).unwrap();
                if should_format {
                    format_result(r).unwrap();
                }
            }
        }
        true
    }

    // Write a message to the output
    fn raw_emit(&mut self, message: &MatchResult) -> anyhow::Result<()>;

    // Send the total count
    fn emit_estimate(&mut self, _count: usize) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }

    // Start a workflow, not needed for normal applies
    fn start_workflow(&mut self) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }

    // Finish workflow, with a message
    fn finish_workflow(&mut self, _outcome: &PackagedWorkflowOutcome) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }

    // Handle state
    fn track_accept(&mut self, _accepted: &MatchResult) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }
    fn track_reject(&mut self, _rejected: &MatchResult) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }
    fn track_supress(&mut self, _rejected: &MatchResult) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }
}

/// Visibility levels dictate *which* objects we show (ex. just rewrites, or also every file analyzed)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ValueEnum, Serialize)]
pub enum VisibilityLevels {
    Primary = 3,      // Always show this to users
    Supplemental = 2, // Show to users as secondary information
    Debug = 1,        // Only show to users if they ask for it
    Hidden = 0,       // Never show to users
}

impl std::fmt::Display for VisibilityLevels {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

pub fn get_visibility(result: &MatchResult) -> VisibilityLevels {
    match result {
        MatchResult::AnalysisLog(log) => {
            if derive_log_level(log) >= AnalysisLogLevel::Debug {
                VisibilityLevels::Debug
            } else {
                VisibilityLevels::Supplemental
            }
        }
        MatchResult::Match(_) => VisibilityLevels::Primary,
        MatchResult::InputFile(_) => VisibilityLevels::Hidden,
        MatchResult::CreateFile(_) => VisibilityLevels::Primary,
        MatchResult::RemoveFile(_) => VisibilityLevels::Primary,
        MatchResult::Rewrite(_) => VisibilityLevels::Primary,
        MatchResult::DoneFile(_) => VisibilityLevels::Debug,
        MatchResult::AllDone(_) => VisibilityLevels::Supplemental,
        MatchResult::PatternInfo(_) => VisibilityLevels::Debug,
    }
}
