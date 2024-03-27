use anyhow::{bail, Result};
use clap::Args;
use dialoguer::Confirm;

use tracing::instrument;
#[cfg(feature = "grit_tracing")]
use tracing::span;
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use indicatif::MultiProgress;
use log::debug;
use marzano_core::pattern::{
    api::{AllDone, AllDoneReason, AnalysisLog, MatchResult},
    compiler::CompilationResult,
};
use marzano_gritmodule::fetcher::KeepFetcherKind;
use marzano_gritmodule::markdown::get_body_from_md_content;
use marzano_gritmodule::searcher::find_grit_modules_dir;
use marzano_language::target_language::{expand_paths, PatternLanguage};

use marzano_util::position::Position;
use serde::{Deserialize, Serialize};
use std::env;
use std::path::PathBuf;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;

use std::collections::BTreeMap;
use tokio::fs;

use crate::{
    analyze::par_apply_pattern, community::parse_eslint_output, error::GoodError,
    flags::OutputFormat, messenger_variant::create_emitter, result_formatting::get_human_error,
    updater::Updater,
};

use marzano_messenger::{
    emit::{ApplyDetails, Messager, VisibilityLevels},
    output_mode::OutputMode,
};

use crate::resolver::{get_grit_files_from_cwd, GritModuleResolver};
use crate::utils::{has_uncommitted_changes, is_pattern_name};

use super::init::init_config_from_cwd;

#[derive(Deserialize)]
pub struct ApplyInput {
    pub pattern_body: String,
    pub pattern_libs: BTreeMap<String, String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct ApplyPatternArgs {
    // Level of detail to show for results
    #[clap(
        long = "output",
        default_value_t = OutputMode::Standard,
    )]
    output: OutputMode,
    #[clap(long = "ignore-limit", default_value = "false", hide = true)]
    ignore_limit: bool,
    // Dry run
    #[clap(
        long = "dry-run",
        default_value = "false",
        conflicts_with_all = &["format", "interactive"],
        help = "Show a dry-run of the changes that would be applied"
    )]
    pub dry_run: bool,
    #[clap(
        long = "force",
        default_value = "false",
        help = "Force apply, even if there are uncommitted changes"
    )]
    force: bool,
    #[clap(long = "format", default_value = "false", conflicts_with_all = &["dry_run"], hide = true)]
    pub format: bool,
    #[clap(
        long = "interactive",
        short = 'i',
        default_value = "false",
        conflicts_with_all = &["dry_run"],
        help = "Selectively apply changes interactively"
    )]
    pub interactive: bool,
    #[clap(
            long = "min-visibility",
            default_value_t = VisibilityLevels::Supplemental,
            hide = true,
        )]
    pub visibility: VisibilityLevels,
    #[clap(
        long = "only-in-json",
        help = "Only rewrite ranges that are inside the provided eslint-style JSON file",
        hide = true
    )]
    only_in_json: Option<PathBuf>,
    #[clap(
        long = "output-file",
        help = "Path to a file to write the results to, defaults to stdout"
    )]
    output_file: Option<PathBuf>,
    /// Use cache
    #[clap(long = "cache", conflicts_with = "refresh_cache")]
    pub cache: bool,
    /// Clear cache before running apply
    #[clap(long = "refresh-cache", conflicts_with = "cache")]
    pub refresh_cache: bool,
    #[clap(long = "language", alias="lang")]
    pub language: Option<PatternLanguage>,
}

impl Default for ApplyPatternArgs {
    fn default() -> Self {
        Self {
            output: Default::default(),
            ignore_limit: Default::default(),
            dry_run: Default::default(),
            force: Default::default(),
            format: Default::default(),
            interactive: Default::default(),
            visibility: VisibilityLevels::Hidden,
            only_in_json: Default::default(),
            output_file: Default::default(),
            cache: Default::default(),
            refresh_cache: Default::default(),
            language: Default::default(),
        }
    }
}

macro_rules! flushable_unwrap {
    ($flushable:expr, $expr:expr) => {
        match $expr {
            Ok(r) => r,
            Err(e) => {
                $flushable.flush().await?;
                bail!(e);
            }
        }
    };
}

#[instrument]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn run_apply_pattern(
    pattern: String,
    paths: Vec<PathBuf>,
    arg: ApplyPatternArgs,
    multi: MultiProgress,
    details: &mut ApplyDetails,
    pattern_libs: Option<BTreeMap<String, String>>,
    lang: Option<PatternLanguage>,
    format: OutputFormat,
    root_path: Option<PathBuf>,
) -> Result<()> {
    let mut context = Updater::from_current_bin()
        .await
        .unwrap()
        .get_context()
        .unwrap();
    if arg.ignore_limit {
        context.ignore_limit_pattern = true;
    }

    let interactive = arg.interactive;
    let min_level = &arg.visibility;

    // Get the current directory
    let cwd = std::env::current_dir().unwrap();

    // Construct a resolver
    let resolver = GritModuleResolver::new(cwd.to_str().unwrap());

    let mut emitter = create_emitter(
        &format,
        arg.output.clone(),
        arg.output_file.as_ref(),
        interactive,
        Some(&pattern),
        root_path.as_ref(),
    )
    .await?;

    let filter_range = if let Some(json_path) = arg.only_in_json.clone() {
        let json_ranges = flushable_unwrap!(emitter, parse_eslint_output(json_path));
        Some(json_ranges)
    } else {
        None
    };

    let (my_input, lang) = if let Some(pattern_libs) = pattern_libs {
        (
            ApplyInput {
                pattern_body: pattern.clone(),
                paths,
                pattern_libs,
            },
            lang,
        )
    } else {
        let mod_dir = find_grit_modules_dir(cwd.clone()).await;
        if !env::var("GRIT_DOWNLOADS_DISABLED")
            .unwrap_or_else(|_| "false".to_owned())
            .parse::<bool>()
            .unwrap_or(false)
            && mod_dir.is_err()
        {
            flushable_unwrap!(
                emitter,
                init_config_from_cwd::<KeepFetcherKind>(cwd.clone(), false).await
            );
        }

        let warn_uncommitted =
            !arg.dry_run && !arg.force && has_uncommitted_changes(cwd.clone()).await;
        if warn_uncommitted {
            let proceed = flushable_unwrap!(emitter, Confirm::new()
                .with_prompt("Your working tree currently has untracked changes and Grit will rewrite files in place. Do you want to proceed?")
                .default(false)
                .interact());

            if !proceed {
                return Ok(());
            }
        }

        let pattern_libs = flushable_unwrap!(emitter, get_grit_files_from_cwd().await);
        let (mut lang, pattern_body) = if pattern.ends_with(".grit") || pattern.ends_with(".md") {
    
            match fs::read_to_string(pattern.clone()).await {
                Ok(pb) => {
                    if pattern.ends_with(".grit") {
                        let lang = PatternLanguage::get_language(&pb);
                        (lang, pb)
                    } else if pattern.ends_with(".md") {
                        let body = flushable_unwrap!(emitter, get_body_from_md_content(&pb));
                        let lang = PatternLanguage::get_language(&body);
                        (lang, body)
                    } else {
                        unreachable!()
                    }
                }
                Err(_) => {
                    let my_err = anyhow::anyhow!("Could not read pattern file: {}", pattern);
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
                    emitter.emit(&log, min_level).unwrap();
                    emitter.flush().await?;
                    if format.is_always_ok().0 {
                        return Ok(());
                    } else {
                        return Err(my_err);
                    }
                }
            }
        } else {
            match is_pattern_name(&pattern) {
                true => {
                    let raw_name = pattern.trim_end_matches("()");
                    details.named_pattern = Some(raw_name.to_string());
                    let presumptive_grit_file =
                        pattern_libs.get(format!("{}.grit", raw_name).as_str());
                    let lang = match presumptive_grit_file {
                        Some(g) => PatternLanguage::get_language(g),
                        None => PatternLanguage::get_language(&pattern),
                    };
                    let body = if pattern.ends_with(')') {
                        pattern.clone()
                    } else {
                        format!("{}()", pattern)
                    };
                    (lang, body)
                }
                false => {
                    let lang = PatternLanguage::get_language(&pattern);
                    (lang, pattern.clone())
                }
            }
        };
        if let Some(lang_option) = &arg.language {
            if let Some(lang) = lang {
                if lang != *lang_option {
                    return Err(anyhow::anyhow!(
                        "Language option {} does not match pattern language {}",
                        lang_option,
                        lang
                    ));
                }
            }
            lang = Some(*lang_option);
        }
        let pattern_libs = flushable_unwrap!(
            emitter,
            pattern_libs.get_language_directory_or_default(lang)
        );
        (
            ApplyInput {
                pattern_body,
                pattern_libs,
                paths: paths.to_owned(),
            },
            lang,
        )
    };

    if my_input.paths.is_empty() {
        let all_done = MatchResult::AllDone(AllDone {
            processed: 0,
            found: 0,
            reason: AllDoneReason::NoInputPaths,
        });
        emitter.emit(&all_done, min_level).unwrap();
        emitter.flush().await?;

        return Ok(());
    }

    let current_name = if is_pattern_name(&pattern) {
        Some(pattern.trim_end_matches("()").to_string())
    } else {
        my_input
            .pattern_libs
            .iter()
            .find(|(_, body)| body.trim() == pattern.trim())
            .map(|(name, _)| name.clone())
    };
    let pattern: crate::resolver::RichPattern<'_> = flushable_unwrap!(
        emitter,
        resolver.make_pattern(&my_input.pattern_body, current_name)
    );

    let CompilationResult {
        problem: compiled,
        compilation_warnings,
    } = match pattern.compile(&my_input.pattern_libs, lang, filter_range) {
        Ok(c) => c,
        Err(e) => {
            let log = match e.downcast::<marzano_util::analysis_logs::AnalysisLog>() {
                Ok(al) => AnalysisLog::from(al),
                Err(er) => AnalysisLog {
                    level: 200,
                    message: er.to_string(),
                    position: Position::first(),
                    file: "PlaygroundPattern".to_string(),
                    engine_id: "marzano".to_string(),
                    syntax_tree: None,
                    range: None,
                    source: None,
                },
            };
            emitter
                .emit(&MatchResult::AnalysisLog(log.clone()), min_level)
                .unwrap();
            emitter.flush().await?;
            match format.is_always_ok() {
                (true, _) => return Ok(()),
                (false, false) => bail!(GoodError::new()),
                (false, true) => bail!(GoodError::new_with_message(get_human_error(
                    log,
                    &my_input.pattern_body
                ))),
            }
        }
    };
    for warn in compilation_warnings.clone().into_iter() {
        emitter
            .emit(&MatchResult::AnalysisLog(warn.into()), min_level)
            .unwrap();
    }

    debug!(
        "Applying pattern: {:?}, {:?}",
        my_input.paths, compiled.language
    );

    let file_walker = flushable_unwrap!(
        emitter,
        expand_paths(&my_input.paths, Some(&[(&compiled.language).into()]))
    );

    let processed = AtomicI32::new(0);

    let mut emitter = par_apply_pattern(
        file_walker,
        multi,
        compiled,
        &my_input,
        emitter,
        &processed,
        details,
        &arg,
        &context,
        &format,
    )
    .await;

    let all_done = MatchResult::AllDone(AllDone {
        processed: processed.load(Ordering::SeqCst),
        found: details.matched,
        reason: AllDoneReason::AllMatchesFound,
    });

    emitter.emit(&all_done, min_level).unwrap();

    emitter.flush().await?;

    match emitter.get_fatal_error() {
        Some(e) => match format.is_always_ok() {
            (true, _) => return Ok(()),
            (false, false) => bail!(GoodError::new()),
            (false, true) => bail!(GoodError::new_with_message(e.message.clone())),
        },
        None => Ok(()),
    }
}
