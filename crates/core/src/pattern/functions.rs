use super::{
    function_definition::FunctionDefinition,
    patterns::{Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{context::ExecContext, context::QueryContext};
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub(crate) struct FuncEvaluation<'a> {
    pub predicator: bool,
    pub ret_val: Option<ResolvedPattern<'a>>,
}

pub(crate) trait Evaluator<Q: QueryContext>: Debug {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation>;
}

#[derive(Debug, Clone)]
pub struct CallFunction<Q: QueryContext> {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern<Q>>>,
}

pub(crate) trait GritCall<Q: QueryContext> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>;
}

impl<Q: QueryContext> CallFunction<Q> {
    pub fn new(index: usize, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self { index, args }
    }
}

impl<Q: QueryContext> GritCall<Q> for CallFunction<Q> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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

impl<Q: QueryContext> PatternName for CallFunction<Q> {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}

#[derive(Debug, Clone)]
pub struct CallForeignFunction<Q: QueryContext> {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> CallForeignFunction<Q> {
    pub fn new(index: usize, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self { index, args }
    }
}

impl<Q: QueryContext> GritCall<Q> for CallForeignFunction<Q> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
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

impl<Q: QueryContext> PatternName for CallForeignFunction<Q> {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}
