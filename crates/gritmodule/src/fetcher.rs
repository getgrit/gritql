use std::path::{Path, PathBuf};

use anyhow::{anyhow, bail, Result};
use git2::Repository;
use regex::Regex;
use serde::{Deserialize, Serialize};

use fs_err;
use lazy_static::lazy_static;

use crate::{config::GRIT_MODULE_DIR, searcher::find_git_dir_from, utils::remove_dir_all_safe};

lazy_static! {
    static ref GIT_REMOTE_REGEX: Regex =
        Regex::new(r"(?P<protocol>(git|ssh|http(s)?)|(?P<git_at>git@[\w.-]+))(?P<separator>:(\/\/)?)(?P<path>[\w.@\:\/~-]+)(?P<git_ext>\.git)?(\/)?")
            .unwrap();
}

fn parse_remote(remote: &str) -> Result<(String, String)> {
    let captures = GIT_REMOTE_REGEX
        .captures(remote)
        .ok_or_else(|| anyhow!("Invalid remote format: could not parse url"))?;

    let host = if let Some(matched) = captures.name("git_at") {
        matched.as_str().split('@').last().map(String::from)
    } else if let Some(matched) = captures.name("path") {
        matched
            .as_str()
            .split('/')
            .next()
            .and_then(|s| s.split('@').last())
            .map(String::from)
    } else {
        bail!("Invalid remote format: missing host")
    };

    let repo = if captures.name("git_at").is_some() {
        captures
            .name("path")
            .map(|s| s.as_str().trim_end_matches(".git"))
            .map(String::from)
    } else if captures.name("path").is_some() {
        Some(
            captures
                .name("path")
                .unwrap()
                .as_str()
                .split('/')
                .skip(1)
                .collect::<Vec<_>>()
                .join("/")
                .replace(".git", ""),
        )
    } else {
        bail!("Invalid remote format: missing repo")
    };

    let host = host.ok_or_else(|| anyhow!("Missing host"))?;
    let repo = repo.ok_or_else(|| anyhow!("Missing repo"))?;

    Ok((host, repo))
}

pub fn inject_token(remote: &str, token: &str) -> Result<String> {
    let (host, repo) = parse_remote(remote)?;
    let remote = format!("https://x-access-token:{}@{}/{}", token, host, repo);
    Ok(remote)
}

/// Represents a local repository
pub struct LocalRepo {
    repo: Repository,
}

impl LocalRepo {
    pub async fn from_dir(dir: &Path) -> Option<Self> {
        let git_dir = match find_git_dir_from(dir.to_path_buf()).await {
            Some(git_dir) => git_dir,
            None => return None,
        };
        let git_repo = match Repository::open(git_dir) {
            Ok(repo) => repo,
            Err(_) => {
                return Default::default();
            }
        };
        Some(Self { repo: git_repo })
    }

    /// Return the repo root path, on the filesystem
    pub fn root(&self) -> Result<PathBuf> {
        self.repo
            .path()
            .parent()
            .map(|p| p.to_path_buf())
            .ok_or_else(|| anyhow!("Failed to get repo root"))
    }

    /// Return the current branch, if any
    ///
    /// NOTE: this will only work if the branch has at least one commit (unborn branches will not be detected)
    pub fn branch(&self) -> Option<String> {
        match self.repo.head() {
            Ok(head) => {
                if head.is_branch() {
                    match head.shorthand() {
                        Some(branch) => Some(branch.to_string()),
                        None => {
                            log::error!("Failed to get branch name");
                            None
                        }
                    }
                } else {
                    log::error!("Head is not a branch");
                    None
                }
            }
            Err(e) => {
                log::error!("Failed to get branch: {}", e);
                None
            }
        }
    }

    /// Return the remote url for the repo, if any is set
    pub fn remote(&self) -> Option<String> {
        let remote = match self.repo.remotes() {
            Ok(remotes) => match remotes.get(0) {
                Some(r) => {
                    let git_remote = Repository::find_remote(&self.repo, r);
                    match git_remote {
                        Ok(remote_obj) => {
                            let url = remote_obj.url();
                            if url.is_none() {
                                return Default::default();
                            }
                            url.unwrap().to_string()
                        }
                        Err(_) => {
                            return Default::default();
                        }
                    }
                }
                None => return Default::default(),
            },
            Err(_) => return Default::default(),
        };
        Some(remote)
    }
}

/// Represents a repository containing .grit patterns, used in our packaging system
#[derive(Eq, Serialize, Deserialize, Default, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ModuleRepo {
    pub host: String,
    pub full_name: String,
    pub remote: String,
    pub provider_name: String,
}

impl PartialEq for ModuleRepo {
    fn eq(&self, other: &Self) -> bool {
        self.provider_name == other.provider_name
    }
}

impl ModuleRepo {
    pub fn from_host_repo(host: &str, repo: &str) -> Result<Self> {
        let remote = format!("https://{}/{}.git", host, repo);
        let provider_name = format!("{}/{}", host, repo);
        Ok(Self {
            host: host.to_string(),
            full_name: repo.to_string(),
            remote,
            provider_name,
        })
    }

    pub fn from_repo_str(repo: &str) -> Result<Self> {
        let slash_pos = repo
            .find('/')
            .ok_or_else(|| anyhow!("Invalid format. Missing slash in repo string"))?;
        let host = &repo[0..slash_pos];
        let full_name = &repo[slash_pos + 1..];

        if host.is_empty() || full_name.is_empty() {
            return Err(anyhow!("Invalid format. Host or full name is empty"));
        }

        Self::from_host_repo(host, full_name)
    }

    pub fn from_remote(remote: &str) -> Result<Self> {
        let (host, repo) = parse_remote(remote)?;
        let provider_name = format!("{}/{}", host, repo);

        Ok(Self {
            host,
            full_name: repo,
            provider_name,
            remote: remote.to_string(),
        })
    }

    pub async fn from_dir(dir: &Path) -> Self {
        let local_repo = LocalRepo::from_dir(dir).await;

        match local_repo {
            Some(local_repo) => (&local_repo).into(),
            None => Default::default(),
        }
    }
}

impl From<&LocalRepo> for ModuleRepo {
    fn from(local_repo: &LocalRepo) -> Self {
        let remote = local_repo.remote().unwrap_or_default();

        match ModuleRepo::from_remote(&remote) {
            Ok(module_repo) => module_repo,
            Err(_) => Default::default(),
        }
    }
}

enum CloneExistingStrategy {
    Preserve,
    Clean,
}

fn clone_repo<'a>(
    repo: &ModuleRepo,
    token: &Option<String>,
    target_dir: &'a PathBuf,
    strategy: CloneExistingStrategy,
) -> Result<&'a PathBuf> {
    if target_dir.exists() {
        match strategy {
            CloneExistingStrategy::Preserve => return Ok(target_dir),
            CloneExistingStrategy::Clean => {
                remove_dir_all_safe(target_dir)?;
            }
        }
    };

    let remote = match &token {
        Some(token) => inject_token(&repo.remote, token)?,
        None => repo.remote.to_string(),
    };

    match Repository::clone(&remote, target_dir) {
        Ok(_) => {}
        Err(e) => {
            if !target_dir.exists() {
                bail!("Failed to clone repo {}: {}", repo.full_name, e.to_string())
            }
        }
    };

    Ok(target_dir)
}

pub trait GritModuleFetcher: Send + Sync {
    fn clone_dir(&self) -> &PathBuf;
    fn fetch_grit_module(&self, repo: &ModuleRepo) -> Result<String>;
    fn prep_grit_modules(&self) -> Result<()>;
}

fn reset_grit_modules(grit_modules_path: &Path) -> Result<()> {
    if grit_modules_path.exists() {
        remove_dir_all_safe(grit_modules_path)?;
    }
    fs_err::create_dir_all(grit_modules_path)?;
    Ok(())
}

pub trait FetcherType {
    type Fetcher: GritModuleFetcher;
    fn make_fetcher(clone_dir: PathBuf, token: Option<String>) -> Self::Fetcher;
}

pub struct KeepFetcherKind;

impl FetcherType for KeepFetcherKind {
    type Fetcher = KeepFetcher;

    fn make_fetcher(clone_dir: PathBuf, token: Option<String>) -> Self::Fetcher {
        KeepFetcher::new(clone_dir, token)
    }
}

pub struct CleanFetcherKind;

impl FetcherType for CleanFetcherKind {
    type Fetcher = CleanFetcher;

    fn make_fetcher(clone_dir: PathBuf, token: Option<String>) -> Self::Fetcher {
        CleanFetcher::new(clone_dir, token)
    }
}

pub struct CleanFetcher {
    clone_dir: PathBuf,
    token: Option<String>,
}

impl CleanFetcher {
    pub fn new(clone_dir: PathBuf, token: Option<String>) -> Self {
        Self { clone_dir, token }
    }

    fn clone_repo<'a>(&self, repo: &ModuleRepo, target_dir: &'a PathBuf) -> Result<&'a PathBuf> {
        clone_repo(repo, &self.token, target_dir, CloneExistingStrategy::Clean)
    }

    fn get_grit_module_dir(&self, repo: &ModuleRepo) -> PathBuf {
        self.clone_dir.join(&repo.provider_name)
    }
}

impl GritModuleFetcher for CleanFetcher {
    fn clone_dir(&self) -> &PathBuf {
        &self.clone_dir
    }

    fn fetch_grit_module(&self, repo: &ModuleRepo) -> Result<String> {
        let target_dir = self.get_grit_module_dir(repo);
        self.clone_repo(repo, &target_dir)?;
        Ok(target_dir.to_str().unwrap().to_string())
    }

    fn prep_grit_modules(&self) -> Result<()> {
        // Reset this dir
        reset_grit_modules(&self.clone_dir)?;
        // Also find any sibling .gritmodules dirs and reset them
        if let Some(parent) = self.clone_dir.parent() {
            let modules_dir = parent.join(GRIT_MODULE_DIR);
            reset_grit_modules(&modules_dir)?;
        }
        Ok(())
    }
}

pub struct KeepFetcher {
    clone_dir: PathBuf,
    token: Option<String>,
}

impl KeepFetcher {
    pub fn new(clone_dir: PathBuf, token: Option<String>) -> Self {
        Self { clone_dir, token }
    }

    fn clone_repo<'a>(&self, repo: &ModuleRepo, target_dir: &'a PathBuf) -> Result<&'a PathBuf> {
        clone_repo(
            repo,
            &self.token,
            target_dir,
            CloneExistingStrategy::Preserve,
        )
    }

    fn get_grit_module_dir(&self, repo: &ModuleRepo) -> PathBuf {
        self.clone_dir.join(&repo.provider_name)
    }
}

impl GritModuleFetcher for KeepFetcher {
    fn clone_dir(&self) -> &PathBuf {
        &self.clone_dir
    }

    fn fetch_grit_module(&self, repo: &ModuleRepo) -> Result<String> {
        let target_dir = self.get_grit_module_dir(repo);
        self.clone_repo(repo, &target_dir)?;
        Ok(target_dir.to_str().unwrap().to_string())
    }

    fn prep_grit_modules(&self) -> Result<()> {
        let _ = fs_err::create_dir_all(&self.clone_dir);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::env::current_exe;

    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn local_repo_remote_and_branch() {
        let dir = tempdir().unwrap().into_path();
        let remote = "https://github.com/getgrit/stdlib.git";
        Repository::clone(remote, dir.clone()).unwrap();

        let repo = LocalRepo::from_dir(&dir).await.unwrap();

        assert_eq!(
            repo.remote().unwrap(),
            "https://github.com/getgrit/stdlib.git"
        );
        assert_eq!(repo.branch().unwrap(), "main");
    }

    #[test]
    fn clone_a_grit_module() {
        let dir = tempdir().unwrap();
        let fetcher = CleanFetcher::new(dir.path().to_path_buf(), None);
        let repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/stdlib".to_string(),
            remote: "https://github.com/getgrit/stdlib.git".to_string(),
            provider_name: "github.com/getgrit/stdlib".to_string(),
        };
        let gritmodule_dir = fetcher.fetch_grit_module(&repo).unwrap();
        assert_eq!(
            gritmodule_dir,
            dir.path()
                .join("github.com/getgrit/stdlib")
                .to_str()
                .unwrap()
        );
    }

    #[test]
    fn module_repo_from_https_remote() {
        let remote = "https://github.com/getgrit/rewriter.git";
        let repo = ModuleRepo::from_remote(remote).unwrap();

        let expected_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/rewriter".to_string(),
            remote: remote.to_string(),
            provider_name: "github.com/getgrit/rewriter".to_string(),
        };

        assert_eq!(repo, expected_repo);
    }

    #[test]
    fn module_repo_from_ssh_remote() {
        let remote = "git@github.com:getgrit/testrepo.git";
        let repo = ModuleRepo::from_remote(remote).unwrap();

        let expected_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/testrepo".to_string(),
            remote: remote.to_string(),
            provider_name: "github.com/getgrit/testrepo".to_string(),
        };

        assert_eq!(repo, expected_repo);
    }

    #[test]
    fn module_from_ssh_with_token() {
        // NOTE: this is not a real token
        let remote = "https://some-org:ghp_abcdefghijklmnopqrstuvwxyzABCD012345@github.com/some-org/some-repo.git";
        let repo = ModuleRepo::from_remote(remote).unwrap();

        let expected_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "some-org/some-repo".to_string(),
            remote: remote.to_string(),
            provider_name: "github.com/some-org/some-repo".to_string(),
        };

        assert_eq!(repo, expected_repo);
    }

    #[test]
    fn module_from_self_hosted_remote() {
        let remote = "git@10.10.0.10:gritlab/private_thing.git";
        let repo = ModuleRepo::from_remote(remote).unwrap();

        let expected_repo = ModuleRepo {
            host: "10.10.0.10".to_string(),
            full_name: "gritlab/private_thing".to_string(),
            remote: remote.to_string(),
            provider_name: "10.10.0.10/gritlab/private_thing".to_string(),
        };

        assert_eq!(repo, expected_repo);
    }

    #[test]
    fn module_repo_from_nested_remote() {
        let remote = "https://internal.gitlab.url.com/group-name/w/subgroup-name/project-name.git";
        let repo = ModuleRepo::from_remote(remote).unwrap();

        let expected_repo = ModuleRepo {
            host: "internal.gitlab.url.com".to_string(),
            full_name: "group-name/w/subgroup-name/project-name".to_string(),
            remote: remote.to_string(),
            provider_name: "internal.gitlab.url.com/group-name/w/subgroup-name/project-name"
                .to_string(),
        };

        assert_eq!(repo, expected_repo);
    }

    #[tokio::test]
    async fn module_repo_from_dir() {
        let dir = tempdir().unwrap().into_path();
        let remote = "https://github.com/getgrit/stdlib.git";
        Repository::clone(remote, dir.clone()).unwrap();

        let module_repo = ModuleRepo::from_dir(&dir).await;

        let expected_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/stdlib".to_string(),
            remote: "https://github.com/getgrit/stdlib.git".to_string(),
            provider_name: "github.com/getgrit/stdlib".to_string(),
        };

        assert_eq!(module_repo, expected_repo);
    }

    #[test]
    fn fails_if_attempting_to_prep_grit_modules_from_executable_ancestor() {
        let exe = current_exe().unwrap();
        let grandparent = exe.parent().unwrap().parent().unwrap();
        let clean_fetcher = CleanFetcherKind::make_fetcher(grandparent.to_path_buf(), None);
        let result = clean_fetcher.prep_grit_modules();
        assert!(result.is_err_and(|e| {
            e.to_string()
                .contains("directory containing the current executable")
        }));
    }
}
