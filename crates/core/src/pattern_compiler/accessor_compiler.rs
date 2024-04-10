use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler, variable_compiler::VariableCompiler,
};
use crate::pattern::{
    accessor::{Accessor, AccessorKey, AccessorMap},
    map::GritMap,
};
use anyhow::{anyhow, Result};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AccessorCompiler;

impl NodeCompiler for AccessorCompiler {
    type TargetPattern = Accessor;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let map = node
            .child_by_field_name("map")
            .ok_or_else(|| anyhow!("missing map of accessor"))?;
        let map = if map.node.kind() == "map" {
            AccessorMap::Map(GritMap::from_node(
                &map.node,
                context.compilation,
                context.vars,
                context.vars_array,
                context.scope_index,
                context.global_vars,
                false,
                context.logs,
            )?)
        } else {
            AccessorMap::Container(ContainerCompiler::from_node(&map, context)?)
        };

        let key = node
            .child_by_field_name("key")
            .ok_or_else(|| anyhow!("missing key of accessor"))?;

        let key = if key.node.kind() == "variable" {
            AccessorKey::Variable(VariableCompiler::from_node(&key, context)?)
        } else {
            AccessorKey::String(key.text().to_string())
        };

        Ok(Accessor::new(map, key))
    }
}
