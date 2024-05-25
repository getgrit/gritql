use crate::paths::absolutize;
use anyhow::Result;
use grit_pattern_matcher::file_owners::FileOwner;
use grit_util::{AnalysisLogs, FileOrigin, MatchRanges};
use marzano_language::language::{MarzanoLanguage, Tree};
use std::path::PathBuf;

pub(crate) struct FileOwnerCompiler;

impl FileOwnerCompiler {
    pub(crate) fn from_matches<'a>(
        name: impl Into<PathBuf>,
        source: String,
        matches: Option<MatchRanges>,
        old_tree: FileOrigin<'_, Tree>,
        language: &impl MarzanoLanguage<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Option<FileOwner<Tree>>> {
        let name = name.into();
        let new = !old_tree.is_fresh();

        println!("New one, from {:?}", old_tree);

        // If we have an old tree, attach it here
        let new_map = if let Some(old_tree) = old_tree.original() {
            // TODO: avoid this clone
            old_tree.source_map.clone()
        } else {
            None
        };

        let Some(mut tree) = language
            .get_parser()
            .parse_file(&source, Some(&name), logs, old_tree)
        else {
            return Ok(None);
        };

        if new_map.is_some() {
            tree.source_map = new_map;
        }

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
