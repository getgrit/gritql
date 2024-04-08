use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::variable::{register_variable, Variable, VariableSourceLocations};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct VariableCompiler;

impl NodeCompiler for VariableCompiler {
    type TargetPattern = Variable;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        _logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let name = node.utf8_text(context.src.as_bytes())?.trim().to_string();
        let range = node.range().into();
        register_variable(
            &name,
            context.file,
            range,
            vars,
            global_vars,
            vars_array,
            scope_index,
        )
    }
}
