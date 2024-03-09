use crate::language::{fields_for_nodes, Field, TSLanguage};
use lazy_static::lazy_static;

static NODE_TYPES_STRING: &str = include_str!("../../../resources/node-types/grit-node-types.json");

lazy_static! {
    pub static ref GRIT_NODE_TYPES: Vec<Vec<Field>> =
        fields_for_nodes(&language(), NODE_TYPES_STRING);
}

#[cfg(not(feature = "builtin-parser"))]
fn language() -> TSLanguage {
    unimplemented!(
        "tree-sitter parser must be initialized before use when [builtin-parser] is off."
    )
}
#[cfg(feature = "builtin-parser")]
fn language() -> TSLanguage {
    tree_sitter_gritql::language().into()
}
