use super::patterns::Name;
use super::resolved_pattern::ResolvedPattern;
use super::{patterns::Matcher, patterns::Pattern, PatternDefinition, State};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Bubble {
    pub pattern_def: PatternDefinition,
    pub args: Vec<Option<Pattern>>,
}

impl Bubble {
    pub fn new(pattern_def: PatternDefinition, args: Vec<Pattern>) -> Self {
        Self {
            pattern_def,
            args: args.into_iter().map(Some).collect(),
        }
    }
}

impl Name for Bubble {
    fn name(&self) -> &'static str {
        "BUBBLE"
    }
}

impl Matcher for Bubble {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.pattern_def
            .call(state, binding, context, logs, &self.args)
    }
}
