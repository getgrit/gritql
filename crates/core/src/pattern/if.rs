use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct If<P: ProblemContext> {
    pub if_: Predicate<P>,
    pub then: Pattern<P>,
    pub else_: Pattern<P>,
}
impl<P: ProblemContext> If<P> {
    pub fn new(if_: Predicate<P>, then: Pattern<P>, else_: Option<Pattern<P>>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Pattern::Top),
        }
    }
}

impl<P: ProblemContext> PatternName for If<P> {
    fn name(&self) -> &'static str {
        "IF"
    }
}

impl<P: ProblemContext> Matcher<P> for If<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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
pub struct PrIf<P: ProblemContext> {
    pub if_: Predicate<P>,
    pub then: Predicate<P>,
    pub else_: Predicate<P>,
}

impl<P: ProblemContext> PrIf<P> {
    pub fn new(if_: Predicate<P>, then: Predicate<P>, else_: Option<Predicate<P>>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Predicate::True),
        }
    }
}

impl<P: ProblemContext> PatternName for PrIf<P> {
    fn name(&self) -> &'static str {
        "PREDICATE_IF"
    }
}

impl<P: ProblemContext> Evaluator<P> for PrIf<P> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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
