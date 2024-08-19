use anyhow::{bail, Result};
use clap::Args;
use futures::stream::StreamExt;
use marzano_auth::{env::get_grit_api_url, info::AuthInfo};
use marzano_messenger::{
    emit::{Messager, VisibilityLevels},
    workflows::WorkflowMessenger,
    SimpleLogMessage,
};
use reqwest::{Client, RequestBuilder};
use serde::Serialize;

use crate::{
    flags::{GlobalFormatFlags, OutputFormat},
    lister::list_applyables,
    messenger_variant::create_emitter,
    resolver::{resolve_from_cwd, Source},
    updater::Updater,
};

#[derive(Args, Debug, Serialize)]
pub struct WorkflowWatchArgs {
    /// The workflow ID to watch
    #[clap(index = 1)]
    workflow_id: String,
}

pub async fn run_watch_workflow(arg: &WorkflowWatchArgs, parent: &GlobalFormatFlags) -> Result<()> {
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

    watch_workflow(&arg.workflow_id, &auth, emitter).await?;

    Ok(())
}

#[derive(Debug)]
enum GritEvent {
    Log(SimpleLogMessage),
    End,
    Unknown,
}

pub async fn watch_workflow<M>(workflow_id: &str, auth: &AuthInfo, mut emitter: M) -> Result<()>
where
    M: Messager + WorkflowMessenger,
{
    let client = Client::new();

    let request = client
        .get(format!(
            "{}/executions/{}/events",
            get_grit_api_url(),
            workflow_id
        ))
        .bearer_auth(&auth.access_token);

    let mut es = reqwest_eventsource::EventSource::new(request)?;

    while let Some(event) = es.next().await {
        match event {
            Ok(reqwest_eventsource::Event::Open) => {
                println!("Watching workflow...");
            }
            Ok(reqwest_eventsource::Event::Message(message)) => {
                let grit = match message.event.as_str() {
                    "log" => {
                        let log = serde_json::from_str::<SimpleLogMessage>(&message.data)?;
                        GritEvent::Log(log)
                    }
                    "end" => GritEvent::End,
                    _ => GritEvent::Unknown,
                };
                match grit {
                    GritEvent::Log(log) => {
                        if let Err(err) = emitter.emit_log(&log) {
                            eprintln!("Error emitting log: {}", err);
                        }
                    }
                    GritEvent::End => {
                        es.close();
                    }
                    GritEvent::Unknown => {
                        eprintln!("Unknown event received: {:?}", message.event);
                    }
                }
            }
            Err(err) => {
                println!("Error watching workflow: {}", err);
                es.close();
            }
        }
    }

    Ok(())
}
