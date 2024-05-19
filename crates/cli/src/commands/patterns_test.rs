use colored::Colorize;
use dashmap::{DashMap, ReadOnlyView};
use log::{debug, info};

use marzano_core::api::MatchResult;
use marzano_gritmodule::config::{GritPatternSample, GritPatternTestInfo};
use marzano_gritmodule::formatting::format_rich_files;
use marzano_gritmodule::markdown::replace_sample_in_md_file;
use marzano_gritmodule::patterns_directory::PatternsDirectory;
use marzano_gritmodule::testing::{
    collect_testable_patterns, get_sample_name, has_output_mismatch, test_pattern_sample,
    GritTestResultState, MismatchInfo, SampleTestResult,
};

use marzano_language::target_language::PatternLanguage;

use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;

use crate::flags::{GlobalFormatFlags, OutputFormat};
use crate::resolver::{
    get_grit_files_from_flags_or_cwd, resolve_from_cwd, GritModuleResolver, Source,
};
use crate::result_formatting::FormattedResult;
use crate::updater::Updater;
use crate::ux::{indent, log_test_diff};
use marzano_messenger::emit::{get_visibility, VisibilityLevels};

use super::patterns::PatternsTestArgs;

use anyhow::{anyhow, bail, Context as _, Result};

pub async fn get_marzano_pattern_test_results(
    patterns: Vec<GritPatternTestInfo>,
    libs: &PatternsDirectory,
    args: PatternsTestArgs,
    output: OutputFormat,
) -> Result<()> {
    let resolver = GritModuleResolver::new();

    let final_results: DashMap<String, Vec<WrappedResult>> = DashMap::new();
    let unformatted_results: DashMap<PatternLanguage, Vec<WrappedResult>> = DashMap::new();

    let runtime = Updater::from_current_bin().await?.get_context()?;

    let test_reports = patterns
        .par_iter()
        .enumerate()
        .map(|(index, pattern)| {
            let lang = PatternLanguage::get_language(&pattern.body);
            let chosen_lang = lang.unwrap_or_default();
            if let PatternLanguage::Universal = chosen_lang {
                return Ok(None);
            }
            let libs = libs.get_language_directory_or_default(lang)?;
            let rich_pattern = resolver
                .make_pattern(&pattern.body, pattern.local_name.clone())
                .unwrap_or_else(|_| panic!("Failed to parse pattern {}", pattern.body));

            let compiled = rich_pattern
                .compile(&libs, None, None, None)
                .map(|cr| cr.problem);

            match compiled {
                Ok(compiled) => {
                    let pattern_name = pattern
                        .local_name
                        .clone()
                        .unwrap_or(format!("<unknown pattern {}>", index));

                    if let Some(samples) = &pattern.config.samples {
                        let mut results = Vec::with_capacity(samples.len());
                        for sample in samples {
                            let result = test_pattern_sample(&compiled, sample, runtime.clone());
                            let mut actual_sample = sample.clone();
                            debug!("Sample: {:?}, result {:?}", sample, result);

                            match &result.actual_output {
                                Some(output) => {
                                    debug!(
                                        "Sample output: {:?}, {:?}",
                                        result.message, result.expected_output
                                    );
                                    actual_sample.output = Some(output.clone())
                                }
                                None => (),
                            }

                            let wrapped = WrappedResult {
                                pattern_name: pattern_name.clone(),
                                result,
                                actual_sample,
                            };
                            if wrapped.result.should_try_formatting() {
                                unformatted_results
                                    .entry(chosen_lang)
                                    .or_default()
                                    .push(wrapped);
                            } else {
                                results.push(wrapped);
                            }
                        }
                        final_results.insert(pattern_name, results);
                    }
                    Ok(None)
                }
                Err(e) => {
                    if output == OutputFormat::Json {
                        let report = TestReport {
                            outcome: TestOutcome::CompilationFailure,
                            message: Some(e.to_string()),
                            samples: vec![],
                        };
                        return Ok(Some(report));
                    }
                    // TODO: this is super hacky, replace with thiserror! codes
                    if e.to_string().contains("No pattern found") {
                        Ok(None)
                    } else {
                        Err(anyhow!(format!(
                            "Failed to compile pattern {}: {}",
                            pattern.local_name.clone().unwrap_or(pattern.body.clone()),
                            e,
                        )))
                    }
                }
            }
        })
        .collect::<Result<Vec<_>>>()?;

    // Filter out the None values
    let mut test_report = test_reports.into_iter().flatten().collect::<Vec<_>>();

    // Now let's attempt formatting the results that need it
    for (lang, lang_results) in unformatted_results.into_iter() {
        let formatted_expected = format_rich_files(
            &lang,
            lang_results
                .iter()
                .flat_map(|r| r.result.expected_outputs.as_ref().unwrap().clone())
                .collect::<Vec<_>>(),
        )
        .await?;
        let formatted_actual = format_rich_files(
            &lang,
            lang_results
                .iter()
                .flat_map(|r| r.result.actual_outputs.as_ref().unwrap().clone())
                .collect::<Vec<_>>(),
        )
        .await?;

        let mut index = 0;
        for wrapped in lang_results.into_iter() {
            let name = wrapped.pattern_name.clone();
            let file_end_offset = index + wrapped.result.actual_outputs.as_ref().unwrap().len();
            let mut our_expected = formatted_expected[index..file_end_offset].to_vec();
            let mut our_actual = formatted_actual[index..file_end_offset].to_vec();
            index = file_end_offset;

            let final_output = has_output_mismatch(&mut our_expected, &mut our_actual);
            let final_result = match final_output {
                None => SampleTestResult::new_passing(wrapped.result.matches.clone(), true),
                Some(MismatchInfo::Content(outcome) | MismatchInfo::Path(outcome)) => {
                    SampleTestResult {
                        matches: wrapped.result.matches.clone(),
                        state: GritTestResultState::FailedOutput,
                        message: Some(
                            "Actual output doesn't match expected output, even after formatting"
                                .to_string(),
                        ),
                        expected_output: Some(outcome.expected),
                        actual_output: Some(outcome.actual),
                        expected_outputs: None,
                        actual_outputs: None,
                    }
                }
            };
            final_results
                .entry(name.clone())
                .or_default()
                .push(WrappedResult {
                    pattern_name: name,
                    actual_sample: wrapped.actual_sample.clone(),
                    result: final_result,
                });
        }
    }

    if args.update {
        update_results(&final_results, patterns)?;
        return Ok(());
    }

    let final_results = final_results.into_read_only();
    log_test_results(&final_results, args.verbose)?;
    let total = final_results.values().flatten().count();
    match output {
        OutputFormat::Standard => {
            if final_results
                .values()
                .any(|v| v.iter().any(|r| !r.result.is_pass()))
            {
                bail!(
                    "{} out of {} samples failed.",
                    final_results
                        .values()
                        .flatten()
                        .filter(|r| !r.result.is_pass())
                        .count(),
                    total
                )
            };
            info!("✓ All {} samples passed.", total);
        }
        OutputFormat::Json => {
            // Collect the test reports
            let mut sample_results = final_results
                .values()
                .map(|r| {
                    let all_pass = r.iter().all(|r| r.result.is_pass());
                    TestReport {
                        outcome: if all_pass {
                            TestOutcome::Success
                        } else {
                            TestOutcome::SampleFailure
                        },
                        message: if all_pass {
                            None
                        } else {
                            Some("One or more samples failed".to_string())
                        },
                        samples: r.iter().map(|r| r.result.clone()).collect(),
                    }
                })
                .collect::<Vec<_>>();
            test_report.append(&mut sample_results);

            log::info!("{}", serde_json::to_string(&test_report)?);
        }
        _ => {
            bail!("Output format not supported for this command");
        }
    }

    Ok(())
}

pub(crate) async fn run_patterns_test(
    arg: PatternsTestArgs,
    flags: GlobalFormatFlags,
) -> Result<()> {
    let (mut patterns, _) = resolve_from_cwd(&Source::Local).await?;
    let libs = get_grit_files_from_flags_or_cwd().await?;

    if arg.filter.is_some() {
        let filter = arg.filter.as_ref().unwrap();
        let regex = regex::Regex::new(filter)?;
        patterns = patterns
            .into_iter()
            .filter(|p| regex.is_match(&p.local_name))
            .collect::<Vec<_>>()
    }

    if !arg.exclude.is_empty() {
        for exclusion in &arg.exclude {
            patterns = patterns
                .into_iter()
                .filter(|p| &p.local_name != exclusion && p.tags().iter().all(|t| t != exclusion))
                .collect::<Vec<_>>()
        }
    }

    let testable_patterns = collect_testable_patterns(patterns);

    if testable_patterns.is_empty() {
        bail!("No testable patterns found. To test a pattern, make sure it is defined in .grit/grit.yaml or a .md file in your .grit/patterns directory.");
    }
    info!("Found {} testable patterns.", testable_patterns.len(),);
    get_marzano_pattern_test_results(testable_patterns, &libs, arg, flags.into()).await
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
enum TestOutcome {
    Success,
    CompilationFailure,
    SampleFailure,
}

#[derive(Debug, Serialize)]
struct TestReport {
    outcome: TestOutcome,
    message: Option<String>,
    /// Sample test details
    samples: Vec<SampleTestResult>,
}

#[derive(Debug)]
struct WrappedResult {
    pattern_name: String,
    actual_sample: GritPatternSample,
    result: SampleTestResult,
}

fn update_results(
    sample_test_results: &DashMap<std::string::String, Vec<WrappedResult>>,
    patterns: Vec<GritPatternTestInfo>,
) -> Result<()> {
    for r in sample_test_results.iter() {
        let pattern_name = r.key();
        let results = r.value();
        if results.iter().all(|r| r.result.is_pure_pass()) {
            continue;
        }
        info!("{} {}", '✓', pattern_name);

        // After replacing the first sample in a file, the offset of the second file will have changed.
        let mut byte_offset: isize = 0;

        for result in results {
            if !result.result.is_pure_pass() {
                let sample_name = get_sample_name(&result.actual_sample);
                info!(
                    "  {} {} - {}",
                    '✓',
                    sample_name,
                    result.result.message.as_ref().unwrap_or(&"".to_string())
                );
                log_test_diff(&result.result);

                if let Some(pattern) = patterns
                    .iter()
                    .find(|p| p.local_name == Some(pattern_name.clone()))
                {
                    if let Some(path) = &pattern.config.path {
                        byte_offset =
                            replace_sample_in_md_file(&result.actual_sample, path, byte_offset)
                                .with_context(|| {
                                    format!(
                                        "Failed to update sample {} in markdown file",
                                        &result
                                            .actual_sample
                                            .name
                                            .as_ref()
                                            .unwrap_or(&"".to_string())
                                    )
                                })?;
                    }
                }
            }
        }
    }

    match sample_test_results {
        _ if sample_test_results
            .iter()
            .any(|r| r.value().iter().any(|r| !r.result.is_pass())) =>
        {
            info!(
                "{} out of {} samples updated.",
                sample_test_results.iter().fold(0, |acc, r| acc
                    + r.value().iter().filter(|r| !r.result.is_pass()).count()),
                sample_test_results
                    .iter()
                    .map(|r| r.value().len())
                    .sum::<usize>(),
            )
        }
        _ => (),
    }

    Ok(())
}

fn log_test_results(
    test_results: &ReadOnlyView<String, Vec<WrappedResult>>,
    verbose: bool,
) -> Result<()> {
    let visiblity_level = if verbose {
        VisibilityLevels::Debug
    } else {
        VisibilityLevels::Supplemental
    };
    let mut sorted_results = test_results.iter().collect::<Vec<_>>();
    sorted_results.sort_by(|a, b| {
        let a = &a.0;
        let b = &b.0;
        a.cmp(b)
    });
    for (local_name, results) in sorted_results.iter() {
        if results.iter().all(|r| r.result.is_pass()) {
            if results.iter().any(|r| r.result.requires_formatting()) {
                info!("{} {} - requires formatting", '⚠', local_name);
            } else {
                info!("{} {}", '✓', local_name);
            }
            if !verbose {
                continue;
            }
        } else {
            info!("{} {}", '✗', local_name);
        }
        for sample_result in results.iter() {
            let name = get_sample_name(&sample_result.actual_sample);
            if sample_result.result.is_pure_pass() {
                info!("  {} {}", '✓', name);
                continue;
            } else if sample_result.result.requires_formatting() {
                info!("  {} {} - requires formatting", '⚠', name);
                continue;
            } else {
                info!("  {} {}", '✗', name);
                for match_result in sample_result.result.matches.iter() {
                    if let MatchResult::AnalysisLog(_) = match_result {
                        if get_visibility(match_result) < visiblity_level {
                            continue;
                        }
                        let formatted = FormattedResult::new(match_result.clone(), false);
                        info!("{}", indent(&format!("{}", formatted.unwrap()), 4));
                    }
                }
                if sample_result.result.actual_output.is_some()
                    && sample_result.result.expected_output.is_some()
                {
                    log_test_diff(&sample_result.result);
                } else {
                    let message = sample_result
                        .result
                        .message
                        .as_ref()
                        .unwrap_or(&"".to_string())
                        .to_string();
                    info!("{}", indent(&message, 4).bright_red());
                }
            }
        }
    }

    Ok(())
}
