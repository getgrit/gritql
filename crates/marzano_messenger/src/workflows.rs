use std::path::PathBuf;

use marzano_core::api::MatchResult;
use serde::{Deserialize, Serialize};
use std::sync::OnceLock;

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

#[derive(Deserialize, Serialize, Debug, Clone)]
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
    pub step_id: Option<String>,
}

/// Wrap match results to account for workflow logic and path normalization
#[derive(Debug, Serialize, Deserialize)]
pub struct WorkflowMatchResult {
    pub result: MatchResult,
    pub workspace_path: Option<PathBuf>,
    pub step_id: String,
}

/// Status manager makes it easier to implement the required parts of the workflow status API
/// It sets the status of the workflow the first time it's updated, and then ignores all further updates
#[derive(Clone)]
pub struct StatusManager {
    status: OnceLock<PackagedWorkflowOutcome>,
}

impl StatusManager {
    pub fn new() -> Self {
        Self {
            status: OnceLock::new(),
        }
    }

    /// Set the status of the workflow, but only if it hasn't been set before
    pub fn upsert(&mut self, outcome: &PackagedWorkflowOutcome) -> bool {
        let did_set = self.status.set(outcome.clone());
        // Ok(true) when we set the status ourselves, Err if it was already set
        if did_set.is_ok() {
            return true;
        }

        false
    }

    /// Get the status of the workflow, if it has been set
    pub fn get_workflow_status(&mut self) -> anyhow::Result<Option<&PackagedWorkflowOutcome>> {
        if let Some(outcome) = self.status.get() {
            return Ok(Some(outcome));
        }

        Ok(None)
    }
}

impl Default for StatusManager {
    fn default() -> Self {
        Self::new()
    }
}
