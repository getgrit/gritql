use super::{Matcher, PatternName, State};
use crate::context::QueryContext;
use grit_util::{error::GritResult, AnalysisLogs};
use std::{fmt::Debug, rc::Rc, sync::Arc};

pub trait CallbackPatternFn<Q: QueryContext>:
    for<'a> Fn(
        &<Q as QueryContext>::ResolvedPattern<'a>,
        &'a Q::ExecContext<'a>,
        &mut State<'a, Q>,
        &mut AnalysisLogs,
    ) -> GritResult<bool>
    + Send
    + Sync
    + 'static
{
}

pub struct CallbackPattern<Q: QueryContext> {
    closure: Arc<Box<dyn CallbackPatternFn<Q>>>,
}

impl<Q: QueryContext> Clone for CallbackPattern<Q> {
    fn clone(&self) -> Self {
        Self {
            closure: self.closure.clone(),
        }
    }
}

impl<Q: QueryContext> CallbackPattern<Q> {
    pub fn new(callback: impl CallbackPatternFn<Q>) -> Self {
        Self {
            closure: Arc::new(Box::new(callback)),
        }
    }
}

impl<Q: QueryContext> PatternName for CallbackPattern<Q> {
    fn name(&self) -> &'static str {
        "CALLBACK_PATTERN"
    }
}

impl<Q: QueryContext> Debug for CallbackPattern<Q> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "CallbackPattern")
    }
}

impl<Q: QueryContext> Matcher<Q> for CallbackPattern<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<bool> {
        // (self.callback)(state, binding, context, logs)
        todo!("Not implemented")
    }
}
