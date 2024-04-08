use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{binding::Constant, context::Context};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Modulo {
    pub lhs: Pattern,
    pub rhs: Pattern,
}

impl Modulo {
    pub fn new(lhs: Pattern, rhs: Pattern) -> Self {
        Self { lhs, rhs }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let res = self.evaluate(state, context, logs)?;
        Ok(ResolvedPattern::Constant(Constant::Integer(res)))
    }

    fn evaluate<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
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

impl Name for Modulo {
    fn name(&self) -> &'static str {
        "MODULO"
    }
}

impl Matcher for Modulo {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding_text = binding.text(&state.files)?;
        let binding_int = binding_text.parse::<i64>()?;
        let target = self.evaluate(state, context, logs)?;
        Ok(binding_int == target)
    }
}
