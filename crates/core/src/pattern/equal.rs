use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Pattern, PatternName},
    variable::Variable,
    State,
};
use crate::context::{ExecContext, QueryContext};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Equal<Q: QueryContext> {
    pub var: Variable,
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Equal<Q> {
    pub fn new(var: Variable, pattern: Pattern<Q>) -> Self {
        Self { var, pattern }
    }
}

impl<Q: QueryContext> PatternName for Equal<Q> {
    fn name(&self) -> &'static str {
        "EQUAL"
    }
}

impl<Q: QueryContext> Evaluator<Q> for Equal<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let lhs_text = self.var.text(state, context.language())?;
        let rhs_text = self.pattern.text(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator: lhs_text == rhs_text,
            ret_val: None,
        })
    }
}
