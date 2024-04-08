use super::{
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

#[derive(Debug, Clone)]
pub struct Limit {
    pub(crate) pattern: Pattern,
    pub limit: usize,
    pub invocation_count: Arc<AtomicUsize>,
}

impl Limit {
    pub fn new(pattern: Pattern, limit: usize) -> Self {
        Self {
            pattern,
            limit,
            invocation_count: Arc::new(AtomicUsize::new(0)),
        }
    }
}

impl Name for Limit {
    fn name(&self) -> &'static str {
        "LIMIT"
    }
}

impl Matcher for Limit {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if context.ignore_limit_pattern() {
            let res = self.pattern.execute(binding, state, context, logs)?;
            return Ok(res);
        }
        if self.invocation_count.load(Ordering::Relaxed) >= self.limit {
            return Ok(false);
        }
        let res = self.pattern.execute(binding, state, context, logs)?;
        if !res {
            return Ok(false);
        }
        loop {
            let current_count = self.invocation_count.load(Ordering::SeqCst);
            if current_count >= self.limit {
                return Ok(false);
            }
            let attempt_increment = self.invocation_count.compare_exchange(
                current_count,
                current_count + 1,
                Ordering::SeqCst,
                Ordering::Relaxed,
            );
            if attempt_increment.is_ok() {
                break;
            }
        }
        Ok(true)
    }
}
