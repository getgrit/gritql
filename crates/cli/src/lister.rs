use std::collections::{HashMap, HashSet};

use anyhow::Result;
use colored::Colorize;
use marzano_core::api::EnforcementLevel;
use marzano_gritmodule::{
    config::{DefinitionKind, DefinitionSource, PatternVisibility, ResolvedGritDefinition},
    fetcher::ModuleRepo,
    searcher::{find_local_workflow_files, WorkflowInfo},
};

use crate::{
    flags::GlobalFormatFlags,
    ux::{heading, indent},
};

pub trait Listable {
    fn name(&self) -> &str;
    fn description(&self) -> Option<&str> {
        None
    }
    fn is_util(&self) -> bool {
        self.name().starts_with('_')
    }
    fn tags(&self) -> Vec<&str> {
        vec![]
    }
    fn source(&self) -> Option<DefinitionSource> {
        None
    }
    fn language(&self) -> Option<&str> {
        None
    }

    fn list(&self, print_source: bool) -> Result<()> {
        if self.is_util() {
            return Ok(());
        }
        let mut message_parts = vec![format!("{} {}", "✔".green(), self.name())];

        let formatted_tags: Vec<_> = self
            .tags()
            .iter()
            .map(|tag| match *tag {
                "warn" => format!("{}", tag.yellow()),
                "error" => format!("{}", tag.red()),
                _ => format!("{}", tag.dimmed()),
            })
            .collect();

        message_parts.extend(formatted_tags);

        if print_source {
            if let Some(source) = self.source() {
                message_parts.push(format!("(source: {})", source.short_name().blue()));
            }
        }

        let message = message_parts.join(" ");
        log::info!("{}", indent(&message, 2));

        if let Some(description) = self.description() {
            log::info!("{}", indent(description, 4).dimmed());
        }

        Ok(())
    }
}

impl Listable for WorkflowInfo {
    fn name(&self) -> &str {
        self.name()
    }
}

fn filter_by_level(
    results: Vec<ResolvedGritDefinition>,
    level: EnforcementLevel,
) -> Vec<ResolvedGritDefinition> {
    results
        .into_iter()
        .filter(|item| item.level() >= level)
        .collect()
}

fn group_by_language(
    patterns: Vec<&ResolvedGritDefinition>,
) -> HashMap<String, Vec<ResolvedGritDefinition>> {
    let mut patterns_by_language: HashMap<String, Vec<ResolvedGritDefinition>> = HashMap::new();

    for pattern in patterns {
        patterns_by_language
            .entry(pattern.language().unwrap_or("").to_string())
            .or_default()
            .push(pattern.to_owned());
    }

    patterns_by_language
}

fn get_unique_source(patterns: &[ResolvedGritDefinition]) -> Option<String> {
    let unique_sources: HashSet<_> = patterns
        .iter()
        .filter_map(|p| p.source().map(|source| source.short_name()))
        .collect();
    if unique_sources.len() == 1 {
        Some(unique_sources.into_iter().next().unwrap())
    } else {
        None
    }
}

pub async fn list_applyables(
    _stdlib_workflows: bool,
    list_local_workflows: bool,
    resolved: Vec<ResolvedGritDefinition>,
    level: Option<EnforcementLevel>,
    format: &GlobalFormatFlags,
    curr_repo: ModuleRepo,
) -> Result<()> {
    if !resolved.is_empty() {
        let mut resolved_patterns = if let Some(level) = &level {
            filter_by_level(resolved, level.clone())
        } else {
            resolved
        };

        // Filter to only visible ones
        resolved_patterns.retain(|pattern| {
            // In JSON mode, we list all definitions so the frontend can use them
            format.json
                || matches!(pattern.visibility, PatternVisibility::Public)
                    && matches!(pattern.kind, DefinitionKind::Pattern)
        });

        resolved_patterns.sort();

        if format.json {
            // grit-ignore
            println!("{}", serde_json::to_string(&resolved_patterns)?);
            return Ok(());
        }

        let (config_patterns, stdlib_patterns): (Vec<_>, Vec<_>) = resolved_patterns
            .iter()
            .partition(|pattern| match pattern.source() {
                Some(source) => match source {
                    DefinitionSource::Module(module) => module == curr_repo,
                    DefinitionSource::Config(_) => true,
                },
                None => false,
            });

        let (user_patterns, local_patterns): (Vec<_>, Vec<_>) = config_patterns
            .into_iter()
            .partition(|pattern| match pattern.source() {
                Some(source) => matches!(source, DefinitionSource::Config(_)),
                None => false,
            });

        if !stdlib_patterns.is_empty() {
            log::info!("{}", heading("STANDARD LIBRARY PATTERNS"));
        }

        let patterns_by_language = group_by_language(stdlib_patterns);
        let mut sorted_patterns: Vec<_> = patterns_by_language.iter().collect();
        sorted_patterns.sort_by_key(|&(language, _)| language);

        for (language, patterns) in sorted_patterns {
            // If all patterns have the same source, print it in the header and not next to each item
            let unique_source = get_unique_source(patterns);
            if let Some(source) = unique_source.clone() {
                log::info!("\n{}", format!("{} patterns ({})", language, source).blue());
            } else {
                log::info!("\n{}", format!("{} patterns", language).blue());
            }

            for pattern in patterns {
                pattern.list(unique_source.is_none())?;
            }
        }

        if !local_patterns.is_empty() {
            log::info!("{}", heading("LOCAL PATTERNS"));
        }

        let mut patterns_by_language = group_by_language(local_patterns);
        for (language, patterns) in &mut patterns_by_language {
            log::info!("\n{}", &format!("{} patterns", language).blue());

            patterns.sort();

            for pattern in patterns {
                pattern.list(false)?;
            }
        }

        if !user_patterns.is_empty() {
            log::info!("{}", heading("USER PATTERNS"));
        }
        let mut patterns_by_language = group_by_language(user_patterns);
        for (language, patterns) in &mut patterns_by_language {
            log::info!("\n{}", &format!("{} patterns", language).blue());

            patterns.sort();

            for pattern in patterns {
                pattern.list(true)?;
            }
        }
    }

    #[cfg(feature = "bundled_workflows")]
    if stdlib_workflows {
        log::info!("{}", heading("STANDARD LIBRARY WORKFLOWS"));

        let mut collected = BUNDLED_WORKFLOWS.clone().into_iter().collect::<Vec<&str>>();
        collected.sort();
        collected
            .into_iter()
            .map(|name| Workflow { name })
            .for_each(|workflow| {
                workflow.list(true).unwrap();
            });
    }

    if list_local_workflows {
        let cwd = std::env::current_dir()?;

        let mut local_workflows = find_local_workflow_files(cwd).await.unwrap_or(vec![]);
        if !local_workflows.is_empty() {
            log::info!("{}", heading("LOCAL WORKFLOWS"));

            local_workflows.sort();

            local_workflows.into_iter().for_each(|workflow| {
                workflow.list(true).unwrap();
            });
        }
    }

    Ok(())
}
