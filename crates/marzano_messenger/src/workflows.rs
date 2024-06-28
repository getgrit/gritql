use std::path::PathBuf;

use marzano_core::api::MatchResult;
use serde::{Deserialize, Serialize};

use crate::emit::{Messager, VisibilityLevels};

#[derive(Deserialize, Serialize, Debug)]
pub struct PackagedWorkflowOutcome {
    pub message: Option<String>,
    pub success: bool,
    pub data: Option<serde_json::Value>,
}

/// Handle workflow-related messages
pub trait WorkflowMessenger: Messager {
    fn save_metadata(&mut self, message: &SimpleWorkflowMessage) -> anyhow::Result<()>;

    /// Emit a match result from a workflow, which has some additional metadata around workspace and paths
    fn emit_from_workflow(&mut self, message: &WorkflowMatchResult) -> anyhow::Result<()>;
}

/// Simple workflow message representation, mainly intended for RPC
#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleWorkflowMessage {
    pub kind: String,
    pub message: serde_json::Value,
}

/// Wrap match results to account for workflow logic and path normalization
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowMatchResult {
    pub result: MatchResult,
    pub workspace_path: Option<PathBuf>,
}
