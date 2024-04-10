use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct And {
    pub patterns: Vec<Pattern>,
}

impl And {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
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
    pub predicates: Vec<Predicate>,
}

impl PrAnd {
    pub fn new(predicates: Vec<Predicate>) -> Self {
        Self { predicates }
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
