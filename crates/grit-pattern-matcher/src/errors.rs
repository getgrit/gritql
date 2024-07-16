use crate::{
    context::QueryContext,
    pattern::{get_file_name, State},
};
use grit_util::{AnalysisLogBuilder, AnalysisLogs};
use regex::Error as RegexError;
use std::io;
use std::num::{ParseFloatError, ParseIntError};
use thiserror::Error;

pub fn debug<'a, Q: QueryContext>(
    analysis_logs: &mut AnalysisLogs,
    state: &State<'a, Q>,
    lang: &Q::Language<'a>,
    message: &str,
) -> GritResult<()> {
    let mut builder = AnalysisLogBuilder::default();
    builder.level(501_u16);
    builder.message(message);

    if let Ok(file) = get_file_name(state, lang) {
        builder.file(file);
    }

    let log = builder.build();
    match log {
        Ok(log) => analysis_logs.push(log),
        Err(err) => return Err(GritPatternError::Builder(err.to_string())),
    }
    Ok(())
}

pub fn warning<'a, Q: QueryContext>(
    analysis_logs: &mut AnalysisLogs,
    state: &State<'a, Q>,
    lang: &Q::Language<'a>,
    message: &str,
) -> GritResult<()> {
    let mut builder = AnalysisLogBuilder::default();
    builder.level(301_u16);
    builder.message(message);

    if let Ok(file) = get_file_name(state, lang) {
        builder.file(file);
    }

    let log = builder.build();
    match log {
        Ok(log) => analysis_logs.push(log),
        Err(err) => return Err(GritPatternError::Builder(err.to_string())),
    }
    Ok(())
}

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

    #[error("[Builder] {0}")]
    Builder(String),

    #[error("{0}")]
    Generic(String),
}

impl GritPatternError {
    pub(crate) fn new_matcher(reason: impl Into<String>) -> Self {
        Self::Matcher(reason.into())
    }

    pub fn new(reason: impl Into<String>) -> Self {
        Self::Generic(reason.into())
    }
}

pub type GritResult<R> = Result<R, GritPatternError>;
