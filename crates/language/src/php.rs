use std::sync::OnceLock;
use std::collections::{BTreeMap, HashMap};
use crate::language::{fields_for_nodes, Field, Language, SortId, TSLanguage};

static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/php-node-types.json");
static NODE_TYPES: OnceLock<Vec<Vec<Field>>> = OnceLock::new();
static LANGUAGE: OnceLock<TSLanguage> = OnceLock::new();

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    print!("php language");
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
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Php {
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

impl Language for Php {

    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "Php"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[  ("", ""),
            ("<?php ", " ?>"),
            ("<?php\n", ";\n?>"),
            ("<?php\n GRIT_VALUE ", " GRIT_VALUE \n?>"),
            ("<?php\n class GRIT_CLASS ", " {} \n?>"),
            ("<?php\n class GRIT_CLASS { ", " GRIT_PROP = 'GRIT_VALUE'; } \n?>"),
            ("<?php\n ", "  function GRIT_FUNCTION() {} \n?>"),
            ("<?php\n GRIT_OBJ = { ", " } \n?>"),
            ("<?php\n class GRIT_CLASS { ", " } \n?>"),
            ("<?php\n GRIT_VAR = ", " \n?>"),
            ("<?php\n function GRIT_FN(", ") {} \n?>"),
            ("<?php\n ", " class GRIT_CLASS {} \n?>"),
            ("<?php\n function GRIT_FN(GRIT_ARG", ") { } \n?>"),]
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
}

#[cfg(test)]
mod tests {
    use crate::language::nodes_from_indices;

    use super::*;

    #[test]
    fn kv_snippet() {
        let snippet = "echo($a)";
        let lang = Php::new(None);
        let snippets = lang.parse_snippet_contexts(snippet);
        let nodes = nodes_from_indices(&snippets);
        println!("nodes: {:#?}", nodes);
        assert!(!nodes.is_empty());
    }

}

