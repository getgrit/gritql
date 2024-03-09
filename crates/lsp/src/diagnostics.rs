use std::path::Path;
#[cfg(feature = "caching")]
use std::sync::{Arc, RwLock};

use anyhow::Result;
#[cfg(feature = "caching")]
use grit_cache::cache::Cache;
use marzano_core::{fs::extract_ranges, pattern::api::EnforcementLevel};

use marzano_gritmodule::fetcher::ModuleRepo;
use tower_lsp::lsp_types::{CodeDescription, Diagnostic, DiagnosticSeverity, TextDocumentItem};

use crate::{
    check::{check_file, CheckInfo},
    util::convert_grit_range_to_lsp_range,
};

pub fn get_diagnostics(
    document: TextDocumentItem,
    check_info: CheckInfo,
    local_repo: &ModuleRepo,
    local_path: &Path,
    #[cfg(feature = "caching")] cache: &Option<Arc<RwLock<Cache>>>,
) -> Result<Vec<Diagnostic>> {
    let pattern_results = check_file(
        &document,
        &check_info,
        #[cfg(feature = "caching")]
        cache,
    )?;
    let mut diagnostics = vec![];
    for (pattern, pattern_result) in pattern_results {
        for result in pattern_result {
            let ranges = extract_ranges(&result).cloned().unwrap_or_default();
            for range in ranges {
                let severity = match &pattern.level() {
                    EnforcementLevel::Error => Some(DiagnosticSeverity::ERROR),
                    EnforcementLevel::Warn => Some(DiagnosticSeverity::WARNING),
                    _ => None,
                };
                let url = pattern.url(local_repo, local_path);
                let diagnostic = Diagnostic {
                    range: convert_grit_range_to_lsp_range(&range),
                    severity,
                    message: pattern
                        .description()
                        .map(|v| v.to_owned())
                        .unwrap_or(format!("Matches pattern {}", pattern.local_name)),
                    source: Some("grit".into()),
                    code: Some(tower_lsp::lsp_types::NumberOrString::String(
                        pattern.name().into(),
                    )),
                    code_description: Some(CodeDescription {
                        href: tower_lsp::lsp_types::Url::parse(&url)?,
                    }),
                    ..Default::default()
                };
                diagnostics.push(diagnostic);
            }
        }
    }

    Ok(diagnostics)
}
