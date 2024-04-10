use super::{
    functions::{Evaluator, FuncEvaluation},
    patterns::Pattern,
    predicates::Predicate,
    resolved_pattern::patterns_to_resolved,
    state::State,
    variable::Variable,
};
use crate::{binding::Constant, context::Context};
use anyhow::{bail, Result};
#[cfg(feature = "external_functions")]
use marzano_externals::function::ExternalFunction;
use marzano_language::foreign_language::ForeignLanguage;
use marzano_util::analysis_logs::AnalysisLogs;
use std::borrow::Cow;

pub(crate) trait FunctionDefinition {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        args: &'a [Option<Pattern>],
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation>;
}

#[derive(Clone, Debug)]
pub struct GritFunctionDefinition {
    pub name: String,
    pub scope: usize,
    pub params: Vec<(String, Variable)>,
    pub local_vars: Vec<usize>,
    pub function: Predicate,
}

impl GritFunctionDefinition {
    pub fn new(
        name: String,
        scope: usize,
        params: Vec<(String, Variable)>,
        local_vars: Vec<usize>,
        function: Predicate,
    ) -> Self {
        Self {
            name,
            scope,
            params,
            local_vars,
            function,
        }
    }
}

impl FunctionDefinition for GritFunctionDefinition {
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        args: &'a [Option<Pattern>],
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        state.reset_vars(self.scope, args);
        self.function.execute_func(state, context, logs)
    }
}

#[derive(Debug, Clone)]
pub struct ForeignFunctionDefinition {
    pub name: String,
    pub params: Vec<(String, Variable)>,
    pub language: ForeignLanguage,
    pub code: Vec<u8>,
}

impl ForeignFunctionDefinition {
    pub fn new(
        name: String,
        _scope: usize,
        params: Vec<(String, Variable)>,
        language: ForeignLanguage,
        code: &[u8],
    ) -> Self {
        Self {
            name,
            params,
            language,
            code: code.to_vec(),
        }
    }
}

impl FunctionDefinition for ForeignFunctionDefinition {
    #[cfg(not(feature = "external_functions_common"))]
    fn call<'a>(
        &'a self,
        _state: &mut State<'a>,
        _context: &'a impl Context,
        _args: &'a [Option<Pattern>],
        _logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        bail!("External functions are not enabled in your environment")
    }
    #[cfg(feature = "external_functions_common")]
    fn call<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        args: &'a [Option<Pattern>],
        logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation> {
        let param_names = self
            .params
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();

        let resolved = patterns_to_resolved(args, state, context, logs)?;
        let mut cow_resolved = Vec::with_capacity(resolved.len());

        for r in resolved.iter() {
            match r {
                Some(r) => match r.text(&state.files) {
                    Ok(t) => cow_resolved.push(t),
                    Err(e) => bail!("failed to get text from resolved pattern: {}", e),
                },
                None => bail!("Foreign function references unbound variable"),
            }
        }

        let resolved_str: Vec<&str> = cow_resolved.iter().map(Cow::as_ref).collect();

        // START Simple externalized version
        #[cfg(all(feature = "external_functions_ffi", target_arch = "wasm32"))]
        let result = context.exec_external(&self.code, param_names, &resolved_str)?;

        // END Simple externalized version

        // START embedded version
        // Really, we should compile ahead of time and then call the compiled function
        // But, the WebAssembly function model is currently *mutable* so state would be contaminated
        #[cfg(feature = "external_functions")]
        let mut function = ExternalFunction::new_js(&self.code, param_names)?;

        #[cfg(feature = "external_functions")]
        let result = function
            .call(&resolved_str)
            .or_else(|e| bail!("failed to call function {}: {}", self.name, e))?;
        // END embedded version

        let string = String::from_utf8(result).or_else(|_| {
            bail!(
                "function {} returned did not return a UTF-8 string",
                self.name
            )
        })?;

        Ok(FuncEvaluation {
            predicator: true,
            ret_val: Some(crate::pattern::resolved_pattern::ResolvedPattern::Constant(
                Constant::String(string),
            )),
        })
    }
}
