use std::{collections::HashMap, path::PathBuf};

use marzano_core::{api::EnforcementLevel, fs::extract_ranges};
use marzano_util::position::Range;
use serde::Serialize;

use crate::ux::CheckResult;

#[derive(Debug, Serialize)]
struct SemgrepPosition {
    line: usize,
    col: usize,
    offset: usize,
}

impl SemgrepPosition {
    fn pair_from_range(range: &Range) -> (Self, Self) {
        (
            Self {
                line: range.start.line as usize,
                col: range.start.column as usize,
                offset: range.start_byte as usize,
            },
            Self {
                line: range.end.line as usize,
                col: range.end.column as usize,
                offset: range.end_byte as usize,
            },
        )
    }
}

#[derive(Debug, Serialize)]
struct SemgrepExtra<'a> {
    message: Option<&'a str>,
    severity: Option<EnforcementLevel>,
}

#[derive(Debug, Serialize)]
struct SemgrepResult<'a> {
    check_id: String,
    // This is not part of Semgrep's schema, but included for continuity with our design
    local_name: &'a str,
    start: SemgrepPosition,
    end: SemgrepPosition,
    path: &'a String,
    extra: SemgrepExtra<'a>,
}

#[derive(Debug, Serialize)]
struct SemgrepScan<'a> {
    paths: Vec<PathBuf>,
    results: Vec<SemgrepResult<'a>>,
}

pub fn log_check_json(check_results: HashMap<String, Vec<CheckResult<'_>>>, files: Vec<PathBuf>) {
    let mut semgrep_results: Vec<SemgrepResult> = Vec::new();
    for (path, results) in check_results.iter() {
        for result in results {
            let full_name = &result.pattern.module.name();
            let language = &result.pattern.language;
            let ranges = match extract_ranges(&result.result) {
                Some(ranges) => ranges,
                None => continue,
            };
            let local_name = result.pattern.name();
            let check_id = format!("{}#{}/{}", full_name, local_name, language);
            for range in ranges {
                let (start, end) = SemgrepPosition::pair_from_range(range);
                let extra = SemgrepExtra {
                    message: result.pattern.description(),
                    severity: Some(result.pattern.level()),
                };
                let semgrep_result = SemgrepResult {
                    check_id: check_id.clone(),
                    local_name,
                    start,
                    end,
                    path,
                    extra,
                };
                semgrep_results.push(semgrep_result);
            }
        }
    }
    let scan = SemgrepScan {
        paths: files,
        results: semgrep_results,
    };
    let json = serde_json::to_string(&scan).unwrap();
    log::info!("{}", json);
}
