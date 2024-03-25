use std::collections::BTreeMap;

use crate::context::Context;

use super::compiler::CompilationContext;
use super::patterns::Name;
use super::resolved_pattern::ResolvedPattern;
use super::variable::{get_variables, register_variable, VariableSourceLocations};
use super::{patterns::Matcher, patterns::Pattern, PatternDefinition, State};
use anyhow::{anyhow, bail, Result};
use itertools::Itertools;
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::position::Range;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Bubble {
    pub(crate) pattern_def: PatternDefinition,
    pub(crate) args: Vec<Option<Pattern>>,
}

impl Bubble {
    pub fn new(pattern_def: PatternDefinition, args: Vec<Pattern>) -> Self {
        Self {
            pattern_def,
            args: args.into_iter().map(Some).collect(),
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

        Ok(Pattern::Bubble(Box::new(Self::new(pattern_def, args))))
    }
}

impl Name for Bubble {
    fn name(&self) -> &'static str {
        "BUBBLE"
    }
}

impl Matcher for Bubble {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.pattern_def
            .call(state, binding, context, logs, &self.args)
    }
}
