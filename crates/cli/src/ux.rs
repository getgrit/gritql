use anyhow::Result;
use colored::Colorize;
use core::fmt;
use grit_util::{Position, Range};
use log::info;
use marzano_core::{
    api::{EnforcementLevel, MatchResult},
    fs::extract_ranges,
};
use marzano_gritmodule::{config::ResolvedGritDefinition, testing::SampleTestResult};
use similar::{ChangeTag, TextDiff};
use std::collections::HashMap;

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

#[derive(Debug, serde::Deserialize)]
pub enum Format {
    #[serde(rename = "table")]
    Table,
}

#[derive(Debug, serde::Deserialize)]
pub struct Table {
    #[allow(unused)]
    pub format: Format,
    pub headers: Option<Vec<String>>,
    pub data: Vec<Vec<String>>,
}

pub fn format_table(table: &Table) -> String {
    if table.data.is_empty() {
        return String::new();
    }

    // Get max width of each column
    let column_count = table.data[0].len();
    let mut column_widths = vec![0; column_count];

    // Account for headers in column widths
    if let Some(headers) = &table.headers {
        for (i, header) in headers.iter().enumerate() {
            column_widths[i] = column_widths[i].max(header.len());
        }
    }

    // Account for data in column widths
    for row in &table.data {
        for (i, cell) in row.iter().enumerate() {
            column_widths[i] = column_widths[i].max(cell.len());
        }
    }

    // Build formatted table string
    let mut output = String::new();
    // Print headers if present
    if let Some(headers) = &table.headers {
        let formatted_headers = headers
            .iter()
            .enumerate()
            .map(|(i, header)| {
                format!(
                    "{:<width$}",
                    header.bold().yellow(),
                    width = column_widths[i]
                )
            })
            .collect::<Vec<_>>()
            .join("  ");

        output.push_str(&formatted_headers);
        output.push('\n');
    }

    // Print data rows
    for row in &table.data {
        let formatted_row = row
            .iter()
            .enumerate()
            .map(|(i, cell)| format!("{:<width$}", cell, width = column_widths[i]))
            .collect::<Vec<_>>()
            .join("  ");

        output.push_str(&formatted_row);
        output.push('\n');
    }

    output
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

pub fn format_result_diff(r: &MatchResult, src: Option<&str>) -> DiffString {
    let old_content = match src {
        Some(s) => s.to_string(),
        None => r.extract_original_content().unwrap_or_default().to_string(),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_table() {
        let table = Table {
            format: Format::Table,
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
            data: vec![
                vec!["Alice".to_string(), "25".to_string()],
                vec!["Bob".to_string(), "30".to_string()],
            ],
        };

        let output = format_table(&table);
        let expected = format!(
            "{}  {}\nAlice  25 \nBob    30 \n",
            "Name ".bold().yellow(),
            "Age".bold().yellow()
        );

        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_table_no_headers() {
        let table = Table {
            format: Format::Table,
            headers: None,
            data: vec![
                vec!["Alice".to_string(), "25".to_string()],
                vec!["Bob".to_string(), "30".to_string()],
            ],
        };

        let output = format_table(&table);
        let expected = "Alice  25\nBob    30\n";

        assert_eq!(output, expected);
    }

    #[test]
    fn test_format_empty_table() {
        let table = Table {
            format: Format::Table,
            headers: Some(vec!["Name".to_string(), "Age".to_string()]),
            data: vec![],
        };

        let output = format_table(&table);
        assert_eq!(output, "");
    }
}
