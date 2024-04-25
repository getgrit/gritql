use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    constant::Constant,
    context::{ExecContext, QueryContext},
};
use anyhow::Result;
use grit_util::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Subtract<Q: QueryContext> {
    pub lhs: Pattern<Q>,
    pub rhs: Pattern<Q>,
}

impl<Q: QueryContext> Subtract<Q> {
    pub fn new(lhs: Pattern<Q>, rhs: Pattern<Q>) -> Self {
        Self { lhs, rhs }
    }

    pub fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Q::ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(Q::ResolvedPattern::from_constant(Constant::Float(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        let lhs = self.lhs.float(state, context, logs)?;
        let rhs = self.rhs.float(state, context, logs)?;
        let res = lhs - rhs;
        Ok(res)
    }
}

impl<Q: QueryContext> PatternName for Subtract<Q> {
    fn name(&self) -> &'static str {
        "SUBTRACT"
    }
}

impl<Q: QueryContext> Matcher<Q> for Subtract<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
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
