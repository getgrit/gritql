use super::{
    compiler::NodeCompilationContext, node_compiler::NodeCompiler,
    pattern_compiler::PatternCompiler,
};
use crate::pattern::map::GritMap;
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;
use std::collections::BTreeMap;

pub(crate) struct MapCompiler;

impl NodeCompiler for MapCompiler {
    type TargetPattern = GritMap<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let children = node.named_children_by_field_name("elements");
        let mut elements = BTreeMap::new();
        for element in children {
            let key = element
                .child_by_field_name("key")
                .ok_or_else(|| anyhow!("key not found in map element"))?
                .text()?
                .to_string();
            let value = element
                .child_by_field_name("value")
                .ok_or_else(|| anyhow!("value not found in map element"))?;
            let pattern = PatternCompiler::from_node_with_rhs(&value, context, is_rhs)?;
            elements.insert(key, pattern);
        }
        Ok(GritMap::new(elements))
    }
}
