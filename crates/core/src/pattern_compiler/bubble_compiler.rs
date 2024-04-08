use super::{compiler::CompilationContext, node_compiler::NodeCompiler};
use crate::pattern::{
    bubble::Bubble,
    pattern_definition::PatternDefinition,
    patterns::Pattern,
    variable::{get_variables, register_variable, VariableSourceLocations},
};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use marzano_util::{analysis_logs::AnalysisLogs, position::Range};
use std::collections::BTreeMap;
use tree_sitter::Node;

pub(crate) struct BubbleCompiler;

impl NodeCompiler for BubbleCompiler {
    type TargetPattern = Bubble;

    fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        logs: &mut AnalysisLogs,
    ) -> Result<Self::TargetPattern> {
        let local_scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut local_vars = BTreeMap::new();
        // important that this occurs first, as calls assume
        // that parameters are registered first

        let parameters = node
            .children_by_field_name("variables", &mut node.walk())
            .filter(|n| n.is_named())
            .map(|n| {
                Ok((
                    n.utf8_text(context.src.as_bytes())?.trim().to_string(),
                    n.range().into(),
                ))
            })
            .collect::<Result<Vec<(String, Range)>>>()?;
        if parameters.iter().unique_by(|n| n.0.clone()).count() != parameters.len() {
            bail!("bubble parameters must be unique, but had a repeated name in its parameters.")
        }
        let params = get_variables(
            &parameters,
            context.file,
            vars_array,
            local_scope_index,
            &mut local_vars,
            global_vars,
        )?;

        let body = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing body of patternDefinition"))?;
        let body = Pattern::from_node(
            &body,
            context,
            &mut local_vars,
            vars_array,
            local_scope_index,
            global_vars,
            false,
            logs,
        )?;

        let args = parameters
            .iter()
            .map(|(name, range)| {
                let v = Pattern::Variable(register_variable(
                    name,
                    context.file,
                    *range,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                )?);
                Ok(v)
            })
            .collect::<Result<Vec<Pattern>>>()?;

        let pattern_def = PatternDefinition::new(
            "<bubble>".to_string(),
            local_scope_index,
            params,
            local_vars.values().cloned().collect(),
            body,
        );

        Ok(Bubble::new(pattern_def, args))
    }
}
