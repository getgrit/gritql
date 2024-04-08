use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{contains::Contains, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct ContainsCompiler;

impl NodeCompiler for ContainsCompiler {
    type TargetPattern = Contains;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let contains = node
            .child_by_field_name("contains")
            .ok_or_else(|| anyhow!("missing contains of patternContains"))?;
        let contains = Pattern::from_node(
            &contains,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let until = node.child_by_field_name("until").map(|n| {
            Pattern::from_node(
                &n,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )
        });
        let until = until.map_or(Ok(None), |v| v.map(Some))?;
        Ok(Contains::new(contains, until))
    }
}
