use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::{pattern_to_binding, ResolvedPattern},
    State,
};
use crate::{
    binding::Constant,
    context::{ExecContext, QueryContext},
    errors::debug,
    resolve,
};
use anyhow::{bail, Result};
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Before<Q: QueryContext> {
    pub before: Pattern<Q>,
}

impl<Q: QueryContext> Before<Q> {
    pub fn new(before: Pattern<Q>) -> Self {
        Self { before }
    }

    pub(crate) fn prev_pattern<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'a>> {
        let binding = pattern_to_binding(&self.before, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            bail!("cannot get the node before this binding")
        };

        if let Some(prev) = node.previous_named_node() {
            Ok(ResolvedPattern::from_node(prev))
        } else {
            debug(
                logs,
                state,
                context.language(),
                "no node before current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::Constant(Constant::Undefined))
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
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let binding = match binding {
            ResolvedPattern::Binding(binding) => resolve!(binding.last()),
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::List(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Constant(_) => return Ok(true),
        };
        let mut cur_state = init_state.clone();
        // todo implement for empty and empty list
        let Some(node) = binding.as_node() else {
            return Ok(true);
        };
        let next_node = resolve!(node.next_named_node());
        if !self.before.execute(
            &ResolvedPattern::from_node(next_node),
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
