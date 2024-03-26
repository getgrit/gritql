use crate::pattern::api::InputFile;
use anyhow::Result;
use marzano_language::target_language::TargetLanguage;
use std::path::Path;
use tree_sitter::Parser;

#[cfg(feature = "grit-parser")]
pub fn parse_input_file(lang: &TargetLanguage, input: &str, path: &Path) -> Result<InputFile> {
    use crate::tree_sitter_serde::tree_sitter_node_to_json;
    use anyhow::Context;
    use marzano_language::language::Language;
    use serde_json::to_string_pretty;

    let mut parser = Parser::new().context("Failed to create new parser")?;
    parser
        .set_language(lang.get_ts_language())
        .context("Failed to set language for parser")?;
    let tree = parser
        .parse(input.as_bytes(), None)
        .context("Failed to parse input")?
        .context("Parsed tree is empty")?;
    let input_file_debug_text = to_string_pretty(&tree_sitter_node_to_json(
        &tree.root_node(),
        input,
        Some(lang),
    ))
    .context("Failed to convert tree to pretty JSON string")?;
    Ok(InputFile {
        source_file: path.to_string_lossy().to_string(),
        syntax_tree: input_file_debug_text,
    })
}
#[cfg(not(feature = "grit-parser"))]
pub fn parse_input_file(_lang: &TargetLanguage, _input: &str, _path: &Path) -> Result<InputFile> {
    use anyhow::anyhow;

    Err(anyhow!(
        "enable grit-parser feature flag to parse a grit file"
    ))
}

#[cfg(feature = "grit-parser")]
pub fn make_grit_parser() -> Result<Parser> {
    use anyhow::Context;

    let mut parser = Parser::new().unwrap();
    parser
        .set_language(&tree_sitter_gritql::language().into())
        .with_context(|| "Failed to load grit language")?;
    Ok(parser)
}

#[cfg(not(feature = "grit-parser"))]
pub fn make_grit_parser() -> Result<Parser> {
    use anyhow::anyhow;

    Err(anyhow!(
        "enable grit-parser feature flag to make a grit parser"
    ))
}
