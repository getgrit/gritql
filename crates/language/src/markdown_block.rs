use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/markdown-block-node-types.json");

static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_md::language().into()
}

#[derive(Debug, Clone)]
pub struct MarkdownBlock {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
}

impl NodeTypes for MarkdownBlock {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl MarkdownBlock {
    pub fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        Self {
            node_types,
            metavariable_sort,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for MarkdownBlock {
    type Node<'a> = NodeWithSource<'a>;

    fn language_name(&self) -> &'static str {
        "MarkdownBlock"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", "")]
    }

    fn is_comment(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_comment_node(self, node)
    }

    fn is_metavariable(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_metavariable_node(self, node)
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("<!-- {} -->\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for MarkdownBlock {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, _sort: SortId) -> bool {
        false
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

#[cfg(test)]
mod tests {
    use tree_sitter::Parser;

    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn block_snippet() {
        let snippet = "---
title:  'This is the title: it contains a colon'
author:
- Author One
- Author Two
keywords: [nothing, nothingness]
abstract: |
  This is the abstract.

  It consists of two paragraphs.
---
";
        let mut parser = Parser::new().unwrap();
        let markdown_block = MarkdownBlock::new(None);
        parser
            .set_language(MarkdownBlock::get_ts_language(&markdown_block))
            .unwrap();
        let lang = MarkdownBlock::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
