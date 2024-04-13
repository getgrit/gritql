use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::ProblemContext, resolve};
use anyhow::Result;
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Within<P: ProblemContext> {
    pub(crate) pattern: Pattern<P>,
}

impl<P: ProblemContext> Within<P> {
    pub fn new(pattern: Pattern<P>) -> Self {
        Self { pattern }
    }
}

impl<P: ProblemContext> PatternName for Within<P> {
    fn name(&self) -> &'static str {
        "WITHIN"
    }
}

impl<P: ProblemContext> Matcher<P> for Within<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
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

        let binding = if let ResolvedPattern::Binding(binding) = binding {
            resolve!(binding.last())
        } else {
            return Ok(did_match);
        };

        let Some(node) = binding.parent_node() else {
            return Ok(did_match);
        };
        for n in node.ancestors() {
            let state = cur_state.clone();
            if self.pattern.execute(
                &ResolvedPattern::from_node(n),
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
