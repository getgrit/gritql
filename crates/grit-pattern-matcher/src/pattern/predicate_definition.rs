use super::{
    functions::Evaluator, patterns::Pattern, predicates::Predicate, variable::Variable, State,
};
use crate::context::QueryContext;
use crate::errors::GritResult;
use grit_util::AnalysisLogs;

#[derive(Clone, Debug)]
pub struct PredicateDefinition<Q: QueryContext> {
    pub name: String,
    pub scope: usize,
    pub params: Vec<(String, Variable)>,
    // this could just be a usize representing the len
    pub local_vars: Vec<usize>,
    pub predicate: Predicate<Q>,
}

impl<Q: QueryContext> PredicateDefinition<Q> {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        predicate: Predicate<Q>,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            predicate,
        }
    }

    pub fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        args: &'a [Option<Pattern<Q>>],
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        state.reset_vars(self.scope, args);
        let res = self.predicate.execute_func(state, context, logs)?;
        Ok(res.predicator)
    }
}
