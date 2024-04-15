use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    binding::Constant,
    context::{ExecContext, QueryContext},
};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Add<Q: QueryContext> {
    pub(crate) lhs: Pattern<Q>,
    pub(crate) rhs: Pattern<Q>,
}

impl<Q: QueryContext> Add<Q> {
    pub fn new(lhs: Pattern<Q>, rhs: Pattern<Q>) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(ResolvedPattern::Constant(Constant::Float(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        let lhs = self.lhs.float(state, context, logs)?;
        let rhs = self.rhs.float(state, context, logs)?;
        let res = lhs + rhs;
        Ok(res)
    }
}

impl<Q: QueryContext> PatternName for Add<Q> {
    fn name(&self) -> &'static str {
        "ADD"
    }
}

impl<Q: QueryContext> Matcher<Q> for Add<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files, context.language())?;
        let binding_int = binding_text.parse::<f64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
