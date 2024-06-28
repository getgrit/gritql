use anyhow::{bail, Result};
use marzano_core::api::AnalysisLog;
use marzano_messenger::{
    emit::{FlushableMessenger, Messager},
    output_mode::OutputMode,
    workflows::{PackagedWorkflowOutcome, WorkflowMessenger},
};
use std::{
    io::{self, Write},
    path::PathBuf,
};

#[cfg(feature = "server")]
use cli_server::combined::CombinedMessenger;
#[cfg(feature = "remote_pubsub")]
use cli_server::pubsub::GooglePubSubMessenger;
#[cfg(feature = "remote_redis")]
use cli_server::redis::RedisMessenger;
#[cfg(feature = "server")]
use cli_server::workflows::RemoteWorkflowMessenger;

use crate::{
    flags::OutputFormat,
    jsonl::JSONLineMessenger,
    result_formatting::{FormattedMessager, TransformedMessenger},
};

#[allow(clippy::large_enum_variant)]
pub enum MessengerVariant<'a> {
    Formatted(FormattedMessager<'a>),
    JsonLine(JSONLineMessenger<'a>),
    Transformed(TransformedMessenger<'a>),
    #[cfg(feature = "remote_redis")]
    Redis(RedisMessenger),
    #[cfg(feature = "remote_pubsub")]
    GooglePubSub(GooglePubSubMessenger),
    #[cfg(feature = "server")]
    Combined(CombinedMessenger),
}

impl<'a> Messager for MessengerVariant<'a> {
    fn raw_emit(&mut self, message: &marzano_core::api::MatchResult) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(m) => m.raw_emit(message),
            MessengerVariant::Transformed(m) => m.raw_emit(message),
            MessengerVariant::JsonLine(m) => m.raw_emit(message),
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.raw_emit(message),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.raw_emit(message),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.raw_emit(message),
        }
    }

    fn emit_log(&mut self, log: &marzano_messenger::SimpleLogMessage) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(m) => m.emit_log(log),
            MessengerVariant::Transformed(m) => m.emit_log(log),
            MessengerVariant::JsonLine(m) => m.emit_log(log),
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.emit_log(log),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.emit_log(log),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.emit_log(log),
        }
    }

    fn emit_estimate(&mut self, count: usize) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(m) => m.emit_estimate(count),
            MessengerVariant::Transformed(m) => m.emit_estimate(count),
            MessengerVariant::JsonLine(m) => m.emit_estimate(count),
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.emit_estimate(count),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.emit_estimate(count),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.emit_estimate(count),
        }
    }

    fn start_workflow(&mut self) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(m) => m.start_workflow(),
            MessengerVariant::Transformed(m) => m.start_workflow(),
            MessengerVariant::JsonLine(m) => m.start_workflow(),
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.start_workflow(),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.start_workflow(),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.start_workflow(),
        }
    }

    fn finish_workflow(&mut self, outcome: &PackagedWorkflowOutcome) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(m) => m.finish_workflow(outcome),
            MessengerVariant::Transformed(m) => m.finish_workflow(outcome),
            MessengerVariant::JsonLine(m) => m.finish_workflow(outcome),
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.finish_workflow(outcome),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.finish_workflow(outcome),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.finish_workflow(outcome),
        }
    }
}

impl<'a> WorkflowMessenger for MessengerVariant<'a> {
    fn save_metadata(
        &mut self,
        message: &marzano_messenger::workflows::SimpleWorkflowMessage,
    ) -> anyhow::Result<()> {
        match self {
            MessengerVariant::Formatted(_)
            | MessengerVariant::Transformed(_)
            | MessengerVariant::JsonLine(_) => {
                // These are local, so no need to save metadata
                log::debug!(
                    "Skipping save_metadata for local messenger: {} {:?}",
                    message.kind,
                    message.message
                );
                Ok(())
            }
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(m) => m.save_metadata(message),
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(m) => m.save_metadata(message),
            #[cfg(feature = "server")]
            MessengerVariant::Combined(m) => m.save_metadata(message),
        }
    }
}

impl<'a> From<FormattedMessager<'a>> for MessengerVariant<'a> {
    fn from(value: FormattedMessager<'a>) -> Self {
        Self::Formatted(value)
    }
}

impl<'a> From<JSONLineMessenger<'a>> for MessengerVariant<'a> {
    fn from(value: JSONLineMessenger<'a>) -> Self {
        Self::JsonLine(value)
    }
}

impl<'a> From<TransformedMessenger<'a>> for MessengerVariant<'a> {
    fn from(value: TransformedMessenger<'a>) -> Self {
        Self::Transformed(value)
    }
}

#[cfg(feature = "remote_redis")]
impl<'a> From<cli_server::redis::RedisMessenger> for MessengerVariant<'a> {
    fn from(value: cli_server::redis::RedisMessenger) -> Self {
        Self::Redis(value)
    }
}

#[cfg(feature = "remote_pubsub")]
impl<'a> From<cli_server::pubsub::GooglePubSubMessenger> for MessengerVariant<'a> {
    fn from(value: cli_server::pubsub::GooglePubSubMessenger) -> Self {
        Self::GooglePubSub(value)
    }
}

#[cfg(feature = "server")]
impl<'a> From<cli_server::combined::CombinedMessenger> for MessengerVariant<'a> {
    fn from(value: cli_server::combined::CombinedMessenger) -> Self {
        Self::Combined(value)
    }
}

impl<'a> MessengerVariant<'a> {
    /// Get the fatal error, if any
    /// If a fatal error is present, it should be shown to the user
    pub fn get_fatal_error(&self) -> Option<&AnalysisLog> {
        match self {
            #[cfg(feature = "server")]
            MessengerVariant::Combined(ref combined) => combined.fatal_error.as_ref(),
            _ => None,
        }
    }
}

impl FlushableMessenger for MessengerVariant<'_> {
    async fn flush(&mut self) -> anyhow::Result<()> {
        match self {
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Redis(ref mut redis) => redis.flush().await,
            #[cfg(feature = "remote_pubsub")]
            MessengerVariant::GooglePubSub(ref mut pubsub) => pubsub.flush().await,
            #[cfg(feature = "remote_redis")]
            MessengerVariant::Combined(ref mut combined) => combined.flush().await,
            _ => {
                // do nothing
                Ok(())
            }
        }
    }
}

pub async fn create_emitter<'a>(
    format: &OutputFormat,
    mode: OutputMode,
    output_file: Option<&PathBuf>,
    interactive: bool,
    pattern: Option<&str>,
    _root_path: Option<&PathBuf>,
) -> Result<MessengerVariant<'a>> {
    let writer: Option<Box<dyn Write + Send>> = if let Some(output_file) = output_file {
        let file = fs_err::File::create(output_file)?;
        let bufwriter = io::BufWriter::new(file);
        Some(Box::new(bufwriter))
    } else {
        None
    };

    let emitter: MessengerVariant = match format {
        OutputFormat::Standard => FormattedMessager::new(
            writer,
            mode,
            interactive,
            pattern.unwrap_or_default().to_string(),
        )
        .into(),
        OutputFormat::Json => {
            bail!("JSON output is not supported for apply_pattern");
        }
        OutputFormat::Transformed => TransformedMessenger::new(writer).into(),
        OutputFormat::Jsonl => {
            let jsonl =
                JSONLineMessenger::new(writer.unwrap_or_else(|| Box::new(io::stdout())), mode);
            jsonl.into()
        }
        #[cfg(feature = "remote_redis")]
        OutputFormat::Redis => {
            let messenger = RedisMessenger::create(mode, None, _root_path)?;
            messenger.into()
        }
        #[cfg(feature = "remote_pubsub")]
        OutputFormat::PubSub => {
            let pubsub = GooglePubSubMessenger::create(mode, None, _root_path).await?;
            pubsub.into()
        }
        #[cfg(feature = "server")]
        OutputFormat::Combined => {
            let combined = CombinedMessenger::create(mode, _root_path).await?;
            combined.into()
        }
    };

    Ok(emitter)
}
