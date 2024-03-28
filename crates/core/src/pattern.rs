mod accessor;
mod accumulate;
mod add;
mod after;
pub mod analysis;
mod and;
mod any;
pub mod api;
mod assignment;
mod ast_node;
mod auto_wrap;
mod before;
mod boolean_constant;
mod bubble;
pub mod built_in_functions;
mod call;
mod code_snippet;
pub mod compiler;
mod container;
mod contains;
mod divide;
mod dynamic_snippet;
mod equal;
mod every;
mod file_pattern;
mod files;
mod float_constant;
pub mod function_definition;
mod functions;
mod r#if;
mod includes;
mod int_constant;
mod iter_pattern;
mod like;
mod limit;
mod list;
mod list_index;
mod log;
mod map;
mod r#match;
mod maybe;
mod modulo;
mod multiply;
pub mod not;
pub mod or;
mod paths;
pub mod pattern_definition;
pub mod patterns;
pub mod predicate_definition;
mod predicate_return;
mod predicates;
mod range;
mod regex;
pub mod resolved_pattern;
mod rewrite;
mod sequential;
mod some;
pub mod state;
mod step;
mod string_constant;
mod subtract;
mod undefined;
pub mod variable;
mod variable_content;
mod r#where;
mod within;
use crate::{
    binding::Binding,
    context::Context,
    pattern::{compiler::DEFAULT_FILE_NAME, patterns::Matcher, resolved_pattern::ResolvedPattern},
};
use ::log::error;
use anyhow::{bail, Ok, Result};
use elsa::FrozenVec;
use im::vector;
use itertools::Itertools;
use marzano_language::{language::Language, target_language::TargetLanguage};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::runtime::ExecutionContext;
use marzano_util::{
    cache::{GritCache, NullCache},
    hasher::hash,
    position::{Position, Range, VariableMatch},
    rich_path::{FileName, RichFile, RichPath, TryIntoInputFile},
};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    cell::RefCell,
    fmt::{self, Debug},
    path::PathBuf,
    sync::mpsc::{self, Sender},
};
use std::{ops, vec};

use tracing::{event, Level};

#[cfg(feature = "grit_tracing")]
use tracing::instrument;

#[cfg(feature = "grit_tracing")]
use tracing::span;
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use tree_sitter::{Node, Tree};
use variable::VariableSourceLocations;

use self::{
    api::{is_match, AnalysisLog, ByteRange, DoneFile, MatchResult},
    built_in_functions::{BuiltIns, CallBuiltIn},
    compiler::NEW_FILES_INDEX,
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    paths::absolutize,
    pattern_definition::PatternDefinition,
    patterns::Pattern,
    predicate_definition::PredicateDefinition,
    resolved_pattern::File,
    state::{FilePtr, State},
    step::Step,
    variable::{register_variable, GLOBAL_VARS_SCOPE_INDEX},
    variable_content::VariableContent,
};

pub const MAX_FILE_SIZE: usize = 1_000_000;

/**
 * We want both Work and State to not contain things that cannot be moved across threads.
 *
 * Even without threads, we want the ability to continue execution with a close of a State and Work.
 *
 * E.g., If a Node would contain a tree-sitter cursor, that would not be safe.
 */
pub trait Work {
    // it is important that any implementors of Work
    // do not compute-expensive things in execute
    // it should be stored somewhere in the struct of the implementor
    // fn execute(&self, state: &mut State) -> Vec<Match>;
    fn execute(&self, state: &mut State);
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
                        ranges: var.locations.clone().into_iter().collect_vec(),
                    });
                }
            }
        }
        variables
    }
}

#[derive(Debug)]
pub struct Problem {
    pub src: String,
    pub tree: Tree,
    // todo replace with pattern: Pattern
    pub pattern: Pattern,
    pub language: TargetLanguage,
    pub built_ins: BuiltIns,
    pub is_multifile: bool,
    pub has_limit: bool,
    pub hash: [u8; 32],
    pub name: Option<String>,
    pub(crate) variables: VariableLocations,
    pub(crate) pattern_definitions: Vec<PatternDefinition>,
    pub(crate) predicate_definitions: Vec<PredicateDefinition>,
    pub(crate) function_definitions: Vec<GritFunctionDefinition>,
    pub(crate) foreign_function_definitions: Vec<ForeignFunctionDefinition>,
}

impl Problem {
    pub fn compiled_vars(&self) -> Vec<VariableMatch> {
        self.variables.compiled_vars()
    }
}

#[derive(Debug, Clone)]
struct InputRanges {
    ranges: Vec<Range>,
    variables: Vec<VariableMatch>,
    suppressed: bool,
}

#[derive(Debug, Clone, Default)]
pub(crate) struct MatchRanges {
    input_matches: Option<InputRanges>,
    byte_ranges: Option<Vec<ByteRange>>,
}

impl MatchRanges {
    pub(crate) fn new(byte_ranges: Vec<ByteRange>) -> Self {
        Self {
            input_matches: None,
            byte_ranges: Some(byte_ranges),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FileOwner {
    absolute_path: PathBuf,
    name: PathBuf,
    // todo wrap in Rc<RefCell<Option<>>>
    // so that we can lazily parse
    tree: Tree,
    source: String,
    matches: RefCell<MatchRanges>,
    new: bool,
}

impl FileOwner {
    pub(crate) fn new(
        name: impl Into<PathBuf>,
        source: String,
        matches: Option<MatchRanges>,
        new: bool,
        language: &impl Language,
        logs: &mut AnalysisLogs,
    ) -> Result<Option<Self>> {
        let name = name.into();
        let Some(tree) =
            language.parse_file(name.to_string_lossy().as_ref(), &source, logs, new)?
        else {
            return Ok(None);
        };
        let absolute_path = PathBuf::from(absolutize(name.to_string_lossy().as_ref())?);
        Ok(Some(FileOwner {
            name,
            absolute_path,
            tree,
            source,
            matches: matches.unwrap_or_default().into(),
            new,
        }))
    }
}

impl PartialEq for FileOwner {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.source == other.source
    }
}

enum FilePattern {
    Single(FilePtr),
    Many(Vec<FilePtr>),
}

impl From<FilePtr> for FilePattern {
    fn from(file: FilePtr) -> Self {
        Self::Single(file)
    }
}

impl From<Vec<FilePtr>> for FilePattern {
    fn from(files: Vec<FilePtr>) -> Self {
        Self::Many(files)
    }
}

impl From<FilePattern> for ResolvedPattern<'_> {
    fn from(val: FilePattern) -> Self {
        match val {
            FilePattern::Single(file) => ResolvedPattern::File(File::Ptr(file)),
            FilePattern::Many(files) => ResolvedPattern::Files(Box::new(ResolvedPattern::List(
                files
                    .into_iter()
                    .map(|f| ResolvedPattern::File(File::Ptr(f)))
                    .collect(),
            ))),
        }
    }
}

struct FilePatternOutput {
    file_pattern: Option<FilePattern>,
    file_owners: FileOwners,
    done_files: Vec<MatchResult>,
    error_files: Vec<MatchResult>,
}

fn send(tx: &Sender<Vec<MatchResult>>, value: Vec<MatchResult>) {
    if let Err(err) = tx.send(value) {
        error!("Failed to emit execution result: {}", err);
    }
}

impl Problem {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        src: String,
        tree: Tree,
        pattern: Pattern,
        language: TargetLanguage,
        built_ins: BuiltIns,
        is_multifile: bool,
        has_limit: bool,
        name: Option<String>,
        variables: VariableLocations,
        pattern_definitions: Vec<PatternDefinition>,
        predicate_definitions: Vec<PredicateDefinition>,
        function_definitions: Vec<GritFunctionDefinition>,
        foreign_function_definitions: Vec<ForeignFunctionDefinition>,
    ) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(
            format!(
                "{:?}{:?}{:?}{:?}{:?}",
                pattern,
                pattern_definitions,
                predicate_definitions,
                function_definitions,
                foreign_function_definitions
            )
            .to_string()
            .as_str(),
        );
        let hash = hasher.finalize().into();

        Self {
            src,
            tree,
            pattern,
            language,
            built_ins,
            is_multifile,
            has_limit,
            hash,
            name,
            variables,
            pattern_definitions,
            predicate_definitions,
            function_definitions,
            foreign_function_definitions,
        }
    }

    fn build_resolved_pattern(
        &self,
        files: &[impl TryIntoInputFile + FileName],
        cache: &impl GritCache,
    ) -> Result<FilePatternOutput> {
        let owned_files = FileOwners::new();
        let mut results = vec![];
        let mut file_pointers = vec![];
        let mut done_files = vec![];
        if !self.is_multifile && files.len() != 1 {
            bail!("Cannot build resolved pattern for single file pattern with more than one file")
        }
        for file in files {
            let file: Cow<RichFile> = match file.try_into_cow() {
                Result::Ok(file) => file,
                Result::Err(err) => {
                    results.push(MatchResult::AnalysisLog(AnalysisLog::new_error(
                        err.to_string(),
                        &file.name(),
                    )));
                    continue;
                }
            };
            if let Some(log) = is_file_too_big(&file) {
                results.push(MatchResult::AnalysisLog(log));
                results.push(MatchResult::DoneFile(DoneFile {
                    relative_file_path: file.path.to_string(),
                    // Don't know if there are results, so we can't cache
                    ..Default::default()
                }))
            } else {
                let file_hash = hash(&file.path);
                if cache.has_no_matches(file_hash, self.hash) {
                    results.push(MatchResult::DoneFile(DoneFile {
                        relative_file_path: file.path.to_string(),
                        has_results: Some(false),
                        file_hash: Some(file_hash),
                        from_cache: true,
                    }));
                } else {
                    let mut logs = vec![].into();
                    let owned_file = FileOwner::new(
                        file.path.to_owned(),
                        file.content.to_owned(),
                        None,
                        false,
                        &self.language,
                        &mut logs,
                    );
                    results.extend(
                        logs.logs()
                            .into_iter()
                            .map(|l| MatchResult::AnalysisLog(l.into())),
                    );
                    match owned_file {
                        Result::Ok(owned_file) => {
                            if let Some(owned_file) = owned_file {
                                file_pointers.push(FilePtr::new(file_pointers.len() as u16, 0));
                                owned_files.push(owned_file);
                            }
                            done_files.push(MatchResult::DoneFile(DoneFile {
                                relative_file_path: file.path.to_string(),
                                has_results: None,
                                file_hash: Some(file_hash),
                                from_cache: false,
                            }))
                        }
                        Result::Err(err) => {
                            results.push(MatchResult::AnalysisLog(AnalysisLog::new_error(
                                err.to_string(),
                                &file.path,
                            )));
                            results.push(MatchResult::DoneFile(DoneFile {
                                relative_file_path: file.path.to_string(),
                                ..Default::default()
                            }))
                        }
                    }
                }
            }
        }
        let binding = if self.is_multifile {
            file_pointers.into()
        } else if file_pointers.is_empty() {
            // single file pattern had file that was too big
            return Ok(FilePatternOutput {
                file_pattern: None,
                file_owners: owned_files,
                done_files,
                error_files: results,
            });
        } else {
            file_pointers[0].into()
        };
        Ok(FilePatternOutput {
            file_pattern: Some(binding),
            file_owners: owned_files,
            done_files,
            error_files: results,
        })
    }

    fn execute_and_send(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: &[impl TryIntoInputFile + FileName],
        binding: FilePattern,
        owned_files: &FileOwners,
        context: &ExecutionContext,
        mut done_files: Vec<MatchResult>,
    ) {
        let mut outputs = match self.execute(binding, owned_files, context) {
            Result::Err(err) => files
                .iter()
                .map(|file| {
                    MatchResult::AnalysisLog(AnalysisLog::new_error(err.to_string(), &file.name()))
                })
                .collect(),
            Result::Ok(messages) => messages,
        };
        if done_files.len() == 1 {
            if let MatchResult::DoneFile(ref mut done_file) = done_files[0] {
                let has_results = outputs
                    .iter()
                    .any(|m| is_match(m) || matches!(m, MatchResult::AnalysisLog(_)));
                done_file.has_results = Some(has_results);
            };
        }
        outputs.extend(done_files);
        if self.is_multifile {
            // to keep snapshot tests happy, not ideal;
            outputs.sort();
        }
        send(tx, outputs);
    }

    fn build_and_execute_resolved_pattern(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: &[impl TryIntoInputFile + FileName],
        context: &ExecutionContext,
        cache: &impl GritCache,
    ) {
        match self.build_resolved_pattern(files, cache) {
            Result::Ok(FilePatternOutput {
                file_pattern,
                file_owners,
                done_files,
                error_files,
            }) => {
                send(tx, error_files);
                if let Some(file_pattern) = file_pattern {
                    self.execute_and_send(
                        tx,
                        files,
                        file_pattern,
                        &file_owners,
                        context,
                        done_files,
                    );
                }
            }
            Result::Err(err) => {
                // might be sending too many donefile here?
                let mut error_files = vec![];
                for file in files {
                    error_files.push(MatchResult::AnalysisLog(AnalysisLog::new_error(
                        err.to_string(),
                        &file.name(),
                    )));
                    error_files.push(MatchResult::DoneFile(DoneFile {
                        relative_file_path: file.name().to_string(),
                        ..Default::default()
                    }))
                }
                send(tx, error_files);
            }
        }
    }

    pub fn execute_files(
        &self,
        files: &[RichFile],
        context: &ExecutionContext,
    ) -> Vec<MatchResult> {
        let mut results = vec![];
        let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();

        self.execute_shared(files, context, tx, &NullCache::new());
        for r in rx.iter() {
            results.extend(r)
        }
        results.sort();
        results
    }

    pub fn execute_paths<'a>(
        &self,
        files: &[&'a RichPath],
        context: &ExecutionContext,
    ) -> (Vec<MatchResult>, Vec<&'a RichPath>) {
        let mut results = vec![];
        let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();
        let mut to_cache = if self.is_multifile {
            vec![]
        } else {
            files.to_owned()
        };
        self.execute_shared(files, context, tx, &NullCache::new());
        for r in rx.iter() {
            if !self.is_multifile {
                for m in r.iter() {
                    if is_match(m) {
                        if let Some(name) = m.file_name() {
                            if let Some(pos) = to_cache
                                .iter()
                                .position(|f| f.path.to_string_lossy() == name)
                            {
                                to_cache.remove(pos);
                            }
                        }
                    }
                }
            }
            results.extend(r)
        }
        results.sort();
        (results, to_cache)
    }

    pub fn execute_file(&self, file: &RichFile, context: &ExecutionContext) -> Vec<MatchResult> {
        let mut results = vec![];
        let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();
        self.execute_shared(std::array::from_ref(file), context, tx, &NullCache::new());
        for r in rx.iter() {
            results.extend(r)
        }
        results.sort();
        results
    }

    pub fn execute_files_streaming(
        &self,
        files: &[RichFile],
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
    ) {
        self.execute_shared(files, context, tx, &NullCache::new())
    }

    pub fn execute_paths_streaming(
        &self,
        files: &[PathBuf],
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
        cache: &impl GritCache,
    ) {
        self.execute_shared(files, context, tx, cache)
    }

    #[cfg_attr(feature = "grit_tracing", instrument(skip_all))]
    fn execute_shared(
        &self,
        files: &[impl TryIntoInputFile + FileName + Send + Sync],
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
        cache: &impl GritCache,
    ) {
        #[cfg(feature = "grit_tracing")]
        let parent_span = span!(Level::INFO, "execute_shared_body",).entered();
        #[cfg(feature = "grit_tracing")]
        let parent_cx = parent_span.context();

        if self.is_multifile {
            self.build_and_execute_resolved_pattern(&tx, files, context, &NullCache::new());
        } else {
            rayon::scope(|s| {
                #[cfg(feature = "grit_tracing")]
                let grouped_ctx = parent_cx;

                s.spawn(|_| {
                    #[cfg(feature = "grit_tracing")]
                    let task_span = tracing::info_span!("apply_file_inner").entered();
                    #[cfg(feature = "grit_tracing")]
                    task_span.set_parent(grouped_ctx);

                    event!(Level::INFO, "spawn execute_shared_body");

                    files.par_iter().for_each_with(tx, |sender, f| {
                        self.build_and_execute_resolved_pattern(
                            sender,
                            std::array::from_ref(f),
                            context,
                            cache,
                        );
                    });
                })
            })
        }
    }

    fn execute(
        &self,
        binding: FilePattern,
        owned_files: &FileOwners,
        context: &ExecutionContext,
    ) -> Result<Vec<MatchResult>> {
        let mut user_logs = vec![].into();

        let context = MarzanoContext::new(
            &self.pattern_definitions,
            &self.predicate_definitions,
            &self.function_definitions,
            &self.foreign_function_definitions,
            owned_files,
            &self.built_ins,
            &self.language,
            context,
            self.name.clone(),
        );

        let bindings = self
            .variables
            .locations
            .iter()
            .map(|scope| {
                vector![scope
                    .iter()
                    .map(|s| Box::new(VariableContent::new(s.name.clone())))
                    .collect()]
            })
            .collect();

        let file_refs: Vec<&FileOwner> = context.files.iter().collect();
        let mut state = State::new(bindings, file_refs);

        let the_new_files =
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[NEW_FILES_INDEX].as_mut();
        the_new_files.value = Some(ResolvedPattern::List(vector!()));

        let mut results: Vec<MatchResult> = Vec::new();
        let binding = binding.into();
        if self
            .pattern
            .execute(&binding, &mut state, &context, &mut user_logs)?
        {
            for file in state.files.files() {
                if let Some(result) = MatchResult::file_to_match_result(file, &self.language)? {
                    results.push(result)
                }
            }
        }

        let mut user_logs: Vec<MatchResult> = user_logs
            .clone()
            .into_iter()
            .map(|l| MatchResult::AnalysisLog(l.into()))
            .collect();
        user_logs.extend(results);
        Ok(user_logs)
    }
}

fn is_file_too_big(file: &RichFile) -> Option<AnalysisLog> {
    if file.path.len() > MAX_FILE_SIZE || file.content.len() > MAX_FILE_SIZE {
        Some(AnalysisLog {
            // TODO: standardize levels
            level: 310,
            message: format!("Skipped {}, it is too big.", file.path),
            file: file.path.to_owned(),
            engine_id: "marzano".to_owned(),
            position: Position::first(),
            syntax_tree: None,
            range: None,
            source: None,
        })
    } else {
        None
    }
}

#[derive(Debug, Clone)]
pub enum EffectKind {
    Rewrite,
    Insert,
}

#[derive(Debug, Clone)]
pub struct Effect<'a> {
    pub binding: Binding<'a>,
    pub(crate) pattern: ResolvedPattern<'a>,
    pub kind: EffectKind,
}

pub struct FileOwners(FrozenVec<Box<FileOwner>>);

impl FileOwners {
    pub fn new() -> Self {
        Self(FrozenVec::new())
    }

    pub fn push(&self, file: FileOwner) {
        self.0.push(Box::new(file))
    }
}
impl Default for FileOwners {
    fn default() -> Self {
        Self::new()
    }
}

impl ops::Deref for FileOwners {
    type Target = FrozenVec<Box<FileOwner>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Debug for FileOwners {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0
            .iter()
            .try_fold((), |_, file| writeln!(f, "{}", file.name.display()))
    }
}

pub struct MarzanoContext<'a> {
    pub pattern_definitions: &'a Vec<PatternDefinition>,
    pub predicate_definitions: &'a Vec<PredicateDefinition>,
    pub function_definitions: &'a Vec<GritFunctionDefinition>,
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
        pattern_definitions: &'a Vec<PatternDefinition>,
        predicate_definitions: &'a Vec<PredicateDefinition>,
        function_definitions: &'a Vec<GritFunctionDefinition>,
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

impl<'a> Context for MarzanoContext<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition] {
        self.pattern_definitions
    }

    fn predicate_definitions(&self) -> &[PredicateDefinition] {
        self.predicate_definitions
    }

    fn function_definitions(&self) -> &[GritFunctionDefinition] {
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
        call: &'b CallBuiltIn,
        context: &'b Self,
        state: &mut State<'b>,
        logs: &mut AnalysisLogs,
    ) -> Result<ResolvedPattern<'b>> {
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
