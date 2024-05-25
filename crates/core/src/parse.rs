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

#[cfg(test)]
mod tests {
    use std::path::Path;

    use grit_util::{traverse, Ast, Order};
    use insta::assert_snapshot;
    use marzano_language::language::MarzanoLanguage;
    use marzano_language::{python::Python, target_language::TargetLanguage};
    use marzano_util::cursor_wrapper::CursorWrapper;

    fn verify_notebook(source: &str, path: &Path) {
        let lang = TargetLanguage::from_string("ipynb", None).unwrap();

        let mut parser = lang.get_parser();
        let tree = parser
            .parse_file(source, Some(path), &mut vec![].into(), false)
            .unwrap();

        let mut simple_rep = String::new();

        let cursor = tree.root_node().node.walk();
        for n in traverse(CursorWrapper::new(cursor, source), Order::Pre) {
            simple_rep += format!(
                "{:<width$} | {}\n",
                n.node.kind(),
                n.node
                    .utf8_text(tree.source.as_bytes())
                    .unwrap()
                    .replace('\n', "\\n"),
                width = 30
            )
            .as_str();
            assert!(
                !n.node.is_error(),
                "Node is an error: {}",
                n.node.utf8_text(tree.source.as_bytes()).unwrap()
            );
        }

        assert_snapshot!(simple_rep);
    }

    #[test]
    fn simple_notebook() {
        let source = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");
        let path = Path::new("tiny_nb.ipynb");
        verify_notebook(source, path);
    }

    #[test]
    fn other_notebook() {
        let source = include_str!("../../../crates/cli_bin/fixtures/notebooks/other_nb.ipynb");
        let path = Path::new("other_nb.ipynb");
        verify_notebook(source, path);
    }
}
