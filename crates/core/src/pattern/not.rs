use super::{
    compiler::CompilationContext,
    functions::{Evaluator, FuncEvaluation},
    iter_pattern::PatternOrPredicate,
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    resolved_pattern::ResolvedPattern,
    variable::VariableSourceLocations,
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Ok, Result};
use core::fmt::Debug;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use marzano_util::position::Range;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Not {
    pub pattern: Pattern,
}
impl Not {
    pub fn new(pattern: Pattern) -> Self {
        Self { pattern }
    }

    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternNot"))?;
        let range: Range = pattern.range().into();
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            false,
            logs,
        )?;
        if pattern.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.file)
                .source(context.src)
                .position(range.start)
                .range(range)
                .message("Warning: rewrites inside of a not will never be applied")
                .build()?;
            logs.push(log);
        }
        Ok(Self::new(pattern))
    }
}

impl Name for Not {
    fn name(&self) -> &'static str {
        "NOT"
    }
}

impl Matcher for Not {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        Ok(!self
            .pattern
            .execute(binding, &mut state.clone(), context, logs)?)
    }
}

#[derive(Debug, Clone)]
pub struct PrNot {
    pub(crate) predicate: Predicate,
}

impl PrNot {
    pub fn new(predicate: Predicate) -> Self {
        Self { predicate }
    }
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let not = node
            .child_by_field_name("predicate")
            .ok_or_else(|| anyhow!("predicateNot missing predicate"))?;
        let range: Range = not.range().into();
        let not = Predicate::from_node(
            &not,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        if not.iter().any(|p| {
            matches!(
                p,
                PatternOrPredicate::Pattern(Pattern::Rewrite(_))
                    | PatternOrPredicate::Predicate(Predicate::Rewrite(_))
            )
        }) {
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.file)
                .source(context.src)
                .position(range.start)
                .range(range)
                .message("Warning: rewrites inside of a not will never be applied")
                .build()?;
            logs.push(log);
        }
        Ok(PrNot::new(not))
    }
}

impl Name for PrNot {
    fn name(&self) -> &'static str {
        "PREDICATE_NOT"
    }
}

impl Evaluator for PrNot {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let res = self
            .predicate
            .execute_func(&mut state.clone(), context, logs)?;
        if res.ret_val.is_some() {
            bail!("Cannot return from within not clause");
        }
        Ok(FuncEvaluation {
            predicator: !res.predicator,
            ret_val: None,
        })
    }
}
