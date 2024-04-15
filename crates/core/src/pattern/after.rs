use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::{pattern_to_binding, ResolvedPattern},
    State,
};
use crate::{binding::Binding, constant::Constant, context::QueryContext, errors::debug, resolve};
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
    ) -> Result<ResolvedPattern<'a, Q>> {
        let binding = pattern_to_binding(&self.after, state, context, logs)?;
        let Some(node) = binding.as_node() else {
            bail!("cannot get the node after this binding")
        };

        if let Some(next) = node.next_named_node() {
            Ok(ResolvedPattern::from_node(next))
        } else {
            debug(
                logs,
                state,
                "no node after current node, treating as undefined",
            )?;
            Ok(ResolvedPattern::Constant(Constant::Undefined))
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
        binding: &ResolvedPattern<'a, Q>,
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
        let prev_node = resolve!(node.previous_named_node());
        if !self.after.execute(
            &ResolvedPattern::from_node(prev_node),
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
