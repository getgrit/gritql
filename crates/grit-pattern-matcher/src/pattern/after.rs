use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::errors::{GritPatternError, GritResult};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
    errors::debug,
};
use core::fmt::Debug;
use grit_util::{AnalysisLogs, AstNode};

#[derive(Debug, Clone)]
pub struct After<Q: QueryContext> {
    pub after: Pattern<Q>,
}

impl<Q: QueryContext> After<Q> {
    pub fn new(after: Pattern<Q>) -> Self {
        Self { after }
    }

    pub fn next_pattern<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Q::ResolvedPattern<'a>> {
        let binding = Q::Binding::from_pattern(&self.after, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            return Err(GritPatternError::new(
                "cannot get the node after this binding",
            ));
        };

        if let Some(next) = node.next_named_node() {
            Ok(Q::ResolvedPattern::from_node_binding(next))
        } else {
            debug(
                logs,
                state,
                context.language(),
                "no node after current node, treating as undefined",
            )?;
            Ok(Q::ResolvedPattern::undefined())
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
        binding: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        let Some(binding) = binding.get_last_binding() else {
            return Ok(true);
        };
        let mut cur_state = init_state.clone();
        // todo implement for empty and empty list
        let Some(node) = binding.as_node() else {
            return Ok(true);
        };
        let Some(prev_node) = node.previous_named_node() else {
            return Ok(false);
        };
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
