use super::{
    accessor_compiler::AccessorCompiler, compiler::CompilationContext, node_compiler::NodeCompiler,
};
use crate::pattern::{
    container::Container,
    list_index::ListIndex,
    variable::{Variable, VariableSourceLocations},
};
use anyhow::{bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct ContainerCompiler;

impl NodeCompiler for ContainerCompiler {
    type TargetPattern = Container;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        match node.kind().as_ref() {
            "variable" => Ok(Container::Variable(Variable::from_node(
                node,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )?)),
            "mapAccessor" => Ok(Container::Accessor(Box::new(AccessorCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "listIndex" => Ok(Container::ListIndex(Box::new(ListIndex::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            s => bail!("Invalid kind for container: {}", s),
        }
    }
}
