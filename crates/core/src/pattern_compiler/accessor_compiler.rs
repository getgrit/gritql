use super::{
    compiler::CompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler,
};
use crate::pattern::{
    accessor::{Accessor, AccessorKey, AccessorMap},
    map::GritMap,
    variable::{Variable, VariableSourceLocations},
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct AccessorCompiler;

impl NodeCompiler for AccessorCompiler {
    type TargetPattern = Accessor;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let map = node
            .child_by_field_name("map")
            .ok_or_else(|| anyhow!("missing map of accessor"))?;
        let map = if map.kind() == "map" {
            AccessorMap::Map(GritMap::from_node(
                &map,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?)
        } else {
            AccessorMap::Container(ContainerCompiler::from_node(
                &map,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)
        };

        let key = node
            .child_by_field_name("key")
            .ok_or_else(|| anyhow!("missing key of accessor"))?;

        let key = if key.kind() == "variable" {
            AccessorKey::Variable(Variable::from_node(
                &key,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )?)
        } else {
            AccessorKey::String(key.utf8_text(context.src.as_bytes())?.to_string())
        };

        Ok(Accessor::new(map, key))
    }
}
