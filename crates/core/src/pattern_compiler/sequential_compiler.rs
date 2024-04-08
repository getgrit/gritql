use super::{
    compiler::CompilationContext, node_compiler::NodeCompiler, step_compiler::StepCompiler,
};
use crate::pattern::{sequential::Sequential, variable::VariableSourceLocations};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct SequentialCompiler;

impl NodeCompiler for SequentialCompiler {
    type TargetPattern = Sequential;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let mut sequential = vec![];
        let mut cursor = node.walk();
        for n in node
            .children_by_field_name("sequential", &mut cursor)
            .filter(|n| n.is_named())
        {
            let step = StepCompiler::from_node(
                &n,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?;
            sequential.push(step);
        }
        Ok(sequential.into())
    }
}
