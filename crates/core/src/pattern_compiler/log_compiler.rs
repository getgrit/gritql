use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    log::{Log, VariableInfo},
    patterns::Pattern,
    variable::{Variable, VariableSourceLocations},
};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct LogCompiler;

impl NodeCompiler for LogCompiler {
    type TargetPattern = Log;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let message = node.child_by_field_name("message");
        let message = if let Some(message) = message {
            Some(Pattern::from_node(
                &message,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?)
        } else {
            None
        };
        let variable_node = node.child_by_field_name("variable");
        let variable = variable_node
            .map(|n| {
                let name = n.utf8_text(context.src.as_bytes()).unwrap().to_string();
                let variable = Variable::from_node(
                    &n,
                    context.file,
                    context.src,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                )?;
                Ok(VariableInfo::new(name, variable))
            })
            .map_or(Ok(None), |v: Result<VariableInfo>| v.map(Some))?;

        Ok(Log::new(variable, message))
    }
}
