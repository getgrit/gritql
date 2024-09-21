use anyhow::{anyhow, Result};
use std::{
    borrow::Cow,
    fmt::{Display, Formatter},
};

#[derive(Debug, Clone, Copy)]
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
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "js" => Ok(Self::JavaScript),
            lang => Err(anyhow!("Foreign language {} is unsupported", lang)),
        }
    }
}

impl TryFrom<Cow<'_, str>> for ForeignLanguage {
    type Error = anyhow::Error;

    fn try_from(value: Cow<str>) -> Result<Self, Self::Error> {
        value.as_ref().try_into()
    }
}
