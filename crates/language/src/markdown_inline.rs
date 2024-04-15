use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, Language, NodeTypes, SortId, TSLanguage};

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/markdown-inline-node-types.json");

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
    tree_sitter_md::inline_language().into()
}

#[derive(Debug, Clone)]
pub struct MarkdownInline {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
}

impl MarkdownInline {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
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

impl NodeTypes for MarkdownInline {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for MarkdownInline {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "MarkdownInline"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", "")]
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("<!-- {} -->\n", text)
    }
}

#[cfg(test)]
mod tests {
    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn link_snippet() {
        let snippet = "[grit](https://app.grit.io)";
        let lang = MarkdownInline::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
