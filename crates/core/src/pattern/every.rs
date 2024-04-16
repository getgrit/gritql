use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Constant, context::QueryContext};
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Every<Q: QueryContext> {
    pub pattern: Pattern<Q>,
}

impl<Q: QueryContext> Every<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
    }
}

impl<Q: QueryContext> PatternName for Every<Q> {
    fn name(&self) -> &'static str {
        "EVERY"
    }
}

impl<Q: QueryContext> Matcher<Q> for Every<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        // might be necessary to clone init state at the top,
        // but more performant to not, so leaving out for now.
        if let Some(items) = binding.get_list_binding_items() {
            let pattern = &self.pattern;
            for item in items {
                if !pattern.execute(&item, init_state, context, logs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        } else if let Some(items) = binding.get_list_items() {
            let pattern = &self.pattern;
            for item in items {
                if !pattern.execute(item, init_state, context, logs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        } else if let Some(map) = binding.get_map() {
            let pattern = &self.pattern;
            for (key, value) in map {
                let key = ResolvedPattern::from_constant(Constant::String(key.clone()));
                let resolved = ResolvedPattern::from_list_parts([key, value.clone()].into_iter());
                if !pattern.execute(&resolved, init_state, context, logs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
