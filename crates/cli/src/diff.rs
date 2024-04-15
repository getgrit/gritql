use anyhow::{bail, Result};
use marzano_util::{
    diff::{parse_modified_ranges, FileDiff},
    position::{FileRange, Position, RangeWithoutByte, UtilRange},
};
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
    let raw_diff = if let Some(Some(diff_path)) = &arg {
        extract_modified_ranges(diff_path)?
    } else if let Some(None) = &arg {
        let diff = run_git_diff(&std::env::current_dir()?)?;
        parse_modified_ranges(&diff)?
    } else {
        return Ok(None);
    };
    Ok(Some(
        raw_diff
            .into_iter()
            .flat_map(|diff| {
                if (diff.new_path.is_none()) {
                    log::info!("Skipping diff with no new path: {:?}", diff);
                }
                let new_path = diff.new_path.as_ref().unwrap().clone();
                diff.after.into_iter().map(move |range| FileRange {
                    range,
                    file_path: new_path.clone(),
                })
            })
            .collect(),
    ))
}
