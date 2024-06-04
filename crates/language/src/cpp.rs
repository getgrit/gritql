use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/cpp-node-types.json");
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
    tree_sitter_cpp::language().into()
}

#[derive(Debug, Clone)]
pub struct Cpp {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Cpp {
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

impl NodeTypes for Cpp {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Cpp {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        // TODO: Confirm C++ vs Cpp, etc. here.
        "C++"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        // TODO: Unclear about other good additions here. Presumably, there are many.
        // TODO: Unclear if the following additions are good or bad.
        &[("", ""), ("", ";"), ("{", "}"), ("", "  void GRIT_FN() {}")]
    }
}

impl<'a> MarzanoLanguage<'a> for Cpp {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }
}

#[cfg(test)]
mod tests {

    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn snippet_nodes_not_empty() {
        let snippet = r#"std::cout << "Hello World!";"#;
        let lang = Cpp::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
