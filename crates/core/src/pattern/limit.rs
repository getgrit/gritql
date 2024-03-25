use super::{
    compiler::CompilationContext,
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::VariableSourceLocations,
};
use crate::context::Context;
use anyhow::{anyhow, Result};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use tree_sitter::Node;

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

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Pattern> {
        let body = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern in limit"))?;
        let body = Pattern::from_node(
            &body,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        let limit = node
            .child_by_field_name("limit")
            .ok_or_else(|| anyhow!("missing limit in limit"))?;
        let limit = limit
            .utf8_text(context.src.as_bytes())?
            .trim()
            .parse::<usize>()?;
        Ok(Pattern::Limit(Box::new(Self::new(body, limit))))
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
