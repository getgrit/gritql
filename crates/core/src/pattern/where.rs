use super::{
    compiler::CompilationContext,
    container::Container,
    functions::Evaluator,
    patterns::{Matcher, Name, Pattern},
    predicates::Predicate,
    r#match::Match,
    resolved_pattern::ResolvedPattern,
    variable::Variable,
    variable::VariableSourceLocations,
    State,
};
use crate::{context::Context, split_snippet::split_snippet};
use anyhow::{anyhow, Result};
use core::fmt::Debug;
use grit_util::{traverse, Order};
use marzano_language::language::Language;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::position::Range;
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct Where {
    pub(crate) pattern: Pattern,
    pub(crate) side_condition: Predicate,
}

impl Where {
    pub fn new(pattern: Pattern, side_condition: Predicate) -> Self {
        Self {
            pattern,
            side_condition,
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
    ) -> Result<Self> {
        let pattern = node
            .child_by_field_name("pattern")
            .ok_or_else(|| anyhow!("missing pattern of patternWhere"))?;
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
        let side_condition = node
            .child_by_field_name("side_condition")
            .ok_or_else(|| anyhow!("missing side condition of patternWhere"))?;
        let side_condition = Predicate::from_node(
            &side_condition,
            context,
            vars,
            vars_array,
            scope_index,
            global_vars,
            logs,
        )?;
        Ok(Self::new(pattern, side_condition))
    }

    // todo make as it's own pattern
    pub(crate) fn as_from_node(
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
            .ok_or_else(|| anyhow!("missing pattern of patternWhere"))?;

        let variable = node
            .child_by_field_name("variable")
            .ok_or_else(|| anyhow!("missing variable of patternWhere"))?;

        let name = variable.utf8_text(context.src.as_bytes())?;
        let name = name.trim();

        // this just searches the subtree for a variables that share the name.
        // could possible lead to some false positives, but more precise solutions
        // require much greater changes.
        if pattern_repeated_variable(&pattern, name, context.src, context.lang)? {
            let range: Range = node.range().into();
            let log = AnalysisLogBuilder::default()
                .level(441_u16)
                .file(context.file)
                .source(context.src)
                .position(range.start)
                .range(range)
                .message(format!(
                    "Warning: it is usually incorrect to redefine a variable {name} using as"
                ))
                .build()?;
            logs.push(log);
        }

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

        let variable = Variable::from_node(
            &variable,
            context.file,
            context.src,
            vars,
            global_vars,
            vars_array,
            scope_index,
        )?;
        Ok(Self::new(
            Pattern::Variable(variable),
            Predicate::Match(Box::new(Match::new(
                Container::Variable(variable),
                Some(pattern),
            ))),
        ))
    }
}

impl Name for Where {
    fn name(&self) -> &'static str {
        "WHERE"
    }
}

impl Matcher for Where {
    // order here is pattern then side condition, do we prefer side condition then pattern?
    // should the state be reset on failure?
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        init_state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut cur_state = init_state.clone();
        if !self
            .pattern
            .execute(binding, &mut cur_state, context, logs)?
        {
            return Ok(false);
        }
        if self
            .side_condition
            .execute_func(&mut cur_state, context, logs)?
            .predicator
        {
            *init_state = cur_state;
            Ok(true)
        } else {
            Ok(false)
        }
    }
}

fn is_variables_in_snippet(name: &str, snippet: &str, lang: &impl Language) -> bool {
    let variables = split_snippet(snippet, lang);
    variables.iter().any(|v| v.1 == name)
}

fn pattern_repeated_variable(
    pattern: &Node,
    name: &str,
    source: &str,
    lang: &impl Language,
) -> Result<bool> {
    let cursor = pattern.walk();
    let cursor = traverse(CursorWrapper::new(cursor, source), Order::Pre);
    Ok(cursor
        .filter(|n| n.node.kind() == "variable" || n.node.kind() == "codeSnippet")
        .map(|n| {
            let s = n.node.utf8_text(source.as_bytes())?.trim().to_string();
            if n.node.kind() == "variable" {
                Ok(s == name)
            } else {
                Ok(is_variables_in_snippet(name, &s, lang))
            }
        })
        .collect::<Result<Vec<bool>>>()?
        .into_iter()
        .any(|b| b))
}
