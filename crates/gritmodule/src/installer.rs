use std::{collections::HashSet, path::PathBuf, str::FromStr};

use crate::{
    config::DEFAULT_STDLIBS,
    fetcher::{GritModuleFetcher, ModuleRepo},
    yaml::{extract_grit_modules, read_grit_yaml},
};
use anyhow::{bail, Result};

pub async fn install_default_stdlib(
    fetcher: &dyn GritModuleFetcher,
    pre_installed: Option<HashSet<String>>,
) -> Result<HashSet<String>> {
    let mut installed_modules = pre_installed.unwrap_or_default();

    for stdlib in DEFAULT_STDLIBS {
        let stdlib = ModuleRepo::from_remote(stdlib)?;
        if !installed_modules.contains(&stdlib.provider_name) {
            match fetcher.fetch_grit_module(&stdlib) {
                Ok(_) => {
                    installed_modules.insert(stdlib.provider_name);
                }
                Err(err) => {
                    bail!(
                        "Failed to fetch standard library grit module {}: {}",
                        stdlib.full_name,
                        err.to_string()
                    )
                }
            }
        }
    }
    Ok(installed_modules)
}

pub async fn install_grit_modules(
    fetcher: &dyn GritModuleFetcher,
    curr_repo: &ModuleRepo,
    curr_repo_dir: &str,
) -> Result<HashSet<String>> {
    let mut installed_modules: HashSet<String> = HashSet::new();
    let mut processing_modules: Vec<ModuleRepo> = Vec::new();
    parse_grit_module(
        &mut installed_modules,
        &mut processing_modules,
        curr_repo,
        curr_repo_dir,
    )
    .await?;

    while let Some(module) = processing_modules.pop() {
        if installed_modules.contains(&module.provider_name) {
            continue;
        }
        let repo_dir = match fetcher.fetch_grit_module(&module) {
            Ok(repo_dir) => repo_dir,
            Err(err) => {
                bail!(
                    "Failed to fetch grit module {}: {}",
                    module.full_name,
                    err.to_string()
                )
            }
        };
        parse_grit_module(
            &mut installed_modules,
            &mut processing_modules,
            &module,
            &repo_dir,
        )
        .await?;
    }

    let installed_modules = install_default_stdlib(fetcher, Some(installed_modules)).await?;

    Ok(installed_modules)
}

async fn parse_grit_module(
    installed_modules: &mut HashSet<String>,
    processing_modules: &mut Vec<ModuleRepo>,
    module: &ModuleRepo,
    repo_dir: &str,
) -> Result<()> {
    installed_modules.insert(module.provider_name.clone());
    let module_config = match read_grit_yaml(&PathBuf::from_str(repo_dir).unwrap()).await {
        Some(config) => config,
        None => {
            return Ok(());
        }
    };
    let referenced_modules = extract_grit_modules(&module_config.content, &module_config.path)?;
    for referenced_module in referenced_modules {
        let referenced_module = ModuleRepo::from_repo_str(&referenced_module)?;
        processing_modules.push(referenced_module);
    }

    Ok(())
}

#[cfg(test)]
mod tests {

    use crate::{fetcher::CleanFetcher, test::initialize_grit};

    use super::*;
    use tempfile::tempdir;

    #[tokio::test]
    async fn install_default_stdlibs() {
        let dir = tempdir().unwrap();
        let grit_dir = dir.path().join(".grit");
        let grit_module_dir = grit_dir.join(".gritmodules");
        let config = r#"version: 0.0.1
patterns: []"#;

        initialize_grit(&dir, config).await.unwrap();

        let fetcher = CleanFetcher::new(grit_module_dir, None);
        let res = install_default_stdlib(&fetcher, None).await;
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn install_multi_level_grit_modules() {
        let dir = tempdir().unwrap();
        let grit_dir = dir.path().join(".grit");
        let grit_module_dir = grit_dir.join(".gritmodules");
        let config = r#"version: 0.0.1
patterns:
  - name: github.com/custodian-sample-org/testrepo-C#*
    level: info"#;

        initialize_grit(&dir, config).await.unwrap();

        let fetcher = CleanFetcher::new(grit_module_dir, None);
        let curr_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/rewriter".to_string(),
            remote: "https://github.com/getgrit/rewriter.git".to_string(),
            provider_name: "github.com/getgrit/rewriter".to_string(),
        };
        let curr_dir = dir.path().to_str().unwrap();

        let installed = install_grit_modules(&fetcher, &curr_repo, curr_dir)
            .await
            .unwrap();
        let mut exp: HashSet<String> = HashSet::new();
        exp.insert("github.com/getgrit/stdlib".to_string());
        exp.insert("github.com/custodian-sample-org/testrepo-A".to_string());
        exp.insert("github.com/custodian-sample-org/testrepo-B".to_string());
        exp.insert("github.com/custodian-sample-org/testrepo-C".to_string());
        exp.insert("github.com/getgrit/rewriter".to_string());
        assert_eq!(installed, exp);
    }

    #[tokio::test]
    async fn install_simple_grit_modules() {
        let dir = tempdir().unwrap();
        let grit_dir = dir.path().join(".grit");
        let grit_module_dir = grit_dir.join(".gritmodules");
        let basic_yaml = r#"version: 0.0.1
patterns:
  - name: github.com/getgrit/js#*
    level: info"#;

        initialize_grit(&dir, basic_yaml).await.unwrap();

        let fetcher = CleanFetcher::new(grit_module_dir, None);
        let curr_repo = ModuleRepo {
            host: "github.com".to_string(),
            full_name: "getgrit/rewriter".to_string(),
            remote: "https://github.com/getgrit/rewriter.git".to_string(),
            provider_name: "github.com/getgrit/rewriter".to_string(),
        };
        let curr_dir = dir.path().to_str().unwrap();
        let installed = install_grit_modules(&fetcher, &curr_repo, curr_dir)
            .await
            .unwrap();
        let mut exp: HashSet<String> = HashSet::new();
        exp.insert("github.com/getgrit/js".to_string());
        exp.insert("github.com/getgrit/rewriter".to_string());
        exp.insert("github.com/getgrit/stdlib".to_string());
        assert_eq!(installed, exp);
    }
}
