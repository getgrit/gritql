use anyhow::{anyhow, bail, Context, Result};
use grit_util::Position;
use marzano_core::{api::MatchResult, pattern_compiler::src_to_problem_libs};
use marzano_gritmodule::{
    markdown::get_patterns_from_md,
    resolver::get_grit_files,
    testing::{get_sample_name, test_pattern_sample, GritTestResultState},
};
use marzano_language::grit_parser::MarzanoGritParser;
use marzano_util::runtime::ExecutionContext;
use std::collections::HashMap;
use tower_lsp::{
    lsp_types::{MessageType, TextDocumentItem},
    Client,
};
use uuid::Uuid;

use crate::{
    commands::{GritHighlightKind, ShowGritHighlights, ShowGritHighlightsRequest},
    manager::GritServerManager,
    notifications::{
        GritTestResult, RunGritTest, RunGritTestParams, RunGritTestState, ShowGritTest,
        ShowGritTestParams,
    },
    patterns::prep_grit_modules,
    util::{document_as_rich_file, get_ai_built_in_functions_for_feature, uri_to_file_path},
};

pub async fn maybe_test_pattern(
    client: &Client,
    manager: &GritServerManager,
    document: &TextDocumentItem,
) -> Result<()> {
    if document.language_id != "markdown" {
        // only test markdown for now
        return Ok(());
    }

    let path = uri_to_file_path(document.uri.as_ref())?;

    // If the path doesn't contain .grit/patterns, return
    if !path.to_string_lossy().contains(".grit/patterns") {
        return Ok(());
    }

    let can_test = manager
        .check_client_configuration(client, "grit.authoring.test_matches".to_string())
        .await;

    client
        .log_message(MessageType::LOG, format!("Grit live testing: {}", can_test))
        .await;
    if !can_test {
        return Ok(());
    }

    // Grab the associated pattern context
    let (module_repo, parent_str, stdlib_modules) =
        prep_grit_modules(document.uri.as_ref(), false).await?;

    let grit_files = get_grit_files(&module_repo, &parent_str, stdlib_modules)
        .await
        .with_context(|| format!("Failed to get grit files for {}", document.uri))?;

    // Parse our pattern directly from the file
    let mut rich_file = document_as_rich_file(document.clone())
        .with_context(|| format!("Failed to parse document as rich file: {}", document.uri))?;

    let root = manager.get_root_uri();
    let root = if let Some(url) = root {
        let file_path = uri_to_file_path(url.as_ref())?;
        Some(file_path.to_string_lossy().to_string())
    } else {
        None
    };
    let found_patterns = get_patterns_from_md(&mut rich_file, &Some(module_repo), &root)?;

    let our_pattern = match found_patterns.first() {
        Some(pattern) => pattern,
        None => {
            client
                .show_message(
                    MessageType::WARNING,
                    format!("No pattern found in {}", path.display()),
                )
                .await;
            return Ok(());
        }
    };

    // Start the test run
    let test_run_id = Uuid::new_v4().to_string();
    let test_id = our_pattern.config.path.clone();

    let mut parser = MarzanoGritParser::new()?;
    let language = our_pattern.language(&mut parser).unwrap_or_default();
    let body = match our_pattern.config.body.as_ref() {
        Some(body) => body,
        None => bail!("Pattern {} has no body", our_pattern.local_name),
    };

    let pattern_libs = grit_files
        .get_language_directory_or_default(Some(language))
        .map_err(|e| anyhow!(e))?;

    send_run_state(client, test_run_id.clone(), RunGritTestState::Started).await?;

    // Attempt compilation and report the base pattern
    let compiled = match src_to_problem_libs(
        body.to_string(),
        &pattern_libs,
        language.try_into()?,
        Some(our_pattern.local_name.to_string()),
        None,
        get_ai_built_in_functions_for_feature(),
        None,
    ) {
        Ok(p) => {
            client
                .send_notification::<ShowGritTest>(ShowGritTestParams {
                    test_id: test_id.clone(),
                    parent_test_id: None,
                    test_display_name: our_pattern.local_name.clone(),
                    test_result: GritTestResult::Completed(GritTestResultState::Pass),
                    test_message: None,
                    expected_output: None,
                    actual_output: None,
                    file_uri: document.uri.to_string(),
                    file_range: None,
                    test_run_id: Some(test_run_id.clone()),
                })
                .await;
            p
        }
        Err(e) => {
            client
                .show_message(
                    MessageType::ERROR,
                    format!(
                        "Pattern {} failed to compile {:?}",
                        our_pattern.local_name, e
                    ),
                )
                .await;
            client
                .send_notification::<ShowGritTest>(ShowGritTestParams {
                    test_id: test_id.clone(),
                    parent_test_id: None,
                    test_display_name: our_pattern.local_name.clone(),
                    test_result: GritTestResult::Completed(GritTestResultState::FailedMatch),
                    test_message: Some(format!("{:?}", e)),
                    expected_output: None,
                    actual_output: None,
                    file_uri: document.uri.to_string(),
                    file_range: None,
                    test_run_id: Some(test_run_id.clone()),
                })
                .await;
            // Send empty highlights to clear it out
            send_highlights(document, vec![], test_run_id.clone(), client).await?;

            return Ok(());
        }
    }
    .problem;

    let samples = our_pattern.config.samples.clone().unwrap_or(vec![]);
    let mut highlights: Vec<MatchResult> = Vec::new();

    // Report and test each sample
    for (index, sample) in samples.iter().enumerate() {
        let (offset_position, offset_bytes) = match sample.input_range {
            Some(range) => (range.start, range.start_byte),
            None => (Position::first(), 0),
        };
        let runtime = ExecutionContext::default();
        let outcome = test_pattern_sample(&compiled, sample, runtime);
        for mut result in outcome.matches {
            match result {
                MatchResult::Match(ref mut m) => {
                    m.ranges.iter_mut().for_each(|r| {
                        r.add(offset_position, offset_bytes);
                    });
                }
                MatchResult::Rewrite(ref mut m) => {
                    m.original.ranges.iter_mut().for_each(|r| {
                        r.add(offset_position, offset_bytes);
                    });
                }
                MatchResult::RemoveFile(ref mut m) => {
                    m.original.ranges.iter_mut().for_each(|r| {
                        r.add(offset_position, offset_bytes);
                    });
                }
                _ => {}
            };
            highlights.push(result);
        }

        let file_range = sample.output_range.map(|range| {
            [
                range.start.line - 1,
                range.start.column - 1,
                range.end.line - 1,
                range.end.column - 1,
            ]
        });

        client
            .send_notification::<ShowGritTest>(ShowGritTestParams {
                test_id: format!("{}/{}", test_id, index),
                parent_test_id: Some(test_id.clone()),
                test_display_name: get_sample_name(sample),
                test_result: GritTestResult::Completed(outcome.state),
                test_message: outcome.message,
                expected_output: outcome.expected_output,
                actual_output: outcome.actual_output,
                file_uri: document.uri.to_string(),
                file_range,
                test_run_id: Some(test_run_id.clone()),
            })
            .await;
    }

    // Send all highlights
    send_highlights(document, highlights, test_run_id, client).await?;

    Ok(())
}

async fn send_run_state(
    client: &Client,
    test_run_id: String,
    state: RunGritTestState,
) -> Result<(), anyhow::Error> {
    client
        .send_notification::<RunGritTest>(RunGritTestParams {
            state,
            test_run_id: test_run_id.clone(),
        })
        .await;
    Ok(())
}

async fn send_highlights(
    document: &TextDocumentItem,
    highlights: Vec<MatchResult>,
    test_run_id: String,
    client: &Client,
) -> Result<(), anyhow::Error> {
    let mut results = HashMap::new();
    results.insert(document.uri.to_string(), highlights);
    client
        .send_request::<ShowGritHighlights>(ShowGritHighlightsRequest {
            kind: GritHighlightKind::PatternTest,
            results,
        })
        .await?;
    send_run_state(client, test_run_id, RunGritTestState::Completed).await?;
    Ok(())
}
