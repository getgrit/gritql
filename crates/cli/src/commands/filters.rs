use anyhow::Result;
use clap::Args;

use grit_util::FileRange;
use serde::Serialize;

use crate::{community::parse_eslint_output, diff::extract_target_ranges};

#[derive(Args, Debug, Serialize, Default)]
/// Shared arguments for apply and check commands.
pub struct SharedFilterArgs {
    #[clap(
        long = "only-in-json",
        help = r#"Only rewrite ranges inside a provided eslint-style JSON string. The JSON should be an array of objects formatted as `[{"filePath": "path/to/file", "messages": [{"line": 1, "column": 1, "endLine": 1, "endColumn": 1}]}]`."#,
        conflicts_with = "only_in_diff"
    )]
    pub(crate) only_in_json: Option<String>,
    #[clap(
        long = "only-in-diff",
        help = "Only rewrite ranges that are inside the provided unified diff, or the results of git diff HEAD if no diff is provided.",
        hide = true,
        conflicts_with = "only_in_json"
    )]
    pub(crate) only_in_diff: Option<Option<String>>,
}

pub(crate) fn extract_filter_ranges(args: &SharedFilterArgs) -> Result<Option<Vec<FileRange>>> {
    if let Some(json_content) = &args.only_in_json {
        let json_ranges = parse_eslint_output(json_content)?;
        Ok(Some(json_ranges))
    } else {
        Ok(extract_target_ranges(&args.only_in_diff)?)
    }
}
