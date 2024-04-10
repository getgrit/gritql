use super::{
    patterns::{Matcher, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::Variable,
    State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Clone, Debug)]
pub struct PatternDefinition {
    pub name: String,
    pub scope: usize,
    pub params: Vec<(String, Variable)>,
    // this could just be a usize representing the len
    pub local_vars: Vec<usize>,
    pub pattern: Pattern,
}

impl PatternDefinition {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        pattern: Pattern,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            pattern,
        }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        binding: &ResolvedPattern<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
        args: &'a [Option<Pattern>],
    ) -> Result<bool> {
        state.reset_vars(self.scope, args);
        let res = self.pattern.execute(binding, state, context, logs);

        let fn_state = state.bindings[self.scope].pop_back().unwrap();
        let cur_fn_state = state.bindings[self.scope].back_mut().unwrap();
        for (cur, last) in cur_fn_state.iter_mut().zip(fn_state) {
            cur.value_history.extend(last.value_history)
        }
        res
    }
}
