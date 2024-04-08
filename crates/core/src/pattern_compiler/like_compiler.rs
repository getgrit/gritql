use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    float_constant::FloatConstant, like::Like, patterns::Pattern, variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct LikeCompiler;

impl NodeCompiler for LikeCompiler {
    type TargetPattern = Like;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let threshold = node
            .child_by_field_name("threshold")
            .map(|n| {
                Pattern::from_node(
                    &n,
                    context,
                    vars,
                    vars_array,
                    scope_index,
                    global_vars,
                    true,
                    logs,
                )
            })
            .unwrap_or(Result::Ok(Pattern::FloatConstant(FloatConstant::new(0.9))))?;
        let like = node
            .child_by_field_name("example")
            .ok_or_else(|| anyhow!("missing field example of patternLike"))?;
        let like = Pattern::from_node(
            &like,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;
        Ok(Like::new(like, threshold))
    }
}
