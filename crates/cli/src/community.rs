use anyhow::Result;
use marzano_util::position::{FileRange, Position, RangeWithoutByte};
use serde::Deserialize;

use std::fs::File;
use std::io::Read;
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
    pub file_path: String,
    pub messages: Vec<EslintMessage>,
}

pub fn parse_eslint_output(file_path: PathBuf) -> Result<Vec<FileRange>> {
    let mut file = File::open(file_path)?;
    let mut json = String::new();

    // TODO(perf): skip reading the whole string into memory, parse the JSON iteratively
    file.read_to_string(&mut json)?;

    let output: Vec<EslintFile> = serde_json::from_str(&json)?;
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
