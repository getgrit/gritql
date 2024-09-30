use anyhow::{bail, Result};
use clap::Args;

use dialoguer::Confirm;

use marzano_gritmodule::config::{init_config_from_path, init_global_grit_modules};
use marzano_gritmodule::resolver::get_grit_files_from_known_grit_dir;
use marzano_util::rich_path::RichFile;
use tracing::instrument;
#[cfg(feature = "grit_tracing")]
use tracing::span;
#[cfg(feature = "grit_tracing")]
#[allow(unused_imports)]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

use grit_pattern_matcher::has_rewrite;
use grit_util::Position;
use indicatif::MultiProgress;
use marzano_core::api::{AllDone, AllDoneReason, AnalysisLog, MatchResult};
use marzano_core::pattern_compiler::CompilationResult;
use marzano_gritmodule::fetcher::KeepFetcherKind;
use marzano_gritmodule::markdown::get_body_from_md_content;
use marzano_gritmodule::searcher::{find_global_grit_dir, find_grit_modules_dir};
use marzano_gritmodule::utils::{infer_pattern, is_pattern_name, parse_remote_name};
use marzano_language::target_language::PatternLanguage;
use marzano_messenger::emit::FlushableMessenger as _;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::env;
use std::path::PathBuf;
use std::sync::atomic::AtomicI32;
use std::sync::atomic::Ordering;
use tokio::fs;

use crate::commands::filters::extract_filter_ranges;

use crate::flags::GlobalFormatFlags;
use crate::{
    analyze::par_apply_pattern, error::GoodError, flags::OutputFormat,
    messenger_variant::create_emitter, result_formatting::get_human_error, updater::Updater,
};

use marzano_messenger::{
    emit::{ApplyDetails, Messager, VisibilityLevels},
    output_mode::OutputMode,
};

use crate::resolver::{get_grit_files_from_flags_or_cwd, GritModuleResolver};
use crate::utils::has_uncommitted_changes;

use super::filters::SharedFilterArgs;

/// Apply a pattern to a set of paths on disk which will be rewritten in place
#[derive(Deserialize)]
pub struct ApplyInputDisk {
    pub pattern_body: String,
    pub pattern_libs: BTreeMap<String, String>,
    pub paths: Vec<PathBuf>,
}

#[derive(Deserialize)]
pub struct ApplyInputVirtual {
    pub pattern_body: String,
    pub pattern_libs: BTreeMap<String, String>,
    pub files: Vec<RichFile>,
}

#[derive(Deserialize)]
pub enum ApplyInput {
    Disk(ApplyInputDisk),
    Virtual(ApplyInputVirtual),
}

impl ApplyInput {
    pub fn pattern_body(&self) -> &str {
        match self {
            ApplyInput::Disk(d) => &d.pattern_body,
            ApplyInput::Virtual(v) => &v.pattern_body,
        }
    }

    pub fn pattern_libs(&self) -> &BTreeMap<String, String> {
        match self {
            ApplyInput::Disk(d) => &d.pattern_libs,
            ApplyInput::Virtual(v) => &v.pattern_libs,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            ApplyInput::Disk(d) => d.paths.is_empty(),
            ApplyInput::Virtual(v) => v.files.is_empty(),
        }
    }
}

#[derive(Args, Clone, Debug, Serialize)]
pub struct ApplyPatternArgs {
    // Level of detail to show for results
    #[clap(
        long = "output",
        default_value_t = OutputMode::Standard,
    )]
    output: OutputMode,
    // Inject a [limit](https://docs.grit.io/language/modifiers#limit-clause) to show only the first N results
    // If the pattern already has a limit, this will override it
    #[clap(short = 'm', long = "limit")]
    pub limit: Option<usize>,
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
        long = "output-file",
        help = "Path to a file to write the results to, defaults to stdout"
    )]
    output_file: Option<PathBuf>,
    /// Use this option when you want to transform code piped from `stdin`, and print the output to `stdout`.
    ///
    /// If you use this option, you *must* specify a file path, to allow Grit to determine the language of the code.
    ///
    /// Example: `echo 'console.log(hello)' | grit apply '`hello` => `goodbye`' file.js --stdin
    /// This will print `console.log(goodbye)` to stdout
    #[clap(long = "stdin")]
    pub stdin: bool,
    /// Use cache
    #[clap(long = "cache", conflicts_with = "refresh_cache")]
    pub cache: bool,
    /// Clear cache before running apply
    #[clap(long = "refresh-cache", conflicts_with = "cache")]
    pub refresh_cache: bool,
    /// Interpret the request as a natural language request
    #[clap(long)]
    ai: bool,
    /// Change the default language to use for the pattern (if unset, JavaScript is used by default)
    #[clap(long = "language", alias = "lang")]
    pub language: Option<PatternLanguage>,
}

impl Default for ApplyPatternArgs {
    fn default() -> Self {
        Self {
            output: Default::default(),
            limit: Default::default(),
            dry_run: Default::default(),
            force: Default::default(),
            format: Default::default(),
            interactive: Default::default(),
            visibility: VisibilityLevels::Hidden,
            output_file: Default::default(),
            cache: Default::default(),
            refresh_cache: Default::default(),
            ai: Default::default(),
            language: Default::default(),
            stdin: Default::default(),
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
#[allow(clippy::too_many_arguments, unused_mut)]
pub(crate) async fn run_apply_pattern(
    mut pattern: String,
    shared: SharedFilterArgs,
    paths: Vec<PathBuf>,
    arg: ApplyPatternArgs,
    multi: MultiProgress,
    details: &mut ApplyDetails,
    pattern_libs: Option<BTreeMap<String, String>>,
    default_lang: Option<PatternLanguage>,
    format_flags: &GlobalFormatFlags,
    root_path: Option<PathBuf>,
) -> Result<()> {
    let mut context = Updater::from_current_bin()
        .await
        .unwrap()
        .get_context()
        .unwrap();

    let format = OutputFormat::from_flags(
        format_flags,
        if arg.stdin {
            OutputFormat::Transformed
        } else {
            OutputFormat::Standard
        },
    );

    let default_lang = default_lang.or(arg.language);

    let default_lang = if !arg.stdin {
        default_lang
    } else if default_lang.is_none() {
        // Look at the first path and get the language from the extension
        let first_path = paths.first().ok_or(anyhow::anyhow!(
            "A path must be provided as the virtual file name for stdin"
        ))?;
        let ext = first_path.extension().ok_or(anyhow::anyhow!(
            "A path must have an extension to determine the language for stdin"
        ))?;
        if let Some(ext) = ext.to_str() {
            PatternLanguage::from_string_or_alias(ext, None)
        } else {
            default_lang
        }
    } else {
        default_lang
    };

    let interactive = arg.interactive;
    let min_level = &arg.visibility;

    let mut emitter = create_emitter(
        &format,
        arg.output.clone(),
        arg.output_file.as_ref(),
        interactive,
        Some(&pattern),
        root_path.as_ref(),
        *min_level,
    )
    .await?;

    #[cfg(feature = "ai_querygen")]
    if arg.ai {
        log::info!("{}", style("Computing query...").bold());

        pattern = ai_builtins::querygen::compute_pattern(&pattern, &context).await?;
        log::info!("{}", style(&pattern).dim());
        log::info!("{}", style("Executing query...").bold());
    }
    #[cfg(not(feature = "ai_querygen"))]
    if arg.ai {
        bail!("Natural language processing is not enabled in this build");
    }

    // Get the current directory
    let cwd = std::env::current_dir().unwrap();

    #[cfg(feature = "grit_tracing")]
    let module_resolution = span!(tracing::Level::INFO, "module_resolution",).entered();

    // Construct a resolver
    let resolver = GritModuleResolver::new();
    let current_repo_root = marzano_gritmodule::fetcher::LocalRepo::from_dir(&cwd)
        .await
        .map(|repo| repo.root())
        .transpose()?;
    #[cfg(feature = "grit_tracing")]
    module_resolution.exit();

    let filter_range = flushable_unwrap!(
        emitter,
        extract_filter_ranges(&shared, current_repo_root.as_ref())
    );

    #[cfg(feature = "grit_tracing")]
    let span_libs = span!(tracing::Level::INFO, "prep_libs",).entered();

    let (my_input, lang) = if let Some(pattern_libs) = pattern_libs {
        (
            ApplyInputDisk {
                pattern_body: pattern.clone(),
                paths,
                pattern_libs,
            },
            default_lang,
        )
    } else {
        #[cfg(feature = "grit_tracing")]
        let stdlib_download_span = span!(tracing::Level::INFO, "stdlib_download",).entered();

        let target_grit_dir = format_flags
            .grit_dir
            .as_ref()
            .and_then(|c| c.parent())
            .unwrap_or_else(|| &cwd)
            .to_path_buf();
        let mod_dir = find_grit_modules_dir(target_grit_dir.clone()).await;
        let target_remote = parse_remote_name(&pattern);

        if !env::var("GRIT_DOWNLOADS_DISABLED")
            .unwrap_or_else(|_| "false".to_owned())
            .parse::<bool>()
            .unwrap_or(false)
            && mod_dir.is_err()
            && target_remote.is_none()
        {
            flushable_unwrap!(
                emitter,
                init_config_from_path::<KeepFetcherKind>(target_grit_dir, false).await
            );
        } else if let Some(target) = &target_remote {
            flushable_unwrap!(
                emitter,
                init_global_grit_modules::<KeepFetcherKind>(Some(target)).await
            );
        }

        #[cfg(feature = "grit_tracing")]
        stdlib_download_span.exit();

        #[cfg(feature = "grit_tracing")]
        let grit_file_discovery = span!(tracing::Level::INFO, "grit_file_discovery",).entered();

        let pattern_libs = if let Some(target) = target_remote {
            let global = find_global_grit_dir().await?;
            flushable_unwrap!(
                emitter,
                get_grit_files_from_known_grit_dir(&global, vec![target]).await
            )
        } else {
            flushable_unwrap!(
                emitter,
                get_grit_files_from_flags_or_cwd(format_flags).await
            )
        };

        let (mut lang, named_pattern, pattern_body) =
            if pattern.ends_with(".grit") || pattern.ends_with(".md") {
                match fs::read_to_string(pattern.clone()).await {
                    Ok(pb) => {
                        if pattern.ends_with(".grit") {
                            let lang = PatternLanguage::get_language(&pb);
                            (lang, None, pb)
                        } else if pattern.ends_with(".md") {
                            let body = flushable_unwrap!(emitter, get_body_from_md_content(&pb));
                            let lang = PatternLanguage::get_language(&body);
                            (lang, None, body)
                        } else {
                            bail!("pattern should end with .grit or .md");
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
                        emitter.emit(&log).unwrap();
                        emitter.flush().await?;
                        if format.is_always_ok().0 {
                            return Ok(());
                        } else {
                            return Err(my_err);
                        }
                    }
                }
            } else {
                infer_pattern(&pattern, &pattern_libs)
            };

        if let Some(named_pattern) = named_pattern {
            details.named_pattern = Some(named_pattern.to_string());
        }

        if let Some(lang_option) = &default_lang {
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
        #[cfg(feature = "grit_tracing")]
        grit_file_discovery.exit();

        (
            ApplyInputDisk {
                pattern_body,
                pattern_libs,
                paths: paths.to_owned(),
            },
            lang,
        )
    };

    let final_input = if arg.stdin {
        let mut content = String::new();
        use std::io::Read;
        std::io::stdin().read_to_string(&mut content)?;

        let ApplyInputDisk {
            pattern_body,
            pattern_libs,
            paths,
        } = my_input;

        if paths.len() != 1 {
            bail!("Only one path can be provided as the virtual file name for --stdin");
        }

        let first_path = paths.first().ok_or(anyhow::anyhow!(
            "A path must be provided as the virtual file name for stdin"
        ))?;

        ApplyInput::Virtual(ApplyInputVirtual {
            pattern_body,
            pattern_libs,
            files: vec![RichFile {
                path: first_path.to_string_lossy().into(),
                content,
            }],
        })
    } else {
        ApplyInput::Disk(my_input)
    };

    if final_input.is_empty() {
        let all_done = MatchResult::AllDone(AllDone {
            processed: 0,
            found: 0,
            reason: AllDoneReason::NoInputPaths,
        });
        emitter.emit(&all_done).unwrap();
        emitter.flush().await?;

        return Ok(());
    }

    #[cfg(feature = "grit_tracing")]
    let collect_name = span!(tracing::Level::INFO, "collect_name",).entered();
    let current_name = if is_pattern_name(&pattern) {
        Some(pattern.trim_end_matches("()").to_string())
    } else {
        final_input
            .pattern_libs()
            .iter()
            .find(|(_, body)| body.trim() == pattern.trim())
            .map(|(name, _)| name.clone())
    };
    #[cfg(feature = "grit_tracing")]
    collect_name.exit();

    let pattern: crate::resolver::RichPattern<'_> = flushable_unwrap!(
        emitter,
        resolver.make_pattern(final_input.pattern_body(), current_name)
    );

    #[cfg(feature = "grit_tracing")]
    span_libs.exit();

    let CompilationResult {
        problem: compiled,
        compilation_warnings,
    } = match pattern.compile(final_input.pattern_libs(), lang, filter_range, arg.limit) {
        Ok(c) => c,
        Err(e) => {
            let log = match e.downcast::<grit_util::AnalysisLog>() {
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
                .emit(&MatchResult::AnalysisLog(log.clone()))
                .unwrap();
            emitter.flush().await?;
            match format.is_always_ok() {
                (true, _) => return Ok(()),
                (false, false) => bail!(GoodError::new()),
                (false, true) => bail!(GoodError::new_with_message(get_human_error(
                    log,
                    final_input.pattern_body(),
                ))),
            }
        }
    };
    for warn in compilation_warnings.clone().into_iter() {
        emitter
            .emit(&MatchResult::AnalysisLog(warn.into()))
            .unwrap();
    }

    let warn_uncommitted = !arg.dry_run && !arg.force && has_uncommitted_changes(cwd.clone()).await;
    if warn_uncommitted && has_rewrite(&compiled.pattern, &compiled.definitions()) {
        let term = console::Term::stderr();
        if !term.is_term() {
            bail!("Error: Untracked changes detected. Grit will not proceed with rewriting files in non-TTY environments unless '--force' is used. Please commit all changes or use '--force' to override this safety check.");
        }

        let proceed = flushable_unwrap!(emitter, Confirm::new()
                .with_prompt("Your working tree currently has untracked changes and Grit will rewrite files in place. Do you want to proceed?")
                .default(false)
                .interact_opt());

        if proceed != Some(true) {
            return Ok(());
        }
    }

    let processed = AtomicI32::new(0);

    let mut emitter = par_apply_pattern(
        multi,
        compiled,
        final_input,
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

    emitter.emit(&all_done).unwrap();

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
