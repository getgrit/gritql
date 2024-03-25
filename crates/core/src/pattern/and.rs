use super::{
    compiler::CompilationContext,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct And {
    pub(crate) patterns: Vec<Pattern>,
}

impl And {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Pattern> {
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
            Ok(Pattern::And(Box::new(Self::new(patterns))))
        }
    }
}

impl Name for And {
    fn name(&self) -> &'static str {
        "AND"
    }
}

impl Matcher for And {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        for p in self.patterns.iter() {
            if !p.execute(binding, state, context, logs)? {
                return Ok(false);
            };
        }
        Ok(true)
    }
}

#[derive(Debug, Clone)]
pub struct PrAnd {
    pub(crate) predicates: Vec<Predicate>,
}
impl PrAnd {
    pub fn new(predicates: Vec<Predicate>) -> Self {
        Self { predicates }
    }
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Predicate> {
        let mut cursor = node.walk();
        let children = node
            .children_by_field_name("predicates", &mut cursor)
            .filter(|n| n.is_named());
        let mut predicates = Vec::new();
        for predicate in children {
            predicates.push(Predicate::from_node(
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

impl Name for PrAnd {
    fn name(&self) -> &'static str {
        "PREDICATE_AND"
    }
}

impl Evaluator for PrAnd {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        for p in self.predicates.iter() {
            let res = p.execute_func(state, context, logs)?;
            match res.predicator {
                true => {}
                false => return Ok(res),
            };
        }
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}
