use std::path::PathBuf;

use marzano_core::api::MatchResult;
use serde::{Deserialize, Serialize};

use crate::emit::Messager;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub enum OutcomeKind {
    #[serde(rename = "success")]
    Success,
    #[serde(rename = "failure")]
    Failure,
    #[serde(rename = "skipped")]
    Skipped,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct PackagedWorkflowOutcome {
    pub message: Option<String>,
    pub outcome: Option<OutcomeKind>,
    pub success: bool,
    pub data: Option<serde_json::Value>,
}

impl PackagedWorkflowOutcome {
    pub fn get_outcome(&self) -> OutcomeKind {
        if let Some(outcome) = self.outcome.as_ref() {
            return outcome.clone();
        }

        if self.success {
            OutcomeKind::Success
        } else {
            OutcomeKind::Failure
        }
    }
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
    pub step_id: String,
}

/// Wrap match results to account for workflow logic and path normalization
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowMatchResult {
    pub result: MatchResult,
    pub workspace_path: Option<PathBuf>,
    pub step_id: String,
}
