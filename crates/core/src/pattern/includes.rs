use super::{
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::QueryContext;
use anyhow::{Context as _, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Includes<Q: QueryContext> {
    pub(crate) includes: Pattern<Q>,
}

impl<Q: QueryContext> Includes<Q> {
    pub fn new(includes: Pattern<Q>) -> Self {
        Self { includes }
    }
}

impl<Q: QueryContext> PatternName for Includes<Q> {
    fn name(&self) -> &'static str {
        "INCLUDES"
    }
}

fn execute<'a, Q: QueryContext>(
    pattern: &'a Pattern<Q>,
    binding: &ResolvedPattern<'a, Q>,
    state: &mut State<'a, Q>,
    context: &'a Q::ExecContext<'a>,
    logs: &mut AnalysisLogs,
) -> Result<bool> {
    match &pattern {
        Pattern::Regex(pattern) => pattern.execute_matching(binding, state, context, logs, false),
        Pattern::Or(pattern) => {
            for p in pattern.patterns.iter() {
                if execute(p, binding, state, context, logs)? {
                    return Ok(true);
                }
            }
            Ok(false)
        }
        Pattern::Any(pattern) => {
            // Any is subtly different from or in that it will not short-circuit so we *must* execute all patterns
            let mut any_matched = false;
            for p in pattern.patterns.iter() {
                if execute(p, binding, state, context, logs)? {
                    any_matched = true;
                }
            }
            Ok(any_matched)
        }
        Pattern::And(pattern) => {
            for p in pattern.patterns.iter() {
                if !execute(p, binding, state, context, logs)? {
                    return Ok(false);
                }
            }
            Ok(true)
        }
        Pattern::AstNode(_)
        | Pattern::List(_)
        | Pattern::ListIndex(_)
        | Pattern::Map(_)
        | Pattern::Accessor(_)
        | Pattern::Call(_)
        | Pattern::File(_)
        | Pattern::Files(_)
        | Pattern::Bubble(_)
        | Pattern::Limit(_)
        | Pattern::CallBuiltIn(_)
        | Pattern::CallFunction(_)
        | Pattern::CallForeignFunction(_)
        | Pattern::Assignment(_)
        | Pattern::Accumulate(_)
        | Pattern::Maybe(_)
        | Pattern::Not(_)
        | Pattern::If(_)
        | Pattern::Undefined
        | Pattern::Top
        | Pattern::Bottom
        | Pattern::Underscore
        | Pattern::StringConstant(_)
        | Pattern::AstLeafNode(_)
        | Pattern::IntConstant(_)
        | Pattern::FloatConstant(_)
        | Pattern::BooleanConstant(_)
        | Pattern::Dynamic(_)
        | Pattern::CodeSnippet(_)
        | Pattern::Variable(_)
        | Pattern::Rewrite(_)
        | Pattern::Log(_)
        | Pattern::Range(_)
        | Pattern::Contains(_)
        | Pattern::Includes(_)
        | Pattern::Within(_)
        | Pattern::After(_)
        | Pattern::Before(_)
        | Pattern::Where(_)
        | Pattern::Some(_)
        | Pattern::Every(_)
        | Pattern::Add(_)
        | Pattern::Subtract(_)
        | Pattern::Multiply(_)
        | Pattern::Divide(_)
        | Pattern::Modulo(_)
        | Pattern::Dots
        | Pattern::Sequential(_)
        | Pattern::Like(_) => {
            let resolved = ResolvedPattern::from_pattern(pattern, state, context, logs)
                .context("includes can only be used with patterns that can be resolved")?;
            let substring = resolved.text(&state.files).context(
                "includes can only be used with patterns that can be resolved to a string",
            )?;
            let string = binding.text(&state.files).context(
                "includes can only be used with patterns that can be resolved to a string",
            )?;
            if string.contains(&*substring) {
                Ok(true)
            } else {
                Ok(false)
            }
        }
    }
}

// Includes and within should call the same function taking an iterator as an argument
// even better two arguments an accumulator and an iterator.
impl<Q: QueryContext> Matcher<Q> for Includes<Q> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a, Q>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        execute(&self.includes, binding, state, context, logs)
    }
}
