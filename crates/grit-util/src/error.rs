use regex::Error as RegexError;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use std::str::Utf8Error;
use std::string::FromUtf8Error;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum GritPatternError {
    #[error("Matcher: {0}")]
    Matcher(String),

    #[error(transparent)]
    Io(#[from] io::Error),

    #[error(transparent)]
    ParseFloat(#[from] ParseFloatError),

    #[error(transparent)]
    ParseInt(#[from] ParseIntError),

    #[error(transparent)]
    Regex(#[from] RegexError),

    #[error(transparent)]
    Utf8(#[from] Utf8Error),

    #[error(transparent)]
    FromUtf8(#[from] FromUtf8Error),

    #[error("[Builder] {0}")]
    Builder(String),

    #[error("{0}")]
    Generic(String),
}

impl GritPatternError {
    pub fn new_matcher(reason: impl Into<String>) -> Self {
        Self::Matcher(reason.into())
    }

    pub fn new(reason: impl Into<String>) -> Self {
        Self::Generic(reason.into())
    }
}

pub type GritResult<R> = Result<R, GritPatternError>;
