use crate::language::{fields_for_nodes, Field, NodeTypes, TSLanguage};
use lazy_static::lazy_static;

pub static NODE_TYPES_STRING: &str =
    include_str!("../../../resources/node-types/grit-node-types.json");

lazy_static! {
    static ref GRIT_NODE_TYPES: Vec<Vec<Field>> = fields_for_nodes(&language(), NODE_TYPES_STRING);
}

pub struct GritNodeTypes<'a> {
    pub node_types: &'a [Vec<Field>],
}

impl NodeTypes for GritNodeTypes<'_> {
    fn node_types(&self) -> &[Vec<Field>] {
        self.node_types
    }
}

pub fn grit_node_types() -> GritNodeTypes<'static> {
    GritNodeTypes {
        node_types: &GRIT_NODE_TYPES,
    }
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
