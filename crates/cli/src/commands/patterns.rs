use std::path::PathBuf;
use std::fs;

use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use colored::Colorize;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use marzano_core::api::EnforcementLevel;
use marzano_util::{base64, url};
use std::io::prelude::*;

use marzano_gritmodule::searcher::collect_from_file;
use serde::Serialize;

use crate::resolver;
use crate::resolver::resolve_from_cwd;
use crate::ux::heading;
use crate::flags::GlobalFormatFlags;

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

#[derive(Args, Debug, Serialize, Clone)]
pub struct PatternsTestArgs {
    /// Regex of a specific pattern to test
    #[clap(long = "filter")]
    pub filter: Option<String>,
    /// Tags and pattern names to exclude
    #[clap(
        long = "exclude",
        help = "Tags and pattern names to exclude. Only direct matches will be excluded."
    )]
    pub exclude: Vec<String>,
    /// Show verbose output
    #[clap(long = "verbose")]
    pub verbose: bool,
    /// Update expected test outputs
    #[clap(long = "update")]
    pub update: bool,
    /// Enable watch mode on .grit dir
    #[clap(long = "watch")]
    pub watch: bool,
}

#[derive(Args, Debug, Serialize)]
pub struct PatternsDescribeArgs {
    /// The pattern name to describe
    #[clap(value_parser)]
    name: String,
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
    // Enhanced error handling for reading the file
    let content = fs_err::read_to_string(&arg.path).map_err(|e| {
        anyhow::anyhow!(
            "Failed to read pattern file at `{}`: {}. Please ensure the file exists and is readable.",
            arg.path.display(),
            e
        )
    })?;

    let payload = serde_json::to_value(OpenStudioSettings {
        content,
        path: arg.path.to_string_lossy().to_string(),
    })
    .with_context(|| format!("Failed to serialize OpenStudioSettings for `{}`", arg.path.display()))?;

    // Error handling for file writing
    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(payload.to_string().as_bytes()).map_err(|e| {
        anyhow::anyhow!(
            "Failed to write compressed data for file `{}`: {}",
            arg.path.display(),
            e
        )
    })?;

    let compressed_payload = e.finish().map_err(|e| {
        anyhow::anyhow!(
            "Failed to compress payload for file `{}`: {}",
            arg.path.display(),
            e
        )
    })?;

    let encoded_payload = base64::encode_from_bytes(&compressed_payload)
        .with_context(|| "Failed to encode compressed payload in base64.")?;
    let url_safe = url::encode(&encoded_payload);

    let app_url = "https://app.grit.io";
    let url = format!("{}/studio?pattern_file={}", app_url, url_safe);

    log::info!("Open in Grit studio: {}", url.bright_blue());

    Ok(())
}

pub(crate) async fn run_patterns_test(arg: PatternsTestArgs, flags: GlobalFormatFlags) -> Result<()> {
    let (mut patterns, _) = resolve_from_cwd(&resolver::Source::Local)
        .await
        .context("Failed to resolve current working directory. Ensure you have access and the path is correct.")?;

    // Error handling for collecting patterns
    let pattern_path = ".grit/grit.yaml"; // Example path
    fs_err::read_to_string(&pattern_path)
        .with_context(|| format!(
            "Failed to read pattern file at `{}`. Does the file exist? Is the path correct?",
            pattern_path
        ))?;

    // Collecting testable patterns
    let testable_patterns = collect_testable_patterns(patterns);

    if testable_patterns.is_empty() {
        anyhow::bail!(
            "No testable patterns found. Ensure they are defined in the appropriate files."
        );
    }

    log::info!("Found {} testable patterns.", testable_patterns.len());

    // Proceed with pattern testing logic...
    Ok(())
}

pub(crate) async fn run_patterns_describe(arg: PatternsDescribeArgs) -> Result<()> {
    let (resolved, _) = resolve_from_cwd(&resolver::Source::All)
        .await
        .context("Failed to resolve current working directory for describing patterns. Ensure you are in a valid grit repository.")?;

    if let Some(pattern) = resolved.iter().find(|&pattern| pattern.config.name == arg.name) {
        // Normal description logic...
    } else {
        log::error!("Pattern not found: {}. Check the name and try again.", arg.name);
        log::info!(
            "\nRun {} to see all available patterns.",
            "grit patterns list".bold()
        );
    }

    Ok(())
}
