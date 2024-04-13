use anyhow::Result;
use core::fmt;
use std::{collections::HashMap, fs::read_to_string};

use colored::Colorize;
use log::info;
use marzano_core::{
    api::{EnforcementLevel, MatchResult},
    fs::extract_ranges,
};
use marzano_gritmodule::{
    config::ResolvedGritDefinition, testing::SampleTestResult, utils::extract_path,
};
use marzano_util::position::{Position, Range};
use similar::{ChangeTag, TextDiff};

use crate::analyze::{extract_rewritten_content, group_checks};

static STANDARD_INDENT: usize = 4;

pub fn indent(s: &str, amount: usize) -> String {
    let padding: String = " ".repeat(amount);
    s.lines()
        .map(|line| format!("{}{}", padding, line))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn heading(s: &str) -> String {
    format!("\n{}", s.bold().yellow())
}

#[derive(Debug)]
pub struct CheckResult<'a> {
    pub pattern: &'a ResolvedGritDefinition,
    pub result: MatchResult,
}

fn log_check_result(range: &Range, result: &CheckResult, fix: bool) {
    let location = format!("{}:{}", range.start.line, range.start.column);
    let kind = match result.result {
        MatchResult::Rewrite(_) => "rewrite",
        _ => "match",
    };

    let fix_text = if fix {
        "Fixed! ✓    ".to_string().green()
    } else {
        "Fix available.    ".to_string().blue()
    };

    info!(
        "  {}    {}    {}    {}{}",
        location.dimmed(),
        kind.red(),
        result.pattern.description().map(|s| s.trim()).unwrap_or(""),
        if kind == "rewrite" {
            fix_text.to_string()
        } else {
            "".to_string()
        },
        result.pattern.local_name.dimmed()
    );
}

pub fn log_file(file: &str, results: &[CheckResult], fix: bool) {
    if results.is_empty() {
        return;
    }

    info!("{}", file.underline());

    for result in results {
        let range = extract_ranges(&result.result);
        if range.is_none() || range.is_some_and(|r| r.is_empty()) {
            log_check_result(
                &Range::new(Position::new(1, 1), Position::new(1, 1), 0, 0),
                result,
                fix,
            );
        }
        for r in range.unwrap() {
            log_check_result(r, result, fix);
        }
    }

    info!("\n");
}

pub fn print_config(patterns: &Vec<ResolvedGritDefinition>, results: Vec<&CheckResult<'_>>) {
    let grouped_results = group_checks(&results);

    info!("{}", "PATTERNS".underline());

    for pattern in patterns {
        let pattern_results = match grouped_results.get(&pattern.local_name) {
            Some(r) => r,
            None => continue,
        };
        let message = if !pattern_results.is_empty() {
            format!("{} {}", "✗".red(), pattern.local_name)
        } else {
            format!("{} {}", "✓".green(), pattern.local_name)
        };

        info!("  {}", message);
    }
}

pub struct DiffString {
    diff: String,
}

impl fmt::Display for DiffString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let indented = indent(&self.diff, STANDARD_INDENT);
        let lines = indented.lines();
        for line in lines {
            let control = line.get(4..5);
            let colored_line = match control {
                Some("-") => line.red(),
                Some("+") => line.green(),
                _ => line.dimmed(),
            };
            writeln!(f, "{}", colored_line)?;
        }
        Ok(())
    }
}

fn format_diff(expected: &str, actual: &str) -> DiffString {
    let mut output = String::new();
    let diff = TextDiff::from_lines(expected, actual);

    let mut unified_diff = diff.unified_diff();
    unified_diff.context_radius(3);

    for hunk in unified_diff.iter_hunks() {
        for change in hunk.iter_changes() {
            match change.tag() {
                ChangeTag::Delete => output.push_str(&format!("-{}", change)),
                ChangeTag::Insert => output.push_str(&format!("+{}", change)),
                ChangeTag::Equal => output.push_str(&format!(" {}", change)),
            }
        }
    }

    output.push('\n');

    DiffString { diff: output }
}

pub fn extract_original_content(r: &MatchResult) -> Option<String> {
    match extract_path(r) {
        Some(p) => match read_to_string(p) {
            Ok(c) => Some(c),
            Err(_) => None,
        },
        None => None,
    }
}

pub fn format_result_diff(r: &MatchResult, src: Option<&str>) -> DiffString {
    let old_content = match src {
        Some(s) => s.to_string(),
        None => extract_original_content(r).unwrap_or_default(),
    };
    let default_rewritten = String::new();
    let new_content = match extract_rewritten_content(r) {
        Some(c) => c,
        None => &default_rewritten,
    };
    format_diff(&old_content, new_content)
}

pub fn log_test_diff(test: &SampleTestResult) {
    if test.is_pass() {
        return;
    }
    let (expected, actual) = match (&test.expected_output, &test.actual_output) {
        (Some(e), Some(a)) => (e, a),
        _ => return,
    };
    let diff = format_diff(expected, actual);
    info!("{}", diff);
}

pub fn get_check_summary(
    results: &[&CheckResult<'_>],
) -> Result<(HashMap<EnforcementLevel, usize>, String)> {
    let mut grouped_results: HashMap<EnforcementLevel, usize> = HashMap::new();

    for result in results.iter() {
        let key = result.pattern.level();
        *grouped_results.entry(key).or_default() += 1;
    }

    let summary = format!(
        "Found {} errors, {} warnings, and {} notices",
        grouped_results.get(&EnforcementLevel::Error).unwrap_or(&0),
        grouped_results.get(&EnforcementLevel::Warn).unwrap_or(&0),
        grouped_results.get(&EnforcementLevel::Info).unwrap_or(&0),
    );

    Ok((grouped_results, summary))
}
