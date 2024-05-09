use anyhow::anyhow;
use colored::Colorize;
use console::style;
use core::fmt;
use log::info;
use marzano_core::api::{
    AllDone, AnalysisLog, CreateFile, DoneFile, FileMatchResult, InputFile, Match, MatchResult,
    PatternInfo, RemoveFile, Rewrite,
};
use marzano_core::constants::DEFAULT_FILE_NAME;
use marzano_messenger::output_mode::OutputMode;
use std::fmt::Display;
use std::fs::read_to_string;
use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use crate::ux::{format_result_diff, indent};
use marzano_messenger::emit::Messager;

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
                let path_title = m.file_name().bold();
                writeln!(f, "{}", path_title)?;
                let source = read_to_string(m.file_name());
                match source {
                    Err(e) => {
                        writeln!(f, "Could not read file: {}", e)?;
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
                let path_title = path_name.bold();
                writeln!(f, "{}", path_title)?;
                let result: MatchResult = item.clone().into();
                let diff = format_result_diff(&result, None);
                write!(f, "{}", diff)?;
                Ok(())
            }
            FormattedResult::CreateFile(item) => {
                let path_title = item.file_name().bold();
                let result: MatchResult = item.clone().into();
                writeln!(f, "{}", path_title)?;
                let diff = format_result_diff(&result, None);
                write!(f, "{}", diff)?;
                Ok(())
            }
            FormattedResult::RemoveFile(item) => {
                let path_title = item.file_name().bold();
                writeln!(f, "{}", path_title)?;
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
}

impl<'a> FormattedMessager<'_> {
    pub fn new(
        writer: Option<Box<dyn Write + Send + 'a>>,
        mode: OutputMode,
        interactive: bool,
        input_pattern: String,
    ) -> FormattedMessager<'a> {
        FormattedMessager {
            writer: writer.map(|w| Arc::new(Mutex::new(w))),
            mode,
            interactive,
            total_accepted: 0,
            total_rejected: 0,
            total_supressed: 0,
            input_pattern,
        }
    }
}

impl Messager for FormattedMessager<'_> {
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
}

/// Prints the transformed files themselves, with no metadata
pub struct TransformedMessenger<'a> {
    writer: Option<Arc<Mutex<Box<dyn Write + Send + 'a>>>>,
    interactive: bool,
    total_accepted: usize,
    total_rejected: usize,
    total_supressed: usize,
}

impl<'a> TransformedMessenger<'_> {
    pub fn new(
        writer: Option<Box<dyn Write + Send + 'a>>,
        interactive: bool,
    ) -> TransformedMessenger<'a> {
        TransformedMessenger {
            writer: writer.map(|w| Arc::new(Mutex::new(w))),
            interactive,
            total_accepted: 0,
            total_rejected: 0,
            total_supressed: 0,
        }
    }
}

impl Messager for TransformedMessenger<'_> {
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
                    writeln!(writer, "{}", file.rewritten.content)?;
                } else {
                    info!("{}", file.rewritten.content);
                }
            }
            MatchResult::CreateFile(file) => {
                // Write the file contents to the output
                if let Some(writer) = &mut self.writer {
                    let mut writer = writer.lock().map_err(|_| anyhow!("Output lock poisoned"))?;
                    writeln!(writer, "{}", file.rewritten.content)?;
                } else {
                    info!("{}", file.rewritten.content);
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
}
