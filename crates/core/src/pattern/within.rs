use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Binding, context::QueryContext};
use anyhow::Result;
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Within<Q: QueryContext> {
    pub(crate) pattern: Pattern<Q>,
}

impl<Q: QueryContext> Within<Q> {
    pub fn new(pattern: Pattern<Q>) -> Self {
        Self { pattern }
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
    ) -> Result<bool> {
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
            if self.pattern.execute(
                &ResolvedPattern::from_node_binding(n),
                &mut cur_state,
                context,
                logs,
            )? {
                did_match = true;
            } else {
                cur_state = state;
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
