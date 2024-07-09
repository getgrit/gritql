use super::{
    dynamic_snippet::DynamicPattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::errors::{GritPatternError, GritResult};
use crate::{context::ExecContext, context::QueryContext, effects::Effect};
use grit_util::{AnalysisLogs, EffectKind};
use std::borrow::Cow;

#[derive(Debug, Clone)]
pub struct Accumulate<Q: QueryContext> {
    pub(crate) left: Pattern<Q>,
    pub(crate) right: Pattern<Q>,
    dynamic_right: Option<DynamicPattern<Q>>,
}

impl<Q: QueryContext> Accumulate<Q> {
    pub fn new(
        left: Pattern<Q>,
        right: Pattern<Q>,
        dynamic_right: Option<DynamicPattern<Q>>,
    ) -> Self {
        Self {
            left,
            right,
            dynamic_right,
        }
    }
}

impl<Q: QueryContext> PatternName for Accumulate<Q> {
    fn name(&self) -> &'static str {
        "ACCUMULATE"
    }
}

impl<Q: QueryContext> Matcher<Q> for Accumulate<Q> {
    fn execute<'a>(
        &'a self,
        context_node: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        if let Pattern::Variable(var) = &self.left {
            let var = state.trace_var(var);
            let append = ResolvedPattern::from_pattern(&self.right, state, context, logs)?;
            if let Some(base) = state.bindings[var.scope].back_mut().unwrap()[var.index]
                .value
                .as_mut()
            {
                base.extend(append, &mut state.effects, context.language())?;
                Ok(true)
            } else {
                return Err(GritPatternError::new(format!(
                    "Variable {} is not bound",
                    state.bindings[var.scope].last().unwrap()[var.index].name
                )));
            }
        } else {
            let resolved = if !self.left.execute(context_node, state, context, logs)? {
                return Ok(false);
            } else {
                Cow::Borrowed(context_node)
            };
            let Some(bindings) = resolved.get_bindings() else {
                return Err(
                    GritPatternError::new("variable on left hand side of insert side-conditions can only be bound to bindings")
                );
            };
            let dynamic_right = match &self.dynamic_right {
                Some(r) => r,
                None => {
                    return Err(
                        GritPatternError::new(format!(
                        "Insert right hand side must be a code snippet when LHS is not a variable, but found: {:?}", self.right
                    )))
                }
            };
            let mut replacement: Q::ResolvedPattern<'a> =
                ResolvedPattern::from_dynamic_pattern(dynamic_right, state, context, logs)?;
            let effects: GritResult<Vec<_>> = bindings
                .map(|binding| {
                    let is_first = !state.effects.iter().any(|e| e.binding == binding);
                    replacement.normalize_insert(&binding, is_first, context.language())?;
                    Ok(Effect {
                        binding,
                        pattern: replacement.clone(),
                        kind: EffectKind::Insert,
                    })
                })
                .collect();
            let effects = effects?;
            state.effects.extend(effects);
            Ok(true)
        }
    }
}

impl<Q: QueryContext> Evaluator<Q> for Accumulate<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<Q>> {
        if let Pattern::Variable(var) = &self.left {
            let var = state.trace_var(var);
            let append = ResolvedPattern::from_pattern(&self.right, state, context, logs)?;
            if let Some(base) = state.bindings[var.scope].back_mut().unwrap()[var.index]
                .value
                .as_mut()
            {
                base.extend(append, &mut state.effects, context.language())?;
                Ok(FuncEvaluation {
                    predicator: true,
                    ret_val: None,
                })
            } else {
                return Err(GritPatternError::new(format!(
                    "Variable {} is not bound",
                    state.bindings[var.scope].last().unwrap()[var.index].name
                )));
            }
        } else {
            return Err(GritPatternError::new(
                "Insert side-conditions must have variable on left-hand side",
            ));
        }
    }
}
