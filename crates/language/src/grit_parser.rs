use crate::language::Tree;
use anyhow::{anyhow, bail, Result};
use grit_util::{traverse, AnalysisLogBuilder, AnalysisLogs, Ast, AstNode, Order};
use regex::Regex;
use std::path::Path;
use tree_sitter::Parser as TSParser;

pub struct MarzanoGritParser {
    parser: TSParser,
}

impl MarzanoGritParser {
    #[cfg(feature = "grit-parser")]
    pub fn new() -> Result<Self> {
        let mut parser = TSParser::new().unwrap();
        parser
            .set_language(&tree_sitter_gritql::language().into())
            .expect("failed to load grit language");
        Ok(Self { parser })
    }

    #[cfg(not(feature = "grit-parser"))]
    pub fn new() -> Result<Self> {
        bail!("enable grit-parser feature flag to make a grit parser")
    }

    pub fn from_initialized_ts_parser(parser: TSParser) -> Self {
        Self { parser }
    }

    pub fn parse(&mut self, source: &str, file_name: Option<&Path>) -> Result<Tree> {
        let tree = self
            .parser
            .parse(source, None)?
            .ok_or_else(|| anyhow!("parse error"))?;

        let tree = Tree::new(tree, source);
        let parse_errors = grit_parsing_errors(&tree, file_name)?;
        if !parse_errors.is_empty() {
            let error = parse_errors[0].clone();
            bail!(error);
        }

        Ok(tree)
    }
}

fn grit_parsing_errors(tree: &Tree, file_name: Option<&Path>) -> Result<AnalysisLogs> {
    let mut errors = vec![];
    let mut log_builder = AnalysisLogBuilder::default();
    let level: u16 = if file_name.is_some() { 300 } else { 299 };
    log_builder
        .level(level)
        .engine_id("marzano(0.1)".to_owned());
    if let Some(file_name) = file_name {
        log_builder.file(file_name.to_owned());
    }

    let root = tree.root_node();
    let cursor = root.walk();
    for n in traverse(cursor, Order::Pre) {
        if n.node.is_error() || n.node.is_missing() {
            let position = n.range().start;

            let error_node = n.text()?;
            let identifier_regex = Regex::new(r"^([A-Za-z0-9_]*)\(\)$")?;
            let message = if let Some(found) = identifier_regex.find(&error_node) {
                format!(
                    "{} is a reserved keyword in Grit. Try renaming your pattern.",
                    found.as_str().trim_end_matches("()")
                )
            } else {
                let file_locations_str = file_name
                    .map(|file_name| format!(" in {}", file_name.display()))
                    .unwrap_or_default();
                format!(
                    "Pattern syntax error at {position}{file_locations_str}. \
                        If you hit this error while running grit apply on a \
                        pattern from the Grit standard library, try running \
                        grit init. If you are running a custom pattern, check \
                        out the docs at https://docs.grit.io/ for help with \
                        writing patterns.",
                )
            };

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