use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::QueryContext;
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct If<Q: QueryContext> {
    pub if_: Predicate<Q>,
    pub then: Pattern<Q>,
    pub else_: Pattern<Q>,
}
impl<Q: QueryContext> If<Q> {
    pub fn new(if_: Predicate<Q>, then: Pattern<Q>, else_: Option<Pattern<Q>>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Pattern::Top),
        }
    }
}

impl<Q: QueryContext> PatternName for If<Q> {
    fn name(&self) -> &'static str {
        "IF"
    }
}

impl<Q: QueryContext> Matcher<Q> for If<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a, Q>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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
pub struct PrIf<Q: QueryContext> {
    pub if_: Predicate<Q>,
    pub then: Predicate<Q>,
    pub else_: Predicate<Q>,
}

impl<Q: QueryContext> PrIf<Q> {
    pub fn new(if_: Predicate<Q>, then: Predicate<Q>, else_: Option<Predicate<Q>>) -> Self {
        Self {
            if_,
            then,
            else_: else_.unwrap_or(Predicate::True),
        }
    }
}

impl<Q: QueryContext> PatternName for PrIf<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_IF"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrIf<Q> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<Q>> {
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
