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
