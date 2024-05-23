use crate::api::InputFile;
use anyhow::Result;
use grit_util::Ast;
use marzano_language::language::MarzanoLanguage;
use std::path::Path;

#[cfg(feature = "grit-parser")]
pub fn parse_input_file<'a>(
    lang: &impl MarzanoLanguage<'a>,
    input: &str,
    path: &Path,
) -> Result<InputFile> {
    use crate::tree_sitter_serde::tree_sitter_node_to_json;
    use anyhow::Context;
    use serde_json::to_string_pretty;

    let mut parser = lang.get_parser();
    let tree = parser
        .parse_file(input, Some(path), &mut vec![].into(), false)
        .context("Parsed tree is empty")?;
    panic!("Lolol language is {}", lang.language_name());
    let input_file_debug_text = to_string_pretty(&tree_sitter_node_to_json(
        &tree.root_node().node,
        input,
        lang,
    ))
    .context("Failed to convert tree to pretty JSON string")?;
    Ok(InputFile {
        source_file: path.to_string_lossy().to_string(),
        syntax_tree: input_file_debug_text,
    })
}
#[cfg(not(feature = "grit-parser"))]
pub fn parse_input_file<'a>(
    _lang: &impl MarzanoLanguage<'a>,
    _input: &str,
    _path: &Path,
) -> Result<InputFile> {
    use anyhow::anyhow;

    Err(anyhow!(
        "enable grit-parser feature flag to parse a grit file"
    ))
}
