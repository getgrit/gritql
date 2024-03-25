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
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Any {
    pub(crate) patterns: Vec<Pattern>,
}

impl Any {
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
        if patterns.len() == 1 {
            Ok(patterns.remove(0))
        } else {
            Ok(Pattern::Any(Box::new(Self::new(patterns))))
        }
    }
}

impl Name for Any {
    fn name(&self) -> &'static str {
        "ANY"
    }
}

impl Matcher for Any {
    // apply all successful updates to the state
    // must have at least one successful match
    // return soft and failed on failure
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut matched = false;
        let mut cur_state = init_state.clone();
        for pattern in &self.patterns {
            let state = cur_state.clone();
            if pattern.execute(binding, &mut cur_state, context, logs)? {
                matched = true;
            } else {
                cur_state = state;
            }
        }
        if matched {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
#[derive(Debug, Clone)]
pub struct PrAny {
    pub(crate) predicates: Vec<Predicate>,
}

impl PrAny {
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
            Ok(Predicate::Any(Box::new(Self::new(predicates))))
        }
    }
}

impl Name for PrAny {
    fn name(&self) -> &'static str {
        "PREDICATE_ANY"
    }
}

impl Evaluator for PrAny {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let mut matched = false;
        let mut cur_state = init_state.clone();
        for predicate in &self.predicates {
            let state = cur_state.clone();
            if predicate
                .execute_func(&mut cur_state, context, logs)?
                .predicator
            {
                matched = true;
            } else {
                cur_state = state;
            }
        }
        if matched {
            *init_state = cur_state;
            Ok(FuncEvaluation {
                predicator: true,
                ret_val: None,
            })
        } else {
            Ok(FuncEvaluation {
                predicator: false,
                ret_val: None,
            })
        }
    }
}
