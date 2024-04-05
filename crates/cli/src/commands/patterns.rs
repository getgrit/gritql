use std::path::PathBuf;

use anyhow::Result;
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use marzano_core::pattern::api::EnforcementLevel;
use marzano_util::{base64, url};
use std::io::prelude::*;

use marzano_gritmodule::searcher::collect_from_file;
use serde::Serialize;

use crate::resolver;
use crate::resolver::resolve_from_cwd;
use crate::ux::heading;

use super::list::ListArgs;

#[derive(Parser, Debug, Serialize)]
pub struct Patterns {
    #[structopt(subcommand)]
    pub patterns_commands: PatternCommands,
}

#[derive(Subcommand, Debug, Serialize)]
pub enum PatternCommands {
    /// List all available named patterns.
    List(ListArgs),
    /// Test patterns against expected output.
    Test(PatternsTestArgs),
    /// Open a pattern in the studio
    Edit(PatternsEditArgs),
    /// Describe a pattern
    Describe(PatternsDescribeArgs),
}

#[derive(Args, Debug, Serialize)]
pub struct PatternsTestArgs {
    /// Regex of a specific pattern to test
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Regex of tags and pattern names to exclude
    #[clap(long = "exclude")]
    pub exclude: Vec<String>,
    /// Show verbose output
    #[clap(long = "verbose")]
    pub verbose: bool,
    /// Update expected test outputs
    #[clap(long = "update")]
    pub update: bool,
}

#[derive(Args, Debug, Serialize)]
pub struct PatternsDescribeArgs {
    /// The pattern name to describe
    #[clap(value_parser)]
    name: String,
}

pub(crate) async fn run_patterns_describe(arg: PatternsDescribeArgs) -> Result<()> {
    let (resolved, _) = resolve_from_cwd(&resolver::Source::All).await?;

    if let Some(pattern) = resolved
        .iter()
        .find(|&pattern| pattern.config.name == arg.name)
    {
        if let Some(title) = &pattern.title() {
            log::info!("{}\n", heading(&format!("# {}", title)));
        }

        if let Some(description) = &pattern.description() {
            log::info!("{}\n", description);
        }

        log::info!("{} {}", "- Name:".blue(), pattern.config.name);
        log::info!(
            "{} {}",
            "- Language:".blue(),
            pattern.language.language_name()
        );

        if pattern.level() != EnforcementLevel::default() {
            log::info!("{} {}", "- Level:".blue(), pattern.level());
        }

        if !pattern.tags().is_empty() {
            let tags_str = pattern.tags().join(", ");
            log::info!("{} {}", "- Tags:".blue(), tags_str);
        }

        if let Some(body) = &pattern.config.body {
            log::info!("{}", heading("# Body"));
            log::info!("\n{}", body.dimmed());
        }

        if let Some(samples) = &pattern.config.samples {
            if !samples.is_empty() {
                log::info!("{}", heading("# Samples"));
            }
            for sample in samples {
                if let Some(name) = &sample.name {
                    log::info!("\n## {}", name);
                }

                let input_lines = sample.input.lines().collect::<Vec<_>>();
                let output_lines = if let Some(output) = &sample.output {
                    output.lines().collect::<Vec<_>>()
                } else {
                    vec!["None"]
                };

                let width = input_lines.iter().map(|line| line.len()).max().unwrap_or(0);
                let output_width = output_lines
                    .iter()
                    .map(|line| line.len())
                    .max()
                    .unwrap_or(0);

                log::info!("\n{:<width$} {}", "Input".blue(), "| Output".blue(),);
                log::info!(
                    "{} {} {}",
                    "-".repeat(width).blue(),
                    "|".blue(),
                    "-".repeat(output_width).blue(),
                );
                let max_len = std::cmp::max(input_lines.len(), output_lines.len());
                for i in 0..max_len {
                    let input_line = input_lines.get(i).unwrap_or(&"");
                    let output_line = output_lines.get(i).unwrap_or(&"");
                    log::info!("{:<width$} {} {}", input_line, "|".blue(), output_line);
                }
                log::info!(
                    "{} {} {}",
                    "-".repeat(width).blue(),
                    "|".blue(),
                    "-".repeat(output_width).blue(),
                );
                log::info!("");
            }
        }
    } else {
        log::error!("Pattern not found: {}", arg.name);
        log::info!(
            "\nRun {} to see all available patterns.",
            "grit patterns list".bold()
        );
    }

    Ok(())
}

#[derive(Args, Debug, Serialize)]
pub struct PatternsEditArgs {
    /// The pattern path to edit
    #[clap(value_parser)]
    path: PathBuf,
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OpenStudioSettings {
    pub content: String,
    pub path: String,
}

pub(crate) async fn run_patterns_edit(arg: PatternsEditArgs) -> Result<()> {
    let (_, repo) = resolve_from_cwd(&resolver::Source::All).await?;
    let _pattern = collect_from_file(&arg.path, &Some(repo)).await?;

    let content = std::fs::read_to_string(&arg.path)?;
    let payload = serde_json::to_value(OpenStudioSettings {
        content,
        path: arg.path.to_string_lossy().to_string(),
    })?;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(payload.to_string().as_bytes())?;
    let compressed_payload = e.finish()?;
    let encoded_payload = base64::encode_from_bytes(&compressed_payload)?;
    let url_safe = url::encode(&encoded_payload);

    let app_url = "https://app.grit.io";
    let url = format!("{}/studio?pattern_file={}", app_url, url_safe);

    println!("Open in Grit studio: {}", url.bright_blue());

    Ok(())
}
