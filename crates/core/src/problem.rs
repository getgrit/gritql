use crate::{
    api::{is_match, AnalysisLog, DoneFile, MatchResult},
    ast_node::{ASTNode, AstLeafNode},
    built_in_functions::BuiltIns,
    foreign_function_definition::ForeignFunctionDefinition,
    marzano_binding::MarzanoBinding,
    marzano_code_snippet::MarzanoCodeSnippet,
    marzano_context::MarzanoContext,
    marzano_resolved_pattern::{MarzanoFile, MarzanoResolvedPattern},
    pattern_compiler::compiler::VariableLocations,
};
use anyhow::{bail, Result};
use grit_pattern_matcher::{
    constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX},
    context::{QueryContext, StaticDefinitions},
    file_owners::FileOwners,
    pattern::{
        FilePtr, FileRegistry, GritFunctionDefinition, Matcher, Pattern, PatternDefinition,
        PredicateDefinition, ResolvedPattern, State, VariableContent,
    },
};
use grit_util::VariableMatch;
use im::vector;
use log::error;
use marzano_language::{language::Tree, target_language::TargetLanguage};
use marzano_util::{
    cache::{GritCache, NullCache},
    hasher::hash,
    node_with_source::NodeWithSource,
    rich_path::{LoadableFile, RichFile, RichPath},
    runtime::ExecutionContext,
};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use sha2::{Digest, Sha256};

use crate::api::FileMatchResult;
use std::{
    collections::HashMap,
    path::PathBuf,
    sync::mpsc::{self, Receiver, Sender},
};
use std::{fmt::Debug, str::FromStr};
use tracing::{event, Level};
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt;

#[derive(Debug)]
pub struct Problem {
    pub tree: Tree,
    pub pattern: Pattern<MarzanoQueryContext>,
    pub language: TargetLanguage,
    pub built_ins: BuiltIns,
    pub is_multifile: bool,
    pub has_limit: bool,
    pub hash: [u8; 32],
    pub name: Option<String>,
    pub(crate) variables: VariableLocations,
    pub pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,
    pub(crate) predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,
    pub(crate) function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    pub(crate) foreign_function_definitions: Vec<ForeignFunctionDefinition>,
}

impl Problem {
    pub fn compiled_vars(&self) -> Vec<VariableMatch> {
        self.variables.compiled_vars(&self.tree.source)
    }

    pub fn definitions(&self) -> StaticDefinitions<'_, MarzanoQueryContext> {
        let mut defs = StaticDefinitions::new(
            &self.pattern_definitions,
            &self.predicate_definitions,
            &self.function_definitions,
        );
        // We use the first 3 indexes for auto-wrap stuff in production
        if self.pattern_definitions.len() >= 3 {
            defs.skippable_indexes = vec![0, 1, 2];
        }
        defs
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

impl From<FilePattern> for MarzanoResolvedPattern<'_> {
    fn from(val: FilePattern) -> Self {
        match val {
            FilePattern::Single(file) => Self::from_file_pointer(file),
            FilePattern::Many(files) => Self::from_files(Self::from_list_parts(
                files.into_iter().map(Self::from_file_pointer),
            )),
        }
    }
}

fn send(tx: &Sender<Vec<MatchResult>>, value: Vec<MatchResult>) {
    if let Err(err) = tx.send(value) {
        error!("Failed to emit execution result: {}", err);
    }
}

impl Problem {
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        tree: Tree,
        pattern: Pattern<MarzanoQueryContext>,
        language: TargetLanguage,
        built_ins: BuiltIns,
        is_multifile: bool,
        has_limit: bool,
        name: Option<String>,
        variables: VariableLocations,
        pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,
        predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,
        function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,
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

    fn build_and_execute_resolved_pattern(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: Vec<impl LoadableFile>,
        context: &ExecutionContext,
        cache: &impl GritCache,
    ) {
        let owned_files = FileOwners::new();
        if !self.is_multifile && files.len() != 1 {
            let results = vec![MatchResult::AnalysisLog(AnalysisLog::floating_error(
                "Cannot build resolved pattern for single file pattern with more than one file"
                    .to_string(),
            ))];
            send(tx, results);
        }
        let mut file_pointers: Vec<FilePtr> = Vec::new();

        let mut done_files: HashMap<String, DoneFile> = HashMap::new();

        for (index, file) in files.iter().enumerate() {
            let path = file.name();
            let file_hash = hash(&path);
            if cache.has_no_matches(file_hash, self.hash) {
                done_files.insert(
                    path.clone(),
                    DoneFile {
                        relative_file_path: path,
                        has_results: Some(false),
                        file_hash: Some(file_hash),
                        from_cache: true,
                    },
                );
            } else {
                done_files.insert(
                    path.clone(),
                    DoneFile {
                        relative_file_path: path,
                        file_hash: Some(file_hash),
                        ..Default::default()
                    },
                );
                file_pointers.push(FilePtr::new(index as u16, 0));
            }
        }

        let binding: FilePattern = if self.is_multifile {
            file_pointers.into()
        } else if file_pointers.is_empty() {
            // we somehow arrived here with no files, so we return Ok
            return;
        } else {
            file_pointers[0].into()
        };

        self.execute_and_send(tx, files, binding, &owned_files, context, done_files);
    }

    fn execute_and_send(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: Vec<impl LoadableFile>,
        binding: FilePattern,
        owned_files: &FileOwners<Tree>,
        context: &ExecutionContext,
        mut done_files: HashMap<String, DoneFile>,
    ) {
        let file_names: Vec<PathBuf> = files
            .iter()
            .map(|f| PathBuf::from_str(&f.name()).unwrap())
            .collect();
        let borrowed_names: Vec<&PathBuf> = file_names.iter().collect();
        let lazy_files: Vec<Box<dyn LoadableFile>> = files
            .into_iter()
            .map(|file| Box::new(file) as Box<dyn LoadableFile>)
            .collect();

        let mut outputs =
            match self.execute(binding, lazy_files, borrowed_names, owned_files, context) {
                Result::Err(err) => file_names
                    .iter()
                    .map(|file| {
                        MatchResult::AnalysisLog(AnalysisLog::new_error(
                            err.to_string(),
                            &file.to_string_lossy(),
                        ))
                    })
                    .collect(),
                Result::Ok(messages) => {
                    // For each message, mark the DoneFile as having results
                    for message in &messages {
                        if !is_match(message) {
                            continue;
                        }
                        if let Some(name) = message.file_name() {
                            if let Ok(path) = PathBuf::from_str(name) {
                                if let Some(done_file) =
                                    done_files.get_mut(path.to_string_lossy().as_ref())
                                {
                                    done_file.has_results = Some(true);
                                }
                            }
                        }
                    }

                    messages
                }
            };

        outputs.extend(done_files.into_values().map(MatchResult::DoneFile));

        if self.is_multifile {
            // to keep snapshot tests happy, not ideal;
            outputs.sort();
        }
        send(tx, outputs);
    }

    pub fn execute_files(
        &self,
        files: Vec<RichFile>,
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

    pub fn execute_files_streaming(
        &self,
        files: Vec<RichFile>,
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
        cache: &impl GritCache,
    ) {
        self.execute_shared(files, context, tx, cache)
    }

    pub fn execute_paths<'a>(
        &self,
        files: Vec<&'a RichPath>,
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
        let files = vec![file];
        self.execute_shared(files, context, tx, &NullCache::new());
        for r in rx.iter() {
            results.extend(r)
        }
        results.sort();
        results
    }

    /// Given a vec of paths, execute the problem on each path and stream the results
    pub fn execute_paths_streaming(
        &self,
        files: Vec<PathBuf>,
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
        cache: &impl GritCache,
    ) {
        self.execute_shared(files, context, tx, cache)
    }

    /// Given an input channel and an output channel, chain the input channel to the output channel
    ///
    /// Files that match from the input channel are executed by this pattern
    /// All other message types are simply forwarded to the output channel
    ///
    pub fn execute_streaming_relay(
        &self,
        incoming_rx: Receiver<Vec<MatchResult>>,
        context: &ExecutionContext,
        outgoing_tx: Sender<Vec<MatchResult>>,
        _cache: &impl GritCache,
    ) -> Result<()> {
        if self.is_multifile {
            bail!("Streaming is not supported for multifile patterns");
        }

        #[cfg(feature = "grit_tracing")]
        let parent_span = tracing::span!(Level::INFO, "execute_shared_body",).entered();
        #[cfg(feature = "grit_tracing")]
        let parent_cx = parent_span.context();

        rayon::scope(|s| {
            #[cfg(feature = "grit_tracing")]
            let grouped_ctx = parent_cx;

            s.spawn(move |_| {
                #[cfg(feature = "grit_tracing")]
                let task_span = tracing::info_span!("apply_file_inner").entered();
                #[cfg(feature = "grit_tracing")]
                task_span.set_parent(grouped_ctx);

                event!(Level::INFO, "spawn execute_shared_body");

                incoming_rx.iter().for_each(|res| {
                    let mut paths = Vec::new();

                    for m in res.into_iter() {
                        match m {
                            MatchResult::Match(m) => {
                                paths.push(PathBuf::from(m.file_name()));
                            }
                            MatchResult::PatternInfo(_)
                            | MatchResult::AllDone(_)
                            | MatchResult::InputFile(_)
                            | MatchResult::AnalysisLog(_)
                            | MatchResult::DoneFile(_) => {
                                outgoing_tx.send(vec![m]).unwrap();
                            }
                            MatchResult::Rewrite(_)
                            | MatchResult::CreateFile(_)
                            | MatchResult::RemoveFile(_) => {
                                outgoing_tx
                                    .send(vec![
                                    m,
                                    MatchResult::AnalysisLog(AnalysisLog::floating_error(
                                        "Streaming does not support rewrites, creates, or removes"
                                            .to_string(),
                                    )),
                                ])
                                    .unwrap();
                            }
                        }
                    }
                    self.execute_shared(paths, context, outgoing_tx.clone(), &NullCache::new());
                });
            })
        });

        Ok(())
    }

    #[cfg_attr(feature = "grit_tracing", tracing::instrument(skip_all))]
    pub(crate) fn execute_shared(
        &self,
        files: Vec<impl LoadableFile + Send + Sync>,
        context: &ExecutionContext,
        tx: Sender<Vec<MatchResult>>,
        cache: &impl GritCache,
    ) {
        #[cfg(feature = "grit_tracing")]
        let parent_span = tracing::span!(Level::INFO, "execute_shared_body",).entered();
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

                    files.into_par_iter().for_each_with(tx, |sender, f| {
                        let vec = vec![f];
                        self.build_and_execute_resolved_pattern(sender, vec, context, cache);
                    });
                })
            })
        }
    }

    /// Construct a context, only for testing
    pub fn get_context<'a>(
        &'a self,
        context: &'a ExecutionContext,
        owned_files: &'a FileOwners<Tree>,
    ) -> (State<MarzanoQueryContext>, MarzanoContext<'a>) {
        let file_registry: FileRegistry<MarzanoQueryContext> = FileRegistry::new_from_paths(vec![]);

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
        let state = State::new(bindings, file_registry);

        (
            state,
            MarzanoContext::new(
                &self.pattern_definitions,
                &self.predicate_definitions,
                &self.function_definitions,
                &self.foreign_function_definitions,
                vec![],
                owned_files,
                &self.built_ins,
                &self.language,
                context,
                self.name.clone(),
            ),
        )
    }

    fn execute<'a>(
        &self,
        binding: FilePattern,
        files: Vec<Box<dyn LoadableFile + 'a>>,
        file_names: Vec<&PathBuf>,
        owned_files: &FileOwners<Tree>,
        context: &ExecutionContext,
    ) -> Result<Vec<MatchResult>> {
        let mut user_logs = vec![].into();

        let lazy_files = files;

        let context = MarzanoContext::new(
            &self.pattern_definitions,
            &self.predicate_definitions,
            &self.function_definitions,
            &self.foreign_function_definitions,
            lazy_files,
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

        let file_registry = FileRegistry::new_from_paths(file_names);
        let mut state = State::new(bindings, file_registry);

        let the_new_files =
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[NEW_FILES_INDEX].as_mut();
        the_new_files.value = Some(MarzanoResolvedPattern::from_list_parts([].into_iter()));

        let mut results: Vec<MatchResult> = Vec::new();
        let binding = binding.into();
        if self
            .pattern
            .execute(&binding, &mut state, &context, &mut user_logs)?
        {
            for file in state.files.files() {
                if let Some(result) = MatchResult::file_to_match_result(file)? {
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

#[derive(Clone, Debug, PartialEq)]
pub struct MarzanoQueryContext;

impl QueryContext for MarzanoQueryContext {
    type Node<'a> = NodeWithSource<'a>;
    type NodePattern = ASTNode;
    type LeafNodePattern = AstLeafNode;
    type ExecContext<'a> = MarzanoContext<'a>;
    type Binding<'a> = MarzanoBinding<'a>;
    type CodeSnippet = MarzanoCodeSnippet;
    type ResolvedPattern<'a> = MarzanoResolvedPattern<'a>;
    type Language<'a> = TargetLanguage;
    type File<'a> = MarzanoFile<'a>;
    type Tree<'a> = Tree;
}
