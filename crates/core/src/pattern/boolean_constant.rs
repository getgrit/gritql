use super::{
    patterns::{Matcher, Name},
    resolved_pattern::ResolvedPattern,
    state::State,
};
use crate::context::Context;
use anyhow::Result;
use marzano_util::analysis_logs::AnalysisLogs;
use tree_sitter::Node;

#[derive(Debug, Clone)]
pub struct BooleanConstant {
    pub value: bool,
}

impl BooleanConstant {
    pub fn new(value: bool) -> Self {
        Self { value }
    }

    pub(crate) fn from_node(node: &Node, src: &str) -> Result<Self> {
        let text = node.utf8_text(src.as_bytes())?.trim().to_string();
        let value = match text.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(anyhow::anyhow!("Invalid boolean value")),
        };
        Ok(Self::new(value))
    }
}

impl Name for BooleanConstant {
    fn name(&self) -> &'static str {
        "BOOLEAN_CONSTANT"
    }
}

impl Matcher for BooleanConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        binding
            .is_truthy(state)
            .map(|truthiness| truthiness == self.value)
    }
}
