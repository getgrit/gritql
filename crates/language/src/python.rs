use crate::language::{
    fields_for_nodes, Field, Language, NodeTypes, Replacement, SortId, TSLanguage,
};
use grit_util::AstNode;
use marzano_util::{file_owner::FileParser, node_with_source::NodeWithSource};
use std::sync::OnceLock;

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
    skip_padding_sorts: [SortId; 1],
    language: &'static TSLanguage,
}

impl Python {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sort = language.id_for_node_kind("comment", true);
        let skip_padding_sorts = [language.id_for_node_kind("string", true)];
        Self {
            node_types,
            metavariable_sort,
            comment_sort,
            skip_padding_sorts,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for Python {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl FileParser for Python {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }
}

impl Language for Python {
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

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn check_replacements(
        &self,
        n: NodeWithSource<'_>,
        replacements: &mut Vec<crate::language::Replacement>,
    ) {
        if n.node.is_error() && n.text().is_ok_and(|t| t == "->") {
            replacements.push(Replacement::new(n.range(), ""));
        }
    }

    fn should_pad_snippet(&self) -> bool {
        true
    }

    fn skip_padding_sort(&self, id: SortId) -> bool {
        self.skip_padding_sorts.contains(&id)
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("# {}\n", text)
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
