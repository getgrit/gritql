use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::{Matcher, Pattern, PatternName},
    resolved_pattern::ResolvedPattern,
    state::State,
    variable::{get_file_name, Variable},
};
use crate::{
    binding::Binding,
    context::{ExecContext, QueryContext},
};
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
pub struct Log<Q: QueryContext> {
    pub variable: Option<VariableInfo>,
    pub message: Option<Pattern<Q>>,
}

impl<Q: QueryContext> Log<Q> {
    pub fn new(variable: Option<VariableInfo>, message: Option<Pattern<Q>>) -> Self {
        Self { variable, message }
    }

    fn add_log<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut message = String::new();
        if let Some(user_message) = &self.message {
            let resolved = Q::ResolvedPattern::from_pattern(user_message, state, context, logs)?;
            let text = resolved.text(&state.files, context.language())?;
            message.push_str(&format!("{}\n", text));
        }
        let mut log_builder = AnalysisLogBuilder::default();
        let file = get_file_name(state, context.language())?;
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
                .map(|v| {
                    v.text(&state.files, context.language())
                        .map(|s| s.to_string())
                })
                .unwrap_or(Ok("Variable has no source".to_string()))?;
            log_builder.source(src);
            let node = value.and_then(|v| v.get_last_binding());
            // todo add support for other types of bindings
            if let Some(node) = node {
                if let Some(range) = node.position(context.language()) {
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

impl<Q: QueryContext> Matcher<Q> for Log<Q> {
    fn execute<'a>(
        &'a self,
        _binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        self.add_log(state, context, logs)
    }
}

impl<Q: QueryContext> Evaluator<Q> for Log<Q> {
    fn execute_func<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<Q>> {
        let predicator = self.add_log(state, context, logs)?;
        Ok(FuncEvaluation {
            predicator,
            ret_val: None,
        })
    }
}

impl<Q: QueryContext> PatternName for Log<Q> {
    fn name(&self) -> &'static str {
        "LOG"
    }
}
