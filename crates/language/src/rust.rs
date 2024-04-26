use crate::language::{
    check_disregarded_field_map, fields_for_nodes, Field, FieldExpectation,
    MarzanoLanguage, NodeTypes, SortId, TSLanguage,
};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/rust-node-types.json");

static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static DISREGARDED_SNIPPET_FIELDS: OnceLock<Vec<FieldExpectation>> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_rust::language().into()
}

#[derive(Debug, Clone)]
pub struct Rust {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sorts: [SortId; 2],
    language: &'static TSLanguage,
    disregarded_snippet_fields: &'static Vec<FieldExpectation>,
}

impl Rust {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let comment_sorts = [
            language.id_for_node_kind("line_comment", true),
            language.id_for_node_kind("block_comment", true),
        ];
        let disregarded_snippet_fields = DISREGARDED_SNIPPET_FIELDS.get_or_init(|| {
            vec![
                (
                    language.id_for_node_kind("struct_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("union_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("enum_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("function_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("function_signature_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("visibility", true),
                    language.field_id_for_name("visibility").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("function_item", true),
                    language.field_id_for_name("type_parameters").unwrap(),
                    Some(vec![""]),
                ),
                (
                    language.id_for_node_kind("function_signature_item", true),
                    language.field_id_for_name("type_parameters").unwrap(),
                    Some(vec![""]),
                ),
            ]
        });
        Self {
            node_types,
            metavariable_sort,
            comment_sorts,
            language,
            disregarded_snippet_fields,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl NodeTypes for Rust {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Rust {
    type Node<'a> = NodeWithSource<'a>;

    fn language_name(&self) -> &'static str {
        "Rust"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("", ";"),
            ("let GRIT_VAR = ", ";"),
            ("fn GRIT_FN(", ") {}"),
            ("fn GRIT_FN(GRIT_ARG:", ") { }"),
        ]
    }

    fn is_comment(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_comment_node(self, node)
    }

    fn is_metavariable(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_metavariable_node(self, node)
    }
}

impl<'a> MarzanoLanguage<'a> for Rust {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn is_disregarded_snippet_field(
        &self,
        sort_id: SortId,
        field_id: crate::language::FieldId,
        field_node: &Option<NodeWithSource<'_>>,
    ) -> bool {
        check_disregarded_field_map(
            self.disregarded_snippet_fields,
            sort_id,
            field_id,
            field_node,
        )
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
        let snippet = "#[cfg(test)] mod $foo { $bar }";
        let lang = Rust::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("NODES: {:#?}", nodes);
        println!("NODE: {}", nodes[0].node.to_sexp());
        assert!(!nodes.is_empty());
    }
}
