use anyhow::Result;
use marzano_util::position::Range;

use crate::pattern::api::MatchResult;

pub fn apply_rewrite(result: &MatchResult) -> Result<()> {
    match result {
        MatchResult::CreateFile(f) => {
            // Write the file
            std::fs::write(
                f.rewritten.source_file.clone(),
                f.rewritten.content.as_bytes(),
            )?;
        }
        MatchResult::RemoveFile(f) => {
            // Delete the file
            std::fs::remove_file(f.original.source_file.clone())?;
        }
        MatchResult::Rewrite(r) => {
            // If the old file name is different, delete it
            if r.rewritten.source_file != r.original.source_file {
                std::fs::remove_file(r.original.source_file.clone())?;
            }
            // Write the file
            std::fs::write(
                r.rewritten.source_file.clone(),
                r.rewritten.content.as_bytes(),
            )?;
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
