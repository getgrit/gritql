use super::{
    list_index,
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::QueryContext;
use core::fmt::Debug;
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct List<Q: QueryContext> {
    pub patterns: Vec<Pattern<Q>>,
}

impl<Q: QueryContext> List<Q> {
    pub fn new(patterns: Vec<Pattern<Q>>) -> Self {
        Self { patterns }
    }

    pub fn get(&self, index: isize) -> Option<&Pattern<Q>> {
        self.patterns
            .get(list_index::to_unsigned(index, self.patterns.len())?)
    }
}

impl<Q: QueryContext> PatternName for List<Q> {
    fn name(&self) -> &'static str {
        "LIST"
    }
}

impl<Q: QueryContext> Matcher<Q> for List<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        if let Some(items) = binding.get_list_binding_items() {
            let patterns: Vec<_> = items.map(Cow::Owned).collect();
            execute_assoc(&self.patterns, &patterns, state, context, logs)
        } else if let Some(items) = binding.get_list_items() {
            let patterns: Vec<_> = items.map(Cow::Borrowed).collect();
            execute_assoc(&self.patterns, &patterns, state, context, logs)
        } else {
            Ok(false)
        }
    }
}

fn execute_assoc<'a, Q: QueryContext>(
    patterns: &'a [Pattern<Q>],
    children: &[Cow<Q::ResolvedPattern<'a>>],
    current_state: &mut State<'a, Q>,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
) -> GritResult<bool> {
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
                return Err(GritPatternError::new(
                    "Multiple subsequent dots are not allowed.",
                ));
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
