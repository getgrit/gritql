use crate::{
    api::{is_match, AnalysisLog, DoneFile, MatchResult},
    ast_node::{ASTNode, AstLeafNode},
    built_in_functions::BuiltIns,
    constants::MAX_FILE_SIZE,
    foreign_function_definition::ForeignFunctionDefinition,
    marzano_binding::MarzanoBinding,
    marzano_code_snippet::MarzanoCodeSnippet,
    marzano_context::MarzanoContext,
    marzano_resolved_pattern::{MarzanoFile, MarzanoResolvedPattern},
    pattern_compiler::{compiler::VariableLocations, file_owner_compiler::FileOwnerCompiler},
};
use anyhow::{bail, Result};
use grit_pattern_matcher::{
    constants::{GLOBAL_VARS_SCOPE_INDEX, NEW_FILES_INDEX},
    context::QueryContext,
    file_owners::{FileOwner, FileOwners},
    pattern::{
        FilePtr, FileRegistry, GritFunctionDefinition, Matcher, Pattern, PatternDefinition,
        PredicateDefinition, ResolvedPattern, State, VariableContent,
    },
};
use grit_util::{AnalysisLogs, Position, VariableMatch};
use im::vector;
use log::error;
use marzano_language::{language::Tree, target_language::TargetLanguage};
use marzano_util::{
    cache::{GritCache, NullCache},
    hasher::hash,
    node_with_source::NodeWithSource,
    rich_path::{FileName, MarzanoFileTrait, RichFile, RichPath, TryIntoInputFile},
    runtime::ExecutionContext,
};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use sha2::{Digest, Sha256};
use std::{
    borrow::Cow,
    path::PathBuf,
    sync::mpsc::{self, Sender},
};
use std::{fmt::Debug, str::FromStr};
use tracing::{event, Level};

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
    pub(crate) pattern_definitions: Vec<PatternDefinition<MarzanoQueryContext>>,
    pub(crate) predicate_definitions: Vec<PredicateDefinition<MarzanoQueryContext>>,
    pub(crate) function_definitions: Vec<GritFunctionDefinition<MarzanoQueryContext>>,
    pub(crate) foreign_function_definitions: Vec<ForeignFunctionDefinition>,
}

impl Problem {
    pub fn compiled_vars(&self) -> Vec<VariableMatch> {
        self.variables.compiled_vars(&self.tree.source)
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

struct FilePatternOutput {
    file_pattern: Option<FilePattern>,
    file_owners: FileOwners<Tree>,
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

    fn build_and_execute_resolved_pattern_internal(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: &[impl MarzanoFileTrait],
        context: &ExecutionContext,
        cache: &impl GritCache,
    ) -> Result<()> {
        let owned_files = FileOwners::new();
        let mut results = vec![];
        let mut file_pointers = vec![];
        let mut done_files = vec![];
        if !self.is_multifile && files.len() != 1 {
            bail!("Cannot build resolved pattern for single file pattern with more than one file")
        }
        for file in files {
            let path = file.name();
            // let file: Cow<RichFile> = match file.try_into_cow() {
            //     Result::Ok(file) => file,
            //     Result::Err(err) => {
            //         results.push(MatchResult::AnalysisLog(AnalysisLog::new_error(
            //             err.to_string(),
            //             &file.name(),
            //         )));
            //         continue;
            //     }
            // };
            // if let Some(log) = is_file_too_big(&file) {
            //     results.push(MatchResult::AnalysisLog(log));
            //     results.push(MatchResult::DoneFile(DoneFile {
            //         relative_file_path: file.path.to_string(),
            //         // Don't know if there are results, so we can't cache
            //         ..Default::default()
            //     }))
            // } else {
            //     let file_hash = hash(&file.path);
            //     if cache.has_no_matches(file_hash, self.hash) {
            //         results.push(MatchResult::DoneFile(DoneFile {
            //             relative_file_path: file.path.to_string(),
            //             has_results: Some(false),
            //             file_hash: Some(file_hash),
            //             from_cache: true,
            //         }));
            //     } else {
            let mut logs: AnalysisLogs = vec![].into();
            // let owned_file = FileOwnerCompiler::from_matches(
            //     file.path.to_owned(),
            //     file.content.to_owned(),
            //     None,
            //     false,
            //     &self.language,
            //     &mut logs,
            // );
            results.extend(
                logs.logs()
                    .into_iter()
                    .map(|l| MatchResult::AnalysisLog(l.into())),
            );
            // match owned_file {
            //     Result::Ok(owned_file) => {
            // if let Some(owned_file) = owned_file {
            file_pointers.push(FilePtr::new(file_pointers.len() as u16, 0));
            // owned_files.push(owned_file);
            // }
            //         done_files.push(MatchResult::DoneFile(DoneFile {
            //             relative_file_path: path.to_string(),
            //             has_results: None,
            //             // file_hash: Some(file_hash),
            //             file_hash: None,
            //             from_cache: false,
            //         }))
            //     }
            //     Result::Err(err) => {
            //         results.push(MatchResult::AnalysisLog(AnalysisLog::new_error(
            //             err.to_string(),
            //             &path,
            //         )));
            //         results.push(MatchResult::DoneFile(DoneFile {
            //             relative_file_path: path.to_string(),
            //             ..Default::default()
            //         }))
            //     }
            // }
            //         }
            //     }
        }
        let binding: FilePattern = if self.is_multifile {
            file_pointers.into()
        } else if file_pointers.is_empty() {
            send(tx, results);
            // single file pattern had file that was too big
            return Ok(());
        } else {
            file_pointers[0].into()
        };

        //         file_pattern: Option<FilePattern>,
        // file_owners: FileOwners<Tree>,
        // done_files: Vec<MatchResult>,
        // error_files: Vec<MatchResult>,

        send(tx, results);
        self.execute_and_send(tx, files, binding, &owned_files, context, done_files);

        Ok(())
    }

    fn execute_and_send(
        &self,
        tx: &Sender<Vec<MatchResult>>,
        files: &[impl MarzanoFileTrait],
        binding: FilePattern,
        owned_files: &FileOwners<Tree>,
        context: &ExecutionContext,
        mut done_files: Vec<MatchResult>,
    ) {
        let file_names: Vec<PathBuf> = files
            .iter()
            .map(|f| PathBuf::from_str(&f.name()).unwrap())
            .collect();
        let borrowed_names: Vec<&PathBuf> = file_names.iter().collect();

        let mut outputs = match self.execute(binding, files, borrowed_names, owned_files, context) {
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
        files: &[impl MarzanoFileTrait],
        context: &ExecutionContext,
        cache: &impl GritCache,
    ) {
        match self.build_and_execute_resolved_pattern_internal(tx, files, context, cache) {
            Result::Ok(_) => {}
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
    pub(crate) fn execute_shared(
        &self,
        files: &[impl MarzanoFileTrait],
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

    fn execute<'a>(
        &self,
        binding: FilePattern,
        files: &[impl MarzanoFileTrait + 'a],
        file_names: Vec<&PathBuf>,
        owned_files: &FileOwners<Tree>,
        context: &ExecutionContext,
    ) -> Result<Vec<MatchResult>> {
        let mut user_logs = vec![].into();

        let lazy_files: Vec<Box<dyn TryIntoInputFile + 'static>> = files
            .iter()
            .map(|f| unsafe {
                std::mem::transmute::<
                    Box<dyn TryIntoInputFile + Send + Sync + 'a>,
                    Box<dyn TryIntoInputFile + 'static>,
                >(Box::new(f.clone()))
            })
            .collect();

        let context = MarzanoContext::new(
            &self.pattern_definitions,
            &self.predicate_definitions,
            &self.function_definitions,
            &self.foreign_function_definitions,
            &lazy_files,
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
    type Tree = Tree;
}
