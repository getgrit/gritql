use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Any {
    pub(crate) patterns: Vec<Pattern>,
}

impl Any {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
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
    pub predicates: Vec<Predicate>,
}

impl PrAny {
    pub fn new(predicates: Vec<Predicate>) -> Self {
        Self { predicates }
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
