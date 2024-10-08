use anyhow::{bail, Context, Result};
use ignore::Walk;
use std::borrow::Cow;
use std::path::Path;
use std::path::PathBuf;

use tokio::fs;

use crate::markdown::GritDefinitionOverrides;
use crate::resolver::find_user_grit_dir;
use crate::{
    config::{
        ModuleGritPattern, GRIT_GLOBAL_DIR_ENV, GRIT_MODULE_DIR, REPO_CONFIG_DIR_NAME,
        REPO_CONFIG_PATTERNS_DIR,
    },
    fetcher::ModuleRepo,
    parser::{get_patterns_from_file, PatternFileExt},
};

pub async fn collect_from_file(
    path: &Path,
    source_module: &Option<ModuleRepo>,
) -> Result<Vec<ModuleGritPattern>> {
    let ext = PatternFileExt::from_path(path).ok_or_else(|| {
        anyhow::anyhow!(
            "File does not have a Grit extension: {}",
            path.to_string_lossy()
        )
    })?;
    get_patterns_from_file(
        path.to_path_buf(),
        source_module.clone(),
        ext,
        GritDefinitionOverrides::default(),
    )
    .await
}

pub async fn collect_patterns(
    grit_parent_dir: &str,
    source_module: &Option<ModuleRepo>,
    ext: PatternFileExt,
) -> Result<Vec<ModuleGritPattern>> {
    let mut all_patterns = Vec::new();

    let patterns_path = Path::new(grit_parent_dir)
        .join(REPO_CONFIG_DIR_NAME)
        .join(REPO_CONFIG_PATTERNS_DIR);

    let mut file_readers = Vec::new();

    let walker = Walk::new(patterns_path);
    for entry in walker {
        match entry {
            Err(e) => {
                if e.io_error().is_some()
                    && e.io_error().unwrap().kind() == std::io::ErrorKind::NotFound
                {
                    continue;
                } else {
                    bail!("Error walking patterns dir: {}", e);
                }
            }
            Ok(entry) => {
                let path = entry.path();
                if path.is_file()
                    && path
                        .extension()
                        .map(|e| e == ext.get_ext())
                        .unwrap_or(false)
                {
                    file_readers.push(tokio::spawn(get_patterns_from_file(
                        path.to_path_buf(),
                        source_module.clone(),
                        ext,
                        GritDefinitionOverrides::default(),
                    )));
                }
            }
        }
    }

    for file_reader in file_readers {
        let patterns = file_reader.await??;
        all_patterns.extend(patterns);
    }

    Ok(all_patterns)
}

pub async fn search(
    current_path: PathBuf,
    config_file_names: &[String],
    stop_file: Option<&str>,
) -> Option<PathBuf> {
    let mut current_dir = current_path;

    loop {
        for config_file_name in config_file_names {
            let config_file = current_dir.join(config_file_name);
            if fs::metadata(&config_file).await.is_ok() {
                return Some(config_file);
            }
        }

        if let Some(stopper) = stop_file {
            let stop_file = current_dir.join(stopper);
            if fs::metadata(&stop_file).await.is_ok() {
                return None;
            }
        }

        let parent_dir = current_dir.parent()?;
        if parent_dir == current_dir {
            return None;
        }

        current_dir = parent_dir.to_path_buf();
    }
}

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
pub struct WorkflowInfo {
    path: PathBuf,
}

impl WorkflowInfo {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }

    pub fn entrypoint(&self) -> Cow<'_, str> {
        self.path.to_string_lossy()
    }

    pub fn absolute_path(&self) -> Result<PathBuf> {
        self.path
            .canonicalize()
            .map_err(|e| anyhow::anyhow!("Failed to get absolute path for workflow: {}", e))
    }

    pub fn name(&self) -> &str {
        if self.path.file_name().unwrap().to_str().unwrap() == "index.ts" {
            self.path
                .parent()
                .unwrap()
                .file_name()
                .unwrap()
                .to_str()
                .unwrap()
        } else {
            self.path
                .file_stem()
                .unwrap()
                .to_str()
                .unwrap()
                .trim_end_matches(".ts")
        }
    }
}

pub async fn find_local_workflow_files(dir: PathBuf) -> Result<Vec<WorkflowInfo>> {
    let grit_dir = find_grit_dir_from(dir).await;
    let user_dir = find_user_grit_dir();
    let is_user_dir_the_grit_dir = match (&user_dir, &grit_dir) {
        (Some(user_dir), Some(grit_dir)) => {
            let user_dir = user_dir.canonicalize()?;
            let grit_dir = grit_dir.canonicalize()?;
            user_dir == grit_dir
        }
        _ => false,
    };

    let mut list = match grit_dir {
        Some(grit_dir) => find_workflow_files_in_grit_dir(grit_dir).await?,
        None => vec![],
    };

    if is_user_dir_the_grit_dir {
        return Ok(list);
    }

    if let Some(user_dir) = user_dir {
        let user_list = find_workflow_files_in_grit_dir(user_dir).await?;
        list.extend(user_list);
    }
    Ok(list)
}

async fn find_workflow_files_in_grit_dir(grit_dir: PathBuf) -> Result<Vec<WorkflowInfo>> {
    let mut workflows_dir = grit_dir;
    workflows_dir.push("workflows");
    let mut files = fs::read_dir(workflows_dir).await?;

    let mut result = vec![];

    while let Some(entry) = files.next_entry().await? {
        let path = entry.path();
        if path.is_file()
            && path.extension().map(|e| e == "ts").unwrap_or(false)
            && path.file_name().unwrap().to_str().unwrap() != "index.ts"
        {
            result.push(WorkflowInfo { path });
        } else if path.is_dir() {
            // Check if it has an index.ts
            let index_path = path.join("index.ts");
            if index_path.exists() {
                result.push(WorkflowInfo { path: index_path });
            }
        }
    }
    Ok(result)
}

pub async fn find_grit_dir_from(dir: PathBuf) -> Option<PathBuf> {
    search(dir, &[REPO_CONFIG_DIR_NAME.to_string()], Some(".git")).await
}

pub async fn find_git_dir_from(dir: PathBuf) -> Option<PathBuf> {
    search(dir, &[".git".to_string()], None).await
}

pub async fn find_repo_root_from(dir: PathBuf) -> Result<Option<String>> {
    let git_dir = find_git_dir_from(dir).await;
    if let Some(git_path) = git_dir {
        Ok(Some(
            git_path
                .parent()
                .context(format!(
                    "Unable to find repo root dir as parent of {}",
                    git_path.to_string_lossy()
                ))?
                .to_string_lossy()
                .to_string(),
        ))
    } else {
        Ok(None)
    }
}

pub async fn find_grit_modules_dir(dir: PathBuf) -> Result<PathBuf> {
    let grit_dir = find_grit_dir_from(dir).await;
    if let Some(grit_dir) = grit_dir {
        let grit_modules_dir = grit_dir.join(GRIT_MODULE_DIR);
        if grit_modules_dir.exists() {
            return Ok(grit_modules_dir);
        }
    }
    bail!("Unable to find .gritmodules directory")
}

pub async fn find_global_grit_dir() -> Result<PathBuf> {
    let global_grit_dir = std::env::var(GRIT_GLOBAL_DIR_ENV);
    if let Ok(global_grit_dir) = global_grit_dir {
        return Ok(PathBuf::from(global_grit_dir));
    }

    let current_bin = std::env::current_exe()?;
    let grit_dir = current_bin
        .parent()
        .context("Unable to find global grit dir")?
        .parent()
        .context("Unable to find global grit dir")?
        .join(REPO_CONFIG_DIR_NAME);
    Ok(grit_dir)
}

pub async fn find_global_grit_modules_dir() -> Result<PathBuf> {
    Ok(find_global_grit_dir().await?.join(GRIT_MODULE_DIR))
}

#[cfg(test)]
mod tests {

    use git2::Repository;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn finds_config_dir_in_same_directory() {
        let config_file = find_grit_dir_from(PathBuf::from("fixtures/searcher/dir/nested"))
            .await
            .unwrap();
        assert_eq!(
            config_file,
            PathBuf::from("fixtures/searcher/dir/nested/.grit")
        );
    }

    #[tokio::test]
    async fn finds_config_dir_in_parent_directory() {
        let config_file = find_grit_dir_from(PathBuf::from("fixtures/searcher/another/nested"))
            .await
            .unwrap();
        assert_eq!(
            config_file,
            PathBuf::from("fixtures/searcher/another/.grit")
        );
    }

    #[tokio::test]
    async fn returns_null_if_root_reached() {
        let config_dir = [".bad-grit".to_string()];
        let config_file = search(
            PathBuf::from("fixtures/searcher/another/nested"),
            &config_dir,
            None,
        )
        .await;
        assert!(config_file.is_none());
    }

    #[tokio::test]
    async fn grit_searcher_stops_traversal_at_repo_boundary() {
        let temp_dir = tempdir().unwrap();
        let grit_dir = temp_dir.path().join(".grit");
        fs::create_dir(&grit_dir).await.unwrap();
        let repo_dir = temp_dir.path().join("repo");
        fs::create_dir(&repo_dir).await.unwrap();
        let remote = "https://github.com/openai/openai-quickstart-node.git";
        Repository::clone(remote, repo_dir.clone()).unwrap();
        let config_file = find_grit_dir_from(repo_dir).await;
        assert!(config_file.is_none());
    }

    #[tokio::test]
    async fn finds_grit_dir_in_repo_root() {
        let temp_dir = tempdir().unwrap();
        let remote = "https://github.com/custodian-sample-org/public-shop.git".to_string();
        Repository::clone(&remote, temp_dir.path()).unwrap();
        let config_file = find_grit_dir_from(temp_dir.path().into()).await;
        let exp_grit = temp_dir.path().join(".grit");
        assert_eq!(config_file.unwrap(), exp_grit);
    }
}
