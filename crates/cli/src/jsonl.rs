use std::{
    io::Write,
    sync::{Arc, Mutex},
};

use anyhow::anyhow;
use marzano_core::{compact_api::compact, pattern::api::MatchResult};

use marzano_messenger::{emit::Messager, output_mode::OutputMode};

pub struct JSONLineMessenger<'a> {
    writer: Arc<Mutex<Box<dyn Write + Send + 'a>>>,
    mode: OutputMode,
}

impl<'a> JSONLineMessenger<'a> {
    pub fn new<W: Write + Send + 'static>(writer: W, mode: OutputMode) -> Self {
        Self {
            writer: Arc::new(Mutex::new(Box::new(writer))),
            mode,
        }
    }
}

impl<'a> Messager for JSONLineMessenger<'a> {
    fn raw_emit(&mut self, item: &MatchResult) -> anyhow::Result<()> {
        let mut writer = self
            .writer
            .lock()
            .map_err(|_| anyhow!("JSONLineMessenger lock poisoned"))?;
        match self.mode {
            OutputMode::None => {
                // do nothing
            }
            OutputMode::Standard => {
                serde_json::to_writer(&mut *writer, item)?;
                writer.write_all(b"\n")?;
            }
            OutputMode::Compact => {
                serde_json::to_writer(&mut *writer, &compact(item.clone()))?;
                writer.write_all(b"\n")?;
            }
        }

        Ok(())
    }
}
