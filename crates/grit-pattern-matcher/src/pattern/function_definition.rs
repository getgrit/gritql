use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::Pattern,
    predicates::Predicate,
    state::State,
    variable::Variable,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};

pub trait FunctionDefinition<Q: QueryContext> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        args: &'a [Option<Pattern<Q>>],
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>>;
}

#[derive(Clone, Debug)]
pub struct GritFunctionDefinition<Q: QueryContext> {
    pub name: String,
    pub scope: usize,
    pub params: Vec<(String, Variable)>,
    pub local_vars: Vec<usize>,
    pub function: Predicate<Q>,
}

impl<Q: QueryContext> GritFunctionDefinition<Q> {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        function: Predicate<Q>,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            function,
        }
    }
}

impl<Q: QueryContext> FunctionDefinition<Q> for GritFunctionDefinition<Q> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        args: &'a [Option<Pattern<Q>>],
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        let tracker = state.enter_scope(self.scope, args);
        let res = self.function.execute_func(state, context, logs);
        state.exit_scope(tracker);
        res
    }
}
