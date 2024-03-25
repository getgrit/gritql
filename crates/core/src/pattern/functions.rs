use super::{
    function_definition::FunctionDefinition,
    patterns::{Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub(crate) struct FuncEvaluation<'a> {
    pub predicator: bool,
    pub ret_val: Option<ResolvedPattern<'a>>,
}

pub(crate) trait Evaluator: Debug {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation>;
}

#[derive(Debug, Clone)]
pub struct CallFunction {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern>>,
}

pub(crate) trait GritCall {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>>;
}

impl CallFunction {
    pub fn new(index: usize, args: Vec<Option<Pattern>>) -> Self {
        Self { index, args }
    }
}

impl GritCall for CallFunction {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
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

impl Name for CallFunction {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}

#[derive(Debug, Clone)]
pub struct CallForeignFunction {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern>>,
}

impl CallForeignFunction {
    pub fn new(index: usize, args: Vec<Option<Pattern>>) -> Self {
        Self { index, args }
    }
}

impl GritCall for CallForeignFunction {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
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

impl Name for CallForeignFunction {
    fn name(&self) -> &'static str {
        "CALL_FUNCTION"
    }
}
