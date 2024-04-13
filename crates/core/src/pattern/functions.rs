use super::{
    function_definition::FunctionDefinition,
    patterns::{Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{context::ExecContext, context::ProblemContext};
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub(crate) struct FuncEvaluation<'a> {
    pub predicator: bool,
    pub ret_val: Option<ResolvedPattern<'a>>,
}

pub(crate) trait Evaluator<P: ProblemContext>: Debug {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation>;
}

#[derive(Debug, Clone)]
pub struct CallFunction<P: ProblemContext> {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern<P>>>,
}

pub(crate) trait GritCall<P: ProblemContext> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>;
}

impl<P: ProblemContext> CallFunction<P> {
    pub fn new(index: usize, args: Vec<Option<Pattern<P>>>) -> Self {
        Self { index, args }
    }
}

impl<P: ProblemContext> GritCall<P> for CallFunction<P> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let function_definition = &context.function_definitions()[self.index];

        match function_definition
            .call(state, context, &self.args, logs)?
            .ret_val
        {
            Some(pattern) => Ok(pattern),
            None => bail!("Function call did not return a value"),
        }
    }
}

impl<P: ProblemContext> PatternName for CallFunction<P> {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}

#[derive(Debug, Clone)]
pub struct CallForeignFunction<P: ProblemContext> {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern<P>>>,
}

impl<P: ProblemContext> CallForeignFunction<P> {
    pub fn new(index: usize, args: Vec<Option<Pattern<P>>>) -> Self {
        Self { index, args }
    }
}

impl<P: ProblemContext> GritCall<P> for CallForeignFunction<P> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let function_definition = &context.foreign_function_definitions()[self.index];

        match function_definition
            .call(state, context, &self.args, logs)?
            .ret_val
        {
            Some(pattern) => Ok(pattern),
            None => bail!("Function call did not return a value"),
        }
    }
}

impl<P: ProblemContext> PatternName for CallForeignFunction<P> {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}
