pub mod accessor;
pub mod accumulate;
pub mod add;
pub mod after;
pub mod and;
pub mod any;
pub mod assignment;
pub mod ast_node_pattern;
pub mod before;
pub mod boolean_constant;
pub mod bubble;
pub mod built_in_functions;
pub mod call;
pub mod code_snippet;
pub mod constants;
pub mod container;
pub mod contains;
pub mod divide;
pub mod dynamic_snippet;
pub mod equal;
pub mod every;
pub mod file_pattern;
pub mod files;
pub mod float_constant;
pub mod function_definition;
pub mod functions;
pub mod r#if;
pub mod includes;
pub mod int_constant;
pub mod iter_pattern;
pub mod like;
pub mod limit;
pub mod list;
pub mod list_index;
pub mod log;
pub mod map;
pub mod r#match;
pub mod maybe;
pub mod modulo;
pub mod multiply;
pub mod not;
pub mod or;
pub mod paths;
pub mod pattern_definition;
pub mod patterns;
pub mod predicate_definition;
pub mod predicate_return;
pub mod predicates;
pub mod range;
pub mod regex;
pub mod resolved_pattern;
pub mod rewrite;
pub mod sequential;
pub mod some;
pub mod state;
pub mod step;
pub mod string_constant;
pub mod subtract;
pub mod undefined;
pub mod variable;
pub mod variable_content;
pub mod r#where;
pub mod within;

use self::{
    built_in_functions::{BuiltIns, CallBuiltIn},
    constants::DEFAULT_FILE_NAME,
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    pattern_definition::PatternDefinition,
    predicate_definition::PredicateDefinition,
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::{
    context::{ExecContext, QueryContext},
    problem::{FileOwners, MarzanoQueryContext},
};
use anyhow::Result;
use marzano_language::target_language::TargetLanguage;
use marzano_util::{
    analysis_logs::AnalysisLogs, position::VariableMatch, runtime::ExecutionContext,
};
use std::fmt::Debug;
use std::vec;
use variable::VariableSourceLocations;

#[cfg(feature = "grit_tracing")]
use tracing::{instrument, span};
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

pub const MAX_FILE_SIZE: usize = 1_000_000;

/**
 * We want both Work and State to not contain things that cannot be moved across threads.
 *
 * Even without threads, we want the ability to continue execution with a close of a State and Work.
 *
 * E.g., If a Node would contain a tree-sitter cursor, that would not be safe.
 */
pub trait Work<Q: QueryContext> {
    // it is important that any implementors of Work
    // do not compute-expensive things in execute
    // it should be stored somewhere in the struct of the implementor
    // fn execute(&self, state: &mut State) -> Vec<Match>;
    fn execute(&self, state: &mut State<Q>);
}

#[derive(Debug, Default)]
pub(crate) struct VariableLocations {
    pub(crate) locations: Vec<Vec<VariableSourceLocations>>,
}

impl VariableLocations {
    pub(crate) fn new(locations: Vec<Vec<VariableSourceLocations>>) -> Self {
        Self { locations }
    }
    pub(crate) fn compiled_vars(&self) -> Vec<VariableMatch> {
        let mut variables = vec![];
        for (i, scope) in self.locations.iter().enumerate() {
            for (j, var) in scope.iter().enumerate() {
                if var.file == DEFAULT_FILE_NAME {
                    variables.push(VariableMatch {
                        name: var.name.clone(),
                        scoped_name: format!("{}_{}_{}", i, j, var.name),
                        ranges: var.locations.iter().cloned().collect(),
                    });
                }
            }
        }
        variables
    }
}

pub struct MarzanoContext<'a> {
    pub pattern_definitions: &'a Vec<PatternDefinition<MarzanoQueryContext>>,
    pub predicate_definitions: &'a Vec<PredicateDefinition<MarzanoQueryContext>>,
    pub function_definitions: &'a Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    pub foreign_function_definitions: &'a Vec<ForeignFunctionDefinition>,
    pub files: &'a FileOwners,
    pub built_ins: &'a BuiltIns,
    pub language: &'a TargetLanguage,
    pub runtime: &'a ExecutionContext,
    pub name: Option<String>,
}

impl<'a> MarzanoContext<'a> {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        pattern_definitions: &'a Vec<PatternDefinition<MarzanoQueryContext>>,
        predicate_definitions: &'a Vec<PredicateDefinition<MarzanoQueryContext>>,
        function_definitions: &'a Vec<GritFunctionDefinition<MarzanoQueryContext>>,
        foreign_function_definitions: &'a Vec<ForeignFunctionDefinition>,
        files: &'a FileOwners,
        built_ins: &'a BuiltIns,
        language: &'a TargetLanguage,
        runtime: &'a ExecutionContext,
        name: Option<String>,
    ) -> Self {
        Self {
            pattern_definitions,
            predicate_definitions,
            function_definitions,
            foreign_function_definitions,
            files,
            built_ins,
            language,
            runtime,
            name,
        }
    }
}

impl<'a> ExecContext<MarzanoQueryContext> for MarzanoContext<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition<MarzanoQueryContext>] {
        self.pattern_definitions
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition<MarzanoQueryContext>] {
        self.predicate_definitions
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition<MarzanoQueryContext>] {
        self.function_definitions
    }

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition] {
        self.foreign_function_definitions
    }

    fn ignore_limit_pattern(&self) -> bool {
        self.runtime.ignore_limit_pattern
    }

    fn call_built_in<'b>(
        &self,
        call: &'b CallBuiltIn<MarzanoQueryContext>,
        context: &'b Self,
        state: &mut State<'b, MarzanoQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'b, MarzanoQueryContext>> {
        self.built_ins.call(call, context, state, logs)
    }

    #[cfg(all(
        feature = "network_requests_external",
        feature = "external_functions_ffi",
        not(feature = "network_requests"),
        target_arch = "wasm32"
    ))]
    fn exec_external(
        &self,
        code: &[u8],
        param_names: Vec<String>,
        input_bindings: &[&str],
    ) -> Result<Vec<u8>> {
        (self.runtime.exec_external)(code, param_names, input_bindings)
    }

    // FIXME: Don't depend on Grit's file handling in context.
    fn files(&self) -> &FileOwners {
        self.files
    }

    // FIXME: This introduces a dependency on TreeSitter.
    fn language(&self) -> &TargetLanguage {
        self.language
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
