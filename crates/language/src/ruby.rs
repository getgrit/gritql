use crate::language::{fields_for_nodes, Field, MarzanoLanguage, NodeTypes, SortId, TSLanguage};
use grit_util::Language;
use marzano_util::node_with_source::NodeWithSource;
use regex::Regex;
use lazy_static::lazy_static;
use std::sync::OnceLock;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/ruby-node-types.json");

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
    tree_sitter_ruby::language().into()
}

#[derive(Debug, Clone)]
pub struct Ruby {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Ruby {
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

impl NodeTypes for Ruby {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

lazy_static! {
    static ref EXACT_VARIABLE_REGEX: Regex = Regex::new(r"^\^([A-Za-z_][A-Za-z0-9_]*)$")
        .expect("Failed to compile EXACT_VARIABLE_REGEX");
    static ref VARIABLE_REGEX: Regex = Regex::new(r"\^(\.\.\.|[A-Za-z_][A-Za-z0-9_]*)")
        .expect("Failed to compile VARIABLE_REGEX");
    static ref BRACKET_VAR_REGEX: Regex = Regex::new(r"\^\[([A-Za-z_][A-Za-z0-9_]*)\]")
        .expect("Failed to compile BRACKET_VAR_REGEX");
}

impl Language for Ruby {
    type Node<'a> = NodeWithSource<'a>;

    fn language_name(&self) -> &'static str {
        "Ruby"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("case GRIT_VARIABLE\n", "\nend"),
            ("{", "}"),
            ("[", "]"),
        ]
    }

    fn comment_prefix(&self) -> &'static str {
        "#"
    }

    fn is_comment(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_comment_node(self, node)
    }

    fn is_metavariable(&self, node: &NodeWithSource) -> bool {
        MarzanoLanguage::is_metavariable_node(self, node)
    }

    fn metavariable_prefix(&self) -> &'static str {
        "^"
    }

    fn metavariable_regex(&self) -> &'static Regex {
        &VARIABLE_REGEX
    }

    fn metavariable_bracket_regex(&self) -> &'static Regex {
        &BRACKET_VAR_REGEX
    }

    fn exact_variable_regex(&self) -> &'static Regex {
        &EXACT_VARIABLE_REGEX
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("# {}\n", text)
    }
}

impl<'a> MarzanoLanguage<'a> for Ruby {
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
