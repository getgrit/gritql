use crate::{
    marzano_context::MarzanoContext, marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::MarzanoQueryContext,
};
use grit_pattern_matcher::{
    constant::Constant,
    context::ExecContext,
    pattern::{
        CallForeignFunction, FuncEvaluation, FunctionDefinition, GritCall, Pattern,
        ResolvedPattern, State, Variable,
    },
};
use grit_util::{
    error::{GritPatternError, GritResult},
    AnalysisLogs,
};
#[cfg(feature = "external_functions")]
use marzano_externals::function::ExternalFunction;
use marzano_language::foreign_language::ForeignLanguage;
use std::borrow::Cow;

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

impl FunctionDefinition<MarzanoQueryContext> for ForeignFunctionDefinition {
    #[cfg(not(feature = "external_functions_common"))]
    fn call<'a>(
        &'a self,
        _state: &mut State<'a, MarzanoQueryContext>,
        _context: &'a MarzanoContext<'a>,
        _args: &'a [Option<Pattern<MarzanoQueryContext>>],
        _logs: &mut AnalysisLogs,
    ) -> Result<FuncEvaluation<MarzanoQueryContext>> {
        return Err(GritPatternError::new(
            "External functions are not enabled in your environment",
        ));
    }

    #[cfg(feature = "external_functions_common")]
    fn call<'a>(
        &'a self,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        args: &'a [Option<Pattern<MarzanoQueryContext>>],
        logs: &mut AnalysisLogs,
    ) -> GritResult<FuncEvaluation<MarzanoQueryContext>> {
        let param_names = self
            .params
            .iter()
            .map(|(name, _)| name.clone())
            .collect::<Vec<_>>();

        let resolved = MarzanoResolvedPattern::from_patterns(args, state, context, logs)?;
        let mut cow_resolved = Vec::with_capacity(resolved.len());

        for r in resolved.iter() {
            match r {
                Some(r) => match r.text(&state.files, context.language()) {
                    Ok(t) => cow_resolved.push(t),
                    Err(e) => {
                        return Err(GritPatternError::new(format!(
                            "failed to get text from resolved pattern: {}",
                            e
                        )))
                    }
                },
                None => {
                    return Err(GritPatternError::new(
                        "Foreign function references unbound variable",
                    ))
                }
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
        let result = function.call(&resolved_str).or_else(|e| {
            return Err(GritPatternError::new(format!(
                "failed to call function {}: {}",
                self.name, e
            )));
        })?;
        // END embedded version

        let string = String::from_utf8(result).or_else(|_| {
            return Err(GritPatternError::new(format!(
                "function {} returned did not return a UTF-8 string",
                self.name,
            )));
        })?;

        Ok(FuncEvaluation {
            predicator: true,
            ret_val: Some(MarzanoResolvedPattern::from_constant(Constant::String(
                string,
            ))),
        })
    }
}

impl GritCall<MarzanoQueryContext> for CallForeignFunction<MarzanoQueryContext> {
    fn call<'a>(
        &'a self,
        state: &mut State<'a, MarzanoQueryContext>,
        context: &'a MarzanoContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> GritResult<MarzanoResolvedPattern<'a>> {
        let function_definition = &context.foreign_function_definitions()[self.index];

        match function_definition
            .call(state, context, &self.args, logs)?
            .ret_val
        {
            Some(pattern) => Ok(pattern),
            None => Err(GritPatternError::new(
                "Function call did not return a value",
            )),
        }
    }
}
