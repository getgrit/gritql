use marzano_core::api::{derive_log_level, AnalysisLog, AnalysisLogLevel};
use serde::{Deserialize, Serialize};

/// Simplified representation of a log message that should be preferred when creating new interfaces
#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct SimpleLogMessage {
    pub message: String,
    pub level: AnalysisLogLevel,
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
    pub step_id: Option<String>,
}

/// Represents a raw log message that can be sent to the server,
/// Some fields are double serialized to handle JSON columns
#[derive(Debug, Serialize, Deserialize)]
pub struct LogMessage<'a> {
    level: AnalysisLogLevel,
    message: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    meta: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    step_id: Option<&'a str>,
}

impl LogMessage<'_> {
    pub fn from_analysis_log<'a>(log: &'a AnalysisLog, step_id: &'a str) -> LogMessage<'a> {
        let meta = serde_json::to_string(&log).unwrap_or_default();
        LogMessage {
            level: derive_log_level(log),
            message: &log.message,
            step_id: Some(step_id),
            meta: Some(meta),
        }
    }

    pub fn from_simple_log(log: &SimpleLogMessage) -> LogMessage<'_> {
        LogMessage {
            level: log.level,
            message: &log.message,
            step_id: log.step_id.as_deref(),
            meta: log.meta.as_ref().map(|m| serde_json::to_string(m).unwrap()),
        }
    }
}

impl<'a> TryFrom<LogMessage<'a>> for SimpleLogMessage {
    type Error = serde_json::Error;

    fn try_from(log: LogMessage<'a>) -> Result<Self, Self::Error> {
        let meta = if let Some(meta) = log.meta.as_ref() {
            serde_json::from_str(meta)?
        } else {
            None
        };
        Ok(SimpleLogMessage {
            message: log.message.to_string(),
            level: log.level,
            meta,
            step_id: log.step_id.map(|s| s.to_string()),
        })
    }
}
