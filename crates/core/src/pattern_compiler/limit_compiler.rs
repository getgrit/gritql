use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{limit::Limit, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct LimitCompiler;

impl NodeCompiler for LimitCompiler {
    type TargetPattern = Pattern;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let body = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern in limit"))?;
        let body = Pattern::from_node(
            &body,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let limit = node
            .child_by_field_name("limit")
            .ok_or_else(|| anyhow!("missing limit in limit"))?;
        let limit = limit
            .utf8_text(context.src.as_bytes())?
            .trim()
            .parse::<usize>()?;
        Ok(Pattern::Limit(Box::new(Limit::new(body, limit))))
    }
}
