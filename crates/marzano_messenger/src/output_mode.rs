
use clap::ValueEnum;
use serde::Serialize;

/// `OutputMode`` represents how *much* information to show for each result
/// cf. with `VisibilityLevels` which represents *which* results to show
#[derive(Debug, PartialEq, PartialOrd, Clone, ValueEnum, Serialize, Default)]
pub enum OutputMode {
    None,
    #[default]
    Standard,
    Compact,
}

impl std::fmt::Display for OutputMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}