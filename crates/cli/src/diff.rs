use anyhow::{bail, Result};
use regex::Regex;
use serde::Serialize;
pub fn run_git_diff(path: &PathBuf) -> Result<String> {
pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileDiff>> {
pub(crate) fn extract_target_ranges(
    arg: &Option<Option<PathBuf>>,
) -> Result<Option<Vec<FileRange>>> {
    // if let Some(Some(diff_path)) = &arg {
    //     let diff_ranges = extract_modified_ranges(diff_path)?;
    //     Ok(Some(
    //         diff_ranges.into_iter().flat_map(|x| x.after).collect(),
    //     ))
    // } else if let Some(None) = &arg {
    //     let diff = git_diff(&std::env::current_dir()?)?;
    //     let diff_ranges = parse_modified_ranges(&diff)?;
    //     Ok(Some(
    //         diff_ranges
    //             .into_iter()
    //             .flat_map(|x| x.after.map(|x| x.into()))
    //             .collect(),
    //     ))
    // } else {
    Ok(None)
    // }