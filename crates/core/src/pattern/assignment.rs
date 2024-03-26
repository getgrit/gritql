use super::{
    compiler::CompilationContext,
    container::Container,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    variable::{is_reserved_metavariable, VariableSourceLocations},
    State,
};
use crate::context::Context;
use anyhow::{anyhow, bail, Result};
use marzano_language::{language::GRIT_METAVARIABLE_PREFIX, target_language::TargetLanguage};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Assignment {
    container: Container,
    pub(crate) pattern: Pattern,
}

impl Assignment {
    fn new(container: Container, pattern: Pattern) -> Self {
        Self { container, pattern }
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
            .ok_or_else(|| anyhow!("missing pattern of assignment"))?;
        let pattern = Pattern::from_node(
            &pattern,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            true,
            logs,
        )?;

        let container = node
            .child_by_field_name("container")
            .ok_or_else(|| anyhow!("missing container of assignment"))?;
        let var_text = container.utf8_text(context.src.as_bytes())?;
        if is_reserved_metavariable(&var_text, None::<&TargetLanguage>) {
            bail!("{} is a reserved metavariable name. For more information, check out the docs at https://docs.grit.io/language/patterns#metavariables.", var_text.trim_start_matches(GRIT_METAVARIABLE_PREFIX));
        }
        let variable = Container::from_node(
            &container,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        Ok(Self::new(variable, pattern))
    }
}

impl Name for Assignment {
    fn name(&self) -> &'static str {
        "assignment"
    }
}

impl Matcher for Assignment {
    fn execute<'a>(
        &'a self,
        _context_node: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let resolved = ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container.set_resolved(state, resolved)?;
        Ok(true)
    }
}

impl Evaluator for Assignment {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let resolved: ResolvedPattern<'_> =
            ResolvedPattern::from_pattern(&self.pattern, state, context, logs)?;
        self.container.set_resolved(state, resolved)?;
        Ok(FuncEvaluation {
            predicator: true,
            ret_val: None,
        })
    }
}
