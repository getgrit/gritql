use super::patterns::PatternName;
use super::resolved_pattern::ResolvedPattern;
use super::{patterns::Matcher, patterns::Pattern, PatternDefinition, State};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Bubble<P: ProblemContext> {
    pub pattern_def: PatternDefinition<P>,
    pub args: Vec<Option<Pattern<P>>>,
}

impl<P: ProblemContext> Bubble<P> {
    pub fn new(pattern_def: PatternDefinition<P>, args: Vec<Pattern<P>>) -> Self {
        Self {
            pattern_def,
            args: args.into_iter().map(Some).collect(),
        }
    }
}

impl<P: ProblemContext> PatternName for Bubble<P> {
    fn name(&self) -> &'static str {
        "BUBBLE"
    }
}

impl<P: ProblemContext> Matcher<P> for Bubble<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.pattern_def
            .call(state, binding, context, logs, &self.args)
    }
}
