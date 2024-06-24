use marzano_core::api::AnalysisLogLevel;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleLogMessage {
    pub message: String,
    pub level: AnalysisLogLevel,
    pub meta: Option<std::collections::HashMap<String, serde_json::Value>>,
}
