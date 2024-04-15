use anyhow::{bail, Result};
use marzano_util::position::{FileRange, Position, RangeWithoutByte, UtilRange};
use regex::Regex;
use serde::Serialize;
use std::{fs::File, io::Read, path::PathBuf, str::FromStr};

pub fn run_git_diff(path: &PathBuf) -> Result<String> {
    let output = std::process::Command::new("git")
        .arg("diff")
        .arg("HEAD")
        .arg("--relative")
        .arg("--unified=0")
        .arg(path)
        .output()?;
    Ok(String::from_utf8(output.stdout)?)
}

pub fn extract_modified_ranges(diff_path: &PathBuf) -> Result<Vec<FileDiff>> {
    let mut file = File::open(diff_path)?;
    let mut diff = String::new();

    file.read_to_string(&mut diff)?;
    parse_modified_ranges(&diff)
}

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
}
