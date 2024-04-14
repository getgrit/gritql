use anyhow::{anyhow, Result};
use marzano_core::{
    api::{make_suppress_comment, MatchResult},
    fs::extract_ranges,
};
use marzano_language::target_language::{PatternLanguage, TargetLanguage};
use tower_lsp::lsp_types::{
    self, CodeAction, CodeActionKind, CodeActionOrCommand, Command, DocumentChanges, OneOf,
    OptionalVersionedTextDocumentIdentifier, Range, TextDocumentEdit, TextDocumentItem, TextEdit,
    WorkspaceEdit,
};

use crate::{
    check::{check_file, CheckInfo},
    commands::LspCommand,
    language::language_id_to_pattern_language,
    util::{check_intersection, convert_grit_range_to_lsp_range},
};

pub fn get_code_actions(
    document: TextDocumentItem,
    check_info: CheckInfo,
    lsp_range: Range,
    #[cfg(feature = "caching")] cache: &Option<Arc<RwLock<Cache>>>,
) -> Result<Vec<CodeActionOrCommand>> {
    let pattern_results = check_file(
        &document,
        &check_info,
        #[cfg(feature = "caching")]
        cache,
    )?;

    let mut code_actions = vec![];

    for (pattern, pattern_result) in pattern_results {
        for result in pattern_result {
            if matches!(result, MatchResult::Match(_) | MatchResult::Rewrite(_)) {
                let ranges = extract_ranges(&result).cloned().unwrap_or_default();
                let intersecting_ranges = ranges
                    .iter()
                    .map(convert_grit_range_to_lsp_range)
                    .filter(|r| check_intersection(r, &lsp_range));
                for range in intersecting_ranges {
                    let local_name = &pattern.name();
                    if matches!(result, MatchResult::Rewrite(_)) {
                        let title = format!("Apply {}", local_name);
                        let body = &pattern.body;
                        let language = &pattern.language;
                        let command = Command::new(
                            title.clone(),
                            LspCommand::ApplyResult.to_string(),
                            Some(vec![
                                serde_json::Value::String(document.uri.to_string()),
                                serde_json::Value::String(body.to_string()),
                                serde_json::Value::String(language.to_string()),
                                serde_json::to_value(range).unwrap(),
                            ]),
                        );
                        let apply_action = CodeAction {
                            title,
                            kind: Some(CodeActionKind::QUICKFIX),
                            command: Some(command),
                            ..Default::default()
                        };
                        code_actions.push(CodeActionOrCommand::CodeAction(apply_action));
                    }
                    let suppress_action = make_suppress_action(&document, &range, local_name)?;
                    code_actions.push(CodeActionOrCommand::CodeAction(suppress_action));
                }
            }
        }
    }

    Ok(code_actions)
}

fn make_suppress_action(
    document: &TextDocumentItem,
    range: &Range,
    local_name: &str,
) -> Result<CodeAction> {
    let language = language_id_to_pattern_language(&document.language_id)
        .unwrap_or(PatternLanguage::JavaScript);
    let target_language =
        TargetLanguage::try_from(language).map_err(|_| anyhow!("Invalid language"))?;
    let insert_text = make_suppress_comment(Some(local_name), &target_language);
    let whitespace = document
        .text
        .lines()
        .nth(range.start.line as usize)
        .map(|line| {
            line.chars()
                .take_while(|c| c.is_whitespace())
                .collect::<String>()
        })
        .unwrap_or_default();
    let insert_text = format!("{}{}", whitespace, insert_text);
    let text_edit = TextEdit {
        range: Range {
            start: lsp_types::Position {
                line: range.start.line,
                character: 0,
            },
            end: lsp_types::Position {
                line: range.start.line,
                character: 0,
            },
        },
        new_text: insert_text,
    };

    Ok(CodeAction {
        title: format!("Suppress {}", local_name),
        kind: Some(CodeActionKind::QUICKFIX),
        edit: Some(WorkspaceEdit {
            document_changes: Some(DocumentChanges::Edits(vec![TextDocumentEdit {
                text_document: OptionalVersionedTextDocumentIdentifier {
                    uri: document.uri.clone(),
                    version: Some(document.version),
                },
                edits: vec![OneOf::Left(text_edit)],
            }])),
            ..Default::default()
        }),
        ..Default::default()
    })
}
