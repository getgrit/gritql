use super::{
    auto_wrap::auto_wrap_pattern,
    compiler::{
        filter_libs, get_definition_info, get_definitions, CompilationContext, DefinitionInfo,
        DefinitionInfoKinds, NodeCompilationContext, VariableLocations,
    },
    pattern_compiler::PatternCompiler,
    CompilationResult, NodeCompiler,
};
use crate::{
    analysis::{has_limit, is_multifile},
    built_in_functions::{BuiltInFunction, BuiltIns},
    foreign_function_definition::ForeignFunctionDefinition,
    marzano_context::MarzanoContext,
    marzano_resolved_pattern::MarzanoResolvedPattern,
    problem::{self, MarzanoQueryContext, Problem},
    variables::variable_from_name,
};
use crate::{built_in_functions::CallableFn, pattern_compiler::compiler::DefinitionOutput};
use anyhow::{bail, Result};
use grit_pattern_matcher::pattern::State;
use grit_pattern_matcher::{
    constant::Constant,
    constants::{
        ABSOLUTE_PATH_INDEX, DEFAULT_FILE_NAME, FILENAME_INDEX, MATCH_VAR, NEW_FILES_INDEX,
        PROGRAM_INDEX,
    },
    pattern::{
        And, BooleanConstant, CallBuiltIn, Container, GritFunctionDefinition, Match, Pattern,
        PatternDefinition, Predicate, PredicateDefinition, ResolvedPattern,
        VariableSourceLocations, Where,
    },
};
use grit_util::{AnalysisLogs, Ast, FileRange};

use marzano_language::{
    self, grit_parser::MarzanoGritParser, language::Tree, target_language::TargetLanguage,
};

use std::{collections::BTreeMap, path::Path, vec};

pub type CallbackMatchFn = dyn for<'a> Fn(
        &<problem::MarzanoQueryContext as grit_pattern_matcher::context::QueryContext>::ResolvedPattern<'a>,
        &'a MarzanoContext<'a>,
        &mut State<'a, MarzanoQueryContext>,
        &mut AnalysisLogs,
    ) -> Result<bool>
    + Send
    + Sync;

/// Pattern Builder allows you to progressively compile a pattern.
/// You always start with a source GritQL string, but additional patterns can be attached before the final query.
pub struct PatternBuilder {
    tree: Tree,
    pattern: Pattern<MarzanoQueryContext>,
    language: TargetLanguage,
    built_ins: BuiltIns,
    is_multifile: bool,
    has_limit: bool,
    name: Option<String>,
    vars: BTreeMap<String, usize>,

    current_scope_index: usize,
    vars_array: Vec<Vec<VariableSourceLocations>>,
    global_vars: BTreeMap<String, usize>,

    pattern_definition_indices: BTreeMap<String, DefinitionInfo>,
    pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,

    predicate_definition_indices: BTreeMap<String, DefinitionInfo>,
    predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,

    function_definition_indices: BTreeMap<String, DefinitionInfo>,
    function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,

    foreign_function_indices: BTreeMap<String, DefinitionInfo>,
    foreign_function_definitions: Vec<ForeignFunctionDefinition>,

    compilation_warnings: AnalysisLogs,
}

impl PatternBuilder {
    #[allow(clippy::too_many_arguments)]
    pub fn start(
        src: String,
        libs: &BTreeMap<String, String>,
        lang: TargetLanguage,
        name: Option<String>,
        grit_parser: &mut MarzanoGritParser,
        custom_built_ins: Option<BuiltIns>,
    ) -> Result<Self> {
        if src == "." {
            let error = ". never matches and should not be used as a pattern. Did you mean to run 'grit apply <pattern> .'?";
            bail!(error);
        }
        let src_tree = grit_parser.parse_file(&src, Some(Path::new(DEFAULT_FILE_NAME)))?;

        let root = src_tree.root_node();
        let mut built_ins = BuiltIns::get_built_in_functions();
        if let Some(custom_built_ins) = custom_built_ins {
            built_ins.extend_builtins(custom_built_ins)?;
        }
        let mut logs: AnalysisLogs = vec![].into();
        let mut global_vars = BTreeMap::from([
            ("$new_files".to_owned(), NEW_FILES_INDEX),
            ("$filename".to_owned(), FILENAME_INDEX),
            ("$program".to_owned(), PROGRAM_INDEX),
            ("$absolute_filename".to_owned(), ABSOLUTE_PATH_INDEX),
        ]);
        let is_multifile = is_multifile(&root, libs, grit_parser)?;
        let has_limit = has_limit(&root, libs, grit_parser)?;
        let libs = filter_libs(libs, &src, grit_parser, !is_multifile)?;
        let DefinitionInfoKinds {
            pattern_indices: pattern_definition_indices,
            predicate_indices: predicate_definition_indices,
            function_indices: function_definition_indices,
            foreign_function_indices,
        } = get_definition_info(&libs, &root, grit_parser)?;

        let context = CompilationContext {
            file: DEFAULT_FILE_NAME,
            built_ins: &built_ins,
            lang: &lang,
            pattern_definition_info: &pattern_definition_indices,
            predicate_definition_info: &predicate_definition_indices,
            function_definition_info: &function_definition_indices,
            foreign_function_definition_info: &foreign_function_indices,
        };

        let DefinitionOutput {
            mut vars_array,
            pattern_definitions,
            predicate_definitions,
            function_definitions,
            foreign_function_definitions,
        } = get_definitions(
            &libs,
            &root,
            grit_parser,
            &context,
            &mut global_vars,
            &mut logs,
        )?;
        let scope_index = vars_array.len();
        vars_array.push(vec![]);
        let mut vars = BTreeMap::new();

        let mut node_context = NodeCompilationContext {
            compilation: &context,
            vars: &mut vars,
            vars_array: &mut vars_array,
            scope_index,
            global_vars: &mut global_vars,
            logs: &mut logs,
        };

        let pattern = if let Some(node) = root.child_by_field_name("pattern") {
            PatternCompiler::from_node(&node, &mut node_context)?
        } else {
            let long_message = "No pattern found.
        If you have written a pattern definition in the form `pattern myPattern() {{ }}`,
        try calling it by adding `myPattern()` to the end of your file.
        Check out the docs at https://docs.grit.io for help with writing patterns.";
            bail!("{}", long_message);
        };

        Ok(Self {
            tree: src_tree,
            pattern,
            language: lang,
            built_ins,
            is_multifile,
            has_limit,
            name,

            current_scope_index: scope_index,
            vars,
            vars_array,
            global_vars,

            pattern_definition_indices,
            pattern_definitions,

            predicate_definition_indices,
            predicate_definitions,

            function_definition_indices,
            function_definitions,

            foreign_function_indices,
            foreign_function_definitions,

            compilation_warnings: logs,
        })
    }

    /// Wrap the pattern so it is independently processable
    /// compile() calls this, so you should *not* call this directly.
    ///
    /// See https://docs.grit.io/language/bubble#pattern-auto-wrap
    fn auto_wrap(
        mut self,
        file_ranges: Option<Vec<FileRange>>,
        injected_limit: Option<usize>,
    ) -> Result<Self> {
        let compilation = CompilationContext {
            file: DEFAULT_FILE_NAME,
            built_ins: &self.built_ins,
            lang: &self.language,
            pattern_definition_info: &self.pattern_definition_indices,
            predicate_definition_info: &self.predicate_definition_indices,
            function_definition_info: &self.function_definition_indices,
            foreign_function_definition_info: &self.foreign_function_indices,
        };

        let mut node_context = NodeCompilationContext {
            compilation: &compilation,
            vars: &mut self.vars,
            vars_array: &mut self.vars_array,
            scope_index: self.current_scope_index,
            global_vars: &mut self.global_vars,
            logs: &mut self.compilation_warnings,
        };

        let pattern = auto_wrap_pattern(
            self.pattern,
            &mut self.pattern_definitions,
            !self.is_multifile,
            file_ranges,
            &mut node_context,
            injected_limit,
        )?;
        Ok(Self { pattern, ..self })
    }

    /// Wrap the pattern in a where clause.
    /// This is the primary way we progressively add patterns to the builder.
    pub fn wrap_with_condition(self, side_condition: Predicate<MarzanoQueryContext>) -> Self {
        let pattern = Pattern::Where(Box::new(Where::new(self.pattern, side_condition)));
        Self { pattern, ..self }
    }

    /// Restrict the pattern
    pub fn matches(self, other: Pattern<MarzanoQueryContext>) -> Self {
        let joined = Pattern::And(Box::new(And::new(vec![self.pattern, other])));
        Self {
            pattern: joined,
            ..self
        }
    }

    /// Add a callback
    pub fn matches_callback(mut self, cb: Box<CallbackMatchFn>) -> Self {
        // This is a bit of a hack, but we can dynamically inject a new built-in and use that to call the callback
        let index = self
            .built_ins
            .add_callback(Box::new(move |args, context, state, logs| {
                let args = MarzanoResolvedPattern::from_patterns(args, state, context, logs)?;
                let first = args[0].as_ref().unwrap();

                let callback_result = cb(first, context, state, logs)?;

                Ok(MarzanoResolvedPattern::Constant(Constant::Boolean(
                    callback_result,
                )))
            }));

        let compilation = CompilationContext {
            file: DEFAULT_FILE_NAME,
            built_ins: &self.built_ins,
            lang: &self.language,
            pattern_definition_info: &self.pattern_definition_indices,
            predicate_definition_info: &self.predicate_definition_indices,
            function_definition_info: &self.function_definition_indices,
            foreign_function_definition_info: &self.foreign_function_indices,
        };

        let mut node_context = NodeCompilationContext {
            compilation: &compilation,
            vars: &mut self.vars,
            vars_array: &mut self.vars_array,
            scope_index: self.current_scope_index,
            global_vars: &mut self.global_vars,
            logs: &mut self.compilation_warnings,
        };

        // We reuse the match name here, not sure that's right.
        let match_var = variable_from_name(MATCH_VAR, &mut node_context).unwrap();
        let predicate_match = Predicate::Match(Box::new(Match::new(
            Container::FunctionCall(Box::new(CallBuiltIn::new(
                index,
                vec![Some(grit_pattern_matcher::pattern::Pattern::Variable(
                    match_var,
                ))],
            ))),
            Some(Pattern::BooleanConstant(BooleanConstant::new(true))),
        )));

        self.wrap_with_condition(predicate_match)
    }

    /// Add a new built in
    pub fn add_built_in(
        &mut self,
        name: &'static str,
        params: Vec<&'static str>,
        func: Box<CallableFn>,
    ) {
        self.built_ins
            .add_built_in(BuiltInFunction::new(name, params, func));
    }

    /// Compile the builder into a final Query
    pub fn compile(
        self,
        file_ranges: Option<Vec<FileRange>>,
        injected_limit: Option<usize>,
    ) -> Result<CompilationResult> {
        let target_builder = self.auto_wrap(file_ranges, injected_limit)?;
        let problem = Problem::new(
            target_builder.tree,
            target_builder.pattern,
            target_builder.language,
            target_builder.built_ins,
            target_builder.is_multifile,
            target_builder.has_limit,
            target_builder.name,
            VariableLocations::new(target_builder.vars_array),
            target_builder.pattern_definitions,
            target_builder.predicate_definitions,
            target_builder.function_definitions,
            target_builder.foreign_function_definitions,
        );
        let result = CompilationResult {
            compilation_warnings: target_builder.compilation_warnings,
            problem,
        };
        Ok(result)
    }
}
