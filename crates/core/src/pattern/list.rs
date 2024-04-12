use super::{
    list_index,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct List {
    pub patterns: Vec<Pattern>,
}

impl List {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }

    pub fn get(&self, index: isize) -> Option<&Pattern> {
        self.patterns
            .get(list_index::to_unsigned(index, self.patterns.len())?)
    }
}

impl Name for List {
    fn name(&self) -> &'static str {
        "LIST"
    }
}

impl Matcher for List {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut super::state::State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match binding {
            ResolvedPattern::Binding(v) => {
                let Some(list_items) = v.last().and_then(|b| b.list_items()) else {
                    return Ok(false);
                };

                let children: Vec<Cow<ResolvedPattern>> = list_items
                    .map(ResolvedPattern::from_node)
                    .map(Cow::Owned)
                    .collect();

                execute_assoc(&self.patterns, &children, state, context, logs)
            }
            ResolvedPattern::List(patterns) => {
                let patterns: Vec<Cow<ResolvedPattern<'_>>> =
                    patterns.into_iter().map(Cow::Borrowed).collect();
                execute_assoc(&self.patterns, &patterns, state, context, logs)
            }
            ResolvedPattern::Snippets(_)
            | ResolvedPattern::File(_)
            | ResolvedPattern::Files(_)
            | ResolvedPattern::Map(_)
            | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}

fn execute_assoc<'a>(
    patterns: &'a [Pattern],
    children: &[Cow<ResolvedPattern<'a>>],
    current_state: &mut State<'a>,
    context: &'a impl Context,
    logs: &mut AnalysisLogs,
) -> Result<bool> {
    let mut working_state = current_state.clone();
    match patterns {
        // short circuit for common case
        [pattern_for_first_node, Pattern::Dots] => {
            if children.is_empty() {
                return Ok(false);
            }
            let first_node = children[0].clone();
            if pattern_for_first_node.execute(&first_node, &mut working_state, context, logs)? {
                *current_state = working_state;
                Ok(true)
            } else {
                Ok(false)
            }
        }
        // short circuit for common case
        [Pattern::Dots, pattern_for_last_node] => {
            if let Some(last_node) = children.last() {
                if pattern_for_last_node.execute(last_node, &mut working_state, context, logs)? {
                    *current_state = working_state;
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        [Pattern::Dots, head_pattern, tail_patterns @ ..] => {
            if let Pattern::Dots = head_pattern {
                return Err(anyhow!("Multiple subsequent dots are not allowed."));
            }
            for index in 0..children.len() {
                if head_pattern.execute(&children[index], &mut working_state, context, logs)?
                    && execute_assoc(
                        tail_patterns,
                        &children[index + 1..],
                        &mut working_state,
                        context,
                        logs,
                    )?
                {
                    *current_state = working_state;
                    return Ok(true);
                }
            }
            Ok(false)
        }
        [Pattern::Dots] => Ok(true),
        [only_pattern] => {
            if children.len() == 1 {
                if only_pattern.execute(&children[0], &mut working_state, context, logs)? {
                    *current_state = working_state;
                    Ok(true)
                } else {
                    Ok(false)
                }
            } else {
                Ok(false)
            }
        }
        [head_pattern, tail_patterns @ ..] => match children {
            [head_node, tail_nodes @ ..] => {
                if head_pattern.execute(head_node, &mut working_state, context, logs)? {
                    if let Ok(true) =
                        execute_assoc(tail_patterns, tail_nodes, &mut working_state, context, logs)
                    {
                        *current_state = working_state;
                        Ok(true)
                    } else {
                        Ok(false)
                    }
                } else {
                    Ok(false)
                }
            }
            [] => Ok(false),
        },
        [] => Ok(children.is_empty()),
    }
}
