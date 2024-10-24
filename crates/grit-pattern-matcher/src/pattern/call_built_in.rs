use super::{
    functions::GritCall,
    patterns::{Pattern, PatternName},
    Evaluator, FuncEvaluation, ResolvedPattern, State,
};
use crate::context::{ExecContext, QueryContext};
use grit_util::{error::GritResult, AnalysisLogs};

// todo we can probably use a macro to generate a function that takes a vec and
// and calls the input function with the vec args unpacked.

#[derive(Debug, Clone)]
pub struct CallBuiltIn<Q: QueryContext> {
    pub index: usize,
    pub name: String,
    pub args: Vec<Option<Pattern<Q>>>,
}

impl<Q: QueryContext> CallBuiltIn<Q> {
    pub fn new(index: usize, name: &str, args: Vec<Option<Pattern<Q>>>) -> Self {
        Self {
            index,
            name: name.to_string(),
            args,
        }
    }
}

impl<Q: QueryContext> GritCall<Q> for CallBuiltIn<Q> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Q::ResolvedPattern<'a>> {
        context.call_built_in(self, context, state, logs)
    }
}

impl<Q: QueryContext> Evaluator<Q> for CallBuiltIn<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        let resolved = self.call(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: resolved.is_truthy(state, context.language())?,
            ret_val: Some(resolved),
        })
    }
}

impl<Q: QueryContext> PatternName for CallBuiltIn<Q> {
    fn name(&self) -> &'static str {
        "CALL_BUILT_IN"
    }
}
