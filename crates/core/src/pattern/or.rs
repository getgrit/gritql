use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    State,
};
use crate::{binding::Binding, context::Context};
use anyhow::Result;
use core::fmt::Debug;
use marzano_util::analysis_logs::AnalysisLogs;

#[derive(Debug, Clone)]
pub struct Or {
    pub patterns: Vec<Pattern>,
}
impl Or {
    pub fn new(patterns: Vec<Pattern>) -> Self {
        Self { patterns }
    }
}

impl Name for Or {
    fn name(&self) -> &'static str {
        "OR"
    }
}

impl Matcher for Or {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let ResolvedPattern::Binding(binding_vector) = &binding {
            for p in self.patterns.iter() {
                // filter out pattern which cannot match because of a mismatched node type
                if let (Some(binding_node), Pattern::ASTNode(node_pattern)) =
                    (binding_vector.last().and_then(Binding::as_node), p)
                {
                    if node_pattern.sort != binding_node.node.kind_id() {
                        continue;
                    }
                }
                let mut state = init_state.clone();
                let res = p.execute(binding, &mut state, context, logs)?;
                if res {
                    *init_state = state;
                    return Ok(true);
                }
            }
        } else {
            for p in self.patterns.iter() {
                let mut state = init_state.clone();
                let res = p.execute(binding, &mut state, context, logs)?;
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
pub struct PrOr {
    pub predicates: Vec<Predicate>,
}
impl PrOr {
    pub fn new(predicates: Vec<Predicate>) -> Self {
        Self { predicates }
    }
}

impl Name for PrOr {
    fn name(&self) -> &'static str {
        "PREDICATE_OR"
    }
}

impl Evaluator for PrOr {
    fn execute_func<'a>(
        &'a self,
        init_state: &mut State<'a>,
        context: &'a impl Context,
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
