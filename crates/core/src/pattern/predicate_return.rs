use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct PrReturn {
    pub pattern: Pattern,
}

impl PrReturn {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Evaluator for PrReturn {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: false,
            ret_val: Some(resolved),
        })
    }
}

impl Name for PrReturn {
    fn name(&self) -> &'static str {
        "RETURN"
    }
}
