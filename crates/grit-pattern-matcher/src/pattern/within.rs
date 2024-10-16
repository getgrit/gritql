use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Binding, context::QueryContext};
use core::fmt::Debug;
use grit_util::{error::GritResult, AnalysisLogs, AstNode};

#[derive(Debug, Clone)]
pub struct Within<Q: QueryContext> {
    pub pattern: Pattern<Q>,
    until: Option<Pattern<Q>>,
}

impl<Q: QueryContext> Within<Q> {
    pub fn new(pattern: Pattern<Q>, until: Option<Pattern<Q>>) -> Self {
        Self { pattern, until }
    }
}

impl<Q: QueryContext> PatternName for Within<Q> {
    fn name(&self) -> &'static str {
        "WITHIN"
    }
}

impl<Q: QueryContext> Matcher<Q> for Within<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let mut did_match = false;
        let mut cur_state = init_state.clone();

        let state = cur_state.clone();
        if self
            .pattern
            .execute(binding, &mut cur_state, context, logs)?
        {
            did_match = true;
        } else {
            cur_state = state;
        }

        let Some(node) = binding.get_last_binding().and_then(Binding::parent_node) else {
            return Ok(did_match);
        };
        for n in node.ancestors() {
            let state = cur_state.clone();
            let resolved = ResolvedPattern::from_node_binding(n);
            if self
                .pattern
                .execute(&resolved, &mut cur_state, context, logs)?
            {
                did_match = true;
                break;
            } else {
                cur_state = state;

                if let Some(until) = &self.until {
                    if until.execute(&resolved, &mut cur_state, context, logs)? {
                        break;
                    }
                }
            }
        }
        if did_match {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}
