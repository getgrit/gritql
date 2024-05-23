use std::path::Path;

use grit_util::{AnalysisLogs, Parser, SnippetTree};

use crate::{json::Json, language::{MarzanoLanguage, MarzanoParser, Tree}};


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
            let notebook_json = match serde_json::from_str::<Notebook>(body) {
                Ok(notebook) => notebook,
                Err(e) => {
                    // logs.error("Failed to parse notebook JSON", e);
                    return None;
                }
            };

            let json_parser

            let included_ranges = notebook_json
                .cells
                .iter()
                .enumerate()
                .map(|(i, cell)| {
                    let start = cell.source.first().map(|s| s.len()).unwrap_or(0);
                    let end = cell.source.last().map(|s| s.len()).unwrap_or(0);
                    (start, end, i)
                })
                .collect::<Vec<_>>();

            println!("Notebook parsing not implemented");

            panic!("Notebook parsing not implemented");
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

pub(crate) fn get_cell_ranges(
    file: &str,
    parent_node_kind: &str,
    name_array: Option<&[&str]>,
) -> Result<Vec<Range>> {
    let vue = Json::new(None);
    let mut parser = Parser::new()?;
    let text = file.as_bytes();
    parser.set_language(vue.get_ts_language())?;
    let tree = parser.parse(file, None)?.ok_or(anyhow!("missing tree"))?;
    let cursor = tree.walk();
    let mut ranges = Vec::new();
    for n in traverse(CursorWrapper::new(cursor, file), Order::Pre) {
        append_code_range(&n.node, text, &mut ranges, parent_node_kind, name_array)
    }
    Ok(ranges)
}
