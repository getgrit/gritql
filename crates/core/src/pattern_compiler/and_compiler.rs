use super::{
    compiler::CompilationContext, node_compiler::NodeCompiler,
    predicate_compiler::PredicateCompiler,
};
use crate::pattern::{
    and::{And, PrAnd},
    patterns::Pattern,
    predicates::Predicate,
    variable::VariableSourceLocations,
};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct AndCompiler;

impl NodeCompiler for AndCompiler {
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
        let mut patterns: Vec<Pattern> = patterns
            .into_iter()
            .filter(|p| !matches!(p, Pattern::Top))
            .collect();
        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::And(Box::new(And::new(patterns))))
        }
    }
}

pub(crate) struct PrAndCompiler;

impl NodeCompiler for PrAndCompiler {
    type TargetPattern = Predicate;

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
            .children_by_field_name("predicates", &mut cursor)
            .filter(|n| n.is_named());
        let mut predicates = Vec::new();
        for predicate in children {
            predicates.push(PredicateCompiler::from_node(
                &predicate,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?);
        }
        if predicates.len() == 1 {
            Ok(predicates.remove(0))
        } else {
            Ok(Predicate::And(Box::new(PrAnd::new(predicates))))
        }
    }
}
