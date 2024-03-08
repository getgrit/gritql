use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PackagedWorkflowOutcome {
    pub message: Option<String>,
    pub success: bool,
    pub data: Option<serde_json::Value>,
}
