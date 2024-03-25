use super::{
    compiler::CompilationContext,
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::{get_file_name, Variable, VariableSourceLocations},
};
use crate::{binding::Binding, context::Context, pattern::patterns::Pattern};
use anyhow::Result;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};
use std::collections::BTreeMap;
use tree_sitter::Node;

#[derive(Debug, Clone)]
struct VariableInfo {
    name: String,
    variable: Variable,
}

#[derive(Debug, Clone)]
pub struct Log {
    variable: Option<VariableInfo>,
    pub(crate) message: Option<Pattern>,
}

impl Log {
    fn new(variable: Option<VariableInfo>, message: Option<Pattern>) -> Self {
        Self { variable, message }
    }

    fn add_log<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut message = String::new();
        if let Some(user_message) = &self.message {
            let resolved = ResolvedPattern::from_pattern(user_message, state, context, logs)?;
            let text = resolved.text(&state.files)?;
            message.push_str(&format!("{}\n", text));
        }
        let mut log_builder = AnalysisLogBuilder::default();
        let file = get_file_name(state)?;
        #[allow(clippy::unnecessary_cast)]
        log_builder.level(441 as u16).file(file);

        if let Some(var) = &self.variable {
            let name = var.name.to_string();
            let var = state.trace_var(&var.variable);
            if self.message.is_none() {
                message.push_str(&format!("Logging {}\n", name));
            }
            let var_content = &state.bindings[var.scope].last().unwrap()[var.index];
            let value = var_content.value.as_ref();
            let src = value
                .map(|v| v.text(&state.files).map(|s| s.to_string()))
                .unwrap_or(Ok("Variable has no source".to_string()))?;
            log_builder.source(src);
            let node: Option<&Binding> = value.and_then(|v| v.get_binding());
            // todo add support for other types of bindings
            if let Some(node) = node {
                if let Some(range) = node.position() {
                    log_builder.range(range);
                }
                if let Some(syntax_tree) = node.get_sexp() {
                    log_builder.syntax_tree(syntax_tree);
                }
            } else {
                message.push_str("attempted to log a non-node binding, such bindings don't have syntax tree or range\n")
            }
        }
        log_builder.message(message);
        logs.push(log_builder.build()?);
        Ok(true)
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
        let message = node.child_by_field_name("message");
        let message = if let Some(message) = message {
            Some(Pattern::from_node(
                &message,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                false,
                logs,
            )?)
        } else {
            None
        };
        let variable_node = node.child_by_field_name("variable");
        let variable = variable_node
            .map(|n| {
                let name = n.utf8_text(context.src.as_bytes()).unwrap().to_string();
                let variable = Variable::from_node(
                    &n,
                    context.file,
                    context.src,
                    vars,
                    global_vars,
                    vars_array,
                    scope_index,
                )?;
                Ok(VariableInfo { name, variable })
            })
            .map_or(Ok(None), |v: Result<VariableInfo>| v.map(Some))?;

        Ok(Self::new(variable, message))
    }
}

impl Matcher for Log {
    fn execute<'a>(
        &'a self,
        _binding: &super::resolved_pattern::ResolvedPattern<'a>,
        state: &mut super::state::State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.add_log(state, context, logs)
    }
}

impl Evaluator for Log {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let predicator = self.add_log(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator,
            ret_val: None,
        })
    }
}

impl Name for Log {
    fn name(&self) -> &'static str {
        "LOG"
    }
}
