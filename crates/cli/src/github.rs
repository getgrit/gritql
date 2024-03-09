use std::fs::OpenOptions;
use std::io::prelude::*;

use anyhow::{Context as _, Result};

use marzano_core::{fs::extract_ranges, pattern::api::EnforcementLevel};
use marzano_gritmodule::config::ResolvedGritDefinition;
use marzano_gritmodule::utils::extract_path;
use marzano_util::position::Range;

use crate::analyze::group_checks;
use crate::ux::CheckResult;

fn format_level(level: &EnforcementLevel) -> String {
    match level {
        EnforcementLevel::Error => "error".to_string(),
        EnforcementLevel::Warn => "warning".to_string(),
        EnforcementLevel::Info => "notice".to_string(),
        EnforcementLevel::None => "notice".to_string(),
    }
}

fn print_one(
    file: &str,
    range: Option<Range>,
    message: &str,
    title: Option<&str>,
    level: &EnforcementLevel,
) {
    let level = format_level(level);
    let mut params = format!("file={}", file);
    if let Some(range) = range {
        params.push_str(
            format!(
                ",line={},col={},endLine={},endColumn={}",
                range.start.line, range.start.column, range.end.line, range.end.column
            )
            .as_str(),
        );
    }
    if let Some(title) = title {
        params.push_str(format!(",title={}", title).as_str());
    }
    println!("::{} {}::{}", level, params, message);
}

pub fn log_check_annotations(check_results: &Vec<&CheckResult<'_>>) {
    for result in check_results {
        let pattern = result.pattern;
        let result = &result.result;

        let level = pattern.level();
        let file = match extract_path(result).map(|p| p.as_str()) {
            Some(path) => path,
            None => continue,
        };
        let (title, message) = match pattern.description() {
            Some(description) => (Some(pattern.name()), description),
            None => (None, pattern.name()),
        };

        match extract_ranges(result) {
            Some(ranges) => {
                for range in ranges {
                    print_one(file, Some(*range), message, title, &level);
                }
            }
            None => print_one(file, None, message, title, &level),
        };
    }
}

pub fn write_check_summary(
    file: &String,
    patterns: &[&ResolvedGritDefinition],
    results: &[&CheckResult<'_>],
) -> Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(file)
        .context(format!("Failed to open output file: {}", file))?;

    let grouped_checks = group_checks(results);

    writeln!(file, "# Summary\n")?;
    writeln!(file, "| Check | Description | Level | Total Findings |")?;
    writeln!(file, "| --- | --- | --- | --- |")?;

    let mut sorted_patterns = patterns.to_owned();
    sorted_patterns.sort();

    for pattern in sorted_patterns {
        let name = pattern.name();
        let description = pattern.description().unwrap_or("-");
        let level = pattern.level();
        let count = grouped_checks.get(name).map(|v| v.len()).unwrap_or(0);

        writeln!(
            file,
            "| {} | {} | {} | {} |",
            name, description, level, count
        )?;
    }

    Ok(())
}
