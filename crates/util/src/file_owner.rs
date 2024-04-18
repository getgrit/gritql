use crate::{
    analysis_logs::{AnalysisLogBuilder, AnalysisLogs},
    cursor_wrapper::CursorWrapper,
    position::Position,
};
use anyhow::{anyhow, Result};
use grit_util::{traverse, Order};
pub(crate) use tree_sitter::Language as TSLanguage;
use tree_sitter::{Parser, Tree};

pub trait FileParser {
    /// tree sitter language to parse the source
    fn get_ts_language(&self) -> &TSLanguage;

    fn parse_file(
        &self,
        name: &str,
        body: &str,
        logs: &mut AnalysisLogs,
        new: bool,
    ) -> Result<Option<Tree>> {
        default_parse_file(self.get_ts_language(), name, body, logs, new)
    }
}

pub fn default_parse_file(
    lang: &TSLanguage,
    name: &str,
    body: &str,
    logs: &mut AnalysisLogs,
    new: bool,
) -> Result<Option<Tree>> {
    let mut parser = Parser::new()?;
    parser.set_language(lang)?;
    let tree = parser
        .parse(body, None)?
        .ok_or_else(|| anyhow!("failed to parse tree"))?;
    let mut errors = file_parsing_error(&tree, name, body, new)?;
    logs.append(&mut errors);
    Ok(Some(tree))
}

fn file_parsing_error(
    tree: &Tree,
    file_name: &str,
    body: &str,
    is_new: bool,
) -> Result<AnalysisLogs> {
    let mut errors = vec![];
    let cursor = tree.walk();
    let mut log_builder = AnalysisLogBuilder::default();
    let level: u16 = if is_new { 531 } else { 300 };
    log_builder
        .level(level)
        .engine_id("marzano(0.1)".to_owned())
        .file(file_name.to_owned());

    for n in traverse(CursorWrapper::new(cursor, body), Order::Pre) {
        let node = &n.node;
        if node.is_error() || node.is_missing() {
            let position: Position = node.range().start_point().into();
            let message = format!("Error parsing source code at {}:{} in {}. This may cause otherwise applicable queries to not match.",
                node.range().start_point().row() + 1, node.range().start_point().column() + 1, file_name);
            let log = log_builder
                .clone()
                .message(message)
                .position(position)
                .build()?;
            errors.push(log);
        }
    }
    Ok(errors.into())
}
