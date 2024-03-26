use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, Language, SortId, TSLanguage, FieldId};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/json-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();
static MANDATORY_EMPTY_FIELD_SORTS: OnceLock<Vec<(SortId, FieldId)>> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_json::language().into()
}

#[derive(Debug, Clone)]
pub struct Json {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
    mandatory_empty_field_sorts: &'static Vec<(SortId, FieldId)>,
}

impl Json {
    pub(crate) fn new(lang: Option<TSLanguage>) -> Self {
        let language = LANGUAGE.get_or_init(|| lang.unwrap_or_else(language));
        let node_types = NODE_TYPES.get_or_init(|| fields_for_nodes(language, NODE_TYPES_STRING));
        let metavariable_sort = language.id_for_node_kind("grit_metavariable", true);
        let mandatory_empty_field_sorts = MANDATORY_EMPTY_FIELD_SORTS.get_or_init(|| {
            vec![
                (
                    language.id_for_node_kind("string", true),
                    language.field_id_for_name("content").unwrap(),
                ),
            ]
        });
        Self {
            node_types,
            metavariable_sort,
            language,
            mandatory_empty_field_sorts,
        }
    }
    pub(crate) fn is_initialized() -> bool {
        LANGUAGE.get().is_some()
    }
}

impl Language for Json {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn mandatory_empty_field(&self, sort_id:SortId, field_id:crate::language::FieldId) -> bool {
        self.mandatory_empty_field_sorts
            .iter()
            .any(|(s, f)| *s == sort_id && *f == field_id)
    }

    fn language_name(&self) -> &'static str {
        "JSON"
    }
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", ""), ("{ ", " }")]
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
        let snippet = "$key: $value";
        let lang = Json::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        assert!(!nodes.is_empty());
    }
}
