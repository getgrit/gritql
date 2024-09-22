use super::{
    patterns::{Matcher, Pattern},
    variable::Variable,
    State,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};
use std::sync::{Arc, OnceLock};

#[derive(Clone, Debug)]
pub enum PatternDefinitionInternal {
    Static { scope: usize },
    Dynamic { scope: Arc<OnceLock<usize>> },
}

#[derive(Clone, Debug)]
pub struct PatternDefinition<Q: QueryContext> {
    pub name: String,
    pattern: Pattern<Q>,
    params: Vec<(String, Variable)>,
    internal: PatternDefinitionInternal,
}

impl<Q: QueryContext> PatternDefinition<Q> {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        pattern: Pattern<Q>,
    ) -> Self {
        Self {
            name,
            pattern,
            params,
            internal: PatternDefinitionInternal::Static { scope },
        }
    }

    pub fn new_dynamic(name: String, params: Vec<(String, Variable)>, pattern: Pattern<Q>) -> Self {
        Self {
            name,
            pattern,
            params,
            internal: PatternDefinitionInternal::Dynamic {
                scope: Arc::new(OnceLock::new()),
            },
        }
    }

    pub fn try_scope(&self) -> GritResult<usize> {
        match &self.internal {
            PatternDefinitionInternal::Static { scope } => Ok(*scope),
            PatternDefinitionInternal::Dynamic { .. } => {
                panic!("Dynamic pattern definition does not have a scope");
            }
        }
    }

    pub fn replace_pattern(&mut self, new_pattern: Pattern<Q>) {
        self.pattern = new_pattern;
    }

    fn get_scope(&self, state: &mut State<'_, Q>) -> usize {
        match &self.internal {
            PatternDefinitionInternal::Static { scope } => *scope,
            PatternDefinitionInternal::Dynamic { scope } => {
                *scope.get_or_init(|| state.register_pattern_definition(&self.name))
            }
        }
    }

    pub(crate) fn call<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        binding: &Q::ResolvedPattern<'a>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
        args: &'a [Option<Pattern<Q>>],
    ) -> GritResult<bool> {
        let scope = self.get_scope(state);
        let tracker = state.enter_scope(scope, args);

        let res = self.pattern.execute(binding, state, context, logs);
        state.exit_scope(tracker);

        let fn_state = state.bindings[scope].pop_back().unwrap();
        let cur_fn_state = state.bindings[scope].back_mut().unwrap();
        for (cur, last) in cur_fn_state.iter_mut().zip(fn_state) {
            cur.value_history.extend(last.value_history)
        }
        res
    }

    pub fn params(&self) -> &Vec<(String, Variable)> {
        &self.params
    }

    pub fn pattern(&self) -> &Pattern<Q> {
        &self.pattern
    }
}
