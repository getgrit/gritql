use crate::{
    config::{DefinitionSource, GritUserConfig},
    fetcher::GritModuleFetcher,
};
use std::{
    collections::{HashMap, HashSet},
    env,
    path::PathBuf,
    str::FromStr,
};

use crate::{
    config::{
        is_namespace_import, ModuleGritPattern, ResolvedGritDefinition, GRIT_MODULE_DIR,
        REPO_CONFIG_DIR_NAME, REPO_CONFIG_PATTERNS_DIR,
    },
    fetcher::{FetcherType, ModuleRepo},
    installer::{install_default_stdlib, install_grit_modules},
    parser::PatternFileExt,
    patterns_directory::PatternsDirectory,
    searcher::{collect_patterns, find_repo_root_from},
    yaml::{get_patterns_from_yaml, read_grit_yaml},
};
use anyhow::{bail, Context, Result};
use homedir::get_my_home;
use marzano_language::{grit_parser::MarzanoGritParser, target_language::PatternLanguage};
use tokio::{fs, join};

pub async fn find_local_patterns(
    module: &ModuleRepo,
    grit_parent_dir: &str,
) -> Result<Vec<ResolvedGritDefinition>> {
    let mut resolved_patterns = HashMap::new();
    let mut errored_patterns = HashMap::new();
    resolve_patterns_for_module(
        &Some(module.clone()),
        grit_parent_dir,
        &mut resolved_patterns,
        &mut errored_patterns,
        &mut Vec::new(),
    )
    .await?;
    Ok(resolved_patterns
        .into_values()
        .flat_map(|v| v.into_values().collect::<Vec<_>>())
        .collect())
}

pub async fn find_user_patterns() -> Result<Vec<ResolvedGritDefinition>> {
    let mut resolved_patterns: HashMap<String, HashMap<String, ResolvedGritDefinition>> =
        HashMap::new();
    let mut errored_patterns = HashMap::new();
    if let Some(user_dir) = find_user_config_dir() {
        let user_grit = user_dir.join(REPO_CONFIG_DIR_NAME);
        let user_dir = user_dir.to_string_lossy().to_string();
        let user_patterns = resolve_patterns_for_module(
            &None,
            &user_dir,
            &mut HashMap::new(),
            &mut errored_patterns,
            &mut vec![],
        )
        .await?;
        for (local_name, patterns) in user_patterns {
            for pattern in patterns {
                let language = pattern.language(&mut MarzanoGritParser::new()?).unwrap();
                let language_string = language.to_string();
                let local_name_map = resolved_patterns.entry(local_name.clone()).or_default();
                let kind = pattern.config.kind.as_ref().cloned().unwrap_or_default();
                let body = pattern.config.body.clone().unwrap();
                let definition = ResolvedGritDefinition {
                    config: pattern.config,
                    module: DefinitionSource::Config(GritUserConfig {
                        path: user_grit.clone(),
                    }),
                    local_name: local_name.clone(),
                    body,
                    language,
                    kind,
                    visibility: pattern.visibility,
                };
                local_name_map.insert(language_string, definition);
            }
        }
    }
    Ok(resolved_patterns
        .into_values()
        .flat_map(|v| v.into_values().collect::<Vec<_>>())
        .collect())
}

pub async fn fetch_modules<T: FetcherType>(
    module: &ModuleRepo,
    grit_parent_dir: &str,
) -> Result<()> {
    let as_path = PathBuf::from_str(grit_parent_dir).unwrap();
    let grit_dir = as_path.join(REPO_CONFIG_DIR_NAME);

    // Since git cloning is slow, two processes can try to clone at the same time and cause issues because they are overwriting each other
    // To avoid this, we create a random dir name and move it to the actual gritmodules dir after cloning is complete
    // The move is ~atomic so it should be safe
    let use_random = false;
    let clone_dir = if use_random {
        let random_suffix = rand::random::<u32>();
        grit_dir.join(format!("{}_{}", GRIT_MODULE_DIR, random_suffix))
    } else {
        grit_dir.join(GRIT_MODULE_DIR)
    };

    let token = env::var("GRIT_PROVIDER_TOKEN").ok();
    let fetcher = T::make_fetcher(clone_dir.clone(), token);

    fetcher.prep_grit_modules()?;

    let no_custom_patterns: bool = !dir_has_config(as_path).await;

    if no_custom_patterns {
        match install_default_stdlib(&fetcher, None).await {
            std::result::Result::Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        };
    } else {
        match install_grit_modules(&fetcher, module, grit_parent_dir).await {
            Ok(_) => {}
            Err(err) => {
                return Err(err);
            }
        };
    };

    // Move the clone dir to the actual gritmodules dir
    if use_random {
        let grit_modules_dir = grit_dir.join(GRIT_MODULE_DIR);
        match fs::rename(&clone_dir, &grit_modules_dir).await {
            Ok(_) => {}
            Err(err) => {
                // This requires the io_error_more feature, use raw code for now
                // if err.kind() != std::io::ErrorKind::DirectoryNotEmpty {
                if err.raw_os_error() != Some(66) {
                    return Err(err.into());
                }
            }
        };
    }

    Ok(())
}

async fn dir_has_config(grit_parent_dir: PathBuf) -> bool {
    let patterns_dir = grit_parent_dir
        .join(REPO_CONFIG_DIR_NAME)
        .join(REPO_CONFIG_PATTERNS_DIR);

    if read_grit_yaml(&grit_parent_dir).await.is_some() {
        return true;
    }

    let metadata = match fs::metadata(&patterns_dir).await {
        Ok(metadata) => metadata,
        Err(_) => return false,
    };

    if !metadata.is_dir() {
        return false;
    }

    let mut entries = match fs::read_dir(patterns_dir).await {
        Ok(entries) => entries,
        Err(_) => return false,
    };

    while let Some(entry) = entries.next_entry().await.unwrap_or(None) {
        if let Ok(file) = entry.file_type().await {
            if file.is_file() {
                if let Some(extension) = entry.path().extension() {
                    if extension == "md" || extension == "grit" {
                        return true;
                    }
                }
            }
        }
    }

    false
}

fn find_user_config_dir() -> Option<PathBuf> {
    let user_dir = match env::var("GRIT_USER_CONFIG").ok() {
        Some(user_grit) => {
            let user_path = PathBuf::from_str(&user_grit).unwrap();
            Some(
                user_path
                    .parent()
                    .map(|p| p.to_path_buf())
                    .unwrap_or(user_path),
            )
        }
        None => match get_my_home() {
            Ok(user_dir) => user_dir,
            Err(_) => None,
        },
    };
    if let Some(user_dir) = user_dir {
        let user_grit = user_dir.join(REPO_CONFIG_DIR_NAME);
        if user_grit.exists() {
            Some(user_dir)
        } else {
            None
        }
    } else {
        None
    }
}

pub async fn get_grit_files(
    module: &ModuleRepo,
    grit_parent_dir: &str,
    must_process: Option<Vec<ModuleRepo>>,
) -> Result<PatternsDirectory> {
    let mut processing_modules: Vec<ModuleRepo> = must_process.unwrap_or_default();
    let mut processed_modules: HashSet<String> = HashSet::new();
    let mut grit_files: PatternsDirectory = PatternsDirectory::new();

    get_grit_files_for_module(
        &Some(module.clone()),
        grit_parent_dir,
        &mut grit_files,
        &mut processing_modules,
    )
    .await?;
    processed_modules.insert(module.provider_name.clone());

    while let Some(module) = processing_modules.pop() {
        if processed_modules.contains(&module.provider_name) {
            continue;
        }
        let repo_dir = PathBuf::from_str(grit_parent_dir)
            .unwrap()
            .join(REPO_CONFIG_DIR_NAME)
            .join(GRIT_MODULE_DIR)
            .join(module.provider_name.clone())
            .to_string_lossy()
            .to_string();
        let provider_name = module.provider_name.clone();
        get_grit_files_for_module(
            &Some(module),
            &repo_dir,
            &mut grit_files,
            &mut processing_modules,
        )
        .await?;
        processed_modules.insert(provider_name);
    }

    if let Some(user_dir) = find_user_config_dir() {
        let user_dir = user_dir.to_string_lossy().to_string();
        get_grit_files_for_module(&None, &user_dir, &mut grit_files, &mut processing_modules)
            .await?;
    }

    Ok(grit_files)
}

pub async fn resolve_patterns(
    module: &ModuleRepo,
    grit_parent_dir: &str,
    must_process: Option<Vec<ModuleRepo>>,
) -> Result<(Vec<ResolvedGritDefinition>, HashMap<String, String>)> {
    let mut resolved_patterns: HashMap<String, HashMap<String, ResolvedGritDefinition>> =
        HashMap::new();
    let mut errored_patterns: HashMap<String, String> = HashMap::new();
    let mut remote_references: HashMap<String, HashMap<String, Vec<ModuleGritPattern>>> =
        HashMap::new();
    let mut processing_modules: Vec<ModuleRepo> = must_process.unwrap_or_default();
    let mut processed_modules: HashSet<String> = HashSet::new();

    let as_path = PathBuf::from_str(grit_parent_dir).unwrap();
    let return_all_patterns: bool = !dir_has_config(as_path).await;

    let specified_required = resolve_patterns_for_module(
        &Some(module.clone()),
        grit_parent_dir,
        &mut resolved_patterns,
        &mut errored_patterns,
        &mut processing_modules,
    )
    .await?;
    processed_modules.insert(module.provider_name.clone());

    while let Some(module) = processing_modules.pop() {
        if processed_modules.contains(&module.provider_name) {
            continue;
        }
        let repo_dir = PathBuf::from_str(grit_parent_dir)
            .unwrap()
            .join(REPO_CONFIG_DIR_NAME)
            .join(GRIT_MODULE_DIR)
            .join(module.provider_name.clone())
            .to_string_lossy()
            .to_string();
        let provider_name = module.provider_name.clone();
        let res = resolve_patterns_for_module(
            &Some(module),
            &repo_dir,
            &mut resolved_patterns,
            &mut errored_patterns,
            &mut processing_modules,
        )
        .await?;
        processed_modules.insert(provider_name.clone());
        remote_references.insert(provider_name, res);
    }

    if let Some(user_dir) = find_user_config_dir() {
        let user_grit = user_dir.join(REPO_CONFIG_DIR_NAME);
        let user_dir = user_dir.to_string_lossy().to_string();
        let user_patterns = resolve_patterns_for_module(
            &None,
            &user_dir,
            &mut HashMap::new(),
            &mut errored_patterns,
            &mut vec![],
        )
        .await?;
        for (local_name, patterns) in user_patterns {
            for pattern in patterns {
                let language = pattern.language(&mut MarzanoGritParser::new()?).unwrap();
                let language_string = language.to_string();
                let local_name_map = resolved_patterns.entry(local_name.clone()).or_default();
                // only insert if there is not already an entry - we never want user patterns to override repo patterns
                local_name_map.entry(language_string).or_insert_with(|| {
                    let kind = pattern.config.kind.as_ref().cloned().unwrap_or_default();
                    let body = pattern.config.body.clone().unwrap();
                    ResolvedGritDefinition {
                        config: pattern.config,
                        module: DefinitionSource::Config(GritUserConfig {
                            path: user_grit.clone(),
                        }),
                        local_name: local_name.clone(),
                        body,
                        language,
                        kind,
                        visibility: pattern.visibility,
                    }
                });
            }
        }
    }

    if return_all_patterns {
        return Ok((
            resolved_patterns
                .into_values()
                .flat_map(|v| v.into_values().collect::<Vec<_>>())
                .collect(),
            errored_patterns,
        ));
    }

    let mut our_patterns = Vec::new();

    for (local_name, pattern) in
        specified_required
            .into_iter()
            .flat_map(|(local_name, patterns)| {
                patterns
                    .into_iter()
                    .map(move |pattern| (local_name.clone(), pattern))
            })
    {
        if is_namespace_import(&pattern) {
            if let Some(referenced) = pattern.module.as_ref() {
                resolve_namespace_import(
                    referenced,
                    &remote_references,
                    &resolved_patterns,
                    &mut our_patterns,
                )
                .await?;
            }
        } else {
            let resolved = if let Some(resolved) = resolved_patterns.get(&local_name) {
                if let Some(body) = &pattern.config.body {
                    resolved.values().find(|p| {
                        if p.body != *body {
                            return false;
                        }
                        if let DefinitionSource::Module(m) = &p.module {
                            *m == *module
                        } else {
                            false
                        }
                    })
                } else {
                    let found = resolved.values().find(|p| {
                        if let Some(name) = pattern.config.name.split('#').next() {
                            let module_repo = match ModuleRepo::from_repo_str(name) {
                                Ok(module_repo) => module_repo,
                                Err(_) => {
                                    return false;
                                }
                            };
                            p.module == DefinitionSource::Module(module_repo)
                        } else {
                            false
                        }
                    });
                    match found {
                        Some(resolved) => Some(resolved),
                        None => resolved.values().next(),
                    }
                }
            } else {
                None
            };
            match resolved {
                Some(resolved) => {
                    let merged = merge_local_with_remote(pattern, resolved.clone());
                    our_patterns.push(merged);
                }
                None => {
                    let error = format!("Unable to resolve pattern {}", pattern.config.name);
                    errored_patterns.insert(local_name, error);
                }
            }
        }
    }

    our_patterns.extend(
        resolved_patterns
            .into_values()
            .flat_map(|v| v.into_values().collect::<Vec<_>>())
            .filter(|p| matches!(p.module, DefinitionSource::Config(_))),
    );

    Ok((our_patterns, errored_patterns))
}

async fn resolve_namespace_import(
    namespace_imported: &ModuleRepo,
    remote_references: &HashMap<String, HashMap<String, Vec<ModuleGritPattern>>>,
    resolved_patterns: &HashMap<String, HashMap<String, ResolvedGritDefinition>>,
    our_patterns: &mut Vec<ResolvedGritDefinition>,
) -> Result<()> {
    let mut stack = Vec::new();
    stack.push(namespace_imported.clone());

    while let Some(current_module) = stack.pop() {
        let referenced_patterns = remote_references.get(&current_module.provider_name);
        match referenced_patterns {
            Some(referenced_patterns) => {
                for (local_name, referenced_pattern) in
                    referenced_patterns
                        .iter()
                        .flat_map(|(local_name, patterns)| {
                            patterns
                                .iter()
                                .map(move |pattern| (local_name.clone(), pattern))
                        })
                {
                    if is_namespace_import(referenced_pattern) {
                        let referenced_module = referenced_pattern.module.as_ref().unwrap().clone();
                        stack.push(referenced_module);
                    } else {
                        let resolved = match resolved_patterns.get(&local_name) {
                            Some(resolved) => {
                                match resolved.values().find(|p| {
                                    if let DefinitionSource::Module(m) = &p.module {
                                        if *m != current_module {
                                            return false;
                                        }
                                    } else {
                                        return false;
                                    }
                                    referenced_pattern.config.body.is_none()
                                        || referenced_pattern
                                            .config
                                            .body
                                            .as_ref()
                                            .is_some_and(|b| *b == p.body)
                                }) {
                                    Some(resolved) => resolved,
                                    None => match resolved.values().next() {
                                        Some(resolved) => resolved,
                                        None => {
                                            let error = format!(
                                            "Unable to resolve pattern {} required by namespace import",
                                            local_name
                                        );
                                            bail!(error);
                                        }
                                    },
                                }
                            }
                            None => {
                                let error = format!(
                                    "Unable to resolve pattern {} required by namespace import",
                                    local_name
                                );
                                bail!(error);
                            }
                        };
                        our_patterns.push(resolved.clone());
                    }
                }
            }
            None => {
                let error = format!(
                    "Unable to resolve namespace import of module {}",
                    current_module.provider_name
                );
                bail!(error);
            }
        }
    }

    Ok(())
}

// TODO instead of iterating over the files searching for each file type
// we should iterate once and call the corresponding file handler as
// we find the different file types
async fn get_grit_files_for_module(
    module: &Option<ModuleRepo>,
    repo_dir: &str,
    grit_files: &mut PatternsDirectory,
    processing_modules: &mut Vec<ModuleRepo>,
) -> Result<()> {
    let repo_path = PathBuf::from_str(repo_dir).unwrap();
    let yaml_patterns = match read_grit_yaml(&repo_path).await {
        Some(config) => {
            if let Some(module) = module {
                let repo_root = find_repo_root_from(repo_path).await?;
                get_patterns_from_yaml(&config, module, &repo_root)?
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    for yaml_pattern in yaml_patterns.iter() {
        if let Some(remote_module) = &yaml_pattern.module {
            if !module
                .as_ref()
                .is_some_and(|m| m.provider_name == remote_module.provider_name)
            {
                processing_modules.push(remote_module.clone());
            }
        }
    }

    let md_patterns = collect_patterns(repo_dir, module, PatternFileExt::Md);
    let grit_patterns = collect_patterns(repo_dir, module, PatternFileExt::Grit);
    let (md_patterns, grit_patterns) = join!(md_patterns, grit_patterns);
    let patterns = yaml_patterns
        .into_iter()
        .chain(md_patterns?)
        .chain(grit_patterns?);

    for referenced_pattern in patterns {
        if let Some(body) = referenced_pattern.config.body {
            let language = PatternLanguage::get_language(&body);
            let key = format!("{}.grit", &referenced_pattern.local_name);
            grit_files.insert(key, body, language);
        }
    }

    Ok(())
}

async fn resolve_patterns_for_module(
    module: &Option<ModuleRepo>,
    repo_dir: &str,
    resolved_patterns: &mut HashMap<String, HashMap<String, ResolvedGritDefinition>>,
    errored_patterns: &mut HashMap<String, String>,
    processing_modules: &mut Vec<ModuleRepo>,
) -> Result<HashMap<String, Vec<ModuleGritPattern>>> {
    let mut module_patterns: HashMap<String, Vec<ModuleGritPattern>> = HashMap::new();
    let repo_path = PathBuf::from_str(repo_dir).unwrap();
    let yaml_patterns = match read_grit_yaml(&repo_path).await {
        Some(config) => {
            if let Some(module) = module {
                let repo_root = find_repo_root_from(repo_path).await?;
                get_patterns_from_yaml(&config, module, &repo_root)?
            } else {
                vec![]
            }
        }
        None => vec![],
    };

    for yaml_pattern in yaml_patterns.iter() {
        if let Some(remote_module) = &yaml_pattern.module {
            if !module
                .as_ref()
                .is_some_and(|m| m.provider_name == remote_module.provider_name)
            {
                processing_modules.push(remote_module.clone());
            }
        }
    }

    let md_patterns = collect_patterns(repo_dir, module, PatternFileExt::Md);
    let grit_patterns = collect_patterns(repo_dir, module, PatternFileExt::Grit);
    let (md_patterns, grit_patterns) = join!(md_patterns, grit_patterns);
    let md_patterns = md_patterns.context("Unable to resolve Markdown patterns")?;
    let grit_patterns = grit_patterns.context("Unable to resolve .grit patterns")?;
    let patterns = yaml_patterns
        .into_iter()
        .chain(md_patterns)
        .chain(grit_patterns);
    let mut parser = MarzanoGritParser::new()?;

    for referenced_pattern in patterns {
        if let Some(module) = referenced_pattern
            .module
            .as_ref()
            .filter(|&m| module.as_ref().is_some_and(|module| m == module))
        {
            if let Some(body) = referenced_pattern.config.body.clone() {
                let language = referenced_pattern.language(&mut parser).unwrap();
                if resolved_patterns.contains_key(&referenced_pattern.local_name) {
                    let existing = resolved_patterns
                        .get(&referenced_pattern.local_name)
                        .unwrap()
                        .get(&language.to_string());
                    if let Some(existing) = existing {
                        errored_patterns.insert(
                        referenced_pattern.local_name.to_string(),
                        format!(
                                "Pattern {} is defined multiple times in the gritmodule dependency tree, in modules {} and {}",
                                referenced_pattern.local_name,
                                existing.module.name(),
                                module.provider_name
                            )
                            .to_string(),
                        );
                        continue;
                    };
                };
                let language_string = language.to_string();
                let resolved_pattern = ResolvedGritDefinition {
                    config: referenced_pattern.config.clone(),
                    module: DefinitionSource::Module(module.clone()),
                    local_name: referenced_pattern.local_name.to_string(),
                    body,
                    language,
                    kind: referenced_pattern
                        .config
                        .kind
                        .as_ref()
                        .cloned()
                        .unwrap_or_default(),
                    visibility: referenced_pattern.visibility.clone(),
                };
                let local_name_map = resolved_patterns
                    .entry(referenced_pattern.local_name.to_string())
                    .or_default();
                local_name_map.insert(language_string, resolved_pattern.clone());
                let local_name_entry = module_patterns
                    .entry(referenced_pattern.local_name.to_string())
                    .or_default();
                local_name_entry.push(referenced_pattern);
            } else {
                errored_patterns.insert(
                    referenced_pattern.local_name.to_string(),
                    format!(
                        "Pattern {} has no body and is not imported from another module",
                        referenced_pattern.local_name
                    )
                    .to_string(),
                );
            }
        } else {
            let local_name_entry = module_patterns
                .entry(referenced_pattern.local_name.to_string())
                .or_default();
            if !local_name_entry.contains(&referenced_pattern) {
                local_name_entry.push(referenced_pattern);
            }
        }
    }

    Ok(module_patterns)
}

fn merge_local_with_remote(
    local: ModuleGritPattern,
    remote: ResolvedGritDefinition,
) -> ResolvedGritDefinition {
    let mut config = remote.config;
    if let Some(title) = local.config.meta.title {
        config.meta.title = Some(title);
    }
    if let Some(description) = local.config.meta.description {
        config.meta.description = Some(description);
    }
    if let Some(tags) = local.config.meta.tags {
        config.meta.tags = Some(tags);
    }
    if let Some(samples) = local.config.samples {
        config.samples = Some(samples);
    }
    if let Some(level) = local.config.meta.level {
        config.meta.level = Some(level);
    }
    ResolvedGritDefinition {
        config,
        module: remote.module,
        local_name: remote.local_name,
        body: remote.body,
        language: remote.language,
        kind: remote.kind,
        visibility: local.visibility,
    }
}

#[cfg(test)]
mod tests {
    use std::{
        collections::{BTreeMap, HashMap},
        path::PathBuf,
        str::FromStr,
    };

    use insta::assert_yaml_snapshot;

    use crate::{fetcher::ModuleRepo, resolver::dir_has_config};

    #[tokio::test]
    async fn resolve_single_module() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/resolver";
        let mut resolved_patterns = HashMap::new();
        let mut errored_patterns = HashMap::new();
        let remote_patterns = super::resolve_patterns_for_module(
            &Some(module_repo),
            repo_dir,
            &mut resolved_patterns,
            &mut errored_patterns,
            &mut Vec::new(),
        )
        .await
        .unwrap();
        assert_eq!(remote_patterns.len(), 9);
        assert_eq!(resolved_patterns.len(), 6);
        assert_eq!(errored_patterns.len(), 0);
        let ordered_resolved_patterns: BTreeMap<_, _> = resolved_patterns.into_iter().collect();
        assert_yaml_snapshot!(ordered_resolved_patterns);
    }

    #[tokio::test]
    async fn without_grit_yml() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/no_yaml";
        let mut resolved_patterns = HashMap::new();
        let mut errored_patterns = HashMap::new();
        let remote_patterns = super::resolve_patterns_for_module(
            &Some(module_repo),
            repo_dir,
            &mut resolved_patterns,
            &mut errored_patterns,
            &mut Vec::new(),
        )
        .await
        .unwrap();
        assert_eq!(remote_patterns.len(), 2);
        assert_eq!(resolved_patterns.len(), 2);
        assert_eq!(errored_patterns.len(), 0);
        let ordered_resolved_patterns: BTreeMap<_, _> = resolved_patterns.into_iter().collect();
        assert_yaml_snapshot!(ordered_resolved_patterns);
    }

    #[tokio::test]
    async fn resolve_from_root() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/resolve_modules";
        let (mut resolved_patterns, _) = super::resolve_patterns(&module_repo, repo_dir, None)
            .await
            .unwrap();
        assert_eq!(resolved_patterns.len(), 12);
        resolved_patterns.sort_by(|a, b| a.local_name.cmp(&b.local_name));
        assert_yaml_snapshot!(resolved_patterns);
    }

    #[tokio::test]
    async fn resolve_multiple_layers() {
        let module_repo =
            ModuleRepo::from_host_repo("github.com", "custodian-sample-org/testrepo-D").unwrap();
        let repo_dir = "fixtures/layered";
        let (mut resolved_patterns, _) = super::resolve_patterns(&module_repo, repo_dir, None)
            .await
            .unwrap();
        assert_eq!(resolved_patterns.len(), 3);
        resolved_patterns.sort_by(|a, b| a.local_name.cmp(&b.local_name));
        assert_yaml_snapshot!(resolved_patterns);
    }

    #[tokio::test]
    async fn resolve_enforcement_level() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/enforcement_level";
        let (mut resolved_patterns, _) = super::resolve_patterns(&module_repo, repo_dir, None)
            .await
            .unwrap();
        assert_eq!(resolved_patterns.len(), 3);
        resolved_patterns.sort_by(|a, b| a.local_name.cmp(&b.local_name));
        assert_yaml_snapshot!(resolved_patterns);
    }

    #[tokio::test]
    async fn dir_has_config_with_grit_yaml() {
        let dir = PathBuf::from_str("fixtures/searcher/dir/nested").unwrap();
        let has_config = dir_has_config(dir).await;
        assert!(has_config);
    }

    #[tokio::test]
    async fn dir_has_config_with_patterns() {
        let dir = PathBuf::from_str("fixtures/no_yaml").unwrap();
        let has_config = dir_has_config(dir).await;
        assert!(has_config);
    }

    #[tokio::test]
    async fn has_no_config_without_grit() {
        let dir: PathBuf = PathBuf::from_str("fixtures/no_grit").unwrap();
        let has_config = dir_has_config(dir).await;
        assert!(!has_config);
    }

    #[tokio::test]
    async fn has_no_config_with_empty_grit() {
        let dir: PathBuf = PathBuf::from_str("fixtures/empty_grit").unwrap();
        let has_config = dir_has_config(dir).await;
        assert!(!has_config);
    }

    #[tokio::test]
    async fn recognizes_different_language_direct_imports_with_same_local_name() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/languages_direct";
        let (mut resolved_patterns, errored_patterns) =
            super::resolve_patterns(&module_repo, repo_dir, None)
                .await
                .unwrap();

        println!("Error patterns: {:?}", errored_patterns);

        assert_eq!(errored_patterns.len(), 0);
        assert_eq!(resolved_patterns.len(), 2);

        resolved_patterns.sort_by(|a, b| a.language.to_string().cmp(&b.language.to_string()));
        assert_yaml_snapshot!(resolved_patterns);
    }

    #[tokio::test]
    async fn recognizes_different_language_namespace_imports_with_same_local_name() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/languages_namespace";
        let (mut resolved_patterns, errored_patterns) =
            super::resolve_patterns(&module_repo, repo_dir, None)
                .await
                .unwrap();

        println!("Error patterns: {:?}", errored_patterns);

        assert_eq!(resolved_patterns.len(), 2);
        assert_eq!(errored_patterns.len(), 0);

        resolved_patterns.sort_by(|a, b| a.language.to_string().cmp(&b.language.to_string()));
        assert_yaml_snapshot!(resolved_patterns);
    }

    #[tokio::test]
    async fn recognizes_mixed_language_imports_with_same_local_name() {
        let module_repo = ModuleRepo::from_host_repo("github.com", "getgrit/rewriter").unwrap();
        let repo_dir = "fixtures/languages_mixed";
        let (mut resolved_patterns, errored_patterns) =
            super::resolve_patterns(&module_repo, repo_dir, None)
                .await
                .unwrap();

        assert_eq!(resolved_patterns.len(), 2);
        assert_eq!(errored_patterns.len(), 0);

        resolved_patterns.sort_by(|a, b| a.language.to_string().cmp(&b.language.to_string()));
        assert_yaml_snapshot!(resolved_patterns);
    }
}
