use anyhow::Result;

use std::collections::BTreeMap;
use std::sync::atomic::AtomicI32;

#[cfg(feature = "grit_tracing")]
use tracing::span;
use tracing::{event, instrument, Level};
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use grit_cache::paths::cache_for_cwd;
use ignore::Walk;
use indicatif::{MultiProgress, ProgressBar, ProgressDrawTarget, ProgressStyle};

#[allow(unused_imports)]
use marzano_core::pattern::built_in_functions::BuiltIns;
use marzano_core::pattern_compiler::{src_to_problem_libs, CompilationResult};
use marzano_core::{
    api::{AnalysisLog, DoneFile, MatchResult},
    problem::Problem,
};
use marzano_language::target_language::PatternLanguage;
use marzano_util::cache::GritCache;
use marzano_util::position::{FileRange, Position};
use marzano_util::runtime::ExecutionContext;

use std::collections::HashMap;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::Ordering;
use std::sync::mpsc;
use std::sync::mpsc::channel;

use crate::commands::apply_pattern::ApplyInput;
use crate::commands::apply_pattern::ApplyPatternArgs;
use crate::flags::OutputFormat;
use crate::resolver::RichPattern;
use crate::ux::CheckResult;
use marzano_messenger::emit::{ApplyDetails, Messager};

impl<'b> RichPattern<'b> {
    #[instrument(skip(self, pattern_libs, targets))]
    pub fn compile(
        &self,
        pattern_libs: &BTreeMap<String, String>,
        language: Option<PatternLanguage>,
        targets: Option<Vec<FileRange>>,
        injected_limit: Option<usize>,
    ) -> Result<CompilationResult> {
        let lang = language.unwrap_or_default();
        #[cfg(not(feature = "ai_builtins"))]
        let injected_builtins: Option<BuiltIns> = None;
        #[cfg(feature = "ai_builtins")]
        let injected_builtins = Some(ai_builtins::ai_builtins::get_ai_built_in_functions());

        src_to_problem_libs(
            self.body.to_owned(),
            pattern_libs,
            lang.try_into().unwrap(),
            self.name.to_owned(),
            targets,
            injected_builtins,
            None,
        )
    }
}

pub fn extract_rewritten_content(result: &MatchResult) -> Option<&String> {
    match result {
        MatchResult::AnalysisLog(_) => None,
        MatchResult::Match(_) => None,
        MatchResult::InputFile(_) => None,
        MatchResult::CreateFile(c) => Some(&c.rewritten.content),
        MatchResult::RemoveFile(_) => None,
        MatchResult::Rewrite(r) => Some(&r.rewritten.content),
        MatchResult::DoneFile(_) => None,
        MatchResult::AllDone(_) => None,
        MatchResult::PatternInfo(_) => None,
    }
}

// Groups checks by pattern name
// We use a BTreeMap to ensure consistent ordering
pub fn group_checks<'a>(
    results: &[&'a CheckResult<'a>],
) -> BTreeMap<String, Vec<&'a CheckResult<'a>>> {
    let mut grouped_results: BTreeMap<String, Vec<&CheckResult>> = BTreeMap::new();

    for result in results.iter() {
        let key = result.pattern.local_name.clone();
        grouped_results.entry(key).or_default().push(result);
    }

    grouped_results
}

fn create_style_template(is_spinner: bool, arg: &ApplyPatternArgs) -> Result<ProgressStyle> {
    let style_template = match (is_spinner, arg.interactive) {
        // If we are in interactive mode, we need an extra newline to leave room for user input
        (false, true) => "\n\n{prefix:.bold.dim} {wide_msg:.bold.dim}\n{wide_bar} {pos:}/{len}",
        (false, false) => "\n{prefix:.bold.dim} {wide_msg:.bold.dim}\n{wide_bar} {pos:}/{len}",
        (true, true) => "\n\n{spinner}{prefix:.bold.dim} {wide_msg:.bold.dim}",
        (true, false) => "\n{spinner}{prefix:.bold.dim} {wide_msg:.bold.dim}",
    };
    let progress_style = ProgressStyle::with_template(style_template)?;
    Ok(progress_style)
}

fn create_apply_progress(multi: MultiProgress, format: &OutputFormat) -> Option<ProgressBar> {
    let bar = match format {
        OutputFormat::Jsonl => Some(ProgressBar::hidden()),
        _ => Some(ProgressBar::with_draw_target(
            Some(0),
            ProgressDrawTarget::stderr(),
        )),
    };
    bar.map(|bar| multi.add(bar))
}

macro_rules! emit_error {
    ($emitter:expr, $min_level:expr, $expr:expr) => {
        match $expr {
            Ok(r) => r,
            Err(my_err) => {
                let log = MatchResult::AnalysisLog(AnalysisLog {
                    level: 100,
                    message: my_err.to_string(),
                    position: Position::first(),
                    file: "PlaygroundPattern".to_string(),
                    engine_id: "marzano".to_string(),
                    syntax_tree: None,
                    range: None,
                    source: None,
                });
                $emitter.emit(&log, $min_level).unwrap();
                return $emitter;
            }
        }
    };
}

#[allow(clippy::too_many_arguments)]
pub async fn par_apply_pattern<M>(
    file_walker: Walk,
    multi: MultiProgress,
    compiled: Problem,
    my_input: &ApplyInput,
    mut owned_emitter: M,
    processed: &AtomicI32,
    details: &mut ApplyDetails,
    arg: &ApplyPatternArgs,
    context: &ExecutionContext,
    format: &OutputFormat,
) -> M
where
    M: Messager,
{
    #[cfg(feature = "grit_tracing")]
    let parent_span = span!(Level::INFO, "parallel_apply_body",).entered();
    #[cfg(feature = "grit_tracing")]
    let parent_cx = parent_span.context();

    let emitter = &mut owned_emitter;

    event!(Level::INFO, "Begin applying pattern");

    let bar = create_apply_progress(multi, format);
    let pg = bar.as_ref();

    // To start, it is a spinner
    if let Some(pg) = pg {
        pg.set_style(emit_error!(
            owned_emitter,
            &arg.visibility,
            create_style_template(true, arg)
        ));
        pg.set_message("Finding files");
    }

    #[cfg(feature = "grit_timing")]
    let current_timer = std::time::Instant::now();

    let should_cache = arg.cache && !compiled.is_multifile && !compiled.has_limit;
    let (cache, manager) = emit_error!(
        owned_emitter,
        &arg.visibility,
        cache_for_cwd(arg.refresh_cache, !should_cache).await
    );
    let cache_ref = &cache;

    let mut interactive = arg.interactive;
    let min_level = &arg.visibility;

    let (file_paths_tx, file_paths_rx) = channel();

    for file in file_walker {
        let file = emit_error!(owned_emitter, &arg.visibility, file);
        if file.file_type().unwrap().is_dir() {
            continue;
        }
        if !&compiled.language.match_extension(
            file.path()
                .extension()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default(),
        ) {
            processed.fetch_add(1, Ordering::SeqCst);
            let path_string = file.path().to_string_lossy().to_string();
            if my_input.paths.contains(&file.path().to_path_buf()) {
                let log = MatchResult::AnalysisLog(AnalysisLog {
                    level: 410,
                    message: format!(
                        "Skipped {} since it is not a {} file",
                        path_string,
                        &compiled.language.to_string()
                    ),
                    position: Position::first(),
                    file: path_string.to_string(),
                    engine_id: "marzano".to_string(),
                    range: None,
                    syntax_tree: None,
                    source: None,
                });
                let done_file = MatchResult::DoneFile(DoneFile {
                    relative_file_path: path_string,
                    has_results: Some(false),
                    file_hash: None,
                    from_cache: false,
                });
                emitter.handle_results(
                    vec![log, done_file],
                    details,
                    arg.dry_run,
                    min_level,
                    arg.format,
                    &mut interactive,
                    None,
                    Some(processed),
                    None,
                    &compiled.language,
                );
            }
            continue;
        }
        file_paths_tx.send(file.path().to_path_buf()).unwrap();
    }

    drop(file_paths_tx);

    let found_paths = file_paths_rx.iter().collect::<Vec<_>>();
    let found_count = found_paths.len();
    if let Some(pg) = pg {
        pg.set_length(found_count.try_into().unwrap());
    }
    emitter.emit_estimate(found_count).unwrap();

    #[cfg(feature = "grit_timing")]
    debug!(
        "Walked {} files in {}ms",
        found_paths.len(),
        current_timer.elapsed().as_millis()
    );

    // Transition to a progress bar
    if let Some(pg) = pg {
        pg.set_style(emit_error!(
            owned_emitter,
            &arg.visibility,
            create_style_template(false, arg)
        ));
        pg.set_prefix("Analyzing");
    }

    let (tx, rx) = mpsc::channel::<Vec<MatchResult>>();

    let should_continue = &AtomicBool::new(true);
    let compiled_language = &compiled.language;

    rayon::scope(|s| {
        #[cfg(feature = "grit_tracing")]
        let grouped_ctx = parent_cx;

        s.spawn(move |_| {
            let mut parse_errors: HashMap<String, usize> = HashMap::new();
            for message in rx {
                if cache_ref.is_useful() {
                    for res in message.iter() {
                        if let MatchResult::DoneFile(done_file) = res {
                            let Some(path_hash) = done_file.file_hash else {
                                continue;
                            };
                            if Some(false) == done_file.has_results && !done_file.from_cache {
                                cache_ref.put_no_matches(path_hash, compiled.hash).unwrap();
                            }
                        }
                    }
                }
                let user_decision = emitter.handle_results(
                    message,
                    details,
                    arg.dry_run,
                    min_level,
                    arg.format,
                    &mut interactive,
                    pg,
                    Some(processed),
                    Some(&mut parse_errors),
                    compiled_language,
                );

                if !user_decision {
                    should_continue.store(false, Ordering::SeqCst);
                    break;
                }

                if !should_continue.load(Ordering::SeqCst) {
                    break;
                }
            }
        });

        let task_span = tracing::info_span!("apply_file_one_streaming").entered();
        #[cfg(feature = "grit_tracing")]
        task_span.set_parent(grouped_ctx);
        task_span.in_scope(|| {
            compiled.execute_paths_streaming(&found_paths, context, tx, cache_ref);
            loop {
                if processed.load(Ordering::SeqCst) >= found_count.try_into().unwrap()
                    || !should_continue.load(Ordering::SeqCst)
                {
                    break;
                }
            }
        });

        if let Some(pg) = pg {
            pg.finish_and_clear();
        }
        event!(Level::INFO, "finish the main loop");
    });

    drop(cache);

    if let Some(manager) = manager {
        match manager.join() {
            Ok(_) => {}
            Err(e) => {
                event!(Level::ERROR, "Error joining cache manager: {:?}", e);
            }
        }
    }

    owned_emitter
}
