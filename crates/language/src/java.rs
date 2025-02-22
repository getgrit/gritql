use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/java-node-types.json");
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
    tree_sitter_java::language().into()
}

#[derive(Debug, Clone, Copy)]
pub struct Java {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sorts: [SortId; 2],
    language: &'static TSLanguage,
}

impl NodeTypes for Java {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Java {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sorts = [
            language.id_for_node_kind("line_comment", true),
            language.id_for_node_kind("block_comment", true),
        ];
        Self {
            node_types,
            metavariable_sort,
            comment_sorts,
            language,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}
impl Language for Java {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        "Java"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("import ", ";"),
            ("GRIT_VAR = ", ";"),
            ("class GRIT_CLASS { ", " }"),
            ("class GRIT_CLASS { ", " GRIT_FUNCTION() {} }"),
            ("class GRIT_CLASS { GRIT_FN(", ") {} }"),
        ]
    }
}

impl<'a> MarzanoLanguage<'a> for Java {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        self.comment_sorts.contains(&id)
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
    fn pair_snippet() {
        let snippet = "public";
        let lang = Java::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }

    #[test]
    fn list_snippet() {
        let snippet = r#"List<String> fruits = Arrays.asList("Apple", "Banana", "Cherry");"#;
        let lang = Java::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("nodes: {:#?}", nodes);
        nodes.iter().for_each(|n| {
            n.print_node_tree();
        });
        assert!(!nodes.is_empty());
    }

    #[test]
    fn array_snippet() {
        let snippet = r#"int[] numbers = {1, 2, 3, 4, 5};"#;
        let lang = Java::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("nodes: {:#?}", nodes);
        nodes.iter().for_each(|n| {
            n.print_node_tree();
        });
        assert!(!nodes.is_empty());
    }
}
