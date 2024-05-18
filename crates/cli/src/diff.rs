use anyhow::Result;
use marzano_util::diff::{parse_modified_ranges, run_git_diff, FileDiff};
use std::path::PathBuf;

pub(crate) fn extract_target_diffs(
    diff_arg: &Option<Option<String>>,
    root: Option<&PathBuf>,
) -> Result<Option<Vec<FileDiff>>> {
    let raw_diff = if let Some(Some(diff_content)) = &diff_arg {
        parse_modified_ranges(diff_content)?
    } else if let Some(None) = &diff_arg {
        let diff = run_git_diff(root.unwrap_or(&std::env::current_dir()?))?;
        parse_modified_ranges(&diff)?
    } else {
        return Ok(None);
    };
    Ok(Some(raw_diff))
}
