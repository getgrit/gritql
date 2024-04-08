use super::{
    accessor_compiler::AccessorCompiler, compiler::CompilationContext,
    list_index_compiler::ListIndexCompiler, node_compiler::NodeCompiler,
    variable_compiler::VariableCompiler,
};
use crate::pattern::{container::Container, variable::VariableSourceLocations};
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
            "variable" => Ok(Container::Variable(VariableCompiler::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
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
            "listIndex" => Ok(Container::ListIndex(Box::new(
                ListIndexCompiler::from_node(
                    node,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    logs,
                )?,
            ))),
            s => bail!("Invalid kind for container: {}", s),
        }
    }
}
