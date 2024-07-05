use anyhow::{bail, Result};
use clap::Args;
use dashmap::DashMap;
use grit_cache::paths::cache_for_cwd;
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use log::info;
use marzano_core::{
    api::{
        is_match, AllDone, AllDoneReason, EnforcementLevel, MatchReason, MatchResult, RewriteSource,
    },
    fs::apply_rewrite,
    problem::Problem,
};
use marzano_gritmodule::{config::ResolvedGritDefinition, utils::extract_path};
use marzano_language::target_language::{expand_paths, PatternLanguage};
use marzano_messenger::emit::FlushableMessenger as _;
use marzano_util::cache::GritCache;
use marzano_util::rich_path::RichPath;
use marzano_util::{finder::get_input_files, rich_path::RichFile};
use rayon::prelude::{IntoParallelRefIterator, ParallelIterator};
use serde::Serialize;
use std::{
    collections::{HashMap, HashSet},
    path::PathBuf,
};
use tokio::try_join;

use marzano_messenger::emit::{Messager, VisibilityLevels};

#[cfg(feature = "server")]
use cli_server::check::CheckMessenger;

use crate::{
    error::GoodError,
    flags::{GlobalFormatFlags, OutputFormat},
    github::{log_check_annotations, write_check_summary},
    messenger_variant::create_emitter,
    resolver::{
        get_grit_files_from, get_grit_files_from_flags_or_cwd, resolve_from, resolve_from_cwd,
        GritModuleResolver, Source,
    },
    scan::log_check_json,
    updater::Updater,
    ux::{get_check_summary, log_file, print_config, CheckResult},
};

use super::filters::{extract_filter_ranges, SharedFilterArgs};

#[derive(Args, Serialize, Debug)]
pub struct CheckArg {
    /// The target paths to apply the checks to
    #[clap(value_parser, default_value = ".")]
    pub paths: Vec<PathBuf>,
    /// Apply fixes to all rewrites
    #[clap(long = "fix")]
    pub fix: bool,
    /// Show verbose output
    #[clap(long = "verbose")]
    pub verbose: bool,
    /// Check only patterns at or above an enforcement level.
    #[clap(long = "level")]
    pub level: Option<EnforcementLevel>,
    /// Do not use cache
    #[clap(long = "no-cache", conflicts_with = "refresh_cache")]
    pub no_cache: bool,
    /// Clear cache before running check
    #[clap(long = "refresh-cache", conflicts_with = "no_cache")]
    pub refresh_cache: bool,
    /// Output annotations for a GitHub actions workflow
    #[clap(long = "github-actions")]
    pub github_actions: bool,
    #[clap(flatten)]
    pub shared_filters: SharedFilterArgs,
}

pub(crate) async fn run_check(
    arg: CheckArg,
    format: &GlobalFormatFlags,
    multi: MultiProgress,
    plumbing: bool,
    root_path: Option<PathBuf>,
) -> Result<()> {
    if format.json && arg.github_actions {
        bail!("--github-actions is not compatible with --json");
    }

    let context = Updater::from_current_bin().await?.get_context()?;

    let (cache, manager) = cache_for_cwd(arg.refresh_cache, arg.no_cache).await?;

    let paths = arg.paths;
    let ((resolved_patterns, _), grit_files) = if plumbing {
        if paths.is_empty() {
            return Ok(());
        }
        let path = Some(PathBuf::from(paths.first().unwrap()));
        let (resolved, mut grit_files, global_files) = try_join![
            resolve_from(paths.first().unwrap().to_owned(), &Source::All),
            get_grit_files_from(path),
            get_grit_files_from(None),
        ]?;
        grit_files.merge(global_files);
        (resolved, grit_files)
    } else {
        try_join![
            resolve_from_cwd(&Source::All),
            get_grit_files_from_flags_or_cwd(format)
        ]?
    };

    let enforced = resolved_patterns
        .iter()
        .filter(|p| {
            &p.level() >= arg.level.as_ref().unwrap_or(&EnforcementLevel::Warn)
                && !matches!(p.language, PatternLanguage::Universal)
        })
        .collect::<Vec<_>>();

    let current_dir = if plumbing {
        paths.first().unwrap().to_owned()
    } else {
        std::env::current_dir()?
    };

    let filter_range = extract_filter_ranges(&arg.shared_filters, Some(&current_dir))?;

    // Construct a resolver
    let resolver = GritModuleResolver::new();

    let mut body_to_pattern: HashMap<String, &ResolvedGritDefinition> = HashMap::new();
    let compile_tasks: Result<HashMap<String, Problem>, _> = enforced
        .iter()
        .map(|p| {
            let body = format!("{}()", p.local_name);
            body_to_pattern.insert(body.clone(), p);
            let lang = PatternLanguage::get_language(&p.body);
            let grit_files = grit_files.get_language_directory_or_default(lang)?;
            let rich_pattern = resolver
                .make_pattern(&body, Some(p.local_name.to_string()))
                .unwrap();
            let lang = PatternLanguage::get_language(&p.body);
            match rich_pattern.compile(&grit_files, lang, filter_range.clone(), None) {
                Ok(c) => Ok((p.local_name.clone(), c.problem)),
                Err(e) => {
                    bail!("Unable to compile pattern {}:\n{}", p.local_name, e);
                }
            }
        })
        .collect();
    let compiled_map = compile_tasks?;
    let problems: Vec<_> = compiled_map.values().collect();

    let results: DashMap<String, Vec<MatchResult>> = DashMap::new();

    let target_languages: Vec<PatternLanguage> = problems
        .iter()
        .map(|problem| (&problem.language).into())
        .collect();

    let found_files: DashMap<String, Vec<RichPath>> = DashMap::new();

    for language in target_languages {
        let file_walker = expand_paths(&paths, Some(&[language]))?;
        let mut language_paths = Vec::new();
        for file in file_walker {
            let file = file?;
            if file.file_type().unwrap().is_dir() {
                continue;
            }
            let path = file.path();
            language_paths.push(path.to_path_buf());
        }
        let input_files = get_input_files(&language_paths);
        found_files.insert(language.to_string(), input_files);
    }

    let pg: ProgressBar = multi.add(ProgressBar::new(compiled_map.len().try_into()?));
    let style = ProgressStyle::with_template(
        "\n{prefix:.bold.dim} {wide_msg:.bold.dim}\n{wide_bar} {pos:}/{len}",
    )
    .unwrap();
    pg.set_style(style);
    pg.set_prefix("Checking");

    problems.par_iter().for_each(|pattern| {
        if let Some(name) = &pattern.name {
            pg.set_message(name.to_string());
        }
        let language_files = match found_files.get(&pattern.language.to_string()) {
            Some(files) => files,
            None => return,
        };
        let un_cached_input_files: Vec<_> = language_files
            .iter()
            .filter(|path| {
                let Some(hash) = path.hash else { return true };
                !cache.has_no_matches(hash, pattern.hash)
            })
            .collect();
        let (result, no_match) = pattern.execute_paths(un_cached_input_files, &context);
        if !no_match.is_empty() {
            for path in no_match.into_iter() {
                let hash = path.hash.unwrap();
                cache.put_no_matches(hash, pattern.hash).unwrap();
            }
        }
        let mut entry = results.entry(pattern.tree.source.clone()).or_default();
        entry.extend(result.into_iter().filter(is_match));
        pg.inc(1);
    });

    let mut check_results: HashMap<String, Vec<CheckResult>> = HashMap::new();

    for result in results.iter() {
        let body = result.key();
        let match_results = result.value();
        let pattern = match body_to_pattern.get(body) {
            Some(p) => p,
            None => bail!("Unable to find pattern for body {}", body),
        };
        let relevant_results = match_results
            .par_iter()
            .filter_map(|r| {
                let path = extract_path(r)?;
                let check_result = CheckResult {
                    pattern,
                    result: r.clone(),
                };
                Some((path.to_string(), check_result))
            })
            .collect::<Vec<_>>();
        for (path, result) in relevant_results {
            let entry = check_results.entry(path.clone()).or_default();
            entry.push(result);
        }
    }

    pg.finish_and_clear();

    if plumbing {
        let format = OutputFormat::from(format);
        let format = if format == OutputFormat::Standard {
            OutputFormat::Jsonl
        } else {
            format
        };
        let mut emitter = create_emitter(
            &format,
            marzano_messenger::output_mode::OutputMode::default(),
            None,
            false,
            None,
            root_path.as_ref(),
        )
        .await?;

        let total_file_count = found_files
            .iter()
            .map(|entry| entry.value().len())
            .sum::<usize>();
        emitter.emit_estimate(total_file_count)?;

        match emitter {
            crate::messenger_variant::MessengerVariant::Formatted(_)
            | crate::messenger_variant::MessengerVariant::Transformed(_)
            | crate::messenger_variant::MessengerVariant::JsonLine(_) => {
                info!("Local only, skipping check registration.");
            }
            #[cfg(feature = "server")]
            crate::messenger_variant::MessengerVariant::Redis(ref mut m) => {
                m.mark_checked_patterns(&enforced)?
            }
            #[cfg(feature = "remote_pubsub")]
            crate::messenger_variant::MessengerVariant::GooglePubSub(ref mut m) => {
                m.mark_checked_patterns(&enforced)?
            }
            #[cfg(feature = "remote_redis")]
            crate::messenger_variant::MessengerVariant::Combined(ref mut m) => {
                m.mark_checked_patterns(&enforced)?
            }
        }

        for (_, results) in check_results {
            for result in results {
                let rewrite_with_reason = match &result.result {
                    MatchResult::Rewrite(r) => {
                        let reason = Some(MatchReason {
                            metadata_json: None,
                            source: RewriteSource::Gritql,
                            title: result.pattern.title().map(|s| s.to_string()),
                            name: Some(result.pattern.local_name.to_string()),
                            level: Some(result.pattern.level()),
                            explanation: None,
                        });
                        let mut rewrite = r.clone();
                        rewrite.reason = reason;
                        Some(MatchResult::Rewrite(rewrite))
                    }
                    _ => None,
                };
                let rewrite_with_reason = rewrite_with_reason.as_ref();
                let message = rewrite_with_reason.unwrap_or(&result.result);
                emitter
                    .emit(message, &VisibilityLevels::Supplemental)
                    .unwrap();
            }
        }
        let safe_total_file_count = std::cmp::min(total_file_count, i32::MAX as usize) as i32;
        let all_done = MatchResult::AllDone(AllDone {
            processed: safe_total_file_count,
            found: 0,
            reason: AllDoneReason::AllMatchesFound,
        });
        emitter
            .emit(&all_done, &VisibilityLevels::Supplemental)
            .unwrap();

        emitter.flush().await?;

        return Ok(());
    }

    if arg.github_actions {
        let flattened_results: Vec<_> = check_results.values().flat_map(|v| v.iter()).collect();

        log_check_annotations(&flattened_results);

        if let Ok(env_file) = std::env::var("GITHUB_STEP_SUMMARY") {
            write_check_summary(&env_file, &enforced, &flattened_results)?;
        }

        // Write a final summary
        let (grouped, message) = get_check_summary(&flattened_results)?;
        if grouped.get(&EnforcementLevel::Error).unwrap_or(&0) > &0 {
            return Err(anyhow::anyhow!(message));
        }
        info!("{}", message);
        return Ok(());
    }

    if format.json {
        let found_files = found_files.into_read_only();
        let all_files = found_files
            .values()
            .flatten()
            .map(|r| r.path.to_path_buf())
            .collect::<Vec<_>>();
        log_check_json(check_results, all_files);
        return Ok(());
    }

    if arg.verbose {
        let flattened_results: Vec<_> = check_results.values().flat_map(|v| v.iter()).collect();
        print_config(&resolved_patterns, flattened_results);
        info!("\n");
    }

    if check_results.is_empty() {
        info!("No results found, checked {} patterns.", enforced.len());
        return Ok(());
    }

    let mut sorted_results: Vec<(&String, &Vec<CheckResult<'_>>)> = check_results.iter().collect();
    sorted_results.sort_by_key(|(k, _)| *k);

    for (file, check_results) in sorted_results.iter() {
        if arg.fix {
            let rewrites = check_results
                .iter()
                .filter(|r| matches!(r.result, MatchResult::Rewrite(_)))
                .collect::<Vec<_>>();

            if rewrites.len() == 1 {
                apply_rewrite(&rewrites[0].result).unwrap();
            } else {
                let applicable_patterns = rewrites
                    .iter()
                    .map(|r| &r.pattern.local_name)
                    .collect::<HashSet<_>>();
                for pattern in applicable_patterns {
                    let problem = compiled_map.get(pattern).unwrap();
                    let src = fs_err::read_to_string(file)?;
                    let res = problem.execute_file(&RichFile::new(file.to_string(), src), &context);
                    for r in res {
                        if let MatchResult::Rewrite(r) = r {
                            apply_rewrite(&MatchResult::Rewrite(r))?;
                        }
                    }
                }
            }
        }
        log_file(file, check_results, arg.fix);
    }

    drop(cache);
    if let Some(manager) = manager {
        match manager.join() {
            Ok(_) => {}
            Err(e) => {
                bail!("Error joining cache manager: {:?}", e);
            }
        }
    }

    let files = check_results.len();
    if arg.fix {
        info!("{} files fixed.", files);
        Ok(())
    } else {
        let msg = format!(
            "{} files with rewrites. Run grit check --fix to apply changes.",
            files
        );
        info!("{}", msg);
        // Make sure we fail if there are rewrites
        if files > 0 && !arg.github_actions {
            bail!(GoodError::new());
        }
        Ok(())
    }
}
