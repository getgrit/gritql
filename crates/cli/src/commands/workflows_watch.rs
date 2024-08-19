use anyhow::{bail, Result};
use clap::Args;



use serde::Serialize;

use crate::{
    flags::{GlobalFormatFlags},
};

#[derive(Args, Debug, Serialize)]
pub struct WorkflowWatchArgs {
    /// The workflow ID to watch
    #[clap(index = 1)]
    workflow_id: String,
}

pub async fn run_watch_workflow(_arg: &WorkflowWatchArgs, _parent: &GlobalFormatFlags) -> Result<()> {
    #[cfg(feature = "remote_workflows")]
    {
        let mut updater = Updater::from_current_bin().await?;

        let auth = updater.get_valid_auth().await?;

        let format = OutputFormat::from(parent);
        let emitter = create_emitter(
            &format,
            marzano_messenger::output_mode::OutputMode::default(),
            None,
            false,
            None,
            None,
            VisibilityLevels::default(),
        )
        .await?;
        grit_cloud_client::watch_workflow(&arg.workflow_id, &auth, emitter).await?;
    }

    #[cfg(not(feature = "remote_workflows"))]
    bail!("Remote workflows are not supported on this platform");

    Ok(())
}
