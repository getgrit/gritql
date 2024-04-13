use super::{
    dynamic_snippet::DynamicPattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    variable_content::VariableContent,
    State,
};
use crate::context::ProblemContext;
use crate::problem::{Effect, EffectKind};
use anyhow::{bail, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Rewrite<P: ProblemContext> {
    pub left: Pattern<P>,
    pub right: DynamicPattern<P>,
    pub(crate) _annotation: Option<String>,
}

impl<P: ProblemContext> Rewrite<P> {
    pub fn new(left: Pattern<P>, right: DynamicPattern<P>, _annotation: Option<String>) -> Self {
        Self {
            left,
            right,
            _annotation,
        }
    }

    /**
     * Execute a rewrite rule, returning the new binding.
     *
     * If called from a rewrite side-condition, the binding should be None.
     * In this case, the left-hand side must be a variable, and the binding
     * will be taken from the current state.
     *
     * If called from a rewrite pattern, the binding should be Some(the current node).
     */
    pub(crate) fn execute_generalized<'a>(
        &'a self,
        resolved: Option<&ResolvedPattern<'a>>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = match resolved {
            Some(b) => {
                if !self.left.execute(b, state, context, logs)? {
                    return Ok(false);
                } else {
                    Cow::Borrowed(b)
                }
            }
            None => {
                if let Pattern::Variable(v) = &self.left {
                    let var = state.trace_var(v);
                    if let Some(VariableContent {
                        value: Some(content),
                        ..
                    }) = state
                        .bindings
                        .get(var.scope)
                        .and_then(|scope| scope.last().unwrap().get(var.index))
                        .cloned()
                        .map(|b| *b)
                    {
                        Cow::Owned(content)
                    } else {
                        bail!("Variable {:?} not bound", v);
                    }
                } else {
                    bail!("Rewrite side-conditions must have variable on left-hand side");
                }
            }
        };
        let bindings = match resolved.as_ref() {
            ResolvedPattern::Binding(b) => b,
            ResolvedPattern::Constant(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a constant")
            }
            ResolvedPattern::File(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a file, try rewriting the content, or name instead")
            }
            ResolvedPattern::Files(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a files node")
            }
            ResolvedPattern::List(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a list pattern")
            }
            ResolvedPattern::Map(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to a map pattern")
            }
            ResolvedPattern::Snippets(_) => {
                bail!("variable on left hand side of rewrite side-conditions cannot be bound to snippets")
            }
        };
        let replacement: ResolvedPattern<'_> =
            ResolvedPattern::from_dynamic_pattern(&self.right, state, context, logs)?;
        let effects = bindings.iter().map(|b| Effect {
            binding: b.clone(),
            pattern: replacement.clone(),
            kind: EffectKind::Rewrite,
        });
        state.effects.extend(effects);
        Ok(true)
    }
}

impl<P: ProblemContext> PatternName for Rewrite<P> {
    fn name(&self) -> &'static str {
        "REWRITE"
    }
}

impl<P: ProblemContext> Matcher<P> for Rewrite<P> {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.execute_generalized(Some(binding), state, context, logs)
    }
}

impl<P: ProblemContext> Evaluator<P> for Rewrite<P> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, P>,
        context: &'a P::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let predicator = self.execute_generalized(None, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator,
            ret_val: None,
        })
    }
}
