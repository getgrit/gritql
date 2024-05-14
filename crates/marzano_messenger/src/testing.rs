use anyhow::Result;
use marzano_core::api::MatchResult;

use crate::emit::Messager;

/// A testing messenger that doesn't actually send messages anywhere.
///
/// This should be used in tests to avoid sending messages to real backends.
pub struct TestingMessenger {
    message_count: usize,
}

impl TestingMessenger {
    pub fn new() -> Self {
        Self { message_count: 0 }
    }

    pub fn message_count(&self) -> usize {
        self.message_count
    }
}

impl Messager for TestingMessenger {
    fn raw_emit(&mut self, message: &MatchResult) -> anyhow::Result<()> {
        self.message_count += 1;
        Ok(())
    }
}
