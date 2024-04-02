use std::sync::OnceLock;

use tree_sitter::Node;

use crate::language::{fields_for_nodes, Field, Language, SortId, TSLanguage};

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/python-node-types.json");
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
    tree_sitter_python::language().into()
}

#[derive(Debug, Clone)]
pub struct Python {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Python {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for Python {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "Python"
    }

    fn comment_prefix(&self) -> &'static str {
        "#"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("{ ", " }"),
            ("", "\ndef GRIT_FUNCTION():\n    return;"),
            ("GRIT_FN(", ")"),
        ]
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn is_comment(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn check_orphaned(&self, n: Node<'_>, src: &str, orphan_ranges: &mut Vec<tree_sitter::Range>) {
        if n.is_error() {
            let Ok(text) = n.utf8_text(src.as_bytes()) else {
                return;
            };
            if &text == "->" {
                orphan_ranges.push(n.range());
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn kv_snippet() {
        let snippet = "$key: $value";
        let lang = Python::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("nodes: {:#?}", nodes);
        assert!(!nodes.is_empty());
    }
}
