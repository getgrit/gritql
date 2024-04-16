use super::{
    ast_node_pattern::AstNodePattern,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::context::QueryContext;
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;
use std::mem::transmute;

#[derive(Debug, Clone)]
pub struct Or<Q: QueryContext> {
    pub patterns: Vec<Pattern<Q>>,
}

impl<Q: QueryContext> Or<Q> {
    pub fn new(patterns: Vec<Pattern<Q>>) -> Self {
        Self { patterns }
    }
}

impl<Q: QueryContext> PatternName for Or<Q> {
    fn name(&self) -> &'static str {
        "OR"
    }
}

impl<Q: QueryContext> Matcher<Q> for Or<Q> {
    fn execute<'a>(
        &'a self,
        resolved: &ResolvedPattern<'a>,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Some(binding) = resolved.get_last_binding() {
            for p in self.patterns.iter() {
                // filter out pattern which cannot match because of a mismatched node type
                if let (Some(binding_node), Pattern::AstNode(node_pattern)) = (binding.as_node(), p)
                {
                    // Safety: This is safe as long as `MarzanoProblemContext` is the
                    // only implementation of `ProblemContext`.
                    if !node_pattern.matches_kind_of(unsafe { transmute(&binding_node) }) {
                        continue;
                    }
                }
                let mut state = init_state.clone();
                let res = p.execute(resolved, &mut state, context, logs)?;
                if res {
                    *init_state = state;
                    return Ok(true);
                }
            }
        } else {
            for p in self.patterns.iter() {
                let mut state = init_state.clone();
                let res = p.execute(resolved, &mut state, context, logs)?;
                if res {
                    *init_state = state;
                    return Ok(true);
                }
            }
        };
        Ok(false)
    }
}

#[derive(Debug, Clone)]
pub struct PrOr<Q: QueryContext> {
    pub predicates: Vec<Predicate<Q>>,
}

impl<Q: QueryContext> PrOr<Q> {
    pub fn new(predicates: Vec<Predicate<Q>>) -> Self {
        Self { predicates }
    }
}

impl<Q: QueryContext> PatternName for PrOr<Q> {
    fn name(&self) -> &'static str {
        "PREDICATE_OR"
    }
}

impl<Q: QueryContext> Evaluator<Q> for PrOr<Q> {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        for p in self.predicates.iter() {
            let mut state = init_state.clone();
            let res = p.execute_func(&mut state, context, logs)?;
            if res.predicator || res.ret_val.is_some() {
                *init_state = state;
                return Ok(res);
            }
        }
        Ok(FuncEvaluation {
            predicator: false,
            ret_val: None,
        })
    }
}
