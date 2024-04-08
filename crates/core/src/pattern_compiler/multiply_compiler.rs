use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{multiply::Multiply, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct MultiplyCompiler;

impl NodeCompiler for MultiplyCompiler {
    type TargetPattern = Multiply;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let left = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing left of multiply"))?;
        let left = Pattern::from_node(
            &left,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;

        let right = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing right of multiply"))?;
        let right = Pattern::from_node(
            &right,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;

        Ok(Multiply::new(left, right))
    }
}
