use std::{env::current_exe, fs::canonicalize, path::Path};

use anyhow::{bail, Result};
use marzano_core::api::MatchResult;
use regex::Regex;

use crate::fetcher::ModuleRepo;

/// Extracts the *rewritten* (after applying a pattern) path from a `MatchResult`.
pub fn extract_path(result: &MatchResult) -> Option<&String> {
    match result {
        MatchResult::AnalysisLog(_) => None,
        MatchResult::Match(m) => Some(&m.file.source_file),
        MatchResult::InputFile(i) => Some(&i.source_file),
        MatchResult::CreateFile(c) => Some(&c.rewritten.source_file),
        MatchResult::RemoveFile(r) => Some(&r.original.source_file),
        MatchResult::Rewrite(r) => Some(&r.original.source_file),
        MatchResult::DoneFile(d) => Some(&d.relative_file_path),
        MatchResult::AllDone(_) => None,
        MatchResult::PatternInfo(_) => None,
    }
}

pub fn remove_dir_all_safe(dir: &Path) -> Result<()> {
    if current_exe()?.starts_with(canonicalize(dir)?) {
        bail!("Fatal error: refusing to remove the directory containing the current executable")
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
