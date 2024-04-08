use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    iter_pattern::PatternOrPredicate, not::Not, patterns::Pattern, predicates::Predicate,
    variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::{
    analysis_logs::{AnalysisLogBuilder, AnalysisLogs},
    position::Range,
};
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct NotCompiler;

impl NodeCompiler for NotCompiler {
    type TargetPattern = Not;

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
            .ok_or_else(|| anyhow!("missing pattern of patternNot"))?;
        let range: Range = pattern.range().into();
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
        if pattern.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.file)
                .source(context.src)
                .position(range.start)
                .range(range)
                .message("Warning: rewrites inside of a not will never be applied")
                .build()?;
            logs.push(log);
        }
        Ok(Not::new(pattern))
    }
}
