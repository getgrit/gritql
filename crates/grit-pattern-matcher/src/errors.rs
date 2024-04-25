use crate::{
    context::QueryContext,
    pattern::{get_file_name, State},
};
use anyhow::{bail, Result};
use grit_util::{AnalysisLogBuilder, AnalysisLogs};

pub fn debug<'a, Q: QueryContext>(
    analysis_logs: &mut AnalysisLogs,
    state: &State<'a, Q>,
    lang: &Q::Language<'a>,
    message: &str,
) -> Result<()> {
    let mut builder = AnalysisLogBuilder::default();
    builder.level(501_u16);
    builder.message(message);

    if let Ok(file) = get_file_name(state, lang) {
        builder.file(file);
    }

    let log = builder.build();
    match log {
        Ok(log) => analysis_logs.push(log),
        Err(err) => {
            bail!(err);
        }
    }
    Ok(())
}
