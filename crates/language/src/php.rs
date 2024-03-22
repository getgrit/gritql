use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, Language, SortId, TSLanguage};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/php-node-types.json");

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
    tree_sitter_php::language_php().into()
}

#[derive(Debug, Clone)]
pub struct Php {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
}

impl Php {
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

impl Language for Php {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn comment_prefix(&self) -> &'static str {
        "//"
    }

    fn language_name(&self) -> &'static str {
        "Php"
    }
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[("", "")]
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn substitute_metavariable_prefix(&self, src: &str) -> String {
        self.metavariable_regex()
            .replace_all(
                src,
                format!("{}[$1]", self.metavariable_prefix_substitute()).as_str(),
            )
            .to_string()
    }
}
