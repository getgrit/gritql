use std::{env::current_exe, fs::canonicalize, path::Path};

use anyhow::{bail, Result};
use marzano_core::pattern::api::MatchResult;

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

pub fn remove_dir_all_safe(dir: &Path) -> Result<()> {
    if current_exe()?.starts_with(canonicalize(dir)?) {
        bail!("Fatal error: refusing to remove the directory containing the current executable")
    }
    std::fs::remove_dir_all(dir)?;
    Ok(())
}
