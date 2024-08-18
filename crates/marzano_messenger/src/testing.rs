use anyhow::Result;
use marzano_core::api::MatchResult;

use crate::{
    emit::{FlushableMessenger, Messager, VisibilityLevels},
    workflows::WorkflowMessenger,
};

/// A testing messenger that doesn't actually send messages anywhere.
///
/// This should be used in tests to avoid sending messages to real backends.
pub struct TestingMessenger {
    message_count: usize,
    log_count: usize,
}

impl TestingMessenger {
    pub fn new() -> Self {
        Self {
            message_count: 0,
            log_count: 0,
        }
    }

    pub fn total_count(&self) -> usize {
        self.log_count + self.message_count
    }
}

impl Default for TestingMessenger {
    fn default() -> Self {
        Self::new()
    }
}

impl Messager for TestingMessenger {
    fn start_workflow(&mut self) -> anyhow::Result<()> {
        Ok(())
    }

    fn finish_workflow(
        &mut self,
        _outcome: &crate::workflows::PackagedWorkflowOutcome,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn get_workflow_status(
        &mut self,
    ) -> anyhow::Result<Option<&crate::workflows::PackagedWorkflowOutcome>> {
        Ok(None)
    }

    fn get_min_level(&self) -> VisibilityLevels {
        VisibilityLevels::Debug
    }

    fn raw_emit(&mut self, _message: &MatchResult) -> Result<()> {
        self.message_count += 1;
        Ok(())
    }

    fn emit_log(&mut self, _log: &crate::SimpleLogMessage) -> anyhow::Result<()> {
        self.log_count += 1;
        Ok(())
    }
}

impl FlushableMessenger for TestingMessenger {
    async fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl WorkflowMessenger for TestingMessenger {
    fn save_metadata(
        &mut self,
        _metadata: &crate::workflows::SimpleWorkflowMessage,
    ) -> anyhow::Result<()> {
        Ok(())
    }

    fn emit_from_workflow(
        &mut self,
        message: &crate::workflows::WorkflowMatchResult,
    ) -> anyhow::Result<()> {
        self.emit(&message.result)
    }
}
