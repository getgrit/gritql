use std::sync::OnceLock;

use crate::language::{fields_for_nodes, Field, Language, NodeTypes, SortId, TSLanguage};

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/sql-node-types.json");
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
    tree_sitter_sql::language().into()
}

#[derive(Debug, Clone)]
pub struct Sql {
    node_types: &'static [Vec<Field>],
    metavariable_sort: SortId,
    comment_sort: SortId,
    language: &'static TSLanguage,
}

impl Sql {
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

impl NodeTypes for Sql {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

impl Language for Sql {
    fn get_ts_language(&self) -> &TSLanguage {
        self.language
    }

    fn language_name(&self) -> &'static str {
        "SQL"
    }

    fn snippet_context_strings(&self) -> &[(&'static str, &'static str)] {
        &[
            ("", ""),
            ("", ";"),
            ("select ", "from GRIT_TABLE;"),
            ("select GRIT_VALUE from ", ";"),
            ("create table GRIT_TABLE(GRIT_COLUMN ", ");"),
            ("", "GRIT_FUNCTION() returns int language sql return;"),
            ("CREATE OR REPLACE PROCEDURE GRIT_PROCEDURE(", ");"),
            (
                "CREATE OR REPLACE PROCEDURE GRIT_PROCEDURE(ARG int) AS ",
                ";",
            ),
        ]
    }

    fn metavariable_sort(&self) -> SortId {
        self.metavariable_sort
    }

    fn is_comment_sort(&self, id: SortId) -> bool {
        id == self.comment_sort
    }

    fn make_single_line_comment(&self, text: &str) -> String {
        format!("-- {}\n", text)
    }
}
