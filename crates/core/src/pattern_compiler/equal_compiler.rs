use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{equal::Equal, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct EqualCompiler;

impl NodeCompiler for EqualCompiler {
    type TargetPattern = Equal;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let variable = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateEqual"))?;
        let variable = Pattern::from_node(
            &variable,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateEqual"))?;
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
        if let Pattern::Variable(var) = variable {
            Ok(Equal::new(var, pattern))
        } else {
            Err(anyhow!(
                "predicateEqual must have a variable as first argument",
            ))
        }
    }
}
