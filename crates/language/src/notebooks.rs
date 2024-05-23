use std::path::Path;

use grit_util::{AnalysisLogs, SnippetTree};

use crate::language::{MarzanoLanguage, MarzanoParser, Tree};

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
