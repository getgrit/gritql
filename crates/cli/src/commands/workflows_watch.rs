use anyhow::Result;
use clap::Args;

use serde::Serialize;

use crate::flags::GlobalFormatFlags;

#[derive(Args, Debug, Serialize)]
pub struct WorkflowWatchArgs {
    /// The workflow ID to watch
    #[clap(index = 1)]
    workflow_id: String,
}

pub async fn run_watch_workflow(
    _arg: &WorkflowWatchArgs,
    _parent: &GlobalFormatFlags,
) -> Result<()> {
    #[cfg(feature = "remote_workflows")]
    {
        let mut updater = crate::updater::Updater::from_current_bin().await?;

        let auth = updater.get_valid_auth().await?;

        let format = crate::flags::OutputFormat::from(_parent);
        let emitter = crate::messenger_variant::create_emitter(
            &format,
            marzano_messenger::output_mode::OutputMode::default(),
            None,
            false,
            None,
            None,
            marzano_messenger::emit::VisibilityLevels::default(),
        )
        .await?;
        grit_cloud_client::watch_workflow(&_arg.workflow_id, &auth, emitter).await?;

        return Ok(());
    }

    #[cfg(not(feature = "remote_workflows"))]
    anyhow::bail!("Remote workflows are not supported on this platform")
}
