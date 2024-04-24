use super::{compiler::NodeCompilationContext, node_compiler::NodeCompiler};
use anyhow::Result;
use grit_core_patterns::pattern::{
    boolean_constant::BooleanConstant, float_constant::FloatConstant, int_constant::IntConstant,
    string_constant::StringConstant,
};
use grit_util::AstNode;
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct BooleanConstantCompiler;

impl NodeCompiler for BooleanConstantCompiler {
    type TargetPattern = BooleanConstant;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let text = node.text()?.trim().to_string();
        let value = match text.as_str() {
            "true" => true,
            "false" => false,
            _ => return Err(anyhow::anyhow!("Invalid boolean value")),
        };
        Ok(BooleanConstant::new(value))
    }
}

pub(crate) struct FloatConstantCompiler;

impl NodeCompiler for FloatConstantCompiler {
    type TargetPattern = FloatConstant;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let text = node.text()?.trim().to_string();
        let value = text.parse::<f64>()?;
        Ok(FloatConstant::new(value))
    }
}

pub(crate) struct IntConstantCompiler;

impl NodeCompiler for IntConstantCompiler {
    type TargetPattern = IntConstant;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let text = node.text()?.trim().to_string();
        let value = text.parse::<i64>()?;
        Ok(IntConstant::new(value))
    }
}

pub(crate) struct StringConstantCompiler;

impl NodeCompiler for StringConstantCompiler {
    type TargetPattern = StringConstant;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        _context: &mut NodeCompilationContext,
        _is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let text = node.text()?.trim().to_string();
        let text = text.strip_prefix('\"').unwrap().strip_suffix('\"').unwrap();
        let text = text.replace("\\\"", "\"").replace("\\\\", "\\");
        Ok(StringConstant::new(text))
    }
}
