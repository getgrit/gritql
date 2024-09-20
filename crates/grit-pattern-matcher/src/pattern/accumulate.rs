use super::{
    dynamic_snippet::DynamicPattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    PatternOrResolved, State,
};
use crate::{
    context::{ExecContext, QueryContext},
    effects::insert_effect,
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};

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
        if let Pattern::Variable(_) = &self.left {
            let left = PatternOrResolved::Pattern(&self.left);
            let right = ResolvedPattern::from_pattern(&self.right, state, context, logs)?;
            insert_effect(&left, right, state, context)
        } else {
            if !self.left.execute(context_node, state, context, logs)? {
                return Ok(false);
            };
            let resolved = context_node;
            let left = PatternOrResolved::Resolved(resolved);

            let right = if let Some(dynamic_right) = &self.dynamic_right {
                ResolvedPattern::from_dynamic_pattern(dynamic_right, state, context, logs)?
            } else {
                ResolvedPattern::from_pattern(&self.right, state, context, logs)?
            };

            insert_effect(&left, right, state, context)
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
            let scope = var.try_scope().unwrap();
            let index = var.try_scope().unwrap();
            if let Some(base) = state.bindings[scope.into()].back_mut().unwrap()[index.into()]
                .value
                .as_mut()
            {
                base.extend(append, &mut state.effects, context.language())?;
                Ok(FuncEvaluation {
                    predicator: true,
                    ret_val: None,
                })
            } else {
                Err(GritPatternError::new(format!(
                    "Variable {} is not bound",
                    state.bindings[var.scope(state).into()].last().unwrap()
                        [var.index(state).into()]
                    .name
                )))
            }
        } else {
            Err(GritPatternError::new(
                "Insert side-conditions must have variable on left-hand side",
            ))
        }
    }
}
