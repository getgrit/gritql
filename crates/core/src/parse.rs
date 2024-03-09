use std::path::Path;

#[cfg(not(feature = "grit-parser"))]
use anyhow::anyhow;
use anyhow::Context;
use anyhow::Result;
use marzano_language::{language::Language, target_language::TargetLanguage};
use serde_json::to_string_pretty;
use tree_sitter::Parser;

use crate::{pattern::api::InputFile, tree_sitter_serde::tree_sitter_node_to_json};

#[cfg(feature = "grit-parser")]
pub fn parse_input_file(lang: &TargetLanguage, input: &str, path: &Path) -> Result<InputFile> {
    let mut parser = Parser::new().unwrap();
    parser.set_language(lang.get_ts_language()).unwrap();
    let tree = parser.parse(input.as_bytes(), None).unwrap().unwrap();
    let input_file_debug_text = to_string_pretty(&tree_sitter_node_to_json(
        &tree.root_node(),
        input,
        Some(lang),
    ))
    .unwrap();
    Ok(InputFile {
        source_file: path.to_string_lossy().to_string(),
        syntax_tree: input_file_debug_text,
    })
}
#[cfg(not(feature = "grit-parser"))]
pub fn parse_input_file(lang: &TargetLanguage, input: &str, path: &Path) -> Result<InputFile> {
    Err(anyhow!(
        "enable grit-parser feature flag to parse a grit file"
    ))
}

#[cfg(feature = "grit-parser")]
pub fn make_grit_parser() -> Result<Parser> {
    let mut parser = Parser::new().unwrap();
    parser
        .set_language(&tree_sitter_gritql::language().into())
        .with_context(|| "Failed to load grit language")?;
    Ok(parser)
}

#[cfg(not(feature = "grit-parser"))]
pub fn make_grit_parser() -> Result<Parser> {
    Err(anyhow!(
        "enable grit-parser feature flag to make a grit parser"
    ))
}
