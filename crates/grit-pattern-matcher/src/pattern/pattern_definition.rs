use super::{
    patterns::{Matcher, Pattern},
    variable::Variable,
    State,
};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};
use std::sync::{Arc, OnceLock};

#[derive(Clone, Debug)]
pub enum PatternDefinitionInternal<Q: QueryContext> {
    Static {
        scope: usize,
        params: Vec<(String, Variable)>,
        pattern: Pattern<Q>,
    },
    Dynamic {
        params: Vec<(String, Variable)>,
        pattern: Pattern<Q>,
        scope: Arc<OnceLock<usize>>,
    },
}

#[derive(Clone, Debug)]
pub struct PatternDefinition<Q: QueryContext> {
    pub name: String,
    internal: PatternDefinitionInternal<Q>,
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
            internal: PatternDefinitionInternal::Static {
                scope,
                params,
                pattern,
            },
        }
    }

    pub fn new_dynamic(name: String, params: Vec<(String, Variable)>, pattern: Pattern<Q>) -> Self {
        Self {
            name,
            internal: PatternDefinitionInternal::Dynamic {
                params,
                pattern,
                scope: Arc::new(OnceLock::new()),
            },
        }
    }

    pub fn try_scope(&self) -> GritResult<usize> {
        match &self.internal {
            PatternDefinitionInternal::Static { scope, .. } => Ok(*scope),
            PatternDefinitionInternal::Dynamic { scope, .. } => {
                panic!("Dynamic pattern definition does not have a scope");
            }
        }
    }

    pub fn replace_pattern(&mut self, pattern: Pattern<Q>) {
        todo!("Not implemented")
        // match &mut self.internal {
        //     PatternDefinitionInternal::Static { pattern, .. } => *pattern = pattern,
        //     PatternDefinitionInternal::Dynamic { pattern, .. } => *pattern = pattern,
        // }
    }

    fn get_scope(&self, state: &mut State<'_, Q>) -> usize {
        match &self.internal {
            PatternDefinitionInternal::Static { scope, .. } => *scope,
            PatternDefinitionInternal::Dynamic { scope, .. } => {
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

        let pattern = match &self.internal {
            PatternDefinitionInternal::Static { pattern, .. } => pattern,
            PatternDefinitionInternal::Dynamic { pattern, .. } => pattern,
        };

        let res = pattern.execute(binding, state, context, logs);
        state.exit_scope(tracker);

        let fn_state = state.bindings[scope].pop_back().unwrap();
        let cur_fn_state = state.bindings[scope].back_mut().unwrap();
        for (cur, last) in cur_fn_state.iter_mut().zip(fn_state) {
            cur.value_history.extend(last.value_history)
        }
        res
    }

    pub fn params(&self) -> &Vec<(String, Variable)> {
        match &self.internal {
            PatternDefinitionInternal::Static { params, .. } => params,
            PatternDefinitionInternal::Dynamic { params, .. } => params,
        }
    }

    pub fn pattern(&self) -> &Pattern<Q> {
        match &self.internal {
            PatternDefinitionInternal::Static { pattern, .. } => pattern,
            PatternDefinitionInternal::Dynamic { pattern, .. } => pattern,
        }
    }
}
