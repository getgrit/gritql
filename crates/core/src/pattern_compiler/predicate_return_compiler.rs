use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    patterns::Pattern, predicate_return::PrReturn, variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct PredicateReturnCompiler;

impl NodeCompiler for PredicateReturnCompiler {
    type TargetPattern = PrReturn;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of return"))?;
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        Ok(PrReturn::new(pattern))
    }
}