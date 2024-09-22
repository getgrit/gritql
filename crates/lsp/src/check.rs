use anyhow::Result;
use std::collections::{BTreeMap, HashMap};

#[cfg(feature = "caching")]
use std::sync::{Arc, RwLock};

#[cfg(feature = "caching")]
use grit_cache::cache::Cache;
#[cfg(feature = "caching")]
use marzano_util::hasher::hash;

use marzano_core::api::{EnforcementLevel, MatchResult};
use marzano_core::pattern_compiler::{src_to_problem_libs, CompilationResult};
use marzano_gritmodule::config::ResolvedGritDefinition;
use marzano_language::target_language::{PatternLanguage, TargetLanguage};
use marzano_util::rich_path::RichFile;
use marzano_util::runtime::ExecutionContext;
use tower_lsp::{lsp_types::TextDocumentItem, Client};

use crate::{
    apply::apply_edits,
    language::language_id_to_pattern_language,
    patterns::{get_grit_files_from_uri, resolve_from_uri},
    util::{get_ai_built_in_functions_for_feature, rewrite_as_edit, uri_to_file_path},
};

#[derive(Clone)]
pub struct CheckInfo {
    language: PatternLanguage,
    pub enforced: Vec<ResolvedGritDefinition>,
    pattern_libs: BTreeMap<String, String>,
}

pub async fn get_check_info(document: &TextDocumentItem) -> Result<Option<CheckInfo>> {
    let language = match language_id_to_pattern_language(&document.language_id) {
        Some(l) => l,
        None => {
            return Ok(None);
        }
    };
    let patterns = resolve_from_uri(document.uri.as_ref(), Some(language), false).await;
    let grit_files = get_grit_files_from_uri(document.uri.as_ref(), false).await;

    let enforced = patterns
        .into_iter()
        .filter(|p| {
            (matches!(&p.level(), EnforcementLevel::Error | EnforcementLevel::Warn))
                && p.language.language_name() == language.language_name()
        })
        .collect::<Vec<_>>();
    let pattern_libs = grit_files.get_language_directory_or_default(Some(language))?;

    Ok(Some(CheckInfo {
        language,
        enforced,
        pattern_libs,
    }))
}

pub fn check_file(
    document: &TextDocumentItem,
    check_info: &CheckInfo,
    #[cfg(feature = "caching")] cache: &Option<Arc<RwLock<Cache>>>,
) -> Result<Vec<(ResolvedGritDefinition, Vec<MatchResult>)>> {
    let context = ExecutionContext::default();
    let file_path = uri_to_file_path(document.uri.as_ref())?;
    let file_content = &document.text;
    #[cfg(feature = "caching")]
    let file_hash = hash(file_content);
    let mut pattern_results = vec![];
    let pattern_libs = &check_info.pattern_libs;
    let language = check_info.language;
    let language: TargetLanguage = match language.try_into() {
        Ok(l) => l,
        Err(e) => {
            let message = format!("Unable to convert language to TargetLanguage: {}", e);
            return Err(anyhow::anyhow!(message));
        }
    };
    for pattern in &check_info.enforced {
        let CompilationResult {
            problem,
            compilation_warnings,
        } = src_to_problem_libs(
            pattern.body.to_string(),
            pattern_libs,
            language,
            Some(pattern.local_name.to_string()),
            None,
            get_ai_built_in_functions_for_feature(),
            None,
        )?;
        let logs = compilation_warnings
            .clone()
            .into_iter()
            .map(|l| MatchResult::AnalysisLog(l.into()))
            .collect();
        pattern_results.push((pattern.clone(), logs));
        #[cfg(feature = "caching")]
        if let Some(cache) = &cache {
            let mut cache_lock = cache.write().unwrap();
            if cache_lock.has_no_matches(file_hash, problem.hash) {
                continue;
            }
        }
        let execution_result = problem.execute_file(
            &RichFile::new(
                file_path.to_string_lossy().to_string(),
                file_content.to_owned(),
            ),
            &context,
        );
        #[cfg(feature = "caching")]
        if let Some(cache) = &cache {
            if execution_result.is_empty() {
                let mut cache_lock = cache.write().unwrap();
                cache_lock.put_no_matches(file_hash, problem.hash);
            }
        }
        pattern_results.push((pattern.clone(), execution_result));
    }

    Ok(pattern_results)
}

pub async fn fix_file(document: &TextDocumentItem, client: &Client) -> Result<bool> {
    let results = match get_check_info(document).await? {
        Some(info) => check_file(
            document,
            &info,
            #[cfg(feature = "caching")]
            &None,
        )?
        .into_iter()
        .flat_map(|r| r.1)
        .collect(),
        None => vec![],
    };
    let edits = results
        .iter()
        .filter_map(|result| match result {
            MatchResult::Rewrite(rewrite) => {
                let text_edit = rewrite_as_edit(document, rewrite.clone());
                Some(text_edit)
            }
            _ => None,
        })
        .collect::<Vec<_>>();
    if edits.is_empty() {
        return Ok(false);
    }
    let mut text_edits = HashMap::new();
    text_edits.insert(document.uri.clone(), edits);
    apply_edits(text_edits, client, None).await;
    Ok(true)
}
