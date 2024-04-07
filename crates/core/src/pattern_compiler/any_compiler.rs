use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{any::Any, patterns::Pattern, variable::VariableSourceLocations};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct AnyCompiler;

impl NodeCompiler for AnyCompiler {
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
        let mut cursor = node.walk();
        let children = node
            .children_by_field_name("patterns", &mut cursor)
            .filter(|n| n.is_named());
        let mut patterns = Vec::new();
        for pattern in children {
            patterns.push(Pattern::from_node(
                &pattern,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?);
        }
        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::Any(Box::new(Any::new(patterns))))
        }
    }
}
