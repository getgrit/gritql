use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
    errors::debug,
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs, AstNode,
};

#[derive(Debug, Clone)]
pub struct Before<Q: QueryContext> {
    pub before: Pattern<Q>,
}

impl<Q: QueryContext> Before<Q> {
    pub fn new(before: Pattern<Q>) -> Self {
        Self { before }
    }

    pub fn prev_pattern<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<Q::ResolvedPattern<'a>> {
        let binding = Q::Binding::from_pattern(&self.before, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            return Err(GritPatternError::new(
                "cannot get the node before this binding",
            ));
        };

        if let Some(prev) = node.previous_named_node() {
            Ok(ResolvedPattern::from_node_binding(prev))
        } else {
            debug(
                logs,
                state,
                context.language(),
                "no node before current node, treating as undefined",
            )?;
            Ok(Q::ResolvedPattern::undefined())
        }
    }
}

impl<Q: QueryContext> PatternName for Before<Q> {
    fn name(&self) -> &'static str {
        "BEFORE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Before<Q> {
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
        let Some(next_node) = node.next_named_node() else {
            return Ok(false);
        };
        if !self.before.execute(
            &ResolvedPattern::from_node_binding(next_node),
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
