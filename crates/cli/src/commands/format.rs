use crate::{
    resolver::{resolve_from_cwd, Source},
    ux::format_diff,
};
use anyhow::{anyhow, ensure, Context, Result};
use biome_grit_formatter::context::GritFormatOptions;
use clap::Args;
use colored::Colorize;
use marzano_gritmodule::{config::ResolvedGritDefinition, parser::PatternFileExt};
use serde::Serialize;

#[derive(Args, Debug, Serialize, Clone)]
pub struct FormatArgs {
    /// Write formats to file instead of just showing them
    #[clap(long)]
    pub write: bool,
}

pub async fn run_format(arg: &FormatArgs) -> Result<()> {
    let (mut resolved, _) = resolve_from_cwd(&Source::Local).await?;
    // sort to have consistent output for tests
    resolved.sort();

    let file_path_to_resolved = group_resolved_patterns_by_group(resolved);
    for (file_path, resovled_patterns) in file_path_to_resolved {
        if let Err(error) =
            format_file_resovled_patterns(file_path.clone(), resovled_patterns, arg.clone()).await
        {
            eprintln!("couldn't format '{}': {error:?}", file_path)
        }
    }
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

async fn format_file_resovled_patterns(
    file_path: String,
    patterns: Vec<ResolvedGritDefinition>,
    arg: FormatArgs,
) -> Result<()> {
    // patterns has atleast one resolve so unwrap is safe
    let first_pattern = patterns.first().unwrap();
    // currently all patterns has raw data so unwrap is safe
    let first_pattern_raw_data = first_pattern.config.raw.as_ref().unwrap();
    let old_file_content = &first_pattern_raw_data.content;

    let new_file_content = match first_pattern_raw_data.format {
        PatternFileExt::Yaml => format_yaml_file(old_file_content)?,
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
        return Ok(());
    }

    if arg.write {
        tokio::fs::write(file_path, new_file_content)
            .await
            .with_context(|| "could not write to file")?;
    } else {
        println!(
            "{}:\n{}",
            file_path.bold(),
            format_diff(old_file_content, &new_file_content)
        );
    }

    Ok(())
}

fn format_yaml_file(file_content: &str) -> Result<String> {
    // deserializing manually and not using `SerializedGritConfig` because
    // i don't want to remove any fields that `SerializedGritConfig` don't have such as 'version'
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(file_content).with_context(|| "couldn't parse yaml file")?;
    let patterns = config
        .get_mut("patterns")
        .ok_or_else(|| anyhow!("couldn't find patterns in yaml file"))?
        .as_sequence_mut()
        .ok_or_else(|| anyhow!("patterns in yaml file are not sequence"))?;
    for pattern in patterns {
        let Some(body) = pattern.get_mut("body") else {
            continue;
        };
        if let serde_yaml::Value::String(body_str) = body {
            *body_str = format_grit_code(body_str)?;
            // extra new line at end of grit body looks more readable
            body_str.push('\n');
        }
    }
    Ok(serde_yaml::to_string(&config)?)
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
    hunks.sort_by_key(|hunk| hunk.starting_byte);
    let mut buffer = String::new();
    let mut last_ending_byte = 0;
    for (index, hunk) in hunks.iter().enumerate() {
        buffer.push_str(&input[last_ending_byte..hunk.starting_byte]);
        buffer.push_str(&hunk.new_content);
        last_ending_byte = hunk.ending_byte;

        if index == hunks.len() - 1 {
            buffer.push_str(&input[last_ending_byte..]);
        }
    }
    buffer
}
