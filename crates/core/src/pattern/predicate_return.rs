use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct PrReturn<P: ProblemContext> {
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> PrReturn<P> {
    pub fn new(pattern: Pattern<P>) -> Self {
        Self { pattern }
    }
}

impl<P: ProblemContext> Evaluator<P> for PrReturn<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: false,
            ret_val: Some(resolved),
        })
    }
}

impl<P: ProblemContext> PatternName for PrReturn<P> {
    fn name(&self) -> &'static str {
        "RETURN"
    }
}
