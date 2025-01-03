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
use std::collections::HashMap;

/// Specifies amount of indent that consumed to get to grit body in yaml files
// Yaml files that contains grit body are like this:
// ```yaml
// patterns:
//   - name: some_name
//     body: |
//       language css
//
//       `a { $props }` where {
//         $props <: contains `aspect-ratio: $x`
//       }
// ```
// the grit body is prefixed by some amount of spaces due to yaml format
const YAML_GRIT_BODY_INDENT_SIZE: usize = 6;

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

    // we process each file resolves separetly because we want
    // to write to file once and also fix the bugs that may happen when
    // overwriting same file multiple times for different patterns
    let file_path_to_resolves: HashMap<String, Vec<ResolvedGritDefinition>> = resolved
        .into_iter()
        .fold(HashMap::new(), |mut acc, resolved| {
            let file_path = resolved.config.path.clone();
            acc.entry(file_path).or_insert_with(Vec::new).push(resolved);
            acc
        });

    // TODO: do we need this to be runned in parallel?
    for (file_path, definitions) in file_path_to_resolves {
        if let Err(error) =
            format_file_definitations(file_path.clone(), definitions, arg.clone()).await
        {
            eprintln!("couldn't format '{}': {error:?}", file_path)
        }
    }
    Ok(())
}

async fn format_file_definitations(
    file_path: String,
    definitions: Vec<ResolvedGritDefinition>,
    arg: FormatArgs,
) -> Result<()> {
    let hunks = definitions
        .iter()
        .map(|definition| definitation_to_hunk(definition))
        .collect::<Result<Vec<HunkChange>>>()?;

    let first_definitation = definitions.first().unwrap();
    let first_definitation_raw_config = first_definitation.config.raw.as_ref().unwrap();
    let old_file_content = &first_definitation_raw_config.content;
    let new_file_content = apply_hunk_changes(&old_file_content, hunks);

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
            format_diff(&old_file_content, &new_file_content)
        );
    }

    Ok(())
}

fn definitation_to_hunk(definition: &ResolvedGritDefinition) -> Result<HunkChange> {
    let unformatted_grit_code = &definition.body;
    let mut formatted_grit_code = format_grit_code(unformatted_grit_code)?;

    let raw_data = definition
        .config
        .raw
        .as_ref()
        .ok_or_else(|| anyhow!("definition doesn't have raw_data"))?;
    let body_range = definition
        .config
        .range
        .as_ref()
        .ok_or_else(|| anyhow!("definition doesn't have config range"))?;

    if raw_data.format == PatternFileExt::Grit {
        // TODO: fix langauge line not getting formatted
        // this needed because we add additional "language {}\n\n" to body when parsing .grit file
        if formatted_grit_code.starts_with("language ") {
            let formatted_lines = formatted_grit_code.lines();
            formatted_grit_code = formatted_lines.skip(1).collect::<Vec<_>>().join("\n");
        }
    }
    println!("range: {:?}", body_range);

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
    let doc_print = doc.print()?;
    Ok(doc_print.into_code())
}

fn grit_code_with_yaml_indent(grit_code: &str) -> String {
    let indent = " ".repeat(YAML_GRIT_BODY_INDENT_SIZE);
    grit_code
        .lines()
        .map(|line| {
            if !line.is_empty() {
                format!("{indent}{line}")
            } else {
                line.to_owned()
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
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
