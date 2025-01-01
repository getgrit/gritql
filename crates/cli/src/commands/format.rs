use crate::{
    resolver::{resolve_from_cwd, Source},
    ux::format_diff,
};
use anyhow::{ensure, Context, Result};
use biome_grit_formatter::context::GritFormatOptions;
use clap::Args;
use serde::Serialize;

#[derive(Args, Debug, Serialize)]
pub struct FormatArgs {
    /// Write formats to file instead of just showing them
    #[clap(long)]
    pub write: bool,
}

pub async fn run_format(arg: &FormatArgs) -> Result<()> {
    let (resolved, _) = resolve_from_cwd(&Source::Local).await?;
    // TODO: make this run in parallel
    for definition in resolved {
        let old_body = &definition.body;
        let parsed = biome_grit_parser::parse_grit(old_body);
        let path = &definition.config.path;
        ensure!(
            parsed.diagnostics().is_empty(),
            "couldn't parse '{path}': {}",
            parsed
                .diagnostics()
                .iter()
                .map(|diag| diag.message.to_string())
                .collect::<Vec<_>>()
                .join("\n")
        );

        let options = GritFormatOptions::default();
        let doc = biome_grit_formatter::format_node(options, &parsed.syntax())
            .with_context(|| format!("couldn't format '{path}'"))?;
        let doc_print = doc.print()?;
        let new_body = doc_print.as_code();

        if arg.write {
            // TODO: i think there is already a rewriting feature that `apply` subcommand uses
            // that i think can be re used here, look into it or at least
            // use definition.config.range instead of replacing
            let raw_data = definition.config.raw.as_ref().unwrap();
            tokio::fs::write(path, raw_data.content.replace(old_body, new_body))
                .await
                .with_context(|| format!("could not write to file at '{path}'"))?;
        } else {
            println!(
                "{}:\n{}",
                definition.config.path,
                format_diff(&definition.body, new_body)
            );
        }
    }
    Ok(())
}
