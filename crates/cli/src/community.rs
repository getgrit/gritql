use anyhow::Result;
use grit_util::{FileRange, Position, RangeWithoutByte};
use serde::Deserialize;

use std::path::PathBuf;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EslintMessage {
    // One indexed
    pub line: usize,
    pub column: usize,
    pub end_line: usize,
    pub end_column: usize,
}

impl From<EslintMessage> for RangeWithoutByte {
    fn from(msg: EslintMessage) -> Self {
        Self {
            start: Position::new(msg.line as u32, msg.column as u32),
            end: Position::new(msg.end_line as u32, msg.end_column as u32),
        }
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct EslintFile {
    pub file_path: PathBuf,
    pub messages: Vec<EslintMessage>,
}

pub fn parse_eslint_output(json: &str) -> Result<Vec<FileRange>> {
    let output: Vec<EslintFile> = serde_json::from_str(json)?;
    let items = output
        .into_iter()
        .flat_map(|file| {
            file.messages.into_iter().map(move |msg| {
                let range: RangeWithoutByte = msg.into();
                FileRange {
                    file_path: file.file_path.clone(),
                    range: range.into(),
                }
            })
        })
        .collect();
    Ok(items)
}
