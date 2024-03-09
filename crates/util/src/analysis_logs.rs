use crate::position::{Position, Range};
use anyhow::bail;
use anyhow::Result;
use derive_builder::Builder;
use std::fmt::{self, Debug, Display};
use std::ops;

// TypedBuilder offers type safe builds at compile time.
// unfortunatly it's a consuming builder, I don't believe it's
// possible to make a non consuming typed-builder.
#[derive(Debug, Clone, Default, Builder)]
#[builder(public, setter(into))]
pub struct AnalysisLog {
    #[builder(setter(into, strip_option), default)]
    pub engine_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub file: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub level: Option<u16>,
    pub message: String,
    #[builder(setter(into, strip_option), default)]
    pub position: Option<Position>,
    #[builder(setter(into, strip_option), default)]
    pub range: Option<Range>,
    #[builder(setter(into, strip_option), default)]
    pub syntax_tree: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub source: Option<String>,
}

pub struct AnalysisLogs(Vec<AnalysisLog>);

impl AnalysisLogs {
    pub fn logs(self) -> Vec<AnalysisLog> {
        self.0
    }

    pub fn debug_no_state(&mut self, message: &str) -> Result<()> {
        let mut builder = AnalysisLogBuilder::default();
        builder.level(501_u16);
        builder.message(message);
        let log = builder.build();
        match log {
            Ok(log) => self.0.push(log),
            Err(err) => {
                bail!(err);
            }
        }
        Ok(())
    }
}

impl fmt::Display for AnalysisLogs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_fold((), |_, log| writeln!(f, "{}", log))
    }
}

impl From<Vec<AnalysisLog>> for AnalysisLogs {
    fn from(logs: Vec<AnalysisLog>) -> Self {
        Self(logs)
    }
}

impl ops::Deref for AnalysisLogs {
    type Target = Vec<AnalysisLog>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl ops::DerefMut for AnalysisLogs {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl Debug for AnalysisLogs {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.0.iter().try_fold((), |_, log| writeln!(f, "{}", log))
    }
}

impl Display for AnalysisLog {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "engine_id: {:?}\nfile_name: {:?}\nlevel: {:?}\nmessage: {:?}\nposition: {:?}",
            self.engine_id, self.file, self.level, self.message, self.position
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::{anyhow, Result};

    fn analysis_error(o: Option<usize>) -> Result<usize> {
        o.ok_or_else(|| {
            anyhow!(AnalysisLogBuilder::default()
                .message("test".to_string())
                .build()
                .unwrap())
        })
    }

    fn call_analysis_error() -> Result<usize> {
        analysis_error(None)?;
        analysis_error(Some(1))
    }

    #[test]
    fn test_analysis_log_downcast() {
        let result = call_analysis_error();
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(err.is::<AnalysisLog>());
        let log = err.downcast::<AnalysisLog>().unwrap();
        assert_eq!(log.message, "test");
    }
}
