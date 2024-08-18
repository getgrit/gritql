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
    api::{derive_log_level, is_match, AnalysisLog, AnalysisLogLevel, MatchResult},
    fs::apply_rewrite,
};
use marzano_language::target_language::TargetLanguage;
use serde::{Deserialize, Serialize};

use crate::{format::format_result, workflows::PackagedWorkflowOutcome, SimpleLogMessage};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
pub struct ApplyDetails {
    /// How many matches were found total (total match range count)
    pub matched: i32,
    pub rewritten: i32,
    pub named_pattern: Option<String>,
}

pub trait Messager: Send + Sync {
    fn get_min_level(&self) -> VisibilityLevels;

    // Write a message to the output
    fn emit(&mut self, message: &MatchResult) -> anyhow::Result<()> {
        if get_visibility(message) >= self.get_min_level() {
            self.raw_emit(message)
        } else {
            Ok(())
        }
    }

    fn apply_rewrite(&mut self, result: &MatchResult) -> anyhow::Result<()> {
        if let Err(e) = apply_rewrite(result) {
            let err_string = format!("Failed to apply rewrite: {}", e);
            let err_log = if let Some(file_name) = result.file_name() {
                AnalysisLog::new_error(err_string, file_name)
            } else {
                AnalysisLog::floating_error(err_string)
            };

            self.emit(&MatchResult::AnalysisLog(err_log))
        } else {
            Ok(())
        }
    }

    /// This is the main entrypoint for handling a group of results
    /// In interactive mode, it is responsible for asking the user what to do with each result
    ///
    /// Returns true if the process should continue, false if it should stop
    #[allow(clippy::too_many_arguments)]
    fn handle_results(
        &mut self,
        execution_result: Vec<MatchResult>,
        details: &mut ApplyDetails,
        dry_run: bool,
        should_format: bool,
        interactive: &mut bool,
        pg: Option<&ProgressBar>,
        processed: Option<&AtomicI32>,
        parse_errors: Option<&mut HashMap<String, usize>>,
        language: &TargetLanguage,
    ) -> bool {
        match self.handle_results_inner(
            execution_result,
            details,
            dry_run,
            should_format,
            interactive,
            pg,
            processed,
            parse_errors,
            language,
        ) {
            Ok(val) => val,
            Err(err) => {
                let err_log = AnalysisLog::new_error(err.to_string(), "unknown");
                self.emit(&MatchResult::AnalysisLog(err_log))
                    .expect("Failed to emit error log");
                true
            }
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_results_inner(
        &mut self,
        execution_result: Vec<MatchResult>,
        details: &mut ApplyDetails,
        dry_run: bool,
        should_format: bool,
        interactive: &mut bool,
        pg: Option<&ProgressBar>,
        processed: Option<&AtomicI32>,
        mut parse_errors: Option<&mut HashMap<String, usize>>,
        language: &TargetLanguage,
    ) -> anyhow::Result<bool> {
        for r in execution_result {
            if is_match(&r) {
                let count = r.get_ranges().map(|ranges| ranges.len()).unwrap_or(0);
                details.matched += count.max(1) as i32;
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

            self.emit(&r)?;

            if !dry_run {
                if is_match(&r) {
                    let file_name = r
                        .file_name()
                        .ok_or_else(|| anyhow::Error::msg("File name is missing"))?;
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
                            .interact_text()?;
                        if let Some(pg) = pg {
                            pg.set_prefix("Analyzing")
                        }
                        match selection.trim().to_lowercase().as_str() {
                            "y" => {
                                self.track_accept(&r)?;
                            }
                            "n" => {
                                self.track_reject(&r)?;
                                continue;
                            }
                            "s" => {
                                self.track_supress(&r)?;
                                let suppress_rewrite = r
                                    .get_rewrite_to_suppress(
                                        language,
                                        details.named_pattern.as_deref(),
                                    )
                                    .ok_or(anyhow::anyhow!("Failed to suppress rewrite"))?;
                                self.apply_rewrite(&suppress_rewrite)?;
                                continue;
                            }
                            "a" => {
                                self.track_accept(&r)?;
                                *interactive = false;
                            }
                            "q" => {
                                self.track_reject(&r)?;
                                *interactive = false;
                                return Ok(false);
                            }
                            _ => unreachable!(),
                        }
                    } else {
                        self.track_accept(&r)?;
                    }
                }
                self.apply_rewrite(&r)?;
                if should_format {
                    format_result(r)?;
                }
            }
        }
        Ok(true)
    }

    // Write a message to the output
    fn raw_emit(&mut self, message: &MatchResult) -> anyhow::Result<()>;

    // Write a log message
    fn emit_log(&mut self, _log: &SimpleLogMessage) -> anyhow::Result<()>;

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

    // Called when a workflow finishes processing, with the outcome
    // Note that this *may* be called multiple times. The *first* time it is called should be considered the "true" outcome.
    fn finish_workflow(&mut self, _outcome: &PackagedWorkflowOutcome) -> anyhow::Result<()> {
        // do nothing
        Ok(())
    }

    // Get the current workflow outcome, if one has been set
    fn get_workflow_status(&mut self) -> anyhow::Result<Option<&PackagedWorkflowOutcome>> {
        Ok(None)
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

pub trait FlushableMessenger {
    fn flush(&mut self) -> impl std::future::Future<Output = Result<()>> + Send;
}

/// Visibility levels dictate *which* objects we show (ex. just rewrites, or also every file analyzed)
#[derive(Debug, PartialEq, PartialOrd, Clone, Copy, ValueEnum, Serialize, Default)]
pub enum VisibilityLevels {
    Primary = 3, // Always show this to users
    #[default]
    Supplemental = 2, // Show to users as secondary information
    Debug = 1,   // Only show to users if they ask for it
    Hidden = 0,  // Never show to users
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
