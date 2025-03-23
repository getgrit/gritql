use anyhow::bail;
use grit_pattern_matcher::pattern::{DynamicPattern, Pattern, StringConstant};
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
    sync::mpsc::{self, Receiver},
};
use tokio::task::JoinSet;

use grit_pattern_matcher::pattern::State;

use marzano_core::{
    api::{is_match, DoneFile, MatchResult},
    marzano_context::MarzanoContext,
    marzano_resolved_pattern::MarzanoResolvedPattern,
    pattern_compiler::{CompilationResult, CompiledPatternBuilder},
    problem::{MarzanoQueryContext, Problem},
};
use marzano_gritmodule::{
    config::{init_config_from_path, init_global_grit_modules},
    fetcher::KeepFetcherKind,
    resolver::find_and_resolve_grit_dir,
    utils::{extract_path, infer_pattern},
};
use napi::{
    bindgen_prelude::*,
    threadsafe_function::{ErrorStrategy, ThreadsafeFunction},
    tokio::{self},
};

use marzano_language::{
    grit_parser::MarzanoGritParser,
    target_language::{expand_paths, PatternLanguage},
};
use marzano_messenger::emit::{FlushableMessenger, Messager};
use marzano_messenger::testing::TestingMessenger;
use marzano_util::{
    cache::NullCache,
    rich_path::RichFile,
    runtime::{ExecutionContext, LanguageModelAPI},
};

use crate::binding::JsResolvedBinding;

type MatchCallback = ThreadsafeFunction<JsResolvedBinding, ErrorStrategy::Fatal>;

#[napi]
#[derive(Clone)]
pub struct QueryBuilder {
    base_query_src: String,
    filters: Vec<MatchCallback>,
    piped_queries: Vec<QueryBuilder>,
    replacement: Option<String>,
    insertions: Vec<String>,
}

#[napi(string_enum)]
#[derive(Default)]
pub enum SearchOutputOptions {
    /// Emit absolute paths
    #[default]
    AbsolutePaths,
    /// Emit paths relative to the provided search/root directory
    RelativePaths,
}

#[napi(object)]
pub struct SearchOptions {
    pub target_paths: Vec<String>,
    pub step_id: String,
    /// Skip emitting any results and applying changes
    pub silent: Option<bool>,
    /// The format of the output paths
    pub output_format: Option<SearchOutputOptions>,
    /// Verify the query without applying any changes
    pub verify_only: Option<bool>,
}

#[napi(object)]
pub struct InputFile {
    pub path: String,
    pub content: String,
}

impl From<InputFile> for RichFile {
    fn from(file: InputFile) -> Self {
        RichFile::new(file.path, file.content)
    }
}

#[napi(object)]
pub struct InputOptions {
    pub files: Vec<InputFile>,
}

#[napi]
impl QueryBuilder {
    /// Construct a new query, starting from a GritQL query
    #[napi(constructor)]
    pub fn new(base_query_src: String) -> Self {
        Self {
            base_query_src,
            filters: Vec::new(),
            piped_queries: Vec::new(),
            replacement: None,
            insertions: Vec::new(),
        }
    }

    /// Add a callback to filter the results of the search
    ///
    /// @param filter A callback that that will be called with each match, it should return true to keep the match
    #[napi(js_name = "filter")]
    pub fn add_filter(&mut self, filter: MatchCallback) {
        self.filters.push(filter);
    }

    /// Set a replacement rewrite to the query
    ///
    /// @param replacement The replacement rewrite to apply to the query
    #[napi]
    pub fn set_replacement(&mut self, replacement: String) {
        self.replacement = Some(replacement);
    }

    /// Add an insertion to the query
    ///
    /// @param insertion The insertion to add to the query
    #[napi]
    pub fn add_insertion(&mut self, insertion: String) {
        self.insertions.push(insertion);
    }

    /// Pipe the query into another query
    /// When you pipe a query, it will stop directly emitting results itself.
    /// Instead, every *matching* file will be passed to the next query.
    /// @param query The query to pipe into
    #[napi]
    pub fn pipe(&mut self, second_query: &QueryBuilder) {
        self.piped_queries.push(second_query.clone());
    }

    /// Run the query on the provided files
    /// @param target_paths A list of paths to search for matches
    /// @returns A list of files that matched the query
    #[napi]
    pub async fn run(&self, options: SearchOptions) -> Result<Vec<String>> {
        if options.target_paths.is_empty() {
            return Ok(vec![]);
        }
        search_internal(self, options)
            .await
            .map_err(|e| napi::Error::from_reason(format!("Error: {:?}", e)))
            .map(|paths| paths.into_iter().collect())
    }

    /// Apply the query to a single file and return the modified file (if any)
    /// @param file The file to apply the query to
    /// @returns The modified file (if any)
    #[napi]
    pub async fn apply_to_file(&self, file: InputFile) -> Result<Option<InputFile>> {
        let results = apply_internal(self, InputOptions { files: vec![file] })
            .await
            .map_err(|e| napi::Error::from_reason(format!("Error: {:?}", e)))?;
        let rewrite = results
            .into_iter()
            .find(|r| matches!(r, MatchResult::Rewrite(_)));
        match rewrite {
            Some(MatchResult::Rewrite(rewrite)) => Ok(Some(InputFile {
                path: rewrite.rewritten.source_file,
                content: rewrite.rewritten.content.unwrap_or_default(),
            })),
            _ => Ok(None),
        }
    }
}

async fn prep_query(
    query: QueryBuilder,
    target_paths: Vec<String>,
) -> anyhow::Result<(Problem, Vec<QueryBuilder>)> {
    let QueryBuilder {
        base_query_src,
        filters,
        piped_queries,
        replacement,
        insertions,
    } = query;

    let grit_files = if target_paths.is_empty() {
        let init = init_global_grit_modules::<KeepFetcherKind>(None).await?;
        init.get_grit_files().await?
    } else {
        let path = PathBuf::from(target_paths.first().unwrap());
        init_config_from_path::<KeepFetcherKind>(path.clone(), false).await?;
        find_and_resolve_grit_dir(Some(path), None).await?
    };

    let (lang, _, pattern_body) = infer_pattern(&base_query_src, &grit_files);

    let pattern_language = match lang {
        Some(l) => l,
        None => PatternLanguage::default(),
    };

    let target_lang = pattern_language.try_into()?;

    let mut grit_parser = MarzanoGritParser::new()?;

    let libs = grit_files.get_language_directory_or_default(lang)?;

    let injected_builtins = Some(ai_builtins::ai_builtins::get_ai_built_in_functions());

    let mut builder = CompiledPatternBuilder::start(
        pattern_body,
        &libs,
        target_lang,
        None,
        &mut grit_parser,
        injected_builtins,
    )?;

    for filter in filters {
        builder = builder.matches_callback(Box::new(move |binding, context, state, _logs| {
            let runtime = context
                .runtime
                .handle
                .as_ref()
                .ok_or(anyhow::anyhow!("Async runtime required"))?;

            /*
            THIS IS INHERENTLY UNSAFE
            We cannot guarantee that the references live for the lifetime of the JavaScript object
            This works for super basic cases around callbacks, but more advanced use cases will require
            a more complex solution
            */

            let inner_context: &'static MarzanoContext = unsafe { std::mem::transmute(context) };
            let inner_state: &'static mut State<MarzanoQueryContext> =
                unsafe { std::mem::transmute(state) };

            let inner_binding: &'static MarzanoResolvedPattern =
                unsafe { std::mem::transmute(binding) };

            let js_binding = JsResolvedBinding {
                inner: inner_binding,
                context: inner_context,
                state: inner_state,
            };

            let val = runtime.block_on(async { filter.call_async::<bool>(js_binding).await })?;
            Ok(val)
        }));
    }

    for insertion in insertions {
        let insertion = Pattern::StringConstant(StringConstant::new(insertion));
        builder = builder.wrap_with_accumulate(insertion);
    }

    if let Some(replacement) = replacement {
        let replacement = DynamicPattern::from_str_constant(&replacement)?;
        builder = builder.wrap_with_rewrite(replacement);
    }

    let CompilationResult {
        problem,
        compilation_warnings,
    } = builder.compile(None, None, true)?;

    if !compilation_warnings.is_empty() {
        println!("Warnings: {:?}", compilation_warnings);
    }

    Ok((problem, piped_queries))
}

#[derive(Debug)]
struct PreppedQuery {
    pattern: Problem,
    children: Vec<PreppedQuery>,
}

/// Currently only 2 levels of queries are supported
async fn prep_queries(
    root: &QueryBuilder,
    target_paths: Vec<String>,
) -> anyhow::Result<PreppedQuery> {
    let (root_pattern, root_child_queries) = prep_query(root.clone(), target_paths.clone()).await?;
    let mut root_query = PreppedQuery {
        pattern: root_pattern,
        children: Vec::new(),
    };

    for child_query in root_child_queries {
        let (pattern, child_queries) = prep_query(child_query, target_paths.clone()).await?;
        let child_query = PreppedQuery {
            pattern,
            children: Vec::new(),
        };

        if !child_queries.is_empty() {
            bail!("Nested query pipes are not currently supported");
        }

        root_query.children.push(child_query);
    }

    Ok(root_query)
}

enum EmbeddedMessenger {
    Testing(TestingMessenger),
    Relay(RelayMessenger),
}

impl EmbeddedMessenger {
    fn emit(&mut self, message: &MatchResult) -> anyhow::Result<()> {
        match self {
            EmbeddedMessenger::Testing(messenger) => messenger.emit(message),
            EmbeddedMessenger::Relay(messenger) => messenger.emit(message),
        }
    }

    async fn flush(&mut self) -> anyhow::Result<()> {
        match self {
            EmbeddedMessenger::Testing(messenger) => messenger.flush().await,
            EmbeddedMessenger::Relay(messenger) => messenger.flush().await,
        }
    }

    fn emit_estimate(&mut self, total: usize) -> anyhow::Result<()> {
        match self {
            EmbeddedMessenger::Testing(messenger) => messenger.emit_estimate(total),
            EmbeddedMessenger::Relay(messenger) => messenger.emit_estimate(total),
        }
    }
}

fn get_messenger(root_address: &Path, step_id: String, silent: bool) -> EmbeddedMessenger {
    if silent {
        return EmbeddedMessenger::Testing(TestingMessenger::new());
    }

    match std::env::var("GRIT_LOCAL_SERVER") {
        Ok(server_addr) => EmbeddedMessenger::Relay(RelayMessenger::new(
            server_addr,
            Some(root_address.into()),
            step_id,
        )),
        Err(_) => EmbeddedMessenger::Testing(TestingMessenger::new()),
    }
}

async fn apply_internal(
    query: &QueryBuilder,
    options: InputOptions,
) -> anyhow::Result<Vec<MatchResult>> {
    let root_query = prep_queries(query, vec![]).await?;
    let context = ExecutionContext::default();

    if !root_query.children.is_empty() {
        bail!("Nested query pipes are not currently supported for .apply()");
    }

    let files: Vec<RichFile> = options.files.into_iter().map(|f| f.into()).collect();
    let results = root_query.pattern.execute_files(files, &context);

    Ok(results)
}

async fn search_internal(
    query: &QueryBuilder,
    options: SearchOptions,
) -> anyhow::Result<HashSet<String>> {
    let SearchOptions {
        target_paths,
        step_id,
        silent,
        output_format,
        verify_only,
    } = options;

    let silent = silent.unwrap_or(false);

    let root_query = prep_queries(query, target_paths.clone()).await?;

    if verify_only.unwrap_or(false) {
        return Ok(HashSet::new());
    }

    let root_path = PathBuf::from(target_paths.first().unwrap());

    let mut final_handles = JoinSet::new();
    let mut stack: Vec<(PreppedQuery, Option<Receiver<Vec<MatchResult>>>)> = Vec::new();

    // Push the root query onto the stack
    stack.push((root_query, None));

    while let Some((current_query, parent_rx)) = stack.pop() {
        let (current_tx, current_rx) = mpsc::channel::<Vec<MatchResult>>();

        if !current_query.children.is_empty() {
            let child_channels = current_query
                .children
                .iter()
                .map(|_| mpsc::channel::<Vec<MatchResult>>())
                .collect::<Vec<_>>();

            let (child_txs, child_rxs): (Vec<_>, Vec<_>) = child_channels.into_iter().unzip();

            for (child_query, child_rx) in current_query.children.into_iter().zip(child_rxs) {
                stack.push((child_query, Some(child_rx)));
            }

            tokio::spawn(async move {
                for matches in current_rx {
                    for tx in &child_txs {
                        match tx.send(matches.clone()) {
                            Ok(_) => {}
                            Err(e) => {
                                println!("Error sending to child: {:?}", e.to_string());
                            }
                        }
                    }
                }
            });
        } else {
            let path = root_path.clone();
            let step_id = step_id.clone();
            let root_path_str = root_path.to_string_lossy().to_string();
            final_handles.spawn(async move {
                let mut paths = HashSet::new();

                let mut messenger = get_messenger(&path, step_id, silent);

                for results in current_rx {
                    for match_result in results {
                        if is_match(&match_result) {
                            if let Some(path) = extract_path(&match_result) {
                                if matches!(output_format, Some(SearchOutputOptions::RelativePaths))
                                {
                                    paths.insert(
                                        path.strip_prefix(&root_path_str)
                                            .map(|p| format!(".{}", p))
                                            .unwrap_or(path.to_string()),
                                    );
                                } else {
                                    paths.insert(path.to_string());
                                }
                            }
                        }
                        messenger.emit(&match_result).unwrap();
                    }
                }

                (paths, messenger)
            });
        }

        let context = ExecutionContext::default()

        if let Some(parent_rx) = parent_rx {
            current_query
                .pattern
                .execute_streaming_relay(parent_rx, &context, current_tx, &NullCache::new())
                .unwrap();
        } else {
            let start_paths = target_paths.iter().map(PathBuf::from).collect::<Vec<_>>();
            let file_walker = expand_paths(
                &start_paths,
                Some(&[current_query.pattern.language.to_module_language()]),
            )
            .unwrap();

            let mut language_paths = Vec::new();
            for file in file_walker {
                let Ok(file) = file else {
                    continue;
                };
                let Some(file_type) = file.file_type() else {
                    continue;
                };
                if file_type.is_dir() {
                    continue;
                }
                if !&current_query.pattern.language.match_extension(
                    file.path()
                        .extension()
                        .unwrap_or_default()
                        .to_str()
                        .unwrap_or_default(),
                ) {
                    let done_file = MatchResult::DoneFile(DoneFile {
                        relative_file_path: file.path().to_string_lossy().to_string(),
                        has_results: Some(false),
                        file_hash: None,
                        from_cache: false,
                    });
                    if let Err(e) = current_tx.send(vec![done_file]) {
                        println!("Error sending to parent: {:?}", e.to_string());
                    }
                    continue;
                }
                let path = file.path();
                language_paths.push(path.to_path_buf());
            }

            let mut messenger = get_messenger(&root_path, step_id.clone(), silent);
            messenger.emit_estimate(language_paths.len()).unwrap();
            // If we don't flush, we risk not sending the estimate
            messenger.flush().await.unwrap();

            current_query.pattern.execute_paths_streaming(
                language_paths,
                &context,
                current_tx,
                &NullCache::new(),
            );
        }
    }

    let mut all_paths = HashSet::new();

    while let Some(res) = final_handles.join_next().await {
        let (this_set, mut messenger) = res?;
        all_paths.extend(this_set);

        messenger.flush().await?;
    }

    Ok(all_paths)
}
