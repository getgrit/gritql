use super::{
    compiler::CompilationContext, node_compiler::NodeCompiler,
    predicate_compiler::PredicateCompiler,
};
use crate::pattern::{patterns::Pattern, r#where::Where, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct WhereCompiler;

impl NodeCompiler for WhereCompiler {
    type TargetPattern = Where;

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
            .ok_or_else(|| anyhow!("missing pattern of patternWhere"))?;
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let side_condition = node
            .child_by_field_name("side_condition")
            .ok_or_else(|| anyhow!("missing side condition of patternWhere"))?;
        let side_condition = PredicateCompiler::from_node(
            &side_condition,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        Ok(Where::new(pattern, side_condition))
    }
}
