use anyhow::{bail, Result};
use grit_util::Position;
use marzano_util::rich_path::RichFile;
use std::{
    collections::HashSet,
    path::{Path, PathBuf},
};
use tokio::{fs, task::JoinSet};

use crate::{
    config::{
        pattern_config_to_model, DefinitionKind, GritConfig, GritDefinitionConfig,
        ModuleGritPattern, SerializedGritConfig, CONFIG_FILE_NAMES, REPO_CONFIG_DIR_NAME,
    },
    fetcher::ModuleRepo,
    parser::{extract_relative_file_path, get_patterns_from_file, PatternFileExt},
};

pub fn get_grit_config(source: &str, source_path: &str) -> Result<GritConfig> {
    let serialized: SerializedGritConfig = match serde_yaml::from_str(source) {
        Ok(config) => config,
        Err(err) => {
            bail!(
                "Invalid configuration file '{}': {}",
                source_path,
                err.to_string()
            )
        }
    };

    let mut patterns = Vec::new();
    let mut pattern_files = Vec::new();

    for pattern in serialized.patterns.into_iter() {
        match pattern {
            crate::config::GritPatternConfig::File(file) => {
                pattern_files.push(file);
            }
            crate::config::GritPatternConfig::Pattern(p) => {
                patterns.push(GritDefinitionConfig::from_serialized(
                    p,
                    source_path.to_string(),
                ));
            }
        }
    }

    let new_config = GritConfig {
        github: serialized.github,
        pattern_files: if pattern_files.is_empty() {
            None
        } else {
            Some(pattern_files)
        },
        patterns,
    };

    Ok(new_config)
}

pub async fn get_patterns_from_yaml(
    file: &RichFile,
    source_module: Option<&ModuleRepo>,
    root: &Option<String>,
    repo_dir: &str,
) -> Result<Vec<ModuleGritPattern>> {
    let grit_path = extract_relative_file_path(file, root);
    let mut config = get_grit_config(&file.content, &grit_path)?;

    for pattern in config.patterns.iter_mut() {
        pattern.kind = Some(DefinitionKind::Pattern);
        let offset = file.content.find(&pattern.name).unwrap_or(0);
        pattern.position = Some(Position::from_byte_index(&file.content, offset));
    }

    let patterns: Result<Vec<_>> = config
        .patterns
        .into_iter()
        .map(|pattern| pattern_config_to_model(pattern, source_module))
        .collect();
    let mut patterns = patterns?;

    if config.pattern_files.is_none() {
        return Ok(patterns);
    }

    // let mut file_readers = JoinSet::new();

    // for pattern_file in config.pattern_files.unwrap() {
    //     let pattern_file = PathBuf::from(repo_dir)
    //         .join(REPO_CONFIG_DIR_NAME)
    //         .join(&pattern_file.file);
    //     let extension = PatternFileExt::from_path(&pattern_file);
    //     if extension.is_none() {
    //         continue;
    //     }
    //     let extension = extension.unwrap();
    //     let source_module = source_module.clone();
    //     file_readers.spawn(async move {
    //         get_patterns_from_file(pattern_file, source_module.cloned(), extension).await
    //     });
    // }

    // while let Some(res) = file_readers.join_next().await {
    //     let this_patterns = res??;
    //     patterns.extend(this_patterns);
    // }

    Ok(patterns)
}

pub fn extract_grit_modules(content: &str, path: &str) -> Result<Vec<String>> {
    let config = get_grit_config(content, path)?;

    let mut unique_names: HashSet<String> = HashSet::new();

    for pattern in config.patterns {
        if let Some(hash_index) = pattern.name.find('#') {
            unique_names.insert(pattern.name[..hash_index].to_string());
        }
    }

    Ok(unique_names.into_iter().collect())
}

pub async fn read_grit_yaml(repo_dir: &Path) -> Option<RichFile> {
    let grit_dir = repo_dir.join(REPO_CONFIG_DIR_NAME);

    for config_file_name in CONFIG_FILE_NAMES.iter() {
        let file_path = grit_dir.join(config_file_name);
        if let Ok(content) = fs::read_to_string(&file_path).await {
            let rich_file = RichFile {
                path: file_path.to_string_lossy().to_string(),
                content,
            };
            return Some(rich_file);
        }
    }

    None
}

#[cfg(test)]
mod tests {
    use insta::assert_yaml_snapshot;

    use super::*;

    #[test]
    fn gets_grit_modules() {
        let grit_yaml = r#"version: 0.0.1
patterns:
  - name: github.com/getgrit/stdlib#*
    level: info
  - name: github.com/getgrit/json#no_console_log
    level: error
  - name: github.com/getgrit/json#strict_tsconfig
    level: error
  - name: remove_console_error
    level: error
    body: |
      engine marzano(0.1)
      language js

      `console.error($_)` => .
    "#;
        let gritmodules = extract_grit_modules(grit_yaml, ".grit/grit.yaml").unwrap();
        let gritmodule_set: HashSet<_> = gritmodules.into_iter().collect();
        let expected_set: HashSet<_> = vec![
            "github.com/getgrit/stdlib".to_string(),
            "github.com/getgrit/json".to_string(),
        ]
        .into_iter()
        .collect();

        assert_eq!(gritmodule_set.len(), 2);
        assert_eq!(gritmodule_set, expected_set);
    }

    #[test]
    fn invalid_grit_yaml() {
        let grit_yaml = "invalid config";
        let gritmodules = extract_grit_modules(grit_yaml, ".grit/grit.yaml");
        if let Err(e) = gritmodules {
            assert!(e.to_string().contains("Invalid configuration file"));
        } else {
            panic!("Expected error");
        }
    }

    #[tokio::test]
    async fn gets_module_patterns() {
        let grit_yaml = RichFile {
            path: String::new(),
            content: r#"version: 0.0.1
patterns:
  - name: github.com/getgrit/js#*
    level: info
  - name: github.com/getgrit/json#no_console_log
    level: error
  - name: github.com/getgrit/json#strict_tsconfig
    level: info
  - name: remove_console_error
    level: error
    body: |
      engine marzano(0.1)
      language js

      `console.error($_)` => .
github:
  reviewers:
  - morgante
  - gritagent
    "#
            .to_string(),
        };
        let repo = Default::default();
        let patterns = get_patterns_from_yaml(&grit_yaml, &repo, &None, "getgrit/rewriter")
            .await
            .unwrap();
        assert_eq!(patterns.len(), 4);
        assert_yaml_snapshot!(patterns);
    }

    #[test]
    fn gets_github_reviewers() {
        let grit_yaml = r#"version: 0.1.0
patterns: []
github:
  reviewers:
  - morgante
  - gritagent
    "#;
        let config = get_grit_config(grit_yaml, ".grit/grit.yaml").unwrap();
        println!("{:?}", config);
        assert_eq!(config.github.unwrap().reviewers.len(), 2);
    }
}
