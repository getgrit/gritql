use std::sync::OnceLock;
use regex::Regex;


use crate::{
    language::{
        fields_for_nodes, Field, Language, SortId, TSLanguage
    }, xscript_util::{
        php_like_exact_variable_regex, php_like_metavariable_bracket_regex, php_like_metavariable_prefix, php_like_metavariable_regex, PHP_ONLY_CODE_SNIPPETS
    }
};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/php_only-node-types.json");

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
    tree_sitter_php::language_php_only().into()
}

#[derive(Debug, Clone)]
pub struct PhpOnly {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    language: &'static TSLanguage,
}
// use std::io::Write;
impl PhpOnly {
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

impl Language for PhpOnly {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn comment_prefix(&self) -> &'static str {
        "//"
    }

    fn language_name(&self) -> &'static str {
        "PhpOnly"
    }
    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &PHP_ONLY_CODE_SNIPPETS
    }

    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn metavariable_prefix(&self) -> &'static str {
        php_like_metavariable_prefix()
    }

    fn metavariable_regex(&self) -> &'static Regex {
        php_like_metavariable_regex()
    }

    fn metavariable_bracket_regex(&self) -> &'static Regex {
        php_like_metavariable_bracket_regex()
    }

    fn exact_variable_regex(&self) -> &'static Regex {
        php_like_exact_variable_regex()
    }
}
#[cfg(test)]
mod tests {
    use crate::{language::Language, php_only::PhpOnly};

    #[test]
    fn test_php_substitute_variable() {
        let snippet = "^foo$('^bar')";
        let lang = PhpOnly::new(None);
        let subbed = lang.substitute_metavariable_prefix(snippet);
        assert_eq!(subbed, "µfoo$('µbar')");
    }
}