use super::{
    compiler::CompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler,
};
use crate::pattern::{patterns::Pattern, r#match::Match, variable::VariableSourceLocations};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct MatchCompiler;

impl NodeCompiler for MatchCompiler {
    type TargetPattern = Match;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let value = node
            .child_by_field_name("left")
            .ok_or_else(|| anyhow!("missing lhs of predicateMatch"))?;
        let value = ContainerCompiler::from_node(
            &value,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        let pattern = node
            .child_by_field_name("right")
            .ok_or_else(|| anyhow!("missing rhs of predicateMatch"))?;
        let pattern = Some(Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?);
        Ok(Match::new(value, pattern))
    }
}
