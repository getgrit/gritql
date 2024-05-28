use marzano_gritmodule::fetcher::GritModuleFetcher;
use std::{env, fmt, io::ErrorKind, path::PathBuf, str::FromStr};

use tracing::instrument;

use anyhow::{bail, Context, Result};
use clap::Args;
use colored::Colorize;
use log::info;
use marzano_gritmodule::{
    config::REPO_CONFIG_DIR_NAME,
    fetcher::{CleanFetcherKind, FetcherType, ModuleRepo},
    installer::install_default_stdlib,
    resolver::fetch_modules,
    searcher::{
        find_git_dir_from, find_global_grit_dir, find_global_grit_modules_dir, find_grit_dir_from,
    },
};
use serde::Serialize;
use tokio::{fs, io::AsyncWriteExt};

#[derive(Args, Debug, Serialize)]
pub struct InitArgs {
    /// Update global grit modules
    #[clap(long = "global", default_value = "false")]
    global: bool,
}

pub(crate) async fn run_init(arg: InitArgs) -> Result<()> {
    if arg.global {
        init_global_grit_modules::<CleanFetcherKind>(None).await?;
    } else {
        let cwd = std::env::current_dir()?;
        init_config_from_cwd::<CleanFetcherKind>(cwd, true).await?;
    }

    Ok(())
}

pub enum ConfigSource {
    Local(PathBuf),
    Global(PathBuf),
}

impl fmt::Display for ConfigSource {
    // This trait requires `fmt` with this exact signature.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigSource::Local(path) => write!(f, "local config at {}", path.display()),
            ConfigSource::Global(path) => write!(f, "global config at {}", path.display()),
        }
    }
}

#[instrument]
pub async fn init_config_from_cwd<T: FetcherType>(
    cwd: PathBuf,
    create_local: bool,
) -> Result<ConfigSource> {
    let existing_config = find_grit_dir_from(cwd.clone()).await;
    let config_path = match existing_config {
        Some(config) => PathBuf::from_str(&config).unwrap(),
        None => {
            if !create_local {
                return init_global_grit_modules::<T>(None).await;
            }
            let git_dir = match find_git_dir_from(cwd).await {
                Some(dir) => dir,
                None => {
                    return init_global_grit_modules::<T>(None).await;
                }
            };
            let git_path = PathBuf::from_str(&git_dir).unwrap();
            let repo_root = git_path.parent().context(format!(
                "Unable to find repo root dir as parent of {}",
                git_dir
            ))?;
            let grit_dir = repo_root.join(REPO_CONFIG_DIR_NAME);
            let default_config = r#"version: 0.0.1
patterns:
  - name: github.com/getgrit/stdlib#*"#;
            let grit_yaml = grit_dir.join("grit.yaml");
            fs::create_dir_all(&grit_dir).await?;
            fs::write(&grit_yaml, default_config).await?;
            let message = format!("Initialized grit config at {}", grit_yaml.display()).bold();
            info!("{}", message);
            grit_dir
        }
    };

    // atomically write .gitignore
    match fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(config_path.join(".gitignore"))
        .await
    {
        Ok(mut f) => {
            f.write_all(".gritmodules*\n*.log\n".as_bytes()).await?;
            f.flush().await?;
        }
        Err(e) => {
            if e.kind() != ErrorKind::AlreadyExists {
                return Err(e.into());
            }
        }
    }

    let grit_parent = PathBuf::from(config_path.parent().context(format!(
        "Unable to find parent of .grit directory at {}",
        config_path.display()
    ))?);
    let parent_str = &grit_parent.to_string_lossy().to_string();
    let repo = ModuleRepo::from_dir(&config_path).await;
    fetch_modules::<T>(&repo, parent_str).await?;
    Ok(ConfigSource::Local(config_path))
}

pub async fn init_global_grit_modules<T: FetcherType>(
    from_module: Option<ModuleRepo>,
) -> Result<ConfigSource> {
    let global_grit_modules_dir = find_global_grit_modules_dir().await?;

    let token = env::var("GRIT_PROVIDER_TOKEN").ok();
    let fetcher = T::make_fetcher(global_grit_modules_dir, token);

    if let Some(module) = from_module {
        match fetcher.fetch_grit_module(&module) {
            Ok(_) => {}
            Err(err) => {
                bail!(
                    "Failed to fetch remote grit module {}: {}",
                    module.full_name,
                    err.to_string()
                )
            }
        }
        fetch_modules::<T>(
            &module,
            &fetcher
                .clone_dir()
                .parent()
                .context("Unable to find global grit dir")?
                .parent()
                .context("Unable to find global grit dir")?
                .to_string_lossy(),
        )
        .await?;
    } else {
        fetcher.prep_grit_modules()?;
        install_default_stdlib(&fetcher, None).await?;
    }

    Ok(ConfigSource::Global(find_global_grit_dir().await?))
}
