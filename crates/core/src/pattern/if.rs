use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct If {
    pub if_: Predicate,
    pub then: Pattern,
    pub else_: Pattern,
}
impl If {
    pub fn new(if_: Predicate, then: Pattern, else_: Option<Pattern>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Pattern::Top),
        }
    }
}

impl Name for If {
    fn name(&self) -> &'static str {
        "IF"
    }
}

impl Matcher for If {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut state = init_state.clone();
        if self.if_.execute_func(&mut state, context, logs)?.predicator {
            *init_state = state;
            self.then.execute(binding, init_state, context, logs)
        } else {
            self.else_.execute(binding, init_state, context, logs)
        }
    }
}

#[derive(Debug, Clone)]
pub struct PrIf {
    pub if_: Predicate,
    pub then: Predicate,
    pub else_: Predicate,
}

impl PrIf {
    pub fn new(if_: Predicate, then: Predicate, else_: Option<Predicate>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Predicate::True),
        }
    }
}

impl Name for PrIf {
    fn name(&self) -> &'static str {
        "PREDICATE_IF"
    }
}

impl Evaluator for PrIf {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let mut state = init_state.clone();
        let condition = self.if_.execute_func(&mut state, context, logs)?;
        if condition.ret_val.is_some() {
            bail!("Cannot return from within if condition");
        }
        if condition.predicator {
            *init_state = state;
            self.then.execute_func(init_state, context, logs)
        } else {
            self.else_.execute_func(init_state, context, logs)
        }
    }
}
