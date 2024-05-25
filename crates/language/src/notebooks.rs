use grit_util::AnalysisLog;
use grit_util::Ast;
use grit_util::AstCursor;
use grit_util::AstNode;
use grit_util::ByteRange;
use grit_util::FileOrigin;
use marzano_util::node_with_source::NodeWithSource;
use std::path::Path;
use tree_sitter::Query;

use anyhow::bail;
use anyhow::Result;
use grit_util::traverse;
use grit_util::Order;
use grit_util::Range;
use grit_util::{AnalysisLogs, Parser, SnippetTree};
use marzano_util::cursor_wrapper::CursorWrapper;

use crate::sourcemap::EmbeddedSourceMap;
use crate::sourcemap::SourceMapSection;
use crate::sourcemap::SourceValueFormat;
use crate::{
    json::Json,
    language::{MarzanoLanguage, MarzanoParser, Tree},
};

const SUPPORTED_VERSION: i64 = 4;

/// Custom Python parser, to include notebooks
pub(crate) struct MarzanoNotebookParser(MarzanoParser);

impl MarzanoNotebookParser {
    pub(crate) fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        Self(MarzanoParser::new(lang))
    }

    fn parse_file_as_notebook(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
    ) -> Option<Tree> {
        let mut inner_code_body = String::new();
        let mut source_map = EmbeddedSourceMap::new(body);

        let mut nbformat_version: Option<i64> = None;

        let json = Json::new(None);
        let mut parser = json.get_parser();
        let tree = parser.parse_file(body, None, logs, FileOrigin::Fresh)?;
        let root = tree.root_node().node;
        let cursor = root.walk();

        for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
            if n.node.kind() == "pair"
                && n.child_by_field_name("key")
                    .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                    .map(|key| key == "\"nbformat\"")
                    .unwrap_or(false)
            {
                let value = n
                    .child_by_field_name("value")
                    .and_then(|value| value.node.utf8_text(body.as_bytes()).ok())
                    .map(|value| value.parse::<i64>().unwrap())
                    .unwrap_or(0);
                if value != SUPPORTED_VERSION {
                    logs.add_warning(
                        path.map(|m| m.into()),
                        format!("Unsupported version {} found", value),
                    );
                    return None;
                }
                nbformat_version = Some(value);
            }
            if n.node.kind() != "object" {
                continue;
            }

            let mut cursor = n.walk();

            let mut is_code_cell = true;

            let mut source_ranges: Option<(String, SourceMapSection)> = None;

            cursor.goto_first_child(); // Enter the object
            while cursor.goto_next_sibling() {
                // Iterate over the children of the object
                let node = cursor.node();

                if node.node.kind() == "pair"
                    && node
                        .child_by_field_name("key")
                        .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                        .map(|key| key == "\"cell_type\"")
                        .unwrap_or(false)
                    && node
                        .child_by_field_name("value")
                        .and_then(|value| value.node.utf8_text(body.as_bytes()).ok())
                        .map(|value| value == "\"code\"")
                        .unwrap_or(false)
                {
                    is_code_cell = true;
                }

                if node.node.kind() == "pair"
                    && node
                        .child_by_field_name("key")
                        .and_then(|key| key.node.utf8_text(body.as_bytes()).ok())
                        .map(|key| key == "\"source\"")
                        .unwrap_or(false)
                {
                    if let Some(value) = node.child_by_field_name("value") {
                        let range = value.node.range();
                        let text = value.node.utf8_text(body.as_bytes()).ok()?;
                        let value: serde_json::Value = serde_json::from_str(&text).ok()?;

                        let (this_content, format) = match value {
                            serde_json::Value::Array(value) => (
                                value
                                    .iter()
                                    .map(|v| v.as_str().unwrap_or(""))
                                    .collect::<Vec<&str>>()
                                    .join(""),
                                SourceValueFormat::Array,
                            ),
                            serde_json::Value::String(s) => (s, SourceValueFormat::String),
                            _ => {
                                // bail!("Unexpected source value: {:?}", value);
                                continue;
                            }
                        };
                        let inner_range = ByteRange::new(
                            inner_code_body.len(),
                            inner_code_body.len() + this_content.len(),
                        );
                        source_ranges = Some((
                            this_content,
                            SourceMapSection {
                                outer_range: ByteRange::new(
                                    range.start_byte().try_into().unwrap(),
                                    range.end_byte().try_into().unwrap(),
                                ),
                                inner_range,
                                format,
                            },
                        ));
                    }
                }
            }

            if is_code_cell {
                if let Some(source_range) = source_ranges {
                    let (content, section) = source_range;
                    inner_code_body.push_str(&content);
                    source_map.add_section(section);
                }
            }

            cursor.goto_parent(); // Exit the object
        }

        // Confirm we have a version
        if nbformat_version.is_none() {
            logs.add_warning(
                path.map(|m| m.into()),
                "No nbformat version found".to_string(),
            );
            return None;
        }

        self.0
            .parser
            .parse(inner_code_body.clone(), None)
            .ok()?
            .map(|tree| {
                let mut tree = Tree::new(tree, inner_code_body);
                tree.source_map = Some(source_map);
                tree
            })
    }
}

impl grit_util::Parser for MarzanoNotebookParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        old_tree: FileOrigin<'_, Tree>,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "ipynb")
            && old_tree.is_fresh()
        {
            let tree = self.parse_file_as_notebook(body, path, logs);
            if let Some(tree) = tree {
                return Some(tree);
            }
        }

        self.0.parse_file(body, path, logs, old_tree)
    }

    fn parse_snippet(
        &mut self,
        pre: &'static str,
        source: &str,
        post: &'static str,
    ) -> SnippetTree<Tree> {
        self.0.parse_snippet(pre, source, post)
    }
}

#[cfg(test)]
mod tests {

    use crate::python::Python;

    use super::*;

    #[test]
    fn simple_notebook() {
        let code = include_str!("../../../crates/cli_bin/fixtures/notebooks/tiny_nb.ipynb");
        let mut parser = MarzanoNotebookParser::new(&Python::new(None));
        let tree = parser
            .parse_file(code, None, &mut AnalysisLogs::default(), FileOrigin::Fresh)
            .unwrap();

        let cursor = tree.root_node().node.walk();

        for n in traverse(CursorWrapper::new(cursor, code), Order::Pre) {
            println!("Node kind: {}", n.node.kind());
            assert!(
                !n.node.is_error(),
                "Node is an error: {}",
                n.node.utf8_text(code.as_bytes()).unwrap()
            );
        }
    }
}
