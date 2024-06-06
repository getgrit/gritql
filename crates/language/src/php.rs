use crate::{
    language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage},
    php_like::{
        php_like_exact_variable_regex, php_like_metavariable_bracket_regex,
        php_like_metavariable_prefix, php_like_metavariable_regex, PHP_CODE_SNIPPETS,
    },
};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use regex::Regex;
use std::sync::OnceLock;

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
    comment_sort: SortId,
    language: &'static TSLanguage,
}
// use std::io::Write;
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

impl NodeTypes for Php {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Php {
    use_marzano_delegate!();

    fn language_name(&self) -> &'static str {
        "PhpWithHTML"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &PHP_CODE_SNIPPETS
    }

    fn comment_prefix(&self) -> &'static str {
        "//"
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

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("<!-- {} -->\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for Php {
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
    use super::Php;
    use grit_util::Language;

    #[test]
    fn test_php_substitute_variable() {
        let snippet = "^foo$('^bar')";
        let lang = Php::new(None);
        let subbed = lang.substitute_metavariable_prefix(snippet);
        assert_eq!(subbed, "µfoo$('µbar')");
    }
}
