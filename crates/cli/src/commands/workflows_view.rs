use anyhow::Result;
use clap::Args;
use serde::Serialize;

use crate::{
    flags::{GlobalFormatFlags, OutputFormat},
    messenger_variant::create_emitter,
};

#[derive(Args, Debug, Serialize)]
pub struct WorkflowViewArgs {
    #[clap(index = 1, help = "The ID of the workflow execution to view")]
    execution_id: String,
}

pub async fn run_view_workflow(_arg: &WorkflowViewArgs, flags: &GlobalFormatFlags) -> Result<()> {
    let format = OutputFormat::from_flags(flags, OutputFormat::Standard);
    let mut emitter = create_emitter(
        &format,
        marzano_messenger::output_mode::OutputMode::default(),
        None,
        false,
        None,
        None,
    )
    .await;

    println!("DO IT!");

    Ok(())
}
