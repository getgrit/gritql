use grit_util::Ast;
use grit_util::AstCursor;
use grit_util::AstNode;
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

use crate::sourcemap::SourceMapSection;
use crate::sourcemap::SourceValueFormat;
use crate::{
    json::Json,
    language::{MarzanoLanguage, MarzanoParser, Tree},
};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Notebook {
    cells: Vec<Cell>,
    /// Notebook format (major number).
    pub nbformat: i64,
    /// Notebook format (minor number).
    pub nbformat_minor: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Cell {
    cell_type: String,
    source: Vec<String>,
}

/// Custom Python parser, to include notebooks
pub(crate) struct MarzanoNotebookParser(MarzanoParser);

impl MarzanoNotebookParser {
    pub(crate) fn new<'a>(lang: &impl MarzanoLanguage<'a>) -> Self {
        Self(MarzanoParser::new(lang))
    }
}

/// Check if a pair in an object is a code cell
fn is_code_cell_pair(node: &NodeWithSource, program_src: &[u8]) -> bool {
    node.node.kind() == "pair"
        && node
            .child_by_field_name("key")
            .and_then(|key| key.node.utf8_text(program_src).ok())
            .map(|key| key == "\"cell_type\"")
            .unwrap_or(false)
        && node
            .child_by_field_name("value")
            .and_then(|value| value.node.utf8_text(program_src).ok())
            .map(|value| value == "\"code\"")
            .unwrap_or(false)
}

/// Given a pair, extract the value
// fn get_pair_sources<'a>(
//     node: &'a NodeWithSource,
//     program_src: &str,
// ) -> Option<Vec<tree_sitter::Range>> {
//     if node.node.kind() != "pair" {
//         return None;
//     }

//     if !node
//         .child_by_field_name("key")
//         .and_then(|key| key.node.utf8_text(program_src.as_bytes()).ok())
//         .map(|key| key == "\"source\"")
//         .unwrap_or(false)
//     {
//         return None;
//     }

//     let Some(value) = node.child_by_field_name("value") else {
//         return None;
//     };

//     return Some(vec![value.range()]);

//     // let mut ranges = Vec::new();

//     // let children = value.node.walk();
//     // let mut cursor = CursorWrapper::new(children, program_src);
//     // for n in traverse(cursor, Order::Pre) {
//     //     println!(
//     //         "Node kind: {} text: {}",
//     //         n.node.kind(),
//     //         n.node.utf8_text(program_src.as_bytes()).unwrap()
//     //     );
//     //     //     if n.node.kind() == "string_content" {
//     //     //         println!(
//     //     //             "Pushing range of {}",
//     //     //             n.node.utf8_text(program_src.as_bytes()).unwrap(),
//     //     //         );
//     //     //         ranges.push(n.node.range());
//     //     //     }
//     // }

//     // if ranges.is_empty() {
//     //     return None;
//     // }

//     // Some(ranges)
// }

impl grit_util::Parser for MarzanoNotebookParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        old_tree: Option<&Self::Tree>,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "ipynb")
            && old_tree.is_none()
        {
            // let notebook: Notebook = serde_json::from_str(body).ok()?;
            // let mut new_src = Vec::new();

            // for cell in notebook.cells {
            //     let source = cell.source.join("\n");
            //     new_src.push(source);
            // }

            // let new_src = new_src.join("\n");

            // let tree = self
            //     .0
            //     .parser
            //     .parse(&new_src, None)
            //     .ok()?
            //     .map(|tree| Tree::new(tree, &new_src));

            // tree

            // TREE SITTER VERSION:
            let mut only_code_body_body = String::new();
            let mut all_ranges: Vec<SourceMapSection> = Vec::new();

            let json = Json::new(None);
            let mut parser = json.get_parser();
            let tree = parser.parse_file(body, None, logs, None)?;
            let root = tree.root_node().node;
            let cursor = root.walk();

            for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
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

                    if is_code_cell_pair(&node, body.as_bytes()) {
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

                            match value {
                                // serde_json::Value::String(_) => {
                                //     let range = value.range();
                                //     source_ranges
                                //         .get_or_insert_with(Vec::new)
                                //         .push(SourceMapSection { range });
                                // }
                                serde_json::Value::Array(value) => {
                                    let this_content = value
                                        .iter()
                                        .map(|v| v.as_str().unwrap_or(""))
                                        .collect::<Vec<&str>>()
                                        .join("\n");
                                    source_ranges = Some((
                                        this_content,
                                        SourceMapSection {
                                            range,
                                            format: SourceValueFormat::Array,
                                        },
                                    ));
                                }
                                _ => {
                                    // bail!("Unexpected source value: {:?}", value);
                                }
                            }
                        }
                    }
                }

                if is_code_cell {
                    if let Some(source_range) = source_ranges {
                        let (content, range) = source_range;
                        only_code_body_body.push_str(&content);
                        all_ranges.push(range);
                    }
                }

                cursor.goto_parent(); // Exit the object
            }

            println!("Found {} code cells", all_ranges.len());

            println!("Only code body: \n{}", only_code_body_body);

            let tree = self
                .0
                .parser
                .parse(only_code_body_body.clone(), None)
                .ok()?
                .map(|tree| Tree::new(tree, only_code_body_body));

            tree
        } else {
            self.0.parse_file(body, path, logs, old_tree)
        }
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
            .parse_file(code, None, &mut AnalysisLogs::default(), None)
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
