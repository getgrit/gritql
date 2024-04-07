use super::{
    compiler::CompilationContext, container_compiler::ContainerCompiler,
    node_compiler::NodeCompiler,
};
use crate::pattern::{
    assignment::Assignment,
    patterns::Pattern,
    variable::{is_reserved_metavariable, VariableSourceLocations},
};
use anyhow::{anyhow, bail, Result};
use marzano_language::{language::GRIT_METAVARIABLE_PREFIX, target_language::TargetLanguage};
use marzano_util::analysis_logs::AnalysisLogs;
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct AssignmentCompiler;

impl NodeCompiler for AssignmentCompiler {
    type TargetPattern = Assignment;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
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
        let variable = ContainerCompiler::from_node(
            &container,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        Ok(Assignment::new(variable, pattern))
    }
}
