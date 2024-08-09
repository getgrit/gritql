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
    call_compiler::CallCompiler,
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
    regex_compiler::RegexCompiler,
    rewrite_compiler::RewriteCompiler,
    sequential_compiler::SequentialCompiler,
    snippet_compiler::CodeSnippetCompiler,
    some_compiler::SomeCompiler,
    subtract_compiler::SubtractCompiler,
    variable_compiler::VariableCompiler,
    where_compiler::WhereCompiler,
    within_compiler::WithinCompiler,
};
use crate::problem::MarzanoQueryContext;
use crate::{
    ast_node::{ASTNode, AstLeafNode},
    variables::register_variable,
};
use anyhow::{anyhow, bail, Result};
use grit_pattern_matcher::{
    context::QueryContext,
    pattern::{
        is_reserved_metavariable, DynamicPattern, DynamicSnippet, DynamicSnippetPart, List,
        Pattern, RegexLike, RegexPattern, Variable,
    },
};
use grit_util::{traverse, AstCursor, AstNode, ByteRange, GritMetaValue, Language, Order};
use marzano_language::language::{Field, MarzanoLanguage, NodeTypes};
use marzano_util::node_with_source::NodeWithSource;
use regex::Match as RegexMatch;
use std::collections::HashMap;

pub(crate) struct PatternCompiler;

impl PatternCompiler {
    // for now nested fields are always AssocNode
    // todo leaf nodes should be string literals for now.

    // BUG: TOP FUNCTION SHOULD CHECK THAT WE DO NOT RETURN TrimmedStringConstant
    // it should wrap TrimmedStringConstant in the appropriate ASTNode
    // cannot fix yet as other code relies on this bug

    pub(crate) fn from_snippet_node(
        node: NodeWithSource,
        context_range: ByteRange,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern<MarzanoQueryContext>> {
        let snippet_start = node.node.start_byte() as usize;
        let ranges = metavariable_ranges(&node, context.compilation.lang);
        let range_map = metavariable_range_mapping(ranges, snippet_start);

        fn node_to_astnode(
            node: NodeWithSource,
            context_range: ByteRange,
            range_map: &HashMap<ByteRange, ByteRange>,
            context: &mut NodeCompilationContext,
            is_rhs: bool,
        ) -> Result<Pattern<MarzanoQueryContext>> {
            let sort = node.node.kind_id();
            // probably safe to assume node is named, but just in case
            // maybe doesn't even matter, but is what I expect,
            // make this ann assertion?
            let node_types = context.compilation.lang.node_types();
            let metavariable =
                metavariable_descendent(&node, context_range, range_map, context, is_rhs)?;
            if let Some(metavariable) = metavariable {
                return Ok(metavariable);
            }
            if node_types[sort as usize].is_empty() {
                let content = node.text()?;
                if (node.node.named_child_count() == 0)
                    && context
                        .compilation
                        .lang
                        .replaced_metavariable_regex()
                        .is_match(&content)
                {
                    let regex =
                        implicit_metavariable_regex(&node, context_range, range_map, context)?;
                    if let Some(regex) = regex {
                        return Ok(Pattern::Regex(Box::new(regex)));
                    }
                }
                return Ok(Pattern::AstLeafNode(AstLeafNode::new(
                    sort,
                    &content,
                    context.compilation.lang,
                )?));
            }
            let fields: &Vec<Field> = &node_types[sort as usize];
            let args =
                fields
                    .iter()
                    .filter(|field| {
                        let child_with_source = node
                            .node
                            .child_by_field_id(field.id())
                            .map(|n| NodeWithSource::new(n, node.source));
                        // Then check if it's an empty, optional field
                        if context.compilation.lang.is_disregarded_snippet_field(
                            sort,
                            field.id(),
                            &child_with_source,
                        ) {
                            return false;
                        }
                        // Otherwise compile it
                        true
                    })
                    .map(|field| {
                        let field_id = field.id();
                        let mut nodes_list = node
                            .named_children_by_field_id(field_id)
                            .map(|n| node_to_astnode(n, context_range, range_map, context, is_rhs))
                            .collect::<Result<Vec<Pattern<MarzanoQueryContext>>>>()?;
                        if !field.multiple() {
                            return Ok((
                                field_id,
                                false,
                                nodes_list.pop().unwrap_or(Pattern::Dynamic(
                                    DynamicPattern::Snippet(DynamicSnippet {
                                        parts: vec![DynamicSnippetPart::String("".to_string())],
                                    }),
                                )),
                            ));
                        }
                        if nodes_list.len() == 1
                            && matches!(
                                nodes_list.first(),
                                Some(Pattern::Variable(_)) | Some(Pattern::Underscore)
                            )
                        {
                            return Ok((field_id, true, nodes_list.pop().unwrap()));
                        }
                        Ok((
                            field_id,
                            true,
                            Pattern::List(Box::new(List::new(nodes_list))),
                        ))
                    })
                    .collect::<Result<Vec<(u16, bool, Pattern<MarzanoQueryContext>)>>>()?;
            Ok(Pattern::AstNode(Box::new(ASTNode { sort, args })))
        }
        node_to_astnode(node, context_range, &range_map, context, is_rhs)
    }
}

impl NodeCompiler for PatternCompiler {
    type TargetPattern = Pattern<MarzanoQueryContext>;

    fn from_node_with_rhs(
        node: &NodeWithSource,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Self::TargetPattern> {
        let kind = node.node.kind();
        match kind.as_ref() {
            "mulOperation" => Ok(Pattern::Multiply(Box::new(
                MultiplyCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "divOperation" => Ok(Pattern::Divide(Box::new(
                DivideCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "modOperation" => Ok(Pattern::Modulo(Box::new(
                ModuloCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "addOperation" => Ok(Pattern::Add(Box::new(AddCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "subOperation" => Ok(Pattern::Subtract(Box::new(
                SubtractCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "patternAs" => Ok(Pattern::Where(Box::new(AsCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "patternLimit" => LimitCompiler::from_node_with_rhs(node, context, is_rhs),
            "assignmentAsPattern" => Ok(Pattern::Assignment(Box::new(
                AssignmentCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "patternAccumulate" => Ok(Pattern::Accumulate(Box::new(
                AccumulateCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "patternWhere" => Ok(Pattern::Where(Box::new(WhereCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "patternNot" => Ok(Pattern::Not(Box::new(NotCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "patternOr" => OrCompiler::from_node_with_rhs(node, context, is_rhs),
            "patternAnd" => AndCompiler::from_node_with_rhs(node, context, is_rhs),
            "patternAny" => AnyCompiler::from_node_with_rhs(node, context, is_rhs),
            "patternMaybe" => Ok(Pattern::Maybe(Box::new(MaybeCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "patternAfter" => Ok(Pattern::After(Box::new(AfterCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "patternBefore" => Ok(Pattern::Before(Box::new(
                BeforeCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "patternContains" => Ok(Pattern::Contains(Box::new(
                ContainsCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "patternIncludes" => Ok(Pattern::Includes(Box::new(
                IncludesCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "rewrite" => Ok(Pattern::Rewrite(Box::new(
                RewriteCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "log" => Ok(Pattern::Log(Box::new(LogCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "range" => Ok(Pattern::Range(RangeCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?)),
            "patternIfElse" => Ok(Pattern::If(Box::new(IfCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "within" => Ok(Pattern::Within(Box::new(
                WithinCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "bubble" => Ok(Pattern::Bubble(Box::new(
                BubbleCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "some" => Ok(Pattern::Some(Box::new(SomeCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "every" => Ok(Pattern::Every(Box::new(EveryCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "nodeLike" => CallCompiler::from_node_with_rhs(node, context, is_rhs),
            "list" => Ok(Pattern::List(Box::new(ListCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "listIndex" => Ok(Pattern::ListIndex(Box::new(
                ListIndexCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "map" => Ok(Pattern::Map(Box::new(MapCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "mapAccessor" => Ok(Pattern::Accessor(Box::new(
                AccessorCompiler::from_node_with_rhs(node, context, is_rhs)?,
            ))),
            "dot" => Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                parts: vec![DynamicSnippetPart::String("".to_string())],
            }))),
            "dotdotdot" => Ok(Pattern::Dots),
            "underscore" => Ok(Pattern::Underscore),
            "regexPattern" => Ok(Pattern::Regex(Box::new(RegexCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "variable" => Ok(Pattern::Variable(VariableCompiler::from_node(
                node, context,
            )?)),
            "codeSnippet" => CodeSnippetCompiler::from_node_with_rhs(node, context, is_rhs),
            "like" => Ok(Pattern::Like(Box::new(LikeCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?))),
            "undefined" => Ok(Pattern::Undefined),
            "top" => Ok(Pattern::Top),
            "bottom" => Ok(Pattern::Bottom),
            "intConstant" => Ok(Pattern::IntConstant(
                IntConstantCompiler::from_node_with_rhs(node, context, is_rhs)?,
            )),
            "sequential" => Ok(Pattern::Sequential(SequentialCompiler::from_node_with_rhs(
                node, context, is_rhs,
            )?)),
            "files" => Ok(Pattern::Sequential(SequentialCompiler::from_files_node(
                node, context,
            )?)),
            "doubleConstant" => Ok(Pattern::FloatConstant(
                FloatConstantCompiler::from_node_with_rhs(node, context, is_rhs)?,
            )),
            "booleanConstant" => Ok(Pattern::BooleanConstant(
                BooleanConstantCompiler::from_node_with_rhs(node, context, is_rhs)?,
            )),
            "stringConstant" => Ok(Pattern::StringConstant(
                StringConstantCompiler::from_node_with_rhs(node, context, is_rhs)?,
            )),
            _ => return Err(GritPatternError::new(format!("unknown pattern kind: {}", kind))),
        }
    }
}

// Transform a regex match range into a range in the original text
#[cfg(not(target_arch = "wasm32"))]
fn derive_range(_text: &str, m: RegexMatch) -> ByteRange {
    ByteRange::new(m.start(), m.end())
}

#[cfg(target_arch = "wasm32")]
fn derive_range(text: &str, m: RegexMatch) -> ByteRange {
    let byte_range = ByteRange::new(m.start(), m.end());
    byte_range.to_char_range(text)
}

fn implicit_metavariable_regex<Q: QueryContext>(
    node: &NodeWithSource,
    context_range: ByteRange,
    range_map: &HashMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Result<Option<RegexPattern<Q>>> {
    let range = node.range();
    let offset = range.start_byte;
    let mut last = if cfg!(target_arch = "wasm32") {
        char_index_to_byte_index(offset, node.source)
    } else {
        offset
    };
    let mut regex_string = String::new();
    let mut variables: Vec<Variable> = vec![];
    let capture_string = "(.*)";
    let uncapture_string = ".*";
    let variable_regex = context.compilation.lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(node.source) {
        if last > m.start() as u32 {
            continue;
        }
        regex_string.push_str(&regex::escape(&node.source[last as usize..m.start()]));
        let range = derive_range(node.source, m);
        last = if cfg!(target_arch = "wasm32") {
            char_index_to_byte_index(range.end as u32, node.source)
        } else {
            range.end as u32
        };
        let name = m.as_str();
        let variable = text_to_var(name, range, context_range, range_map, context)?;
        match variable {
            SnippetValues::Dots => return Ok(None),
            SnippetValues::Underscore => regex_string.push_str(uncapture_string),
            SnippetValues::Variable(var) => {
                regex_string.push_str(capture_string);
                variables.push(var);
            }
        }
    }
    if last < range.end_byte {
        regex_string.push_str(&regex::escape(
            &node.source[last as usize..range.end_byte as usize],
        ));
    }
    let regex = regex_string.to_string();
    let regex = RegexLike::Regex(regex);
    Ok(Some(RegexPattern::new(regex, variables)))
}

fn metavariable_descendent<Q: QueryContext>(
    node: &NodeWithSource,
    context_range: ByteRange,
    range_map: &HashMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Option<Pattern<Q>>> {
    let mut cursor = node.walk();
    loop {
        let node = cursor.node();
        if context.compilation.lang.is_metavariable(&node) {
            let name = node.text()?;
            if is_reserved_metavariable(name.trim(), Some(context.compilation.lang)) && !is_rhs {
                return Err(GritPatternError::new("{} is a reserved metavariable name. For more information, check out the docs at https://docs.grit.io/language/patterns#metavariables.", name.trim_start_matches(context.compilation.lang.metavariable_prefix_substitute())));
            }
            let range = node.byte_range();
            return text_to_var(&name, range, context_range, range_map, context)
                .map(|s| Some(s.into()));
        }
        if node.node.child_count() == 1 {
            cursor.goto_first_child();
        } else {
            return Ok(None);
        }
    }
}

fn metavariable_ranges<'a, Lang: Language<Node<'a> = NodeWithSource<'a>>>(
    node: &NodeWithSource<'a>,
    lang: &Lang,
) -> Vec<ByteRange> {
    let cursor = node.walk();
    traverse(cursor, Order::Pre)
        .flat_map(|child| {
            if lang.is_metavariable(&child) {
                vec![child.byte_range()]
            } else {
                node_sub_variables(child, lang)
            }
        })
        .collect()
}

// assumes that metavariable substitute is 1 byte larger than the original. eg.
// len(µ) = 2 bytes, len($) = 1 byte
fn metavariable_range_mapping(
    mut ranges: Vec<ByteRange>,
    snippet_offset: usize,
) -> HashMap<ByteRange, ByteRange> {
    // assumes metavariable ranges do not enclose one another
    ranges.sort_by_key(|r| r.start);
    let mut byte_offset = snippet_offset;
    let mut map = HashMap::new();
    for range in ranges.into_iter() {
        let start_byte = range.start - byte_offset;
        if !cfg!(target_arch = "wasm32") {
            byte_offset += 1;
        }
        let end_byte = range.end - byte_offset;
        let new_range = ByteRange::new(start_byte, end_byte);
        map.insert(range, new_range);
    }
    map
}

fn node_sub_variables(node: NodeWithSource, lang: &impl Language) -> Vec<ByteRange> {
    let mut ranges = vec![];
    if node.node.named_child_count() > 0 {
        return ranges;
    }
    let variable_regex = lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(node.source) {
        let var_range = derive_range(node.source, m);
        let start_byte = node.node.start_byte() as usize;
        let end_byte = node.node.end_byte() as usize;
        if var_range.start >= start_byte && var_range.end <= end_byte {
            ranges.push(var_range);
        }
    }
    ranges
}

enum SnippetValues {
    Dots,
    Underscore,
    Variable(Variable),
}

impl<Q: QueryContext> From<SnippetValues> for Pattern<Q> {
    fn from(value: SnippetValues) -> Self {
        match value {
            SnippetValues::Dots => Pattern::Dots,
            SnippetValues::Underscore => Pattern::Underscore,
            SnippetValues::Variable(v) => Pattern::Variable(v),
        }
    }
}

fn text_to_var(
    name: &str,
    range: ByteRange,
    context_range: ByteRange,
    range_map: &HashMap<ByteRange, ByteRange>,
    context: &mut NodeCompilationContext,
) -> Result<SnippetValues> {
    let name = context
        .compilation
        .lang
        .snippet_metavariable_to_grit_metavariable(name)
        .ok_or_else(|| GritPatternError::new(format!("metavariable |{}| not found in snippet", name)))?;
    match name {
        GritMetaValue::Dots => Ok(SnippetValues::Dots),
        GritMetaValue::Underscore => Ok(SnippetValues::Underscore),
        GritMetaValue::Variable(name) => {
            let range = *range_map.get(&range).ok_or_else(|| {
                GritPatternError::new("{} not found in map {:?}",
                    range.abbreviated_debug(),
                    range_map
                        .keys()
                        .map(|k| k.abbreviated_debug())
                        .collect::<Vec<_>>())
            })?;
            let var = register_variable(&name, range + context_range.start, context)?;
            Ok(SnippetValues::Variable(var))
        }
    }
}

fn char_index_to_byte_index(index: u32, text: &str) -> u32 {
    text.chars()
        .take(index as usize)
        .map(|c| c.len_utf8() as u32)
        .sum()
}
