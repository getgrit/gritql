use crate::{Position, Range};
use derive_builder::Builder;
use std::fmt::{self, Debug, Display};
use std::ops;
use std::path::PathBuf;

// TypedBuilder offers type safe builds at compile time.
// unfortunatly it's a consuming builder, I don't believe it's
// possible to make a non consuming typed-builder.
#[derive(Builder, Clone, Debug, Default)]
pub struct AnalysisLog {
    #[builder(setter(into, strip_option), default)]
    pub engine_id: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub file: Option<PathBuf>,
    #[builder(setter(into, strip_option), default)]
    pub level: Option<u16>,
    #[builder(setter(into))]
    pub message: String,
    #[builder(setter(into, strip_option), default)]
    pub position: Option<Position>,
    // FIXME: We seem to only need this for the end position, and I don't see
    // any usages for the byte indices either. We can probably trim this.
    #[builder(setter(into, strip_option), default)]
    pub range: Option<Range>,
    #[builder(setter(into, strip_option), default)]
    pub syntax_tree: Option<String>,
    #[builder(setter(into, strip_option), default)]
    pub source: Option<String>,
}

pub struct AnalysisLogs(Vec<AnalysisLog>);

impl AnalysisLogs {
    pub fn new() -> Self {
        Self(Vec::new())
    }
}

impl Default for AnalysisLogs {
    fn default() -> Self {
        Self::new()
    }
}

impl AnalysisLogs {
    pub fn logs(self) -> Vec<AnalysisLog> {
        self.0
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
