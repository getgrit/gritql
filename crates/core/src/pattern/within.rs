use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{context::Context, resolve};
use anyhow::Result;
use core::fmt::Debug;
use grit_util::AstNode;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Within {
    pub(crate) pattern: Pattern,
}

impl Within {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }
}

impl Name for Within {
    fn name(&self) -> &'static str {
        "WITHIN"
    }
}

impl Matcher for Within {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
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
