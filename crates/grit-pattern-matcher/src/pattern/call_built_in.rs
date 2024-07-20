use super::{
    functions::GritCall,
    patterns::{Pattern, PatternName},
    State,
};
use crate::context::{ExecContext, QueryContext};
use anyhow::Result;
use grit_util::AnalysisLogs;

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
    ) -> Result<Q::ResolvedPattern<'a>> {
        context.call_built_in(self, context, state, logs)
    }
}

impl<Q: QueryContext> PatternName for CallBuiltIn<Q> {
    fn name(&self) -> &'static str {
        "CALL_BUILT_IN"
    }
}
