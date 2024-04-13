use super::{
    functions::Evaluator, patterns::Pattern, predicates::Predicate, variable::Variable, State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Clone, Debug)]
pub struct PredicateDefinition<P: ProblemContext> {
    pub name: String,
    pub scope: usize,
    pub params: Vec<(String, Variable)>,
    // this could just be a usize representing the len
    pub local_vars: Vec<usize>,
    pub predicate: Predicate<P>,
}

impl<P: ProblemContext> PredicateDefinition<P> {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        predicate: Predicate<P>,
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
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        args: &'a [Option<Pattern<P>>],
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        state.reset_vars(self.scope, args);
        let res = self.predicate.execute_func(state, context, logs)?;
        Ok(res.predicator)
    }
}
