use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct SimpleLogMessage {
    pub message: String,
}
