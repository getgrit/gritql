use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Pattern, PatternName},
    variable::Variable,
    State,
};
use crate::context::ProblemContext;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Equal<P: ProblemContext> {
    pub var: Variable,
    pub pattern: Pattern<P>,
}

impl<P: ProblemContext> Equal<P> {
    pub fn new(var: Variable, pattern: Pattern<P>) -> Self {
        Self { var, pattern }
    }
}

impl<P: ProblemContext> PatternName for Equal<P> {
    fn name(&self) -> &'static str {
        "EQUAL"
    }
}

impl<P: ProblemContext> Evaluator<P> for Equal<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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
