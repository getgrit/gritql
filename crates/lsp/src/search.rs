use anyhow::Error;
use marzano_core::pattern::api::{is_match, MatchResult};
use marzano_core::pattern_factory::compiler::src_to_problem_libs;
use marzano_util::rich_path::RichFile;
use marzano_util::runtime::ExecutionContext;
use std::collections::HashMap;
use tower_lsp::lsp_types::TextDocumentItem;

use crate::{
    language::language_id_to_pattern_language,
    patterns::get_grit_files_from_uri,
    util::{get_ai_built_in_functions_for_feature, uri_to_file_path},
};

pub async fn search_query(
    documents: Vec<TextDocumentItem>,
    query: String,
) -> (Vec<Error>, HashMap<String, Vec<MatchResult>>) {
    let context = ExecutionContext::default();
    let mut results = HashMap::new();
    let mut errors = Vec::new();

    // iterate through the document map
    for document in documents.iter() {
        let file_path = match uri_to_file_path(document.uri.as_ref()) {
            Ok(path) => path,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };
        let file_content = &document.text;
        let language = match language_id_to_pattern_language(&document.language_id) {
            Some(lang) => lang,
            None => {
                continue;
            }
        };
        let grit_files = get_grit_files_from_uri(document.uri.as_ref(), false).await;
        let pattern_libs = match grit_files.get_language_directory_or_default(Some(language)) {
            Ok(lib) => lib,
            Err(e) => {
                errors.push(e);
                continue;
            }
        };

        let problem = match src_to_problem_libs(
            query.to_string(),
            &pattern_libs,
            language.try_into().unwrap(),
            None,
            None,
            get_ai_built_in_functions_for_feature(),
        ) {
            Ok(p) => p,
            Err(e) => {
                errors.push(e);
                continue;
            }
        }
        .problem;
        let execution_result = problem.execute_file(
            &RichFile::new(
                file_path.to_string_lossy().to_string(),
                file_content.to_owned(),
            ),
            &context,
        );
        let relevant = execution_result
            .into_iter()
            .filter(is_match)
            .collect::<Vec<_>>();
        results.insert(document.uri.to_string(), relevant);
    }
    (errors, results)
}
