use crate::{
    resolver::{resolve_from_cwd, Source},
    ux::format_diff,
};
use anyhow::{ensure, Context, Result};
use biome_grit_formatter::context::GritFormatOptions;
use clap::Args;
use marzano_gritmodule::config::ResolvedGritDefinition;
use serde::Serialize;

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
    let old_body = &definition.body;
    let parsed = biome_grit_parser::parse_grit(old_body);
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
    let new_body = doc_print.as_code();

    // don't show any output when the file is already formatted
    if old_body == new_body {
        return Ok(());
    }

    if arg.write {
        // TODO: i think there is already a rewriting feature that `apply` subcommand uses
        // that i think can be re used here, look into it or at least
        // use definition.config.range instead of replacing
        let content = if let Some(raw_data) = &definition.config.raw {
            raw_data.content.clone()
        } else {
            old_body.clone()
        };
        tokio::fs::write(&definition.config.path, content.replace(old_body, new_body))
            .await
            .with_context(|| "could not write to file")?;
    } else {
        println!(
            "{}:\n{}",
            definition.config.path,
            format_diff(&definition.body, new_body)
        );
    }
    Ok(())
}
