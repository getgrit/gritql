use super::{
    function_definition::FunctionDefinition,
    functions::{Evaluator, FuncEvaluation},
    patterns::Matcher,
    patterns::{Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::ExecContext, context::ProblemContext};
use anyhow::{bail, Result};
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Clone, Debug)]
pub struct Call<P: ProblemContext> {
    pub(crate) index: usize,
    pub(crate) args: Vec<Option<Pattern<P>>>,
}

impl<P: ProblemContext> Call<P> {
    pub fn new(index: usize, args: Vec<Option<Pattern<P>>>) -> Self {
        Self { index, args }
    }
}

impl<P: ProblemContext> PatternName for Call<P> {
    fn name(&self) -> &'static str {
        "CALL"
    }
}

// todo parameters, and name should both be usize references
// argument should throw an error if its not a parameter at compile time
impl<P: ProblemContext> Matcher<P> for Call<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let pattern_definition = &context.pattern_definitions()[self.index];

        pattern_definition.call(state, binding, context, logs, &self.args)
    }
}

#[derive(Debug, Clone)]
pub struct PrCall<P: ProblemContext> {
    index: usize,
    pub args: Vec<Option<Pattern<P>>>,
}

impl<P: ProblemContext> PrCall<P> {
    pub fn new(index: usize, args: Vec<Option<Pattern<P>>>) -> Self {
        Self { index, args }
    }
}

impl<P: ProblemContext> PatternName for PrCall<P> {
    fn name(&self) -> &'static str {
        "PREDICATE_CALL"
    }
}

impl<P: ProblemContext> Evaluator<P> for PrCall<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let predicate_definition = &context.predicate_definitions().get(self.index);
        if let Some(predicate_definition) = predicate_definition {
            let predicator = predicate_definition.call(state, context, &self.args, logs)?;
            Ok(FuncEvaluation {
                predicator,
                ret_val: None,
            })
        } else {
            let function_definition = &context.function_definitions().get(self.index);
            if let Some(function_definition) = function_definition {
                let res = function_definition.call(state, context, &self.args, logs)?;
                Ok(res)
            } else {
                bail!(
                    "predicate or function definition not found: {}. Try running grit init.",
                    self.index
                );
            }
        }
    }
}
