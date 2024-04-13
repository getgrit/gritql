use super::{
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Assignment<P: ProblemContext> {
    pub container: Container<P>,
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> Assignment<P> {
    pub fn new(container: Container<P>, pattern: Pattern<P>) -> Self {
        Self { container, pattern }
    }
}

impl<P: ProblemContext> PatternName for Assignment<P> {
    fn name(&self) -> &'static str {
        "assignment"
    }
}

impl<P: ProblemContext> Matcher<P> for Assignment<P> {
    fn execute<'a>(
        &'a self,
        _context_node: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container.set_resolved(state, resolved)?;
        Ok(true)
    }
}

impl<P: ProblemContext> Evaluator<P> for Assignment<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let resolved: ResolvedPattern<'_> =
            ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container.set_resolved(state, resolved)?;
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}
