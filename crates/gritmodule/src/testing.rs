use marzano_core::pattern::{
    api::{derive_log_level, is_match, AnalysisLogLevel, MatchResult},
    Problem,
};
use marzano_language::target_language::TargetLanguage;
use marzano_util::runtime::ExecutionContext;

use marzano_util::rich_path::RichFile;
use serde::{Deserialize, Serialize};

use crate::config::{
    GritPatternSample, GritPatternTestConfig, GritPatternTestInfo, ResolvedGritDefinition,
};

fn map_pattern_to_test_info(pattern: &mut ResolvedGritDefinition) -> GritPatternTestInfo {
    let samples = pattern.config.samples.take();
    GritPatternTestInfo {
        body: pattern.body.clone(),
        config: GritPatternTestConfig {
            path: Some(pattern.config.path.clone()),
            samples,
        },
        local_name: Some(pattern.local_name.clone()),
    }
}

pub fn collect_testable_patterns(
    mut patterns: Vec<ResolvedGritDefinition>,
) -> Vec<GritPatternTestInfo> {
    let testable_patterns: Vec<GritPatternTestInfo> =
        patterns.iter_mut().map(map_pattern_to_test_info).collect();
    testable_patterns
}

const SAMPLE_NAME_LENGTH: usize = 50;

pub fn get_sample_name(sample: &GritPatternSample) -> String {
    if let Some(ref name) = sample.name {
        return name.clone();
    }

    let line_break_index = sample.input.find('\n');
    let max_length = match line_break_index {
        Some(index) => std::cmp::min(SAMPLE_NAME_LENGTH, index),
        None => SAMPLE_NAME_LENGTH,
    };

    if sample.input.len() > max_length {
        return format!("{}…", &sample.input[..max_length]);
    }

    sample.input.clone()
}

fn infer_rich_files_from_content(language: &TargetLanguage, content: &str) -> Vec<RichFile> {
    let mut files: Vec<RichFile> = Vec::new();
    let mut current_filename: Option<String> = None;
    let mut current_content = String::new();

    if content.is_empty() {
        return vec![RichFile {
            path: format!("test-file-0.{}", language.get_default_extension()).to_string(),
            content: String::new(),
        }];
    }

    for line in content.lines() {
        if let Some(stripped) = line.strip_prefix("// @filename: ") {
            // Finish up the current file (if any)
            if let Some(filename) = current_filename.take() {
                files.push(RichFile {
                    path: filename,
                    content: current_content.clone(),
                });
                current_content.clear();
            }
            current_filename = Some(stripped.to_string());
        } else {
            // If we haven't found a file declaration, return early
            if current_filename.is_none() {
                return vec![RichFile {
                    path: format!(
                        "test-file-{}.{}",
                        files.len(),
                        language.get_default_extension()
                    ),
                    content: content.to_string(),
                }];
            }
            current_content.push_str(line);
            current_content.push('\n');
        }
    }

    // Add the final file (if any)
    if let Some(filename) = current_filename {
        files.push(RichFile {
            path: filename,
            content: current_content,
        });
    }

    files
}

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum GritTestResultState {
    /// Passed, but the ouput required formatting to match the expected output
    PassWithFormat,
    /// Output is an exact match with the expected output
    Pass,
    /// Match found, but output does not match the expected output
    FailedOutput,
    /// No match found in the input, or match found when none was expected
    FailedMatch,
    /// Compilation or execution error
    FailedPattern,
}

#[derive(Debug, Serialize, Clone)]
pub struct SampleTestResult {
    pub matches: Vec<MatchResult>,
    pub state: GritTestResultState,
    pub message: Option<String>,
    pub expected_output: Option<String>,
    pub actual_output: Option<String>,
    /// The expected outputs, if any and useful
    pub expected_outputs: Option<Vec<RichFile>>,
    /// The actual outputs, if any and useful
    pub actual_outputs: Option<Vec<RichFile>>,
}

impl SampleTestResult {
    pub fn new_passing(matches: Vec<MatchResult>, required_format: bool) -> SampleTestResult {
        SampleTestResult {
            matches,
            state: if required_format {
                GritTestResultState::PassWithFormat
            } else {
                GritTestResultState::Pass
            },
            message: None,
            expected_output: None,
            actual_output: None,
            expected_outputs: None,
            actual_outputs: None,
        }
    }

    pub fn is_pass(&self) -> bool {
        self.state == GritTestResultState::Pass || self.state == GritTestResultState::PassWithFormat
    }

    pub fn is_pure_pass(&self) -> bool {
        self.state == GritTestResultState::Pass
    }

    pub fn should_try_formatting(&self) -> bool {
        self.state == GritTestResultState::FailedOutput
            && self.expected_outputs.is_some()
            && self.actual_outputs.is_some()
    }

    pub fn requires_formatting(&self) -> bool {
        self.state == GritTestResultState::PassWithFormat
    }
}

pub fn test_pattern_sample(
    compiled: &Problem,
    sample: &GritPatternSample,
    runtime: ExecutionContext,
) -> SampleTestResult {
    let inferred_inputs = infer_rich_files_from_content(&compiled.language, &sample.input);

    let mut matches: Vec<MatchResult> = Vec::new();

    let rich_files = inferred_inputs
        .iter()
        .map(|input| RichFile::new(input.path.to_owned(), input.content.to_owned()))
        .collect::<Vec<_>>();
    let res = compiled.execute_files(&rich_files, &runtime);

    for result in res.into_iter() {
        if is_match(&result) {
            matches.push(result);
        } else if let MatchResult::AnalysisLog(log) = result {
            let level = derive_log_level(&log);
            matches.push(MatchResult::AnalysisLog(log.clone()));
            match level {
                AnalysisLogLevel::Error | AnalysisLogLevel::Warn => {
                    return SampleTestResult {
                        matches,
                        state: GritTestResultState::FailedPattern,
                        message: Some(format!("Received error: {}", log.message)),
                        expected_output: None,
                        actual_output: None,
                        expected_outputs: None,
                        actual_outputs: None,
                    };
                }
                _ => {}
            }
        }
    }

    let mut raw_actual_outputs: Vec<RichFile> = Vec::new();

    // We only want mutation results
    for item in &matches {
        match item {
            MatchResult::Rewrite(r) => {
                raw_actual_outputs.push(RichFile {
                    path: r.original.source_file.clone(),
                    content: r.rewritten.content.clone(),
                });
            }
            MatchResult::CreateFile(r) => {
                raw_actual_outputs.push(RichFile {
                    path: r.rewritten.source_file.clone(),
                    content: r.rewritten.content.clone(),
                });
            }
            MatchResult::Match(r) => {
                if sample.input.contains("// @filename:") {
                    continue;
                }
                raw_actual_outputs.push(RichFile {
                    path: r.source_file.clone(),
                    content: sample.input.clone(),
                });
            }
            _ => {}
        }
    }

    // First handle the case where we have no output
    if raw_actual_outputs.is_empty() {
        if sample.output.is_none() {
            return SampleTestResult::new_passing(matches, false);
        } else {
            return SampleTestResult {
                matches,
                state: GritTestResultState::FailedMatch,
                message: Some("Expected output, but got none".to_string()),
                expected_output: sample.output.clone(),
                actual_output: None,
                expected_outputs: None,
                actual_outputs: None,
            };
        }
    }

    let sample_output = if let Some(output) = sample.output.as_ref() {
        output
    } else {
        return SampleTestResult {
            matches,
            state: GritTestResultState::FailedMatch,
            message: Some("Expected no matches, but got one".to_string()),
            expected_output: None,
            actual_output: None,
            expected_outputs: None,
            actual_outputs: None,
        };
    };

    let mut raw_expected_outputs = infer_rich_files_from_content(&compiled.language, sample_output);

    if raw_actual_outputs.len() < raw_expected_outputs.len() && compiled.is_multifile {
        for file in rich_files.iter() {
            if raw_actual_outputs.iter().any(|f| f.path == file.path) {
                continue;
            }
            raw_actual_outputs.push(RichFile {
                path: file.path.to_string(),
                content: file.content.to_string(),
            });
        }
    }

    // Make sure the lengths match
    if raw_actual_outputs.len() != raw_expected_outputs.len() {
        return SampleTestResult {
            matches,
            state: GritTestResultState::FailedOutput,
            message: Some(format!(
                "Expected {} output files, got {}",
                raw_expected_outputs.len(),
                raw_actual_outputs.len()
            )),
            expected_output: None,
            actual_output: None,
            expected_outputs: None,
            actual_outputs: None,
        };
    }

    let candidate_output = has_output_mismatch(&mut raw_expected_outputs, &mut raw_actual_outputs);
    match candidate_output {
        None => SampleTestResult::new_passing(matches, false),
        Some(MismatchInfo::Content(output)) => SampleTestResult {
            matches,
            state: GritTestResultState::FailedOutput,
            message: Some(
                "Actual output doesn't match expected output and formatting is disabled"
                    .to_string(),
            ),
            expected_output: Some(output.expected),
            actual_output: Some(output.actual),
            expected_outputs: Some(raw_expected_outputs),
            actual_outputs: Some(raw_actual_outputs),
        },
        Some(MismatchInfo::Path(output)) => SampleTestResult {
            matches,
            state: GritTestResultState::FailedOutput,
            message: Some(format!(
                "Expected output file {} but got {}",
                output.expected, output.actual
            )),
            expected_output: Some(output.expected),
            actual_output: Some(output.actual),
            expected_outputs: None,
            actual_outputs: None,
        },
    }
}

#[derive(Debug)]
pub enum MismatchInfo {
    Path(OutputInfo),
    Content(OutputInfo),
}

#[derive(Debug)]
pub struct OutputInfo {
    pub expected: String,
    pub actual: String,
}

pub fn has_output_mismatch(
    expected: &mut [RichFile],
    actual: &mut [RichFile],
) -> Option<MismatchInfo> {
    // TODO: use a more efficient sorting approach make this more efficient
    expected.sort();
    actual.sort();

    // Now compare the contents
    for (actual, expected) in actual.iter().zip(expected.iter()) {
        if actual.path != expected.path {
            return Some(MismatchInfo::Path(OutputInfo {
                expected: expected.path.clone(),
                actual: actual.path.clone(),
            }));
        }
        if actual.content != expected.content {
            return Some(MismatchInfo::Content(OutputInfo {
                expected: expected.content.clone(),
                actual: actual.content.clone(),
            }));
        }
    }
    None
}
