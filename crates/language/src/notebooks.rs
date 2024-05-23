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

use crate::{
    json::Json,
    language::{MarzanoLanguage, MarzanoParser, Tree},
};

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
fn get_pair_sources<'a>(
    node: &'a NodeWithSource,
    program_src: &[u8],
) -> Option<NodeWithSource<'a>> {
    if node.node.kind() != "pair" {
        return None;
    }

    if !node
        .child_by_field_name("key")
        .and_then(|key| key.node.utf8_text(program_src).ok())
        .map(|key| key == "\"source\"")
        .unwrap_or(false)
    {
        return None;
    }

    node.child_by_field_name("value")
}

impl grit_util::Parser for MarzanoNotebookParser {
    type Tree = Tree;

    fn parse_file(
        &mut self,
        body: &str,
        path: Option<&Path>,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Option<Tree> {
        if path
            .and_then(Path::extension)
            .is_some_and(|ext| ext == "ipynb")
        {
            //             let gritql_pattern = r#"engine marzano(0.1)
            // language json

            // file($body) where {
            //     $body <: contains bubble object($properties) where {
            //         $properties <: some `"cell_type": "markdown"`,
            //         $properties <: some `"source": [$source]`,
            //         $source <: some bubble `"$partial_source"`
            //     }
            // }"#;

            let json = Json::new(None);
            let mut parser = json.get_parser();
            // let text = body.as_bytes();
            // parser.set_language(json.get_ts_language())?;
            let tree = parser.parse_file(body, None, logs, false)?;
            let root = tree.root_node().node;
            let mut cursor = root.walk();

            for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
                if n.node.kind() != "object" {
                    continue;
                }

                let mut cursor = n.walk();

                let mut is_code_cell = true;

                // let source_cells = None;

                cursor.goto_first_child(); // Enter the object
                while cursor.goto_next_sibling() {
                    // Iterate over the children of the object
                    let node = cursor.node();

                    if is_code_cell_pair(&node, body.as_bytes()) {
                        println!("Found code cell");
                        is_code_cell = true;
                    }

                    // if node.node.kind() == "pair" {
                    //     let key_node = node.child_by_field_name("key");
                    //     if let Some(key) = key_node {
                    //         let child = key.named_children().next();
                    //         if let Some(child) = child {
                    //             if child.node.utf8_text(body.as_bytes()).ok()
                    //                 == Some(std::borrow::Cow::Borrowed("cell_type"))
                    //             {
                    //                 println!(
                    //                     "We found the cell: {:?}",
                    //                     child.node.utf8_text(body.as_bytes()).ok()?
                    //                 );
                    //             }
                    //             println!("key: {:?}", child.node.utf8_text(body.as_bytes()).ok()?);
                    //         }
                    //     }

                    // let value_node = node.child_by_field_name("value");
                    // println!("key: {:?}", key_node);
                    // if let (Some(key), Some(value)) = (key_node, value_node) {
                    //     if key.utf8_text(text).ok()? == "\"code\"" {
                    //         // Do something with the code value
                    //         // For example, print it
                    //         println!("Found code: {:?}", value.utf8_text(text).ok()?);
                    //     }
                    // }
                    // }
                }
                cursor.goto_parent(); // Exit the object

                // println!(
                //     "node_kind: {}, node_text: {:?}",
                //     n.node.kind(),
                //     n.node.utf8_text(text)
                // );
                // append_code_range(&n.node, text, &mut ranges, parent_node_kind, name_array)
            }

            panic!("Notebook parsing not implemented");

            // // Document -> object
            // cursor.goto_first_child();
            // // Object -> values
            // cursor.goto_first_child();

            // while cursor.goto_next_sibling() {
            //     let node = cursor.node();
            //     if node.kind() != "pair" {
            //         continue;
            //     }
            //     cursor.goto_first_child();
            //     cursor.goto_first_child();
            //     cursor.goto_first_child();
            //     println!("node: {} - {:?}", node.kind(), node.utf8_text(text));
            //     cursor.goto_parent();
            //     cursor.goto_parent();
            //     cursor.goto_parent();
            // }

            // let root_doc = root.child_by_field_name("value")?;
            // let props = root_doc.children_by_field_name("properties", &mut cursor);

            // for prop in props {
            //     // println!("cell: {:?}", cell.utf8_text(text));
            //     if let Some(cell_value) = prop.child_by_field_name("value") {
            //         println!("cell_value: {:?}", cell_value.utf8_text(text));
            //     }
            // }

            // panic!("Notebook parsing not implemented");
            // let parent_node_kind = "style_element";
            // let ranges = get_vue_ranges(body, parent_node_kind, None).ok()?;
            // if ranges.is_empty() {
            //     return None;
            // }

            // self.0.parser.set_included_ranges(&ranges).ok()?;
            // self.0
            //     .parser
            //     .parse(body, None)
            //     .ok()?
            //     .map(|tree| Tree::new(tree, body))
        } else {
            self.0.parse_file(body, path, logs, new)
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
