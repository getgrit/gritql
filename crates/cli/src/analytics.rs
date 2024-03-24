use anyhow::Context;
use anyhow::Result;
use clap::Args;
use lazy_static::lazy_static;
use marzano_gritmodule::{fetcher::ModuleRepo, searcher::find_git_dir_from};
use marzano_messenger::emit::ApplyDetails;
use serde::{Deserialize, Serialize};
use std::{env, fmt, path::PathBuf, time::Duration};
use uuid::Uuid;

use crate::commands::Commands;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum AnalyticsEventName {
    #[serde(rename = "command-invoked")]
    Invoked,
    #[serde(rename = "command-completed")]
    Completed,
    #[serde(rename = "command-errored")]
    Errored,
}

impl fmt::Display for AnalyticsEventName {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalyticsEventName::Invoked => write!(f, "command-invoked"),
            AnalyticsEventName::Completed => write!(f, "command-completed"),
            AnalyticsEventName::Errored => write!(f, "command-errored"),
        }
    }
}

impl<'a> From<&'a AnalyticsEvent<'a>> for AnalyticsEventName {
    fn from(event: &'a AnalyticsEvent) -> Self {
        match event {
            AnalyticsEvent::Invoked(_) => AnalyticsEventName::Invoked,
            AnalyticsEvent::Completed(_) => AnalyticsEventName::Completed,
            AnalyticsEvent::Errored(_) => AnalyticsEventName::Errored,
        }
    }
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum AnalyticsEvent<'a> {
    Invoked(InvokedEvent<'a>),
    Completed(CompletedEvent),
    Errored(ErroredEvent),
}

impl AnalyticsEvent<'_> {
    pub fn from_cmd(cmd: &Commands) -> AnalyticsEvent {
        AnalyticsEvent::Invoked(InvokedEvent { cmd })
    }
}

impl fmt::Display for AnalyticsEvent<'_> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AnalyticsEvent::Invoked(_) => write!(f, "command-invoked"),
            AnalyticsEvent::Completed(_) => write!(f, "command-completed"),
            AnalyticsEvent::Errored(_) => write!(f, "command-errored"),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct InvokedEvent<'a> {
    cmd: &'a Commands,
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct CompletedEvent {
    runtime: u128,
    #[serde(flatten)]
    details: Option<ApplyDetails>,
}

impl CompletedEvent {
    pub fn from_res(elapsed: Duration, details: Option<ApplyDetails>) -> Self {
        Self {
            runtime: elapsed.as_millis(),
            details,
        }
    }
}

#[derive(Debug, Serialize, PartialEq, Clone)]
pub struct ErroredEvent {
    runtime: u128,
}

impl ErroredEvent {
    pub fn from_elapsed(elapsed: Duration) -> Self {
        Self {
            runtime: elapsed.as_millis(),
        }
    }
}

#[derive(Args, Debug, Serialize)]
pub struct AnalyticsArgs {
    #[clap(long = "command")]
    pub command: String,

    #[clap(long = "args")]
    pub args: Vec<String>,

    #[clap(long = "installation-id")]
    pub installation_id: Uuid,

    #[clap(long = "user-id")]
    pub user_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AnalyticsProperties {
    pub command: String,
    pub args: Vec<String>,
    pub repository: Option<ModuleRepo>,
    #[serde(flatten)]
    pub data: Option<serde_json::Value>,
}

lazy_static! {
    pub static ref SEGMENT_WRITE_KEY: String = String::from("iWHCQWfroQzvbUKTJ9xlXB7U9YDQWnyD");
}

#[derive(Serialize, Debug)]
#[serde(rename_all = "camelCase")]
struct SegmentPayload<'a> {
    user_id: Option<String>,
    ///
    /// Anonymous ID is used, as we don't
    /// actually know who the human behind
    /// this installation is.
    ///
    /// https://segment.com/docs/connections/spec/identify/#anonymous-id
    ///
    anonymous_id: Uuid,
    event: &'a AnalyticsEventName,
    properties: AnalyticsProperties,
}

pub async fn track_event_line(
    line: &str,
    command: String,
    args: Vec<String>,
    installation_id: Uuid,
    user_id: Option<String>,
) -> Result<()> {
    let (name, json) = line
        .split_once('\t')
        .ok_or(anyhow::anyhow!("Invalid line, no tab found"))?;
    let event = serde_json::from_str::<AnalyticsEventName>(name).context("Invalid event name")?;
    let data = serde_json::from_str::<serde_json::Value>(json).context("Invalid event data")?;

    track_event(event, data, command, args, installation_id, user_id).await;

    Ok(())
}

pub async fn track_event(
    analytics_event_name: AnalyticsEventName,
    analytics_event_data: serde_json::Value,
    command: String,
    args: Vec<String>,
    installation_id: Uuid,
    user_id: Option<String>,
) {
    if is_telemetry_disabled() {
        return;
    }
    let command_str = command.to_string();
    let command_parts: Vec<&str> = command_str.split_whitespace().collect();
    let last_command_part = command_parts.last().unwrap().to_string();

    let git_dir = match env::current_dir() {
        Ok(dir) => find_git_dir_from(dir).await,
        Err(_) => None,
    };
    let repository = git_dir.map(|dir| ModuleRepo::from_git_dir(&PathBuf::from(dir)));

    let properties = AnalyticsProperties {
        command: command_str,
        args: if let Some(last_index) = args.iter().rposition(|arg| *arg == last_command_part) {
            args[(last_index + 1)..].to_vec()
        } else {
            args
        },
        repository,
        data: Some(analytics_event_data),
    };

    let payload = SegmentPayload {
        user_id,
        anonymous_id: installation_id,
        event: &analytics_event_name,
        properties,
    };

    //
    // https://segment.com/docs/connections/sources/catalog/libraries/server/http-api/#track
    //
    match reqwest::Client::new()
        .post("https://api.segment.io/v1/track")
        .basic_auth::<&String, &str>(&SEGMENT_WRITE_KEY, None)
        .json(&payload)
        .timeout(Duration::from_secs(5))
        .send()
        .await
    {
        Ok(response) => {
            if !response.status().is_success() {
                eprintln!(
                    "Failed to send event {}: {}",
                    analytics_event_name,
                    response.status()
                );
            }
        }
        Err(e) => {
            eprintln!("Failed to send event {}: {:#}", analytics_event_name, e);
        }
    }

    println!("Successfully sent event {}", analytics_event_name);
}

pub fn is_telemetry_disabled() -> bool {
    env::var("GRIT_TELEMETRY_DISABLED")
        .unwrap_or_else(|_| "false".to_owned())
        .parse::<bool>()
        .unwrap_or(false)
}

/// By default, telemetry is sent in the background so the main process can exit quickly.
/// If this environment variable is set to true, telemetry will be sent in the foreground.
/// This is useful for debugging telemetry issues.
pub fn is_telemetry_foregrounded() -> bool {
    env::var("GRIT_TELEMETRY_FOREGROUND")
        .unwrap_or_else(|_| "false".to_owned())
        .parse::<bool>()
        .unwrap_or(false)
}
