use super::{
    accessor_compiler::AccessorCompiler,
    accumulate_compiler::AccumulateCompiler,
    add_compiler::AddCompiler,
    after_compiler::AfterCompiler,
    and_compiler::AndCompiler,
    any_compiler::AnyCompiler,
    as_compiler::AsCompiler,
    assignment_compiler::AssignmentCompiler,
    before_compiler::BeforeCompiler,
    bubble_compiler::BubbleCompiler,
    compiler::NodeCompilationContext,
    constant_compiler::{
        BooleanConstantCompiler, FloatConstantCompiler, IntConstantCompiler, StringConstantCompiler,
    },
    contains_compiler::ContainsCompiler,
    divide_compiler::DivideCompiler,
    every_compiler::EveryCompiler,
    if_compiler::IfCompiler,
    includes_compiler::IncludesCompiler,
    like_compiler::LikeCompiler,
    limit_compiler::LimitCompiler,
    list_compiler::ListCompiler,
    list_index_compiler::ListIndexCompiler,
    log_compiler::LogCompiler,
    map_compiler::MapCompiler,
    maybe_compiler::MaybeCompiler,
    modulo_compiler::ModuloCompiler,
    multiply_compiler::MultiplyCompiler,
    node_compiler::NodeCompiler,
    not_compiler::NotCompiler,
    or_compiler::OrCompiler,
    range_compiler::RangeCompiler,
    rewrite_compiler::RewriteCompiler,
    sequential_compiler::SequentialCompiler,
    some_compiler::SomeCompiler,
    subtract_compiler::SubtractCompiler,
    variable_compiler::VariableCompiler,
    where_compiler::WhereCompiler,
    within_compiler::WithinCompiler,
};
use crate::pattern::{
    call::Call,
    code_snippet::CodeSnippet,
    dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart},
    patterns::Pattern,
    regex::RegexPattern,
};
use anyhow::{bail, Result};
use marzano_util::node_with_source::NodeWithSource;

pub(crate) struct PatternCompiler;

impl NodeCompiler for PatternCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let kind = node.node.kind();
        match kind.as_ref() {
            "mulOperation" => Ok(Pattern::Multiply(Box::new(MultiplyCompiler::from_node(
                node, context,
            )?))),
            "divOperation" => Ok(Pattern::Divide(Box::new(DivideCompiler::from_node(
                node, context,
            )?))),
            "modOperation" => Ok(Pattern::Modulo(Box::new(ModuloCompiler::from_node(
                node, context,
            )?))),
            "addOperation" => Ok(Pattern::Add(Box::new(AddCompiler::from_node(
                node, context,
            )?))),
            "subOperation" => Ok(Pattern::Subtract(Box::new(SubtractCompiler::from_node(
                node, context,
            )?))),
            "patternAs" => Ok(Pattern::Where(Box::new(AsCompiler::from_node(
                node, context,
            )?))),
            "patternLimit" => LimitCompiler::from_node(node, context),
            "assignmentAsPattern" => Ok(Pattern::Assignment(Box::new(
                AssignmentCompiler::from_node(node, context)?,
            ))),
            "patternAccumulate" => Ok(Pattern::Accumulate(Box::new(
                AccumulateCompiler::from_node(node, context)?,
            ))),
            "patternWhere" => Ok(Pattern::Where(Box::new(WhereCompiler::from_node(
                node, context,
            )?))),
            "patternNot" => Ok(Pattern::Not(Box::new(NotCompiler::from_node(
                node, context,
            )?))),
            "patternOr" => OrCompiler::from_node(node, context),
            "patternAnd" => AndCompiler::from_node(node, context),
            "patternAny" => AnyCompiler::from_node(node, context),
            "patternMaybe" => Ok(Pattern::Maybe(Box::new(MaybeCompiler::from_node(
                node, context,
            )?))),
            "patternAfter" => Ok(Pattern::After(Box::new(AfterCompiler::from_node(
                node, context,
            )?))),
            "patternBefore" => Ok(Pattern::Before(Box::new(BeforeCompiler::from_node(
                node, context,
            )?))),
            "patternContains" => Ok(Pattern::Contains(Box::new(ContainsCompiler::from_node(
                node, context,
            )?))),
            "patternIncludes" => Ok(Pattern::Includes(Box::new(IncludesCompiler::from_node(
                node, context,
            )?))),
            "rewrite" => Ok(Pattern::Rewrite(Box::new(RewriteCompiler::from_node(
                node, context,
            )?))),
            "log" => Ok(Pattern::Log(Box::new(LogCompiler::from_node(
                node, context,
            )?))),
            "range" => Ok(Pattern::Range(RangeCompiler::from_node(node, context)?)),
            "patternIfElse" => Ok(Pattern::If(Box::new(IfCompiler::from_node(node, context)?))),
            "within" => Ok(Pattern::Within(Box::new(WithinCompiler::from_node(
                node, context,
            )?))),
            "bubble" => Ok(Pattern::Bubble(Box::new(BubbleCompiler::from_node(
                node, context,
            )?))),
            "some" => Ok(Pattern::Some(Box::new(SomeCompiler::from_node(
                node, context,
            )?))),
            "every" => Ok(Pattern::Every(Box::new(EveryCompiler::from_node(
                node, context,
            )?))),
            "nodeLike" => Call::from_node(
                &node.node,
                context.compilation,
                context.vars,
                context.vars_array,
                context.scope_index,
                context.global_vars,
                is_rhs,
                context.logs,
            ),
            "list" => Ok(Pattern::List(Box::new(ListCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "listIndex" => Ok(Pattern::ListIndex(Box::new(ListIndexCompiler::from_node(
                node, context,
            )?))),
            "map" => Ok(Pattern::Map(Box::new(MapCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "mapAccessor" => Ok(Pattern::Accessor(Box::new(AccessorCompiler::from_node(
                node, context,
            )?))),
            "dot" => Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                parts: vec![DynamicSnippetPart::String("".to_string())],
            }))),
            "dotdotdot" => Ok(Pattern::Dots),
            "underscore" => Ok(Pattern::Underscore),
            "regexPattern" => RegexPattern::from_node(
                &node.node,
                context.compilation,
                context.vars,
                context.global_vars,
                context.vars_array,
                context.scope_index,
                context.compilation.lang,
                is_rhs,
                context.logs,
            ),
            "variable" => Ok(Pattern::Variable(VariableCompiler::from_node(
                node, context,
            )?)),
            "codeSnippet" => CodeSnippet::from_node(
                &node.node,
                context.compilation.file,
                node.source,
                context.vars,
                context.global_vars,
                context.vars_array,
                context.scope_index,
                context.compilation.lang,
                is_rhs,
            ),
            "like" => Ok(Pattern::Like(Box::new(LikeCompiler::from_node(
                node, context,
            )?))),
            "undefined" => Ok(Pattern::Undefined),
            "top" => Ok(Pattern::Top),
            "bottom" => Ok(Pattern::Bottom),
            "intConstant" => Ok(Pattern::IntConstant(IntConstantCompiler::from_node(
                node, context,
            )?)),
            "sequential" => Ok(Pattern::Sequential(SequentialCompiler::from_node(
                node, context,
            )?)),
            "files" => Ok(Pattern::Sequential(SequentialCompiler::from_files_node(
                node, context,
            )?)),
            "doubleConstant" => Ok(Pattern::FloatConstant(FloatConstantCompiler::from_node(
                node, context,
            )?)),
            "booleanConstant" => Ok(Pattern::BooleanConstant(
                BooleanConstantCompiler::from_node(node, context)?,
            )),
            "stringConstant" => Ok(Pattern::StringConstant(StringConstantCompiler::from_node(
                node, context,
            )?)),
            _ => bail!("unknown pattern kind: {}", kind),
        }
    }
}
