use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Name, Pattern},
    variable::Variable,
    State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Equal {
    pub var: Variable,
    pub pattern: Pattern,
}

impl Equal {
    pub fn new(var: Variable, pattern: Pattern) -> Self {
        Self { var, pattern }
    }
}

impl Name for Equal {
    fn name(&self) -> &'static str {
        "EQUAL"
    }
}

impl Evaluator for Equal {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let lhs_text = self.var.text(state)?;
        let rhs_text = self.pattern.text(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: lhs_text == rhs_text,
            ret_val: None,
        })
    }
}
