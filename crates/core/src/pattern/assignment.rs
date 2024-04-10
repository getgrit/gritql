use super::{
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub container: Container,
    pub pattern: Pattern,
}

impl Assignment {
    pub fn new(container: Container, pattern: Pattern) -> Self {
        Self { container, pattern }
    }
}

impl Name for Assignment {
    fn name(&self) -> &'static str {
        "assignment"
    }
}

impl Matcher for Assignment {
    fn execute<'a>(
        &'a self,
        _context_node: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container.set_resolved(state, resolved)?;
        Ok(true)
    }
}

impl Evaluator for Assignment {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
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
