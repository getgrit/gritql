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
pub struct FloatConstant {
    pub value: f64,
}

impl FloatConstant {
    pub fn new(value: f64) -> Self {
        Self { value }
    }

    pub(crate) fn from_node(node: &Node, src: &str) -> Result<Self> {
        let text = node.utf8_text(src.as_bytes())?.trim().to_string();
        let value = text.parse::<f64>()?;
        Ok(Self::new(value))
    }
}

impl Name for FloatConstant {
    fn name(&self) -> &'static str {
        "DOUBLE_CONSTANT"
    }
}

impl Matcher for FloatConstant {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        _context: &'a impl Context,
        _logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        let text = binding.text(&state.files)?;
        let parsed_double = text.parse::<f64>()?;
        Ok(parsed_double == self.value)
    }
}
