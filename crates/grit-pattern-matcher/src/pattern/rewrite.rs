use super::{
    dynamic_snippet::DynamicPattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    variable_content::VariableContent,
    State,
};
use crate::{context::QueryContext, effects::Effect};
use core::fmt::Debug;
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs, EffectKind,
};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Rewrite<Q: QueryContext> {
    pub left: Pattern<Q>,
    pub right: DynamicPattern<Q>,
    pub annotation: Option<String>,
}

impl<Q: QueryContext> Rewrite<Q> {
    pub fn new(left: Pattern<Q>, right: DynamicPattern<Q>, annotation: Option<String>) -> Self {
        Self {
            left,
            right,
            annotation,
        }
    }

    pub fn new_pattern(
        left: Pattern<Q>,
        right: DynamicPattern<Q>,
        annotation: Option<String>,
    ) -> Pattern<Q> {
        Pattern::Rewrite(Box::new(Rewrite {
            left,
            right,
            annotation,
        }))
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
        resolved: Option<&Q::ResolvedPattern<'a>>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
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
                    let var = state.trace_var_mut(v);
                    if let Some(VariableContent {
                        value: Some(content),
                        ..
                    }) = state
                        .bindings
                        .get(var.try_scope().unwrap().into())
                        .and_then(|scope| {
                            scope.last().unwrap().get(var.try_index().unwrap().into())
                        })
                        .cloned()
                        .map(|b| *b)
                    {
                        Cow::Owned(content)
                    } else {
                        return Err(GritPatternError::new(format!("Variable {:?} not bound", v)));
                    }
                } else {
                    return Err(GritPatternError::new(
                        "Rewrite side-conditions must have variable on left-hand side",
                    ));
                }
            }
        };
        let Some(bindings) = resolved.get_bindings() else {
            return Err(
                GritPatternError::new(

            "variable on left hand side of rewrite side-conditions can only be bound to bindings"
                )
            );
        };
        let replacement: Q::ResolvedPattern<'_> =
            ResolvedPattern::from_dynamic_pattern(&self.right, state, context, logs)?;
        let effects = bindings.map(|b| Effect {
            binding: b.clone(),
            pattern: replacement.clone(),
            kind: EffectKind::Rewrite,
        });
        state.effects.extend(effects);
        Ok(true)
    }
}

impl<Q: QueryContext> PatternName for Rewrite<Q> {
    fn name(&self) -> &'static str {
        "REWRITE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Rewrite<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        self.execute_generalized(Some(binding), state, context, logs)
    }
}

impl<Q: QueryContext> Evaluator<Q> for Rewrite<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        let predicator = self.execute_generalized(None, state, context, logs)?;
        Ok(FuncEvaluation {
            predicator,
            ret_val: None,
        })
    }
}
