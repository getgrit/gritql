use super::{
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::{ExecContext, QueryContext};
use grit_util::{error::GritResult, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct Assignment<Q: QueryContext> {
    pub container: Container<Q>,
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Assignment<Q> {
    pub fn new(container: Container<Q>, pattern: Pattern<Q>) -> Self {
        Self { container, pattern }
    }
}

impl<Q: QueryContext> PatternName for Assignment<Q> {
    fn name(&self) -> &'static str {
        "assignment"
    }
}

impl<Q: QueryContext> Matcher<Q> for Assignment<Q> {
    fn execute<'a>(
        &'a self,
        _context_node: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container
            .set_resolved(state, context.language(), resolved)?;
        Ok(true)
    }
}

impl<Q: QueryContext> Evaluator<Q> for Assignment<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        let resolved: Q::ResolvedPattern<'_> =
            ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container
            .set_resolved(state, context.language(), resolved)?;
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}
