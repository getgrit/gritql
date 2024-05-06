use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::{LazyBuiltIn, ResolvedPattern, ResolvedSnippet},
    State,
};
use crate::{
    binding::Binding, constants::PROGRAM_INDEX, context::QueryContext,
    pattern::resolved_pattern::File,
};
use crate::{constants::GLOBAL_VARS_SCOPE_INDEX, context::ExecContext};
use anyhow::Result;
use core::fmt::Debug;
use grit_util::AnalysisLogs;
use grit_util::{AstCursor, AstNode};

#[derive(Debug, Clone)]
pub struct Contains<Q: QueryContext> {
    pub contains: Pattern<Q>,
    pub until: Option<Pattern<Q>>,
}

impl<Q: QueryContext> Contains<Q> {
    pub fn new(contains: Pattern<Q>, until: Option<Pattern<Q>>) -> Self {
        Self { contains, until }
    }
}

impl<Q: QueryContext> PatternName for Contains<Q> {
    fn name(&self) -> &'static str {
        "CONTAINS"
    }
}

fn execute_until<'a, Q: QueryContext>(
    init_state: &mut State<'a, Q>,
    node: &Q::Node<'a>,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
    the_contained: &'a Pattern<Q>,
    until: &'a Option<Pattern<Q>>,
) -> Result<bool, anyhow::Error> {
    let mut did_match = false;
    let mut cur_state = init_state.clone();
    let mut cursor = node.walk();
    let mut still_computing = true;
    while still_computing {
        let node = cursor.node();
        let node_lhs = ResolvedPattern::from_node_binding(node);

        let state = cur_state.clone();
        if the_contained.execute(&node_lhs, &mut cur_state, context, logs)? {
            did_match = true;
        } else {
            cur_state = state;
        }

        let mut state = cur_state.clone();
        let skip_children = if let Some(until) = until {
            until.execute(&node_lhs, &mut state, context, logs)?
        } else {
            false
        };

        if (!skip_children && cursor.goto_first_child()) || cursor.goto_next_sibling() {
            // all good
            continue;
        }
        // go up the parent chain until we find a sibling
        loop {
            if !cursor.goto_parent() {
                still_computing = false;
                break;
            }

            if cursor.goto_next_sibling() {
                break;
            }
        }
    }
    if did_match {
        *init_state = cur_state;
    }
    Ok(did_match)
}

// Contains and within should call the same function taking an iterator as an argument
// even better two arguments an accumulator and an iterator.
impl<Q: QueryContext> Matcher<Q> for Contains<Q> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &Q::ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Some(binding) = resolved_pattern.get_last_binding() {
            if let Some(node) = binding.as_node() {
                execute_until(
                    init_state,
                    &node,
                    context,
                    logs,
                    &self.contains,
                    &self.until,
                )
            } else if let Some(items) = binding.list_items() {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for item in items {
                    let state = cur_state.clone();
                    if self.execute(
                        &ResolvedPattern::from_node_binding(item),
                        &mut cur_state,
                        context,
                        logs,
                    )? {
                        did_match = true;
                    } else {
                        cur_state = state;
                    }
                }
                if !did_match {
                    return Ok(false);
                }
                *init_state = cur_state;
                Ok(true)
            } else if let Some(_c) = binding.as_constant() {
                // this seems like an infinite loop, todo return false?
                self.contains
                    .execute(resolved_pattern, init_state, context, logs)
            } else {
                Ok(false)
            }
        } else if let Some(items) = resolved_pattern.get_list_items() {
            let mut cur_state = init_state.clone();
            let mut did_match = false;
            for item in items {
                let state = cur_state.clone();
                if self.execute(item, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = state;
                }
            }
            if !did_match {
                return Ok(false);
            }
            *init_state = cur_state;
            Ok(true)
        } else if let Some(file) = resolved_pattern.get_file() {
            // Load the file in, if it wasn't already
            if !context.load_file(file, init_state, logs)? {
                return Ok(false);
            }

            let mut cur_state = init_state.clone();
            let mut did_match = false;
            let prev_state = cur_state.clone();

            if self
                .contains
                .execute(resolved_pattern, &mut cur_state, context, logs)?
            {
                did_match = true;
            } else {
                cur_state = prev_state;
            }

            let prev_state = cur_state.clone();
            if self
                .contains
                .execute(&file.name(&cur_state.files), &mut cur_state, context, logs)?
            {
                did_match = true;
            } else {
                cur_state = prev_state;
            }

            let prev_state = cur_state.clone();
            if self.execute(
                &file.binding(&cur_state.files),
                &mut cur_state,
                context,
                logs,
            )? {
                did_match = true;
            } else {
                cur_state = prev_state;
            }
            if !did_match {
                return Ok(false);
            }
            *init_state = cur_state;
            Ok(true)
        } else if let Some(files) = resolved_pattern.get_files() {
            let mut cur_state = init_state.clone();
            let mut did_match = false;
            let prev_state = cur_state.clone();
            if self
                .contains
                .execute(resolved_pattern, &mut cur_state, context, logs)?
            {
                did_match = true;
            } else {
                cur_state = prev_state;
            }
            let prev_state = cur_state.clone();
            if self.execute(files, &mut cur_state, context, logs)? {
                did_match = true;
            } else {
                cur_state = prev_state;
            }
            if !did_match {
                return Ok(false);
            }
            *init_state = cur_state;
            Ok(true)
        } else if let Some(snippets) = resolved_pattern.get_snippets() {
            let mut cur_state = init_state.clone();
            let mut did_match = false;
            for snippet in snippets {
                let state = cur_state.clone();
                let resolved = match snippet {
                    ResolvedSnippet::Text(_) => {
                        ResolvedPattern::from_resolved_snippet(snippet.to_owned())
                    }
                    ResolvedSnippet::Binding(b) => ResolvedPattern::from_binding(b.to_owned()),
                    ResolvedSnippet::LazyFn(l) => match &*l {
                        LazyBuiltIn::Join(j) => {
                            ResolvedPattern::from_list_parts(j.list.iter().cloned())
                        }
                    },
                };
                if self.execute(&resolved, &mut cur_state, context, logs)? {
                    did_match = true;
                } else {
                    cur_state = state;
                }
            }
            if !did_match {
                return Ok(false);
            }
            *init_state = cur_state;
            Ok(true)
        } else {
            return Ok(false);
        }
    }
}
