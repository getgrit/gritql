use std::{env::current_exe, fs::canonicalize, path::Path};

use grit_util::error::{GritPatternError, GritResult};
use marzano_core::api::MatchResult;
use marzano_language::target_language::PatternLanguage;
use regex::Regex;

use crate::{fetcher::ModuleRepo, patterns_directory::PatternsDirectory};

/// Extracts the *rewritten* (after applying a pattern) path from a `MatchResult`.
pub fn extract_path(result: &MatchResult) -> Option<&String> {
    match result {
        MatchResult::AnalysisLog(_) => None,
        MatchResult::Match(m) => Some(&m.source_file),
        MatchResult::InputFile(i) => Some(&i.source_file),
        MatchResult::CreateFile(c) => Some(&c.rewritten.source_file),
        MatchResult::RemoveFile(r) => Some(&r.original.source_file),
        MatchResult::Rewrite(r) => Some(&r.original.source_file),
        MatchResult::DoneFile(d) => Some(&d.relative_file_path),
        MatchResult::AllDone(_) => None,
        MatchResult::PatternInfo(_) => None,
    }
}

pub fn remove_dir_all_safe(dir: &Path) -> GritResult<()> {
    if current_exe()?.starts_with(canonicalize(dir)?) {
        return Err(GritPatternError::new(
            "Fatal error: refusing to remove the directory containing the current executable",
        ));
    }
    fs_err::remove_dir_all(dir)?;
    Ok(())
}

pub fn is_pattern_name(pattern: &str) -> bool {
    let regex = Regex::new(r"^[a-zA-Z_][a-zA-Z0-9_]*(\(\))?$").unwrap();
    regex.is_match(pattern)
}

pub fn parse_remote_name(pattern: &str) -> Option<ModuleRepo> {
    let hash_index = pattern.find('#');
    let hash_index = match hash_index {
        Some(index) => index,
        None => return None,
    };
    let repo_str = &pattern[..hash_index];
    let pattern_name = &pattern[hash_index + 1..];
    if is_pattern_name(pattern_name) {
        ModuleRepo::from_repo_str(repo_str).ok()
    } else {
        None
    }
}

/// Given a raw pattern, try to expand into the actual applyable pattern body
///
/// Returns a tuple of:
/// - The language of the pattern
/// - The name of the pattern (if it is a named pattern)
/// - The body of the pattern to apply
pub fn infer_pattern<'a>(
    pattern: &'a str,
    pattern_libs: &'a PatternsDirectory,
) -> (Option<PatternLanguage>, Option<&'a str>, String) {
    if is_pattern_name(pattern) {
        let raw_name = pattern.trim_end_matches("()");
        // details.named_pattern = Some(raw_name.to_string());
        let presumptive_grit_file = pattern_libs.get(format!("{}.grit", raw_name).as_str());
        let lang = match presumptive_grit_file {
            Some(g) => PatternLanguage::get_language(g),
            None => PatternLanguage::get_language(pattern),
        };
        let body = if pattern.ends_with(')') {
            pattern.to_owned()
        } else {
            format!("{}()", pattern)
        };
        (lang, Some(raw_name), body)
    } else if parse_remote_name(pattern).is_some() {
        let raw_name = pattern.split('#').last().unwrap_or(pattern);
        let presumptive_grit_file = pattern_libs.get(format!("{}.grit", raw_name).as_str());
        let lang = match presumptive_grit_file {
            Some(g) => PatternLanguage::get_language(g),
            None => PatternLanguage::get_language(raw_name),
        };
        let body = format!("{}()", raw_name);
        (lang, Some(raw_name), body)
    } else {
        let lang = PatternLanguage::get_language(pattern);
        (lang, None, pattern.to_owned())
    }
}
