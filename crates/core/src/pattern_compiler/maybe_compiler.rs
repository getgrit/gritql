use super::{
    compiler::CompilationContext, node_compiler::NodeCompiler,
    predicate_compiler::PredicateCompiler,
};
use crate::pattern::{
    maybe::{Maybe, PrMaybe},
    patterns::Pattern,
    variable::VariableSourceLocations,
};
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct MaybeCompiler;

impl NodeCompiler for MaybeCompiler {
    type TargetPattern = Maybe;

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
            .ok_or_else(|| anyhow!("missing pattern of patternMaybe"))?;
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
        Ok(Maybe::new(pattern))
    }
}

pub(crate) struct PrMaybeCompiler;

impl NodeCompiler for PrMaybeCompiler {
    type TargetPattern = PrMaybe;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let predicate = node
            .child_by_field_name("predicate")
            .ok_or_else(|| anyhow!("missing predicate of predicateMaybe"))?;
        let predicate = PredicateCompiler::from_node(
            &predicate,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        Ok(PrMaybe::new(predicate))
    }
}
