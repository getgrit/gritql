use crate::paths::absolutize;
use anyhow::Result;
use grit_pattern_matcher::file_owners::FileOwner;
use grit_util::{AnalysisLogs, FileOrigin, MatchRanges};
use marzano_language::{
    language::{MarzanoLanguage, Tree},
    sourcemap::EmbeddedSourceMap,
};
use std::path::PathBuf;

pub(crate) struct FileOwnerCompiler;

impl FileOwnerCompiler {
    pub(crate) fn from_matches<'a>(
        name: impl Into<PathBuf>,
        source: String,
        matches: Option<MatchRanges>,
        old_tree: FileOrigin<'_, Tree>,
        new_map: Option<EmbeddedSourceMap>,
        language: &impl MarzanoLanguage<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Option<FileOwner<Tree>>> {
        let name = name.into();
        let new = !old_tree.is_fresh();

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
