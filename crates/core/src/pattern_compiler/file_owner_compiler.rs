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
        old_tree: Option<&Tree>,
        language: &impl MarzanoLanguage<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Option<FileOwner<Tree>>> {
        let name = name.into();
        let Some(mut tree) = language
            .get_parser()
            .parse_file(&source, Some(&name), logs, old_tree)
        else {
            return Ok(None);
        };

        // If we have an old tree, attach it here
        if let Some(old_tree) = old_tree {
            // TODO: avoid this clone
            tree.source_map = old_tree.source_map.clone();
        }

        let absolute_path = absolutize(&name)?;
        Ok(Some(FileOwner {
            name,
            absolute_path,
            tree,
            matches: matches.unwrap_or_default().into(),
            new: old_tree.is_some(),
        }))
    }
}
