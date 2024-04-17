use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::{pattern_to_binding, ResolvedPattern},
    State,
};
use crate::{
    context::{ExecContext, QueryContext},
    errors::debug,
    resolve,
};
use anyhow::{bail, Result};
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct After<Q: QueryContext> {
    pub(crate) after: Pattern<Q>,
}

impl<Q: QueryContext> After<Q> {
    pub fn new(after: Pattern<Q>) -> Self {
        Self { after }
    }

    pub(crate) fn next_pattern<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let binding = pattern_to_binding(&self.after, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            bail!("cannot get the node after this binding")
        };

        if let Some(next) = node.next_named_node() {
            Ok(ResolvedPattern::from_node_binding(next))
        } else {
            debug(
                logs,
                state,
                context.language(),
                "no node after current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::undefined())
        }
    }
}

impl<Q: QueryContext> PatternName for After<Q> {
    fn name(&self) -> &'static str {
        "AFTER"
    }
}

impl<Q: QueryContext> Matcher<Q> for After<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let Some(binding) = binding.get_last_binding() else {
            return Ok(true);
        };
        let mut cur_state = init_state.clone();
        // todo implement for empty and empty list
        let Some(node) = binding.as_node() else {
            return Ok(true);
        };
        let prev_node = resolve!(node.previous_named_node());
        if !self.after.execute(
            &ResolvedPattern::from_node_binding(prev_node),
            &mut cur_state,
            context,
            logs,
        )? {
            return Ok(false);
        }
        *init_state = cur_state;
        Ok(true)
    }
}
