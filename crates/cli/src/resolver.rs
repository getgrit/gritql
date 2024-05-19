use clap::ValueEnum;
use colored::Colorize;
use core::fmt;
use log::{info, warn};
use serde::Serialize;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use marzano_gritmodule::{
    config::{get_stdlib_modules, ResolvedGritDefinition},
    fetcher::ModuleRepo,
    patterns_directory::PatternsDirectory,
    resolver::{find_local_patterns, find_user_patterns, get_grit_files, resolve_patterns},
    searcher::find_grit_dir_from,
};

use crate::updater::Updater;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum Source {
    /// All patterns
    All,
    /// Only patterns from the local repo
    Local,
    /// Only patterns from the user config
    User,
}

// Equivalent to our PatternResolver in zesty, but more minimal
pub struct GritModuleResolver {}

impl GritModuleResolver {
    pub fn new() -> Self {
        Self {}
    }

    pub fn make_pattern<'b>(
        &self,
        pattern_input: &'b str,
        name: Option<String>,
    ) -> Result<RichPattern<'b>> {
        let pattern = RichPattern {
            body: pattern_input,
            name,
        };
        Ok(pattern)
    }
}

#[derive(Debug)]
pub struct RichPattern<'b> {
    pub body: &'b str,
    pub name: Option<String>,
}

impl<'b> fmt::Display for RichPattern<'b> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.body)
    }
}

pub async fn get_grit_files_from(cwd: Option<PathBuf>) -> Result<PatternsDirectory> {
    let existing_config = if let Some(cwd) = cwd {
        find_grit_dir_from(cwd).await
    } else {
        None
    };
    let stdlib_modules = get_stdlib_modules();

    match existing_config {
        Some(config) => {
            let config_path = PathBuf::from_str(&config).unwrap();
            let grit_parent = PathBuf::from(config_path.parent().context(format!(
                "Unable to find parent of .grit directory at {}",
                config
            ))?);
            let parent_str = &grit_parent.to_string_lossy().to_string();
            let repo = ModuleRepo::from_dir(&config_path).await;
            get_grit_files(&repo, parent_str, Some(stdlib_modules)).await
        }
        None => {
            let updater = Updater::from_current_bin().await?;
            let install_path = updater.install_path;
            let repo = ModuleRepo::from_dir(&install_path).await;
            get_grit_files(&repo, &install_path.to_string_lossy(), Some(stdlib_modules)).await
        }
    }
}

#[tracing::instrument]
pub async fn get_grit_files_from_cwd() -> Result<PatternsDirectory> {
    let cwd = std::env::current_dir()?;
    get_grit_files_from(Some(cwd)).await
}

pub async fn resolve_from(
    cwd: PathBuf,
    source: &Source,
) -> Result<(Vec<ResolvedGritDefinition>, ModuleRepo)> {
    let existing_config = find_grit_dir_from(cwd).await;
    let stdlib_modules = get_stdlib_modules();

    match existing_config {
        Some(config) => {
            let config_path = PathBuf::from_str(&config).unwrap();
            let grit_parent = PathBuf::from(config_path.parent().context(format!(
                "Unable to find parent of .grit directory at {}",
                config
            ))?);
            let parent_str = &grit_parent.to_string_lossy().to_string();
            let repo = ModuleRepo::from_dir(&config_path).await;
            let resolved = match source {
                Source::Local => find_local_patterns(&repo, parent_str).await?,
                Source::All => {
                    let (resolved, errored_patterns) =
                        resolve_patterns(&repo, parent_str, Some(stdlib_modules)).await?;
                    log_errored_patterns(&errored_patterns);
                    resolved
                }
                Source::User => find_user_patterns().await?,
            };
            Ok((resolved, repo))
        }
        None => {
            let updater = Updater::from_current_bin().await?;
            let install_path = updater.install_path;
            let repo = ModuleRepo::from_dir(&install_path).await;
            let resolved = match source {
                Source::Local => vec![],
                Source::All => {
                    let (resolved, errored_patterns) = resolve_patterns(
                        &repo,
                        &install_path.to_string_lossy(),
                        Some(stdlib_modules),
                    )
                    .await?;
                    log_errored_patterns(&errored_patterns);
                    resolved
                }
                Source::User => find_user_patterns().await?,
            };

            Ok((resolved, repo))
        }
    }
}

pub async fn resolve_from_cwd(
    source: &Source,
) -> Result<(Vec<ResolvedGritDefinition>, ModuleRepo)> {
    let cwd = std::env::current_dir()?;
    resolve_from(cwd, source).await
}

fn log_errored_patterns(errored_patterns: &HashMap<String, String>) {
    if !errored_patterns.is_empty() {
        let warning = "⚠️ The following patterns did not resolve cleanly:\n".yellow();
        warn!("{}", warning);
        for (pattern, message) in errored_patterns {
            info!("{}: {}\n", pattern.bold(), message);
        }
    }
}
