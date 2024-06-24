use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct PackagedWorkflowOutcome {
    pub message: Option<String>,
    pub success: bool,
    pub data: Option<serde_json::Value>,
}

/// Handle workflow-related messages
pub trait WorkflowMessenger {
    fn save_metadata(&mut self, message: &SimpleWorkflowMessage) -> anyhow::Result<()>;
}

/// Simple workflow message representation, mainly intended for RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleWorkflowMessage {
    pub kind: String,
    pub message: serde_json::Value,
}
