use grit_util::Ast;
use grit_util::AstNode;
use std::path::Path;

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
            // let json = Json::new(None);
            // let mut parser = json.get_parser();
            // let text = body.as_bytes();
            // // parser.set_language(json.get_ts_language())?;
            // let tree = parser.parse_file(body, None, logs, false)?;
            // let root = tree.root_node().node;
            // let mut cursor = root.walk();

            // let root_doc = root.child_by_field_name("value")?;
            // let properties = root_doc.child_by_field_name("properties")?;

            // // println!("Props: {:?}", properties);

            // panic!("Notebook parsing not implemented");

            // let notebook_json = match serde_json::from_str::<Notebook>(body) {
            //     Ok(notebook) => notebook,
            //     Err(e) => {
            //         // logs.error("Failed to parse notebook JSON", e);
            //         return None;
            //     }
            // };

            // let json_parser

            // let included_ranges = notebook_json
            //     .cells
            //     .iter()
            //     .enumerate()
            //     .map(|(i, cell)| {
            //         let start = cell.source.first().map(|s| s.len()).unwrap_or(0);
            //         let end = cell.source.last().map(|s| s.len()).unwrap_or(0);
            //         (start, end, i)
            //     })
            //     .collect::<Vec<_>>();

            // println!("Notebook parsing not implemented");

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
