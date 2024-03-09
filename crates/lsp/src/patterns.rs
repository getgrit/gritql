use std::{path::PathBuf, str::FromStr};

use anyhow::{Context, Result};
use marzano_gritmodule::{
    config::{get_stdlib_modules, ResolvedGritDefinition, REPO_CONFIG_DIR_NAME},
    fetcher::{KeepFetcherKind, ModuleRepo},
    patterns_directory::PatternsDirectory,
    resolver::{fetch_modules, get_grit_files, resolve_patterns},
    searcher::find_grit_dir_from,
};
use marzano_language::target_language::PatternLanguage;

use crate::util::uri_to_file_path;

pub async fn prep_grit_modules(
    uri: &str,
    fetch: bool,
) -> Result<(ModuleRepo, String, Option<Vec<ModuleRepo>>)> {
    let file_path = uri_to_file_path(uri).unwrap_or_else(|_| std::env::current_dir().unwrap());
    let existing_config = find_grit_dir_from(file_path).await;
    let stdlib_modules = get_stdlib_modules();
    let grit_parent = match existing_config {
        Some(config) => {
            let config_path = PathBuf::from_str(&config).unwrap();
            let parent = config_path.parent().context(format!(
                "Unable to find parent of .grit directory at {}",
                config
            ))?;
            parent.to_path_buf()
        }
        None => {
            let current_bin = std::env::current_exe().unwrap();
            let parent = current_bin.parent().context(format!(
                "Unable to find parent of current binary at {}",
                current_bin.display()
            ))?;
            parent.to_path_buf()
        }
    };
    let grit_dir = grit_parent.join(REPO_CONFIG_DIR_NAME);
    let repo = ModuleRepo::from_dir(&grit_dir).await;
    if fetch {
        let _ = fetch_modules::<KeepFetcherKind>(&repo, &grit_parent.to_string_lossy()).await;
    }
    let parent_str = grit_parent.to_string_lossy().to_string();
    Ok((repo, parent_str, Some(stdlib_modules)))
}

pub async fn resolve_from_uri(
    uri: &str,
    lang: Option<PatternLanguage>,
    fetch: bool,
) -> Vec<ResolvedGritDefinition> {
    let (repo, parent_str, stdlib_modules) = match prep_grit_modules(uri, fetch).await {
        Ok((repo, parent_str, stdlib_modules)) => (repo, parent_str, stdlib_modules),
        Err(_) => return vec![],
    };
    let all_patterns = match resolve_patterns(&repo, &parent_str, stdlib_modules).await {
        Ok((resolved, _)) => resolved,
        Err(_) => vec![],
    };
    match lang {
        Some(lang) => all_patterns
            .into_iter()
            .filter(|p| {
                let language = PatternLanguage::get_language(&p.body);
                language.unwrap_or_default().language_name() == lang.language_name()
            })
            .collect(),
        None => all_patterns,
    }
}

pub async fn get_grit_files_from_uri(uri: &str, fetch: bool) -> PatternsDirectory {
    let (repo, parent_str, stdlib_modules) = match prep_grit_modules(uri, fetch).await {
        Ok((repo, parent_str, stdlib_modules)) => (repo, parent_str, stdlib_modules),
        Err(_) => return PatternsDirectory::new(),
    };
    match get_grit_files(&repo, &parent_str, stdlib_modules).await {
        Ok(patterns) => patterns,
        Err(_) => PatternsDirectory::new(),
    }
}
