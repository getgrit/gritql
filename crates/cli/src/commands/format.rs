use crate::{
    resolver::{resolve_from_cwd, Source},
    ux::format_diff,
};
use anyhow::{ensure, Context, Result};
use biome_grit_formatter::context::GritFormatOptions;
use clap::Args;
use colored::Colorize;
use marzano_gritmodule::{config::ResolvedGritDefinition, parser::PatternFileExt};
use serde::Serialize;

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

#[derive(Args, Debug, Serialize)]
pub struct FormatArgs {
    /// Write formats to file instead of just showing them
    #[clap(long)]
    pub write: bool,
}

pub async fn run_format(arg: &FormatArgs) -> Result<()> {
    let (mut resolved, _) = resolve_from_cwd(&Source::Local).await?;
    // sort to have consistent output for tests
    resolved.sort();

    // TODO: do we need this to be runned in parallel?
    for definition in resolved {
        if let Err(error) = format_resolv(&definition, &arg).await {
            eprintln!("couldn't format '{}': {error:?}", definition.config.path)
        }
    }
    Ok(())
}

async fn format_resolv(definition: &ResolvedGritDefinition, arg: &FormatArgs) -> Result<()> {
    let mut unformatted_grit_code = definition.body.clone();
    let mut formatted_grit_code = format_grit_code(&unformatted_grit_code)?;
    if unformatted_grit_code == formatted_grit_code {
        return Ok(());
    }

    let file_content = if let Some(raw_data) = &definition.config.raw {
        // TODO: find a better way to do this
        if raw_data.format == PatternFileExt::Yaml {
            formatted_grit_code = grit_code_with_yaml_indent(&formatted_grit_code);
            unformatted_grit_code = grit_code_with_yaml_indent(&unformatted_grit_code);
        }
        raw_data.content.clone()
    } else {
        unformatted_grit_code.clone()
    };
    let new_file_content = file_content.replace(&unformatted_grit_code, &formatted_grit_code);
    ensure!(file_content != new_file_content);

    if arg.write {
        // TODO: i think there is already a rewriting feature that `apply` subcommand uses
        // that i think can be re used here, look into it or at least
        // use definition.config.range instead of replacing
        tokio::fs::write(&definition.config.path, new_file_content)
            .await
            .with_context(|| "could not write to file")?;
    } else {
        println!(
            "{}:\n{}",
            definition.config.path.bold(),
            format_diff(&file_content, &new_file_content)
        );
    }
    Ok(())
}

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
