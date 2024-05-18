use std::path::PathBuf;

use anyhow::Result;
use clap::Args;

use grit_util::{FileRange, UtilRange};
use marzano_util::diff::FileDiff;
use serde::Serialize;

use crate::{community::parse_eslint_output, diff::extract_target_diffs};

#[derive(Args, Debug, Serialize, Default)]
/// Shared arguments for apply and check commands.
pub struct SharedFilterArgs {
    #[clap(
        long = "only-in-json",
        help = r#"Only analyze ranges inside a provided eslint-style JSON string. The JSON should be an array of objects formatted as `[{"filePath": "path/to/file", "messages": [{"line": 1, "column": 1, "endLine": 1, "endColumn": 1}]}]`."#,
        conflicts_with = "only_in_diff"
    )]
    pub(crate) only_in_json: Option<String>,
    #[clap(
        long = "only-in-diff",
        help = "Only analyze ranges that are inside the provided unified diff, or the results of git diff HEAD if no diff is provided.",
        hide = true,
        conflicts_with = "only_in_json"
    )]
    pub(crate) only_in_diff: Option<Option<String>>,
}

#[tracing::instrument]
pub(crate) fn extract_filter_ranges(
    args: &SharedFilterArgs,
    root: Option<&PathBuf>,
) -> Result<Option<Vec<FileRange>>> {
    if let Some(json_content) = &args.only_in_json {
        let json_ranges = parse_eslint_output(json_content)?;
        Ok(Some(json_ranges))
    } else {
        let raw_diff = extract_target_diffs(&args.only_in_diff, root)?;
        let Some(raw_diff) = raw_diff else {
            log::info!("No diff found, skipping range extraction");
            return Ok(None);
        };
        Ok(Some(
            raw_diff
                .into_iter()
                .flat_map(|diff| match diff.new_path {
                    Some(new_path) => {
                        let mapped = diff.ranges.into_iter().map(|range| FileRange {
                            range: UtilRange::RangeWithoutByte(range.after),
                            file_path: PathBuf::from(&new_path),
                        });
                        mapped.collect::<Vec<_>>()
                    }
                    None => {
                        log::info!("Skipping diff with no new path: {:?}", diff);
                        vec![]
                    }
                })
                .collect(),
        ))
    }
}

#[tracing::instrument]
pub(crate) fn extract_filter_diff(
    args: &SharedFilterArgs,
    root: Option<&PathBuf>,
) -> Result<Option<Vec<FileDiff>>> {
    extract_target_diffs(&args.only_in_diff, root)
}
