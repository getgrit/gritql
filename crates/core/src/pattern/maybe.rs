use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Maybe {
    pub pattern: Pattern,
}
impl Maybe {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Matcher for Maybe {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut state = init_state.clone();
        if self.pattern.execute(binding, &mut state, context, logs)? {
            *init_state = state;
        }
        Ok(true)
    }
}

impl Name for Maybe {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}

#[derive(Debug, Clone)]
pub struct PrMaybe {
    pub(crate) predicate: Predicate,
}
impl PrMaybe {
    pub fn new(predicate: Predicate) -> Self {
        Self { predicate }
    }
}

impl Evaluator for PrMaybe {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let mut state = init_state.clone();
        if self
            .predicate
            .execute_func(&mut state, context, logs)?
            .predicator
        {
            *init_state = state;
        }
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}

impl Name for PrMaybe {
    fn name(&self) -> &'static str {
        "MAYBE"
    }
}
