use clap_markdown;

use anyhow::Result;
use clap::Args;
use serde::Serialize;

use crate::commands::App;

#[derive(Args, Debug, Serialize)]
pub struct DocGenArgs {
    outpath: String,
}

pub(crate) async fn run_docgen(arg: DocGenArgs) -> Result<()> {
    log::info!("Writing docs to {}", arg.outpath);

    let output = clap_markdown::custom_help_markdown::<App>(clap_markdown::MarkdownOptions {
        title: Some("Grit CLI Reference".to_string()),
        hide_footer: true,
    });

    fs_err::write(arg.outpath, output)?;

    Ok(())
}
