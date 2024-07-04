/// API for the gritmodule
/// This module exposes functions that should be callable anywhere, including from the bridge
use std::path::Path;

use anyhow::Result;

use crate::{
    config::GritConfig,
    yaml::{get_grit_config, read_grit_yaml},
};

pub async fn read_grit_config(repo_dir: &Path) -> Result<Option<GritConfig>> {
    let yaml_content = read_grit_yaml(repo_dir).await;
    match yaml_content {
        None => Ok(None),
        Some(yaml_content) => {
            let config = get_grit_config(
                &yaml_content.content,
                &yaml_content.path,
                &Some(repo_dir.to_string_lossy().to_string()),
            )?;
            Ok(Some(config))
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{path::PathBuf, str::FromStr};

    use super::*;

    #[tokio::test]
    async fn reads_yaml() {
        let dir: PathBuf = PathBuf::from_str("fixtures/enforcement_level").unwrap();
        let config = read_grit_config(&dir).await.unwrap().unwrap();
        println!("{:?}", config);
        assert_eq!(config.patterns.len(), 3);
    }
}
