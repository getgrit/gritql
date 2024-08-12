use super::{
    function_definition::FunctionDefinition,
    patterns::{Pattern, PatternName},
    state::State,
};
use crate::{context::ExecContext, context::QueryContext};
use core::fmt::Debug;
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};

#[derive(Debug, Clone)]
pub struct FuncEvaluation<'a, Q: QueryContext> {
    pub predicator: bool,
    pub ret_val: Option<Q::ResolvedPattern<'a>>,
}

pub trait Evaluator<Q: QueryContext>: Debug {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>>;
}

#[derive(Debug, Clone)]
pub struct CallFunction<Q: QueryContext> {
    pub index: usize,
    pub args: Vec<Option<Pattern<Q>>>,
}

pub trait GritCall<Q: QueryContext> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Q::ResolvedPattern<'a>>;
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
    ) -> GritResult<Q::ResolvedPattern<'a>> {
        let function_definition = &context.function_definitions()[self.index];

        match function_definition
            .call(state, context, &self.args, logs)?
            .ret_val
        {
            Some(pattern) => Ok(pattern),
            None => Err(GritPatternError::new(
                "Function call did not return a value",
            )),
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
    pub index: usize,
    pub args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> CallForeignFunction<Q> {
    pub fn new(index: usize, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self { index, args }
    }
}

impl<Q: QueryContext> PatternName for CallForeignFunction<Q> {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}
