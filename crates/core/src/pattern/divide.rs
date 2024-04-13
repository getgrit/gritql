use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{binding::Constant, context::ProblemContext};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Divide<P: ProblemContext> {
    pub lhs: Pattern<P>,
    pub rhs: Pattern<P>,
}

impl<P: ProblemContext> Divide<P> {
    pub fn new(lhs: Pattern<P>, rhs: Pattern<P>) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(ResolvedPattern::Constant(Constant::Float(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        let lhs = self.lhs.float(state, context, logs)?;
        let rhs = self.rhs.float(state, context, logs)?;
        let res = lhs / rhs;
        Ok(res)
    }
}

impl<P: ProblemContext> PatternName for Divide<P> {
    fn name(&self) -> &'static str {
        "DIVIDE"
    }
}

impl<P: ProblemContext> Matcher<P> for Divide<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files)?;
        let binding_int = binding_text.parse::<f64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
