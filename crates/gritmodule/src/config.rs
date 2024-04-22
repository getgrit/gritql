use grit_util::{Position, Range};
use marzano_core::api::EnforcementLevel;
use marzano_language::{grit_parser::MarzanoGritParser, target_language::PatternLanguage};
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    path::{Path, PathBuf},
    vec::Vec,
};

use crate::{fetcher::ModuleRepo, parser::PatternFileExt, utils::is_pattern_name};
use anyhow::{bail, Result};

#[derive(Debug, Deserialize)]
pub struct GritGitHubConfig {
    #[serde(default)]
    pub reviewers: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct GritConfig {
    pub patterns: Vec<GritDefinitionConfig>,
    pub github: Option<GritGitHubConfig>,
}

#[derive(Debug, Deserialize)]
pub struct SerializedGritConfig {
    pub patterns: Vec<GritSerializedDefinitionConfig>,
    pub github: Option<GritGitHubConfig>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub enum PatternVisibility {
    #[serde(rename = "public")]
    #[default]
    Public,
    #[serde(rename = "private")]
    Private,
}

/// Core Grit metadata for a pattern (defined in yaml or markdown)
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GritPatternMetadata {
    pub level: Option<EnforcementLevel>,
    pub title: Option<String>,
    pub description: Option<String>,
    pub tags: Option<Vec<String>>,
}

/// This contains the raw pattern data
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct RawGritDefinition {
    pub format: PatternFileExt,
    pub content: String,
}

/// This is the pure implementation of a pattern definition, which can be picked up from any source
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GritDefinitionConfig {
    pub name: String,
    pub body: Option<String>,
    #[serde(flatten)]
    pub(crate) meta: GritPatternMetadata,
    #[serde(skip)]
    pub kind: Option<DefinitionKind>,
    pub samples: Option<Vec<GritPatternSample>>,
    pub path: String,
    pub position: Option<Position>,
    pub raw: Option<RawGritDefinition>,
}

impl GritDefinitionConfig {
    pub fn from_serialized(serialized: GritSerializedDefinitionConfig, path: String) -> Self {
        Self {
            name: serialized.name,
            body: serialized.body,
            meta: serialized.meta,
            kind: serialized.kind,
            samples: serialized.samples,
            path,
            position: None,
            raw: None,
        }
    }
}

/// This is a variation of GritDefinitionConfig that is *only* sourced from yaml.
/// It excludes fields which cannot be specified directly in the yaml
#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GritSerializedDefinitionConfig {
    pub name: String,
    pub body: Option<String>,
    #[serde(flatten)]
    pub(crate) meta: GritPatternMetadata,
    #[serde(skip)]
    pub kind: Option<DefinitionKind>,
    pub samples: Option<Vec<GritPatternSample>>,
}

#[derive(Deserialize, Serialize)]
pub struct GritPatternTestConfig {
    pub path: Option<String>,
    pub samples: Option<Vec<GritPatternSample>>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GritPatternTestInfo {
    pub body: String,
    pub config: GritPatternTestConfig,
    pub local_name: Option<String>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
pub struct ModuleGritPattern {
    pub config: GritDefinitionConfig,
    pub module: Option<ModuleRepo>,
    pub local_name: String,
    #[serde(skip)]
    pub(crate) visibility: PatternVisibility,
}

impl ModuleGritPattern {
    pub fn language(&self, parser: &mut MarzanoGritParser) -> Option<PatternLanguage> {
        let Some(body) = self.config.body.as_ref() else {
            return None;
        };
        Some(PatternLanguage::get_language_with_parser(parser, body).unwrap_or_default())
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct GritUserConfig {
    pub path: PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum DefinitionSource {
    Module(ModuleRepo),
    Config(GritUserConfig),
}

impl DefinitionSource {
    pub fn name(&self) -> String {
        match self {
            DefinitionSource::Module(module) => module.provider_name.to_string(),
            DefinitionSource::Config(config) => config.path.to_string_lossy().to_string(),
        }
    }

    pub fn short_name(&self) -> String {
        match self {
            DefinitionSource::Module(module) => module.full_name.to_string(),
            DefinitionSource::Config(config) => config.path.to_string_lossy().to_string(),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResolvedGritDefinition {
    pub config: GritDefinitionConfig,
    pub module: DefinitionSource,
    pub local_name: String,
    pub body: String,
    pub kind: DefinitionKind,
    pub language: PatternLanguage,
    pub visibility: PatternVisibility,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DefinitionKind {
    #[default]
    Pattern,
    Predicate,
    Function,
}

static EMPTY_VEC: Vec<String> = Vec::new();

impl ResolvedGritDefinition {
    pub fn is_visible(&self) -> bool {
        self.visibility == PatternVisibility::Public && matches!(self.kind, DefinitionKind::Pattern)
    }

    // Fetch a URL where the pattern is defined
    // This is currently a file:// URL, but could be an HTTP URL in the future
    pub fn url(&self, local_repo: &ModuleRepo, local_path: &Path) -> String {
        let base_url = match &self.module {
            DefinitionSource::Module(ref module) => {
                if module == local_repo {
                    local_path.join(&self.config.path)
                } else {
                    let gritmodule_path = format!(
                        "{}/{}/{}",
                        REPO_CONFIG_DIR_NAME, GRIT_MODULE_DIR, module.provider_name
                    );
                    let gritmodule_path = PathBuf::from(gritmodule_path);
                    local_path.join(gritmodule_path).join(&self.config.path)
                }
            }
            DefinitionSource::Config(config) => config.path.clone(),
        };
        format!("file://{}", base_url.to_string_lossy())
    }

    // A name usable in check outputs or other automated tools
    pub fn name(&self) -> &str {
        &self.local_name
    }

    // Retrieve the level from the config, or fall back to a default
    pub fn level(&self) -> EnforcementLevel {
        self.config
            .meta
            .level
            .as_ref()
            .unwrap_or(&EnforcementLevel::default())
            .to_owned()
    }

    // Retrieve the description from the config, if any
    pub fn description(&self) -> Option<&str> {
        self.config.meta.description.as_deref()
    }

    // A longer title for displaying the pattern
    pub fn title(&self) -> Option<&str> {
        self.config.meta.title.as_deref()
    }

    // Retrieve tags
    pub fn tags(&self) -> &Vec<String> {
        self.config.meta.tags.as_ref().unwrap_or(&EMPTY_VEC)
    }
}

impl PartialEq for ResolvedGritDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.local_name == other.local_name
    }
}

impl Eq for ResolvedGritDefinition {}

impl PartialOrd for ResolvedGritDefinition {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ResolvedGritDefinition {
    fn cmp(&self, other: &Self) -> Ordering {
        self.config.name.cmp(&other.config.name)
    }
}

pub fn pattern_config_to_model(
    pattern: GritDefinitionConfig,
    source: &ModuleRepo,
) -> Result<ModuleGritPattern> {
    let mut split_name = pattern.name.split('#');
    let repo = split_name.next();
    let defined_local_name = split_name.next();
    let local_name = defined_local_name.unwrap_or(&pattern.name).to_string();

    if !is_pattern_name(&local_name) && local_name != NAMESPACE_IMPORT_INDICATOR {
        bail!("Invalid pattern name: {}. Grit patterns must match the regex /[\\^#A-Za-z_][A-Za-z0-9_]*/. For more info, consult the docs at https://docs.grit.io/guides/patterns#pattern-definitions.", local_name);
    }

    let module: Option<ModuleRepo> = match repo {
        None => None,
        Some(_) => {
            let mut split_repo = repo.unwrap().split('/');
            let host = split_repo.next();
            let full_name = if host.is_none() {
                None
            } else {
                Some(split_repo.collect::<Vec<_>>().join("/"))
            };
            if defined_local_name.is_none() {
                Some(source.clone())
            } else if host.is_none() || full_name.is_none() {
                None
            } else {
                Some(ModuleRepo::from_host_repo(
                    host.unwrap(),
                    &full_name.unwrap(),
                )?)
            }
        }
    };

    let model = ModuleGritPattern {
        config: pattern,
        module,
        local_name,
        ..Default::default()
    };

    Ok(model)
}

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct GritPatternSample {
    pub name: Option<String>,
    pub input: String,
    pub output: Option<String>,
    pub input_range: Option<Range>,
    pub output_range: Option<Range>,
}

pub const GRIT_GLOBAL_DIR_ENV: &str = "GRIT_GLOBAL_DIR";
pub const REPO_CONFIG_DIR_NAME: &str = ".grit";
pub const CONFIG_FILE_NAMES: [&str; 2] = ["grit.yml", "grit.yaml"];
pub const REPO_CONFIG_PATTERNS_DIR: &str = "patterns";
pub const GRIT_MODULE_DIR: &str = ".gritmodules";
pub const NAMESPACE_IMPORT_INDICATOR: &str = "*";

pub fn is_namespace_import(pattern: &ModuleGritPattern) -> bool {
    pattern.local_name == NAMESPACE_IMPORT_INDICATOR
}

pub const DEFAULT_STDLIBS: [&str; 1] = ["https://github.com/getgrit/stdlib.git"];

pub fn get_stdlib_modules() -> Vec<ModuleRepo> {
    DEFAULT_STDLIBS
        .map(|s| ModuleRepo::from_remote(s).unwrap())
        .to_vec()
}
