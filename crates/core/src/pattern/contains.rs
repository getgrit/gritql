use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::{LazyBuiltIn, ResolvedPattern, ResolvedSnippet},
    Node, State,
};
use crate::{context::ProblemContext, resolve};
use anyhow::Result;
use core::fmt::Debug;
use im::vector;
use marzano_util::{analysis_logs::AnalysisLogs, node_with_source::NodeWithSource};

#[derive(Debug, Clone)]
pub struct Contains<P: ProblemContext> {
    pub contains: Pattern<P>,
    pub until: Option<Pattern<P>>,
}

impl<P: ProblemContext> Contains<P> {
    pub fn new(contains: Pattern<P>, until: Option<Pattern<P>>) -> Self {
        Self { contains, until }
    }
}

impl<P: ProblemContext> PatternName for Contains<P> {
    fn name(&self) -> &'static str {
        "CONTAINS"
    }
}

fn execute_until<'a, P: ProblemContext>(
    init_state: &mut State<'a, P>,
    node: &Node<'a>,
    src: &'a str,
    context: &'a P::ExecContext<'a>,
    logs: &mut AnalysisLogs,
    the_contained: &'a Pattern<P>,
    until: &'a Option<Pattern<P>>,
) -> Result<bool, anyhow::Error> {
    let mut did_match = false;
    let mut cur_state = init_state.clone();
    let mut cursor = node.walk();
    let mut still_computing = true;
    while still_computing {
        let node = cursor.node();
        let node_lhs = ResolvedPattern::from_node(NodeWithSource::new(node, src));

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
impl<P: ProblemContext> Matcher<P> for Contains<P> {
    fn execute<'a>(
        &'a self,
        resolved_pattern: &ResolvedPattern<'a>,
        init_state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        match resolved_pattern {
            ResolvedPattern::Binding(bindings) => {
                let binding = resolve!(bindings.last());
                if let Some(node) = binding.as_node() {
                    execute_until(
                        init_state,
                        &node.node,
                        node.source,
                        context,
                        logs,
                        &self.contains,
                        &self.until,
                    )
                } else if let Some(list_items) = binding.list_items() {
                    let mut did_match = false;
                    let mut cur_state = init_state.clone();
                    for item in list_items {
                        let state = cur_state.clone();
                        if self.execute(
                            &ResolvedPattern::from_node(item),
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
                    }
                    Ok(did_match)
                } else if let Some(_c) = binding.as_constant() {
                    // this seems like an infinite loop, todo return false?
                    self.contains
                        .execute(resolved_pattern, init_state, context, logs)
                } else {
                    Ok(false)
                }
            }
            ResolvedPattern::List(elements) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for element in elements {
                    let state = cur_state.clone();
                    if self.execute(element, &mut cur_state, context, logs)? {
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
            }
            ResolvedPattern::File(file) => {
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
                if self.contains.execute(
                    &file.name(&cur_state.files),
                    &mut cur_state,
                    context,
                    logs,
                )? {
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
            }
            ResolvedPattern::Files(files) => {
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
            }
            ResolvedPattern::Snippets(snippets) => {
                let mut cur_state = init_state.clone();
                let mut did_match = false;
                for snippet in snippets {
                    let state = cur_state.clone();
                    let resolved = match snippet {
                        ResolvedSnippet::Text(_) => {
                            ResolvedPattern::Snippets(vector![snippet.to_owned()])
                        }
                        ResolvedSnippet::Binding(b) => {
                            ResolvedPattern::Binding(vector![b.to_owned()])
                        }
                        ResolvedSnippet::LazyFn(l) => match &**l {
                            LazyBuiltIn::Join(j) => ResolvedPattern::List(j.list.clone()),
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
            }
            ResolvedPattern::Map(_) | ResolvedPattern::Constant(_) => Ok(false),
        }
    }
}
