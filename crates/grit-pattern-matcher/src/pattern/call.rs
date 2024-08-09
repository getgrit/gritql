use super::{
    function_definition::FunctionDefinition,
    functions::{Evaluator, FuncEvaluation},
    patterns::Matcher,
    patterns::{Pattern, PatternName},
    State,
};
use crate::{context::ExecContext, context::QueryContext};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};

#[derive(Clone, Debug)]
pub struct Call<Q: QueryContext> {
    pub index: usize,
    pub args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> Call<Q> {
    pub fn new(index: usize, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self { index, args }
    }
}

impl<Q: QueryContext> PatternName for Call<Q> {
    fn name(&self) -> &'static str {
        "CALL"
    }
}

// todo parameters, and name should both be usize references
// argument should throw an error if its not a parameter at compile time
impl<Q: QueryContext> Matcher<Q> for Call<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let pattern_definition = &context.pattern_definitions()[self.index];

        pattern_definition.call(state, binding, context, logs, &self.args)
    }
}

#[derive(Debug, Clone)]
pub struct PrCall<Q: QueryContext> {
    pub(crate) index: usize,
    pub args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> PrCall<Q> {
    pub fn new(index: usize, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self { index, args }
    }
}

impl<Q: QueryContext> PatternName for PrCall<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_CALL"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrCall<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
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
                Err(GritPatternError::new(format!(
                    "predicate or function definition not found: {}. Try running grit init.",
                    self.index
                )))
            }
        }
    }
}
