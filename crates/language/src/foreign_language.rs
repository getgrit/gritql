use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

use grit_util::error::{GritPatternError, GritResult};

#[derive(Debug, Clone)]
pub enum ForeignLanguage {
    JavaScript,
}

impl Display for ForeignLanguage {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            ForeignLanguage::JavaScript => write!(f, "js"),
        }
    }
}

impl TryFrom<&str> for ForeignLanguage {
    type Error = GritPatternError;

    fn try_from(value: &str) -> GritResult<Self> {
        match value {
            "js" => Ok(Self::JavaScript),
            lang => Err(GritPatternError::new(format!(
                "Foreign language {} is unsupported",
                lang
            ))),
        }
    }
}

impl TryFrom<Cow<'_, str>> for ForeignLanguage {
    type Error = GritPatternError;

    fn try_from(value: Cow<str>) -> GritResult<Self> {
        value.as_ref().try_into()
    }
}
