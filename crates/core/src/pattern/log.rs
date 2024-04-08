use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Name, Pattern},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::{get_file_name, Variable},
};
use crate::{binding::Binding, context::Context};
use anyhow::Result;
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};

#[derive(Debug, Clone)]
pub struct VariableInfo {
    name: String,
    variable: Variable,
}

impl VariableInfo {
    pub fn new(name: String, variable: Variable) -> Self {
        Self { name, variable }
    }
}

#[derive(Debug, Clone)]
pub struct Log {
    pub variable: Option<VariableInfo>,
    pub message: Option<Pattern>,
}

impl Log {
    pub fn new(variable: Option<VariableInfo>, message: Option<Pattern>) -> Self {
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
