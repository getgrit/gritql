use std::{
    fmt,
    path::{Path, PathBuf},
};

use crate::{
    config::{ModuleGritPattern, REPO_CONFIG_DIR_NAME},
    dot_grit::get_patterns_from_grit,
    fetcher::ModuleRepo,
    markdown::get_patterns_from_md,
    searcher::find_repo_root_from,
};
use anyhow::{Context, Result};
use marzano_util::rich_path::RichFile;
use tokio::fs;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, serde::Serialize, serde::Deserialize,
)]
pub enum PatternFileExt {
    #[serde(rename = "grit")]
    Grit,
    #[serde(rename = "markdown")]
    Md,
}

impl fmt::Display for PatternFileExt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PatternFileExt::Grit => write!(f, "grit"),
            PatternFileExt::Md => write!(f, "md"),
        }
    }
}

impl PatternFileExt {
    pub fn from_path(file_name: &Path) -> Option<PatternFileExt> {
        let ext = file_name.extension()?.to_str()?;
        match ext {
            "grit" => Some(PatternFileExt::Grit),
            "md" => Some(PatternFileExt::Md),
            _ => None,
        }
    }

    fn get_patterns(
        &self,
        file: &mut RichFile,
        source_module: &Option<ModuleRepo>,
        root: &Option<String>,
    ) -> Result<Vec<ModuleGritPattern>, anyhow::Error> {
        match self {
            PatternFileExt::Grit => {
                get_patterns_from_grit(file, source_module, root).with_context(|| {
                    format!(
                        "Failed to parse .grit pattern {}",
                        extract_relative_file_path(file, root)
                    )
                })
            }
            PatternFileExt::Md => {
                get_patterns_from_md(file, source_module, root).with_context(|| {
                    format!(
                        "Failed to parse markdown pattern {}",
                        extract_relative_file_path(file, root)
                    )
                })
            }
        }
    }

    pub fn get_ext(&self) -> &str {
        match self {
            PatternFileExt::Grit => "grit",
            PatternFileExt::Md => "md",
        }
    }
}

pub async fn get_patterns_from_file(
    path: PathBuf,
    source_module: Option<ModuleRepo>,
    ext: PatternFileExt,
) -> Result<Vec<ModuleGritPattern>> {
    let repo_root = find_repo_root_from(path.clone()).await?;
    let content = fs::read_to_string(&path).await?;
    let mut file = RichFile {
        path: path.to_string_lossy().to_string(),
        content,
    };
    ext.get_patterns(&mut file, &source_module, &repo_root)
}

pub fn extract_relative_file_path(file: &RichFile, root: &Option<String>) -> String {
    extract_relative_path(&file.path, root)
}

pub fn extract_relative_path(path: &str, root: &Option<String>) -> String {
    if let Some(root) = root {
        let root_path = Path::new(root);
        let file_path = Path::new(path);
        let relative_path = file_path.strip_prefix(root_path).unwrap_or(file_path);
        return relative_path.to_string_lossy().to_string();
    }
    let search_pattern = REPO_CONFIG_DIR_NAME.to_string();
    let start_index = path.find(&search_pattern).unwrap_or(0);
    path[start_index..].to_string()
}
