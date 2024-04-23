use anyhow::Result;
use grit_util::{FileRange, UtilRange};
use marzano_util::diff::{parse_modified_ranges, FileDiff};
use std::{fs::File, io::Read, path::PathBuf};

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

pub(crate) fn extract_target_ranges(
    diff_arg: &Option<Option<String>>,
) -> Result<Option<Vec<FileRange>>> {
    let raw_diff = if let Some(Some(diff_content)) = &diff_arg {
        parse_modified_ranges(&diff_content)?
    } else if let Some(None) = &arg {
        let diff = run_git_diff(&std::env::current_dir()?)?;
        parse_modified_ranges(&diff)?
    } else {
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
