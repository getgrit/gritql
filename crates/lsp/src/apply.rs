use grit_util::{FileRange, Range};
use marzano_core::api::MatchResult;
use marzano_core::pattern_compiler::src_to_problem_libs;
use marzano_gritmodule::patterns_directory::PatternsDirectory;
use marzano_language::target_language::PatternLanguage;
use marzano_util::rich_path::RichFile;
use marzano_util::runtime::ExecutionContext;
use std::collections::HashMap;
use tower_lsp::{
    lsp_types::{MessageType, TextDocumentItem, TextEdit, Url, WorkspaceEdit},
    Client,
};

use crate::{
    patterns::get_grit_files_from_uri,
    util::{get_ai_built_in_functions_for_feature, rewrite_as_edit, uri_to_file_path},
};

pub async fn apply_named_pattern(document: &TextDocumentItem, pattern_name: &str, client: &Client) {
    let grit_files: PatternsDirectory = get_grit_files_from_uri(document.uri.as_ref(), true).await;
    let presumptive_grit_file = grit_files.get(format!("{}.grit", pattern_name).as_str());
    let lang = match presumptive_grit_file {
        Some(g) => PatternLanguage::get_language(g),
        None => PatternLanguage::get_language(pattern_name),
    }
    .unwrap_or_default();
    let body = format!("{}()", pattern_name);
    apply_pattern_body(
        document,
        &body,
        grit_files,
        lang,
        client,
        Some(pattern_name.to_string()),
        None,
    )
    .await;
}

pub async fn apply_pattern_body(
    document: &TextDocumentItem,
    body: &str,
    grit_files: PatternsDirectory,
    lang: PatternLanguage,
    client: &Client,
    name: Option<String>,
    range: Option<Range>,
) {
    let file_path = match uri_to_file_path(document.uri.as_ref()) {
        Ok(path) => path,
        Err(e) => {
            client.show_message(MessageType::ERROR, e).await;
            return;
        }
    };

    let context = ExecutionContext::default();
    let pattern_libs = match grit_files.get_language_directory_or_default(Some(lang)) {
        Ok(lib) => lib,
        Err(e) => {
            client.show_message(MessageType::ERROR, e).await;
            return;
        }
    };
    let file_path_string = file_path.to_string_lossy().to_string();
    let problem = match src_to_problem_libs(
        body.to_owned(),
        &pattern_libs,
        lang.try_into().unwrap(),
        name,
        range.map(|range| {
            vec![FileRange {
                file_path,
                range: range.into(),
            }]
        }),
        get_ai_built_in_functions_for_feature(),
        None,
    ) {
        Ok(p) => p.problem,
        Err(e) => {
            client.show_message(MessageType::ERROR, e.to_string()).await;
            return;
        }
    };

    let file_content = &document.text;
    let execution_result = problem.execute_file(
        &RichFile::new(file_path_string, file_content.to_owned()),
        &context,
    );
    let mut text_edits = HashMap::new();
    for r in execution_result {
        if let MatchResult::Rewrite(rewrite) = r {
            let text_edit = rewrite_as_edit(document, rewrite);
            let changes = text_edits
                .entry(document.uri.clone())
                .or_insert_with(Vec::new);
            changes.push(text_edit);
        }
    }
    if text_edits.is_empty() {
        let message = "Pattern was applied, but no changes were made.";
        client.show_message(MessageType::INFO, message).await;
    } else {
        apply_edits(text_edits, client, Some(body)).await;
    }
}

pub async fn apply_edits(
    text_edits: HashMap<Url, Vec<TextEdit>>,
    client: &Client,
    body: Option<&str>,
) {
    let edit = WorkspaceEdit {
        changes: Some(text_edits),
        ..Default::default()
    };
    match client.apply_edit(edit).await {
        Ok(_) => {
            client
                .show_message(MessageType::INFO, "Pattern was applied successfully")
                .await;
        }
        Err(e) => {
            let message = match body {
                Some(body) => {
                    let truncated_body = body.get(..100).unwrap_or(body);
                    format!("Error applying pattern {:?}: {}", truncated_body, e)
                }
                None => format!("Error applying pattern: {}", e),
            };
            client.show_message(MessageType::ERROR, message).await;
        }
    }
}
