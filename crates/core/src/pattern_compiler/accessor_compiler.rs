use super::{
    compiler::NodeCompilationContext, container_compiler::ContainerCompiler,
    map_compiler::MapCompiler, node_compiler::NodeCompiler, variable_compiler::VariableCompiler,
};
use crate::problem::MarzanoQueryContext;
use anyhow::{anyhow, Result};
use grit_pattern_matcher::pattern::{Accessor, AccessorKey, AccessorMap};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct AccessorCompiler;

impl NodeCompiler for AccessorCompiler {
    type TargetPattern = Accessor<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let map = node
            .child_by_field_name("map")
            .ok_or_else(|| anyhow!("missing map of accessor"))?;
        let map = if map.node.kind() == "map" {
            AccessorMap::Map(MapCompiler::from_node(&map, context)?)
        } else {
            AccessorMap::Container(ContainerCompiler::from_node(&map, context)?)
        };

        let key = node
            .child_by_field_name("key")
            .ok_or_else(|| anyhow!("missing key of accessor"))?;

        let key = if key.node.kind() == "variable" {
            AccessorKey::Variable(VariableCompiler::from_node(&key, context)?)
        } else {
            AccessorKey::String(key.text()?.to_string())
        };

        Ok(Accessor::new(map, key))
    }
}
