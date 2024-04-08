use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::{bail, Ok, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Not {
    pub pattern: Pattern,
}

impl Not {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Name for Not {
    fn name(&self) -> &'static str {
        "NOT"
    }
}

impl Matcher for Not {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(!self
            .pattern
            .execute(binding, &mut state.clone(), context, logs)?)
    }
}

#[derive(Debug, Clone)]
pub struct PrNot {
    pub(crate) predicate: Predicate,
}

impl PrNot {
    pub fn new(predicate: Predicate) -> Self {
        Self { predicate }
    }
}

impl Name for PrNot {
    fn name(&self) -> &'static str {
        "PREDICATE_NOT"
    }
}

impl Evaluator for PrNot {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let res = self
            .predicate
            .execute_func(&mut state.clone(), context, logs)?;
        if res.ret_val.is_some() {
            bail!("Cannot return from within not clause");
        }
        Ok(FuncEvaluation {
            predicator: !res.predicator,
            ret_val: None,
        })
    }
}
