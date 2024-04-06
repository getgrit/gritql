use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, FieldId, Language, SortId, TSLanguage};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/rust-node-types.json");

static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static OPTIONAL_EMPTY_FIELD_COMPILATION: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();

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
    language: &'static TSLanguage,
    optional_empty_field_compilation: &'static Vec<(SortId, FieldId)>,
}

impl Rust {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let optional_empty_field_compilation = OPTIONAL_EMPTY_FIELD_COMPILATION.get_or_init(|| {
            vec![
                (
                    language.id_for_node_kind("struct_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("union_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("enum_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature_item", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("visibility", true),
                    language.field_id_for_name("visibility").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_item", true),
                    language.field_id_for_name("type_parameters").unwrap(),
                ),
                (
                    language.id_for_node_kind("function_signature_item", true),
                    language.field_id_for_name("type_parameters").unwrap(),
                ),
            ]
        });
        Self {
            node_types,
            metavariable_sort,
            language,
            optional_empty_field_compilation,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for Rust {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn optional_empty_field_compilation(
        &self,
        sort_id: SortId,
        field_id: crate::language::FieldId,
    ) -> bool {
        self.optional_empty_field_compilation
            .iter()
            .any(|(s, f)| *s == sort_id && *f == field_id)
    }

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

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
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
