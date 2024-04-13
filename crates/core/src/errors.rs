use anyhow::{bail, Result};
use marzano_util::analysis_logs::{AnalysisLogBuilder, AnalysisLogs};

use crate::{
    context::ProblemContext,
    pattern::{state::State, variable::get_file_name},
};

pub fn debug<P: ProblemContext>(
    analysis_logs: &mut AnalysisLogs,
    state: &State<'_, P>,
    message: &str,
) -> Result<()> {
    let mut builder = AnalysisLogBuilder::default();
    builder.level(501_u16);
    builder.message(message);

    if let Ok(file) = get_file_name(state) {
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
