use anyhow::anyhow;
use colored::Colorize;
use console::style;
use core::fmt;
use log::{debug, error, info, warn};
use marzano_core::api::{
    AllDone, AnalysisLog, AnalysisLogLevel, CreateFile, DoneFile, FileMatchResult, InputFile,
    Match, MatchReason, MatchResult, PatternInfo, RemoveFile, Rewrite,
};
use marzano_core::constants::DEFAULT_FILE_NAME;
use marzano_messenger::output_mode::OutputMode;
use std::fmt::Display;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::ux::{format_result_diff, indent};
use marzano_messenger::emit::{Messager, VisibilityLevels};

#[derive(Debug)]
pub enum FormattedResult {
    AnalysisLog(AnalysisLog),
    Match(Match),
    InputFile(InputFile),
    DoneFile(DoneFile),
    AllDone(AllDone),
    PatternInfo(PatternInfo),
    Rewrite(Rewrite),
    CreateFile(CreateFile),
    RemoveFile(RemoveFile),
    Compact(MatchResult),
}

impl FormattedResult {
    pub fn new(result: MatchResult, compact: bool) -> Option<FormattedResult> {
        if compact {
            return Some(FormattedResult::Compact(result));
        }
        match result {
            MatchResult::AnalysisLog(log) => Some(FormattedResult::AnalysisLog(log)),
            MatchResult::Match(m) => Some(FormattedResult::Match(m)),
            MatchResult::InputFile(f) => Some(FormattedResult::InputFile(f)),
            MatchResult::DoneFile(f) => Some(FormattedResult::DoneFile(f)),
            MatchResult::AllDone(f) => Some(FormattedResult::AllDone(f)),
            MatchResult::PatternInfo(f) => Some(FormattedResult::PatternInfo(f)),
            MatchResult::Rewrite(r) => Some(FormattedResult::Rewrite(r)),
            MatchResult::CreateFile(r) => Some(FormattedResult::CreateFile(r)),
            MatchResult::RemoveFile(r) => Some(FormattedResult::RemoveFile(r)),
        }
    }
}

fn print_file_ranges<T: FileMatchResult>(item: &mut T, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = item.file_name().bold();
    for range in item.ranges() {
        writeln!(
            f,
            "{}:{}:{} - {}",
            name,
            range.start.line,
            range.start.column,
            T::action()
        )?;
    }
    Ok(())
}

fn print_all_done(item: &AllDone, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(
        f,
        "Processed {} files and found {} matches",
        item.processed, item.found
    )
}

fn print_error_log(log: &AnalysisLog, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let file_prefix = if log.file.is_empty() || log.file == DEFAULT_FILE_NAME {
        "".to_owned()
    } else {
        format!("{}: ", log.file)
    };
    if log.level >= 500 {
        let msg = format!("{}DEBUG - {}", file_prefix, log.message).dimmed();
        writeln!(f, "{}", msg)
    } else if log.level == 310 {
        style(log.message.to_string()).dim().fmt(f)
    } else if log.level == 441 {
        let title = format!("Log in {}", log.file).bold();
        writeln!(f, "{}: {}", title, log.message)?;
        let empty_string = String::new();
        let range = match log.range.as_ref() {
            Some(range) => indent(
                &format!(
                    "- Range: {}:{} - {}:{}",
                    range.start.line, range.start.column, range.end.line, range.end.column
                ),
                2,
            ),
            None => indent("- Range:", 2).to_owned(),
        };
        writeln!(f, "{}", range)?;
        let source = log.source.as_ref().unwrap_or(&empty_string);
        let source = source.dimmed();
        let source = indent(&format!("- Source:\n{}", source), 2);
        writeln!(f, "{}", source)?;
        let syntax_tree = log.syntax_tree.as_ref().unwrap_or(&empty_string).dimmed();
        let syntax_tree = indent(&format!("- Syntax tree:\n{}\n", syntax_tree), 2);
        write!(f, "{}", syntax_tree)
    } else {
        write!(
            f,
            "{}ERROR (code: {}) - {}",
            file_prefix, log.level, log.message
        )
    }
}

/// Implement some log overrides to make CLI usage more friendly
fn humanize_log(log: &mut AnalysisLog, input_pattern: &str) {
    if log.level == 299
        && (log.file.is_empty() || log.file == DEFAULT_FILE_NAME)
        && (input_pattern.ends_with(".yaml") || input_pattern.ends_with(".yml"))
    {
        log.message = format!(
            "{} is a config file, not a pattern. Try applying the pattern by name.",
            input_pattern.bold().red()
        );
    }
}

/// Get a friendly, contextual error for an error log
pub fn get_human_error(mut log: AnalysisLog, input_pattern: &str) -> String {
    humanize_log(&mut log, input_pattern);
    let formatted = FormattedResult::AnalysisLog(log);
    let result = format!("{}", formatted);
    result
}

/// Print a header for a match, with the path and (maybe) a title/explanation
pub fn print_file_header(
    path: &str,
    reason: &Option<MatchReason>,
    f: &mut fmt::Formatter<'_>,
) -> fmt::Result {
    let path_title = path.bold();
    if let Some(r) = reason {
        if let Some(title) = &r.title {
            writeln!(f, "{}: {}", path_title, title)?;
        } else {
            writeln!(f, "{}", path_title)?;
        }
        if let Some(explanation) = &r.explanation {
            writeln!(f, "  {}", explanation.italic())?;
        }
    } else {
        writeln!(f, "{}", path_title)?;
    }
    Ok(())
}

fn get_pretty_workflow_message(
    outcome: &marzano_messenger::workflows::PackagedWorkflowOutcome,
) -> String {
    let emoji = match outcome.get_outcome() {
        marzano_messenger::workflows::OutcomeKind::Success => "✅",
        marzano_messenger::workflows::OutcomeKind::Failure => "❌",
        marzano_messenger::workflows::OutcomeKind::Skipped => "⚪️",
    };
    let message = outcome.message.as_deref().unwrap_or("Workflow finished");
    format!("{} {}", emoji, message)
}

impl fmt::Display for FormattedResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FormattedResult::Compact(result) => {
                match result {
                    MatchResult::AnalysisLog(log) => {
                        print_error_log(log, f)?;
                    }
                    MatchResult::InputFile(_)
                    | MatchResult::PatternInfo(_)
                    | MatchResult::DoneFile(_) => {
                        // These are not shown in compact mode
                    }
                    MatchResult::AllDone(r) => {
                        print_all_done(r, f)?;
                    }
                    MatchResult::Match(r) => print_file_ranges(&mut r.clone(), f)?,
                    MatchResult::Rewrite(r) => print_file_ranges(&mut r.clone(), f)?,
                    MatchResult::CreateFile(r) => print_file_ranges(&mut r.clone(), f)?,
                    MatchResult::RemoveFile(r) => print_file_ranges(&mut r.clone(), f)?,
                }
                Ok(())
            }
            FormattedResult::AnalysisLog(log) => {
                print_error_log(log, f)?;
                Ok(())
            }
            FormattedResult::Match(m) => {
                print_file_header(m.file_name(), &m.reason, f)?;

                let source = m.content();
                match source {
                    Err(e) => {
                        writeln!(f, "Could not read flie: {}", e)?;
                        return Ok(());
                    }
                    Ok(source) => {
                        let ranges = &mut m.ranges.iter();
                        // Iterate through the lines of the file
                        let mut line_number = 0;
                        let mut next_range = ranges.next();
                        let lines = source.lines();
                        for line in lines {
                            line_number += 1;
                            if let Some(range) = next_range {
                                if line_number <= range.end.line {
                                    let overlap =
                                        range.get_line_range(line_number, line.len() as u32);
                                    match overlap {
                                        None => {}
                                        Some((start_col, end_col)) => {
                                            // This line is part of the match
                                            let prefix = &line[0..start_col];
                                            let highlight = &line[start_col..end_col];
                                            let suffix = &line[end_col..];
                                            writeln!(
                                                f,
                                                "{:6}  {}{}{}",
                                                line_number,
                                                prefix.dimmed(),
                                                highlight.blue().bold(),
                                                suffix.dimmed()
                                            )?;
                                        }
                                    }
                                } else {
                                    // We are beyond the current range, search for a new range that is after or on this line
                                    while let Some(range) = next_range {
                                        if range.end.line >= line_number {
                                            break;
                                        }
                                        next_range = ranges.next();
                                    }
                                }
                            }
                        }
                    }
                }

                Ok(())
            }
            FormattedResult::InputFile(item) => {
                write!(f, "Parsed input file: {}", item.source_file)
            }
            FormattedResult::AllDone(item) => print_all_done(item, f),
            FormattedResult::DoneFile(_) => Ok(()),
            FormattedResult::Rewrite(item) => {
                let path_name = if item.original.source_file == item.rewritten.source_file {
                    item.file_name().to_string()
                } else {
                    format!(
                        "{} -> {}",
                        item.original.source_file, item.rewritten.source_file
                    )
                };
                print_file_header(&path_name, &item.reason, f)?;

                let result: MatchResult = item.clone().into();
                let diff = format_result_diff(&result, None);
                write!(f, "{}", diff)?;
                Ok(())
            }
            FormattedResult::CreateFile(item) => {
                print_file_header(item.file_name(), &item.reason, f)?;
                let result: MatchResult = item.clone().into();
                let diff = format_result_diff(&result, None);
                write!(f, "{}", diff)?;
                Ok(())
            }
            FormattedResult::RemoveFile(item) => {
                print_file_header(item.file_name(), &item.reason, f)?;
                let result: MatchResult = item.clone().into();
                let diff = format_result_diff(&result, None);
                write!(f, "{}", diff)?;
                Ok(())
            }
            fallback => {
                write!(f, "Unsupported result type: {:?}", fallback)
            }
        }
    }
}

pub struct FormattedMessager<'a> {
    writer: Option<Arc<Mutex<Box<dyn Write + Send + 'a>>>>,
    mode: OutputMode,
    interactive: bool,
    total_accepted: usize,
    total_rejected: usize,
    total_supressed: usize,
    input_pattern: String,
    min_level: VisibilityLevels,
    workflow_done: bool,
}

impl<'a> FormattedMessager<'_> {
    pub fn new(
        writer: Option<Box<dyn Write + Send + 'a>>,
        mode: OutputMode,
        interactive: bool,
        input_pattern: String,
        min_level: VisibilityLevels,
    ) -> FormattedMessager<'a> {
        FormattedMessager {
            writer: writer.map(|w| Arc::new(Mutex::new(w))),
            mode,
            interactive,
            total_accepted: 0,
            total_rejected: 0,
            total_supressed: 0,
            input_pattern,
            min_level,
            workflow_done: false,
        }
    }
}

impl Messager for FormattedMessager<'_> {
    fn get_min_level(&self) -> VisibilityLevels {
        self.min_level
    }

    fn raw_emit(&mut self, message: &MatchResult) -> anyhow::Result<()> {
        if self.interactive && !(self.mode == OutputMode::None) {
            if let MatchResult::AllDone(item) = message {
                info!(
                    "Processed {} files. Accepted {} rewrites, rejected {} rewrites and suppressed {} rewrites.",
                    item.processed, self.total_accepted, self.total_rejected, self.total_supressed
                );
                return Ok(());
            }
        }

        match self.mode {
            OutputMode::None => {}
            OutputMode::Standard | OutputMode::Compact => {
                let mut message = message.clone();

                // Override the message if the pattern name is a file name
                if let MatchResult::AnalysisLog(ref mut log) = message {
                    humanize_log(log, &self.input_pattern);
                }
                let formatted = FormattedResult::new(message, self.mode == OutputMode::Compact);
                if let Some(formatted) = formatted {
                    if let Some(writer) = &mut self.writer {
                        let mut writer =
                            writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
                        writeln!(writer, "{}", formatted)?;
                    } else {
                        info!("{}", formatted);
                    }
                }
            }
        }

        Ok(())
    }

    fn track_accept(&mut self, _accepted: &MatchResult) -> anyhow::Result<()> {
        self.total_accepted += 1;
        Ok(())
    }
    fn track_reject(&mut self, _rejected: &MatchResult) -> anyhow::Result<()> {
        self.total_rejected += 1;
        Ok(())
    }
    fn track_supress(&mut self, _supressed: &MatchResult) -> anyhow::Result<()> {
        self.total_supressed += 1;
        Ok(())
    }

    fn finish_workflow(
        &mut self,
        outcome: &marzano_messenger::workflows::PackagedWorkflowOutcome,
    ) -> anyhow::Result<()> {
        if self.workflow_done {
            // If we already finished once, short-circuit it
            return Ok(());
        }

        if let Some(writer) = &mut self.writer {
            let mut writer = writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
            writeln!(writer, "{}", get_pretty_workflow_message(outcome))?;
        } else {
            log::info!("{}", get_pretty_workflow_message(outcome));
        }

        self.workflow_done = true;

        Ok(())
    }

    fn emit_log(&mut self, log: &marzano_messenger::SimpleLogMessage) -> anyhow::Result<()> {
        if let Some(writer) = &mut self.writer {
            let mut writer = writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
            writeln!(writer, "[{:?}] {}", log.level, log.message)?;
        } else {
            match log.level {
                AnalysisLogLevel::Debug => {
                    debug!("{}", log.message);
                }
                AnalysisLogLevel::Info => {
                    info!("{}", log.message);
                }
                AnalysisLogLevel::Warn => {
                    warn!("{}", log.message);
                }
                AnalysisLogLevel::Error => {
                    error!("{}", log.message);
                }
            }
        }
        Ok(())
    }
}

/// Prints the transformed files themselves, with no metadata
pub struct TransformedMessenger<'a> {
    writer: Option<Arc<Mutex<Box<dyn Write + Send + 'a>>>>,
    total_accepted: usize,
    total_rejected: usize,
    total_supressed: usize,
}

impl<'a> TransformedMessenger<'_> {
    pub fn new(writer: Option<Box<dyn Write + Send + 'a>>) -> TransformedMessenger<'a> {
        TransformedMessenger {
            writer: writer.map(|w| Arc::new(Mutex::new(w))),
            total_accepted: 0,
            total_rejected: 0,
            total_supressed: 0,
        }
    }
}

impl Messager for TransformedMessenger<'_> {
    fn get_min_level(&self) -> VisibilityLevels {
        VisibilityLevels::Primary
    }

    fn raw_emit(&mut self, message: &MatchResult) -> anyhow::Result<()> {
        match message {
            MatchResult::PatternInfo(_)
            | MatchResult::AllDone(_)
            | MatchResult::InputFile(_)
            | MatchResult::DoneFile(_) => {
                // ignore these
            }
            MatchResult::Match(message) => {
                info!("Matched file {}", message.file_name());
            }
            MatchResult::Rewrite(file) => {
                // Write the file contents to the output
                if let Some(writer) = &mut self.writer {
                    let mut writer = writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
                    writeln!(writer, "{}", file.content().unwrap_or_default())?;
                } else {
                    info!("{}", file.content().unwrap_or_default());
                }
            }
            MatchResult::CreateFile(file) => {
                // Write the file contents to the output
                if let Some(writer) = &mut self.writer {
                    let mut writer = writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
                    writeln!(writer, "{}", file.content().unwrap_or_default())?;
                } else {
                    info!("{}", file.content().unwrap_or_default());
                }
            }
            MatchResult::RemoveFile(file) => {
                info!("File {} should be removed", file.original.source_file);
            }
            MatchResult::AnalysisLog(_) => {
                // TODO: should this go somewhere else
                let formatted = FormattedResult::new(message.clone(), false);
                if let Some(formatted) = formatted {
                    info!("{}", formatted);
                }
            }
        }

        Ok(())
    }

    fn track_accept(&mut self, _accepted: &MatchResult) -> anyhow::Result<()> {
        self.total_accepted += 1;
        Ok(())
    }
    fn track_reject(&mut self, _rejected: &MatchResult) -> anyhow::Result<()> {
        self.total_rejected += 1;
        Ok(())
    }
    fn track_supress(&mut self, _supressed: &MatchResult) -> anyhow::Result<()> {
        self.total_supressed += 1;
        Ok(())
    }

    fn emit_log(&mut self, log: &marzano_messenger::SimpleLogMessage) -> anyhow::Result<()> {
        log::debug!("Log received over RPC: {:?}", log);
        Ok(())
    }
}
