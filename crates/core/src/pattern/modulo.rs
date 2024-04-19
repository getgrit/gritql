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
pub struct Modulo<Q: QueryContext> {
    pub lhs: Pattern<Q>,
    pub rhs: Pattern<Q>,
}

impl<Q: QueryContext> Modulo<Q> {
    pub fn new(lhs: Pattern<Q>, rhs: Pattern<Q>) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<Q::ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(Q::ResolvedPattern::from_constant(Constant::Integer(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<i64> {
        let lhs = self.lhs.text(state, context, logs)?;
        let rhs = self.rhs.text(state, context, logs)?;
        let lhs_int = lhs.parse::<i64>()?;
        let rhs_int = rhs.parse::<i64>()?;
        let res = lhs_int % rhs_int;
        Ok(res)
    }
}

impl<Q: QueryContext> PatternName for Modulo<Q> {
    fn name(&self) -> &'static str {
        "MODULO"
    }
}

impl<Q: QueryContext> Matcher<Q> for Modulo<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files, context.language())?;
        let binding_int = binding_text.parse::<i64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
