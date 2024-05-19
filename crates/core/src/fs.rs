use std::path::Path;

use crate::api::MatchResult;
use anyhow::Result;
use grit_util::Range;

pub fn apply_rewrite(result: &MatchResult) -> Result<()> {
    match result {
        // TODO: consider making this more DRY
        MatchResult::CreateFile(f) => {
            let path = Path::new(&f.rewritten.source_file);
            if let Some(parent) = path.parent() {
                fs_err::create_dir_all(parent)?;
            }
            // Write the file
            fs_err::write(path, f.rewritten.content.as_bytes())?;
        }

        MatchResult::Rewrite(r) => {
            let new_path = Path::new(&r.rewritten.source_file);
            if let Some(parent) = new_path.parent() {
                fs_err::create_dir_all(parent)?;
            }
            if r.rewritten.source_file != r.original.source_file {
                let old_path = Path::new(&r.original.source_file);
                if old_path.exists() {
                    fs_err::remove_file(old_path)?;
                }
            }
            // Write the file
            fs_err::write(new_path, r.rewritten.content.as_bytes())?;
        }

        MatchResult::RemoveFile(f) => {
            let path = Path::new(&f.original.source_file);
            if path.exists() {
                fs_err::remove_file(path)?;
            }
        }

        MatchResult::AnalysisLog(_) => {}
        MatchResult::Match(_) => {}
        MatchResult::InputFile(_) => {}

        MatchResult::DoneFile(_) => {}
        MatchResult::AllDone(_) => {}
        MatchResult::PatternInfo(_) => {}
    }
    Ok(())
}

pub fn extract_ranges(result: &MatchResult) -> Option<&Vec<Range>> {
    match result {
        MatchResult::AnalysisLog(_) => None,
        MatchResult::Match(m) => Some(&m.ranges),
        MatchResult::InputFile(_) => None,
        MatchResult::CreateFile(_) => None,
        MatchResult::RemoveFile(r) => Some(&r.original.ranges),
        MatchResult::Rewrite(r) => Some(&r.original.ranges),
        MatchResult::DoneFile(_) => None,
        MatchResult::AllDone(_) => None,
        MatchResult::PatternInfo(_) => None,
    }
}
