use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct PrReturn<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> PrReturn<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrReturn<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: false,
            ret_val: Some(resolved),
        })
    }
}

impl<Q: QueryContext> PatternName for PrReturn<Q> {
    fn name(&self) -> &'static str {
        "RETURN"
    }
}
