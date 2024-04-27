use crate::{
    built_in_functions::BuiltIns,
    clean::{get_replacement_ranges, replace_cleaned_ranges},
    foreign_function_definition::ForeignFunctionDefinition,
    marzano_resolved_pattern::MarzanoResolvedPattern,
    pattern_compiler::file_owner_compiler::FileOwnerCompiler,
    problem::MarzanoQueryContext,
    text_unparser::apply_effects,
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::{
    binding::Binding,
    constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX},
    context::ExecContext,
    file_owners::FileOwners,
    pattern::{
        CallBuiltIn, File, FilePtr, GritFunctionDefinition, Matcher, Pattern, PatternDefinition,
        PredicateDefinition, ResolvedPattern, State,
    },
};
use grit_util::{AnalysisLogs, Ast, InputRanges, MatchRanges};
use im::vector;
use marzano_language::{
    language::{MarzanoLanguage, Tree},
    target_language::TargetLanguage,
};
use marzano_util::runtime::ExecutionContext;
use std::path::PathBuf;

pub struct MarzanoContext<'a> {
    pub pattern_definitions: &'a Vec<PatternDefinition<MarzanoQueryContext>>,
    pub predicate_definitions: &'a Vec<PredicateDefinition<MarzanoQueryContext>>,
    pub function_definitions: &'a Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    pub foreign_function_definitions: &'a Vec<ForeignFunctionDefinition>,
    pub files: &'a FileOwners<Tree>,
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
        files: &'a FileOwners<Tree>,
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

    #[cfg(all(
        feature = "network_requests_external",
        feature = "external_functions_ffi",
        not(feature = "network_requests"),
        target_arch = "wasm32"
    ))]
    pub fn exec_external(
        &self,
        code: &[u8],
        param_names: Vec<String>,
        input_bindings: &[&str],
    ) -> Result<Vec<u8>> {
        (self.runtime.exec_external)(code, param_names, input_bindings)
    }

    pub(crate) fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition] {
        self.foreign_function_definitions
    }
}

impl<'a> ExecContext<'a, MarzanoQueryContext> for MarzanoContext<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition<MarzanoQueryContext>] {
        self.pattern_definitions
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition<MarzanoQueryContext>] {
        self.predicate_definitions
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition<MarzanoQueryContext>] {
        self.function_definitions
    }

    fn ignore_limit_pattern(&self) -> bool {
        self.runtime.ignore_limit_pattern
    }

    fn call_built_in(
        &self,
        call: &'a CallBuiltIn<MarzanoQueryContext>,
        context: &'a Self,
        state: &mut State<'a, MarzanoQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<MarzanoResolvedPattern<'a>> {
        self.built_ins.call(call, context, state, logs)
    }

    // FIXME: Don't depend on Grit's file handling in context.
    fn files(&self) -> &FileOwners<Tree> {
        self.files
    }

    fn language(&self) -> &TargetLanguage {
        self.language
    }

    fn exec_step(
        &'a self,
        step: &'a Pattern<MarzanoQueryContext>,
        binding: &MarzanoResolvedPattern<'a>,
        state: &mut State<'a, MarzanoQueryContext>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let mut parser = self.language().get_parser();

        let files = if let Some(files) = binding.get_file_pointers() {
            files
                .iter()
                .map(|f| state.files.latest_revision(f))
                .collect::<Vec<FilePtr>>()
        } else {
            return Ok(false);
        };

        let binding = if files.len() == 1 {
            ResolvedPattern::from_file_pointer(*files.last().unwrap())
        } else {
            ResolvedPattern::from_files(ResolvedPattern::from_list_parts(
                files.iter().map(|f| ResolvedPattern::from_file_pointer(*f)),
            ))
        };
        if !step.execute(&binding, state, self, logs)? {
            return Ok(false);
        }

        // todo, for multifile we need to split up the matches by file.
        let (variables, ranges, suppressed) =
            state.bindings_history_to_ranges(self.language(), self.name());

        let input_ranges = InputRanges {
            ranges,
            variables,
            suppressed,
        };
        for file_ptr in files {
            let file = state.files.get_file(file_ptr);
            let mut match_log = file.matches.borrow_mut();

            let filename_path = &file.name;

            let mut new_filename = filename_path.clone();

            let src = &file.tree.source;

            if match_log.input_matches.is_none() {
                match_log.input_matches = Some(input_ranges.clone());
            }

            if state
                .effects
                .iter()
                .find(|e| {
                    e.binding.source() == Some(src)
                        || e.binding.as_filename() == Some(filename_path)
                })
                .cloned()
                .is_some()
            {
                let code = file.tree.root_node();
                let (new_src, new_ranges) = apply_effects(
                    code,
                    state.effects.clone(),
                    &state.files,
                    &file.name,
                    &mut new_filename,
                    self,
                    logs,
                )?;
                if let Some(new_ranges) = new_ranges {
                    let tree = parser.parse_file(&new_src, None, logs, true).unwrap();
                    let root = tree.root_node();
                    let replacement_ranges = get_replacement_ranges(root, self.language());
                    let cleaned_src = replace_cleaned_ranges(replacement_ranges, &new_src)?;
                    let new_src = if let Some(src) = cleaned_src {
                        src
                    } else {
                        new_src
                    };

                    let ranges =
                        MatchRanges::new(new_ranges.into_iter().map(|r| r.into()).collect());
                    let owned_file = FileOwnerCompiler::from_matches(
                        new_filename.clone(),
                        new_src,
                        Some(ranges),
                        true,
                        self.language(),
                        logs,
                    )?
                    .ok_or_else(|| {
                        anyhow!(
                            "failed to construct new file for file {}",
                            new_filename.to_string_lossy()
                        )
                    })?;
                    self.files().push(owned_file);
                    state
                        .files
                        .push_revision(&file_ptr, self.files().last().unwrap())
                }
            };
        }

        let Some(new_files) = state.bindings[GLOBAL_VARS_SCOPE_INDEX]
            .last()
            .and_then(|binding| binding[NEW_FILES_INDEX].value.as_ref())
            .and_then(ResolvedPattern::get_list_items)
        else {
            bail!("Expected a list of files")
        };

        for f in new_files {
            let Some(file) = f.get_file() else {
                bail!("Expected a list of files")
            };

            let name: PathBuf = file
                .name(&state.files)
                .text(&state.files, self.language())
                .unwrap()
                .as_ref()
                .into();
            let body = file
                .body(&state.files)
                .text(&state.files, self.language())
                .unwrap()
                .into();
            let owned_file = FileOwnerCompiler::from_matches(
                name.clone(),
                body,
                None,
                true,
                self.language(),
                logs,
            )?
            .ok_or_else(|| {
                anyhow!(
                    "failed to construct new file for file {}",
                    name.to_string_lossy()
                )
            })?;
            self.files().push(owned_file);
            let _ = state.files.push_new_file(self.files().last().unwrap());
        }

        state.effects = vector![];
        let the_new_files =
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[NEW_FILES_INDEX].as_mut();
        the_new_files.value = Some(ResolvedPattern::from_list_parts([].into_iter()));
        Ok(true)
    }

    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }
}
