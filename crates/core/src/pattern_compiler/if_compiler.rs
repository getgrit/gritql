use super::{
    compiler::CompilationContext, node_compiler::NodeCompiler,
    predicate_compiler::PredicateCompiler,
};
use crate::pattern::{patterns::Pattern, r#if::If, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct IfCompiler;

impl NodeCompiler for IfCompiler {
    type TargetPattern = If;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let if_ = node
            .child_by_field_name("if")
            .ok_or_else(|| anyhow!("missing condition of if"))?;
        let if_ = PredicateCompiler::from_node(
            &if_,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let then = node
            .child_by_field_name("then")
            .ok_or_else(|| anyhow!("missing consequence of if"))?;
        let then = Pattern::from_node(
            &then,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let else_ = node
            .child_by_field_name("else")
            .map(|e| {
                Pattern::from_node(
                    &e,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    false,
                    logs,
                )
            })
            .map_or(Ok(None), |v| v.map(Some))?;
        Ok(If::new(if_, then, else_))
    }
}
