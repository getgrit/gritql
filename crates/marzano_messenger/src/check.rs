use marzano_core::api::EnforcementLevel;
use serde::{Deserialize, Serialize};

/// Handle messages from check runs
pub trait CheckMessenger {
    fn mark_check(&mut self, name: &str, level: &EnforcementLevel) -> anyhow::Result<()>;
}

/// A simple message to send to the check messenger, for use from workflows
#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleCheckMessage {
    pub name: String,
    pub level: EnforcementLevel,
}
