use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Constant, context::QueryContext};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Some<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Some<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> PatternName for Some<Q> {
    fn name(&self) -> &'static str {
        "SOME"
    }
}

impl<Q: QueryContext> Matcher<Q> for Some<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Some(items) = binding.get_list_binding_items() {
            let mut did_match = false;
            let mut cur_state = init_state.clone();
            for item in items {
                let state = cur_state.clone();
                if self.pattern.execute(&item, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = state;
                }
            }
            *init_state = cur_state;
            Ok(did_match)
        } else if let Some(items) = binding.get_list_items() {
            let mut cur_state = init_state.clone();
            let mut did_match = false;
            for item in items {
                let state = cur_state.clone();
                if self.pattern.execute(item, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = state;
                }
            }
            *init_state = cur_state;
            Ok(did_match)
        } else if let Some(map) = binding.get_map() {
            let pattern = &self.pattern;
            let mut cur_state = init_state.clone();
            let mut did_match = false;
            for (key, value) in map {
                let state = cur_state.clone();
                let key = ResolvedPattern::from_constant(Constant::String(key.clone()));
                let resolved = ResolvedPattern::from_list_parts([key, value.clone()].into_iter());
                if pattern.execute(&resolved, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = state;
                }
            }
            *init_state = cur_state;
            Ok(did_match)
        } else {
            Ok(false)
        }
    }
}
