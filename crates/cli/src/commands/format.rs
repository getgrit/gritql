use crate::{
    resolver::{resolve_from_cwd, GritModuleResolver, Source},
    ux::{format_diff, DiffString},
};
use anyhow::{anyhow, bail, ensure, Context, Result};
use biome_grit_formatter::context::GritFormatOptions;
use clap::Args;
use colored::Colorize;
use marzano_core::api::MatchResult;
use marzano_gritmodule::{config::ResolvedGritDefinition, parser::PatternFileExt};
use marzano_util::{rich_path::RichFile, runtime::ExecutionContext};
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Args, Debug, Serialize, Clone)]
pub struct FormatArgs {
    /// Write formats to file instead of just showing them
    #[clap(long)]
    pub write: bool,
}

pub async fn run_format(arg: &FormatArgs) -> Result<()> {
    let (resolved, _) = resolve_from_cwd(&Source::Local).await?;

    let file_path_to_resolved = group_resolved_patterns_by_group(resolved);

    println!("Formtting {} files", file_path_to_resolved.len());

    let _ = file_path_to_resolved
        .into_par_iter()
        .map(|(file_path, resolved_patterns)| {
            let result = format_file_resolved_patterns(&file_path, resolved_patterns, arg.clone());
            (file_path, result)
        });

    // sort outputs to ensure consistent stdout output
    // also avoid using sort_by_key to prevent additional cloning of file_path
    // results.sort_by(|(file_path, _), (other_file_path, _)| file_path.cmp(other_file_path));

    // for (file_path, result) in results {
    //     match result {
    //         Err(error) => eprintln!("couldn't format '{}': {error:?}", file_path),
    //         Ok(Some(diff)) => println!("{}:\n{}", file_path.bold(), diff),
    //         Ok(None) => (), // `args.write` is true or file is already formated
    //     }
    // }
    Ok(())
}

fn group_resolved_patterns_by_group(
    resolved: Vec<ResolvedGritDefinition>,
) -> Vec<(String, Vec<ResolvedGritDefinition>)> {
    resolved.into_iter().fold(Vec::new(), |mut acc, resolved| {
        let file_path = &resolved.config.path;
        if let Some((_, resolved_patterns)) = acc
            .iter_mut()
            .find(|(resolv_file_path, _)| resolv_file_path == file_path)
        {
            resolved_patterns.push(resolved);
        } else {
            acc.push((file_path.clone(), vec![resolved]));
        }
        acc
    })
}

fn format_file_resolved_patterns(
    file_path: &str,
    patterns: Vec<ResolvedGritDefinition>,
    arg: FormatArgs,
) -> Result<Option<DiffString>> {
    let first_pattern = patterns
        .first()
        .ok_or_else(|| anyhow!("patterns is empty"))?;
    let first_pattern_raw_data = first_pattern
        .config
        .raw
        .as_ref()
        .ok_or_else(|| anyhow!("pattern doesn't have raw data"))?;
    let old_file_content = &first_pattern_raw_data.content;

    let new_file_content = match first_pattern_raw_data.format {
        PatternFileExt::Yaml => format_yaml_file(&patterns, old_file_content)?,
        PatternFileExt::Grit => format_grit_code(old_file_content)?,
        PatternFileExt::Md => {
            let hunks = patterns
                .iter()
                .map(format_pattern_as_hunk_changes)
                .collect::<Result<Vec<HunkChange>>>()?;
            apply_hunk_changes(old_file_content, hunks)
        }
    };

    if &new_file_content == old_file_content {
        return Ok(None);
    }

    if arg.write {
        std::fs::write(file_path, new_file_content).with_context(|| "could not write to file")?;
        Ok(None)
    } else {
        Ok(Some(format_diff(old_file_content, &new_file_content)))
    }
}

/// bubble clause that finds a grit pattern with name "\<pattern_name\>" in yaml and
/// replaces it's body to "\<new_body\>", `format_yaml_file` uses this pattern to replace
/// pattern bodies with formatted ones
const YAML_REPLACE_BODY_PATERN: &str = r#"
    bubble file($body) where {
        $body <: contains block_mapping(items=$items) where {
            $items <: within `patterns: $_`,
            $items <: contains `name: $name`,
            $name <: "<pattern_name>",
            $items <: contains `body: $yaml_body`,
            $new_body = "<new_body>",
            $yaml_body => $new_body
        },
    }
"#;

/// format each pattern and use gritql pattern to match and rewrite
fn format_yaml_file(patterns: &[ResolvedGritDefinition], file_content: &str) -> Result<String> {
    let bubbles = patterns
        .iter()
        .map(|pattern| {
            let formatted_body = format_grit_code(&pattern.body)
                .with_context(|| format!("could not format '{}'", pattern.name()))?;
            let bubble = YAML_REPLACE_BODY_PATERN
                .replace("<pattern_name>", pattern.name())
                .replace("<new_body>", &format_yaml_body_code(&formatted_body));
            Ok(bubble)
        })
        .collect::<Result<Vec<_>>>()?
        .join(",\n");
    let pattern_body = format!("language yaml\nsequential{{ {bubbles} }}");
    apply_grit_rewrite(file_content, &pattern_body)
}

fn format_yaml_body_code(input: &str) -> String {
    // yaml body still needs two indentation to look good
    let body_with_prefix = prefix_lines(input, &" ".repeat(2));
    let escaped_body = body_with_prefix.replace("\"", "\\\"");
    // body: |
    //   escaped_body
    format!("|\n{escaped_body}")
}

fn prefix_lines(input: &str, prefix: &str) -> String {
    input
        .lines()
        .map(|line| {
            if line.is_empty() {
                line.to_owned()
            } else {
                format!("{prefix}{line}")
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn apply_grit_rewrite(input: &str, pattern: &str) -> Result<String> {
    let resolver = GritModuleResolver::new();
    let rich_pattern = resolver.make_pattern(pattern, None)?;

    let compiled = rich_pattern
        .compile(&BTreeMap::new(), None, None, None)
        .map(|cr| cr.problem)
        .with_context(|| "could not compile pattern")?;

    let rich_file = RichFile::new(String::new(), input.to_owned());
    let runtime = ExecutionContext::default();
    for result in compiled.execute_file(&rich_file, &runtime) {
        if let MatchResult::Rewrite(rewrite) = result {
            let content = rewrite
                .rewritten
                .content
                .ok_or_else(|| anyhow!("rewritten content is empty"))?;
            return Ok(content);
        }
    }
    bail!("no rewrite result after applying grit pattern")
}

fn format_pattern_as_hunk_changes(pattern: &ResolvedGritDefinition) -> Result<HunkChange> {
    let formatted_grit_code = format_grit_code(&pattern.body)?;
    let body_range = pattern
        .config
        .range
        .ok_or_else(|| anyhow!("pattern doesn't have config range"))?;
    Ok(HunkChange {
        starting_byte: body_range.start_byte as usize,
        ending_byte: body_range.end_byte as usize,
        new_content: formatted_grit_code,
    })
}

/// format grit code using `biome`
fn format_grit_code(source: &str) -> Result<String> {
    let parsed = biome_grit_parser::parse_grit(source);
    ensure!(
        parsed.diagnostics().is_empty(),
        "biome couldn't parse: {}",
        parsed
            .diagnostics()
            .iter()
            .map(|diag| diag.message.to_string())
            .collect::<Vec<_>>()
            .join("\n")
    );

    let options = GritFormatOptions::default();
    let doc = biome_grit_formatter::format_node(options, &parsed.syntax())
        .with_context(|| "biome couldn't format")?;
    Ok(doc.print()?.into_code())
}

/// Represent a hunk of text that needs to be changed
#[derive(Debug)]
struct HunkChange {
    starting_byte: usize,
    ending_byte: usize,
    new_content: String,
}

/// returns a new string that applies hunk changes
fn apply_hunk_changes(input: &str, mut hunks: Vec<HunkChange>) -> String {
    if hunks.is_empty() {
        return input.to_string();
    }
    hunks.sort_by_key(|hunk| -(hunk.starting_byte as isize));
    let mut buffer = input.to_owned();
    for hunk in hunks {
        let hunk_range = hunk.starting_byte..hunk.ending_byte;
        buffer.replace_range(hunk_range, &hunk.new_content);
    }
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[tokio::test]
    async fn test_format_fixtures() -> Result<()> {
        // Change to the fixtures directory relative to the project root
        let fixtures_path = PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("cli_bin")
            .join("fixtures");

        println!("fixtures_path: {:?}", fixtures_path);
        std::env::set_current_dir(&fixtures_path)?;

        let args = FormatArgs { write: false };
        run_format(&args).await?;

        Ok(())
    }
}
