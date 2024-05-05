use crate::paths::absolutize;
use anyhow::Result;
use grit_pattern_matcher::file_owners::FileOwner;
use grit_util::{AnalysisLogs, MatchRanges};
use marzano_language::language::{MarzanoLanguage, Tree};
use std::path::PathBuf;

pub(crate) struct FileOwnerCompiler;

impl FileOwnerCompiler {
    pub(crate) fn from_matches<'a>(
        name: impl Into<PathBuf>,
        source: String,
        matches: Option<MatchRanges>,
        new: bool,
        language: &impl MarzanoLanguage<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Option<FileOwner<Tree>>> {
        let name = name.into();
        let Some(tree) = language
            .get_parser()
            .parse_file(&source, Some(&name), logs, new)
        else {
            return Ok(None);
        };
        let absolute_path = absolutize(&name)?;
        Ok(Some(FileOwner {
            name,
            absolute_path,
            tree,
            matches: matches.unwrap_or_default().into(),
            new,
        }))
    }
}
