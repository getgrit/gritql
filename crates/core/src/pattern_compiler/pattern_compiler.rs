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
use crate::pattern::{
    ast_node::ASTNode,
    dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart},
    list::List,
    patterns::Pattern,
    regex::{RegexLike, RegexPattern},
    string_constant::AstLeafNode,
    variable::{is_reserved_metavariable, register_variable, Variable},
};
use anyhow::{anyhow, bail, Result};
use grit_util::AstNode;
use grit_util::{traverse, Order};
use marzano_language::language::{Field, GritMetaValue, Language, SnippetNode};
use marzano_util::{
    cursor_wrapper::CursorWrapper,
    node_with_source::NodeWithSource,
    position::{char_index_to_byte_index, Position, Range},
};
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
        node: SnippetNode,
        context_range: Range,
        context: &mut NodeCompilationContext,
        is_rhs: bool,
    ) -> Result<Pattern> {
        let snippet_start = node.node.start_byte();
        let node = NodeWithSource::new(node.node, &node.context);
        let ranges = metavariable_ranges(&node, context.compilation.lang);
        let range_map = metavariable_range_mapping(ranges, snippet_start);

        #[allow(clippy::too_many_arguments)]
        fn node_to_astnode(
            node: NodeWithSource,
            context_range: Range,
            range_map: &HashMap<Range, Range>,
            context: &mut NodeCompilationContext,
            is_rhs: bool,
        ) -> Result<Pattern> {
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
                let content = node.text();
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
                        // Ordinarily, we want to match on all possible fields, including the absence of nodes within a field.
                        // e.g., `func_call()` should not match `func_call(arg)`, however, sometimes we want to allow people to
                        // save some boilerplate and by default match a node even if a field is present in the code but not
                        // in the snippet. e.g.,
                        // `func name(args) {}` will match `async name(args) {}` because async is an optional_empty_field for tsx.
                        // To explicitly only match synchronous functions, you could write:
                        // `$async func name(args)` where $async <: .
                        !((node.node.child_by_field_id(field.id()).is_none() && context.compilation.lang.optional_empty_field_compilation(sort, field.id()))
                        // we wanted to be able to match on the presence of parentheses in an arrow function manually
                        // using ast_node syntax, but we wanted snippets to match regardless of weather or not the
                        // parenthesis are present, so we made the parenthesis a  named node within a field, but
                        // added then to this list so that they wont be compiled. fields in this list are
                        // destinguished by fields in the above list in that they will NEVER prevent a match
                        // while fields in the above list wont prevent a match if they are absent in the snippet,
                        // but they will prevent a match if present in the snippet, and not present in the target
                        // file.
                        // in react to hooks we manually match the parenthesis like so:
                        // arrow_function(parameters=$props, $body, $parenthesis) where {
                        //     $props <: contains or { `props`, `inputProps` },
                        //     $body <: not contains `props`,
                        //     if ($parenthesis <: .) {
                        //         $props => `()`
                        //     } else {
                        //         $props => .
                        //     }
                        // }
                        || context.compilation.lang.skip_snippet_compilation_of_field(sort, field.id()))
                    })
                    .map(|field| {
                        let field_id = field.id();
                        let mut nodes_list = node
                            .named_children_by_field_id(field_id)
                            .map(|n| node_to_astnode(n, context_range, range_map, context, is_rhs))
                            .collect::<Result<Vec<Pattern>>>()?;
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
                    .collect::<Result<Vec<(u16, bool, Pattern)>>>()?;
            Ok(Pattern::ASTNode(Box::new(ASTNode { sort, args })))
        }
        node_to_astnode(node, context_range, &range_map, context, is_rhs)
    }
}

impl NodeCompiler for PatternCompiler {
    type TargetPattern = Pattern;

    fn from_node_with_rhs(
        node: NodeWithSource,
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
            "variable" => Ok(Pattern::Variable(VariableCompiler::from_node_with_rhs(
                node, context, is_rhs,
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
            _ => bail!("unknown pattern kind: {}", kind),
        }
    }
}

// Transform a regex match range into a range in the original text
#[cfg(not(target_arch = "wasm32"))]
fn derive_range(text: &str, m: RegexMatch) -> Range {
    make_regex_match_range(text, m)
}

#[cfg(target_arch = "wasm32")]
fn derive_range(text: &str, m: RegexMatch) -> Range {
    let byte_range = make_regex_match_range(text, m);
    byte_range.byte_range_to_char_range(text)
}

fn implicit_metavariable_regex(
    node: &NodeWithSource,
    context_range: Range,
    range_map: &HashMap<Range, Range>,
    context: &mut NodeCompilationContext,
) -> Result<Option<RegexPattern>> {
    let range: Range = node.range().into();
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
            char_index_to_byte_index(range.end_byte, node.source)
        } else {
            range.end_byte
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

fn is_metavariable(node: &NodeWithSource, lang: &impl Language) -> bool {
    node.node.is_named()
        && (node.node.kind_id() == lang.metavariable_sort()
            || (lang
                .alternate_metavariable_kinds()
                .contains(&node.node.kind().as_ref())
                && lang.exact_replaced_variable_regex().is_match(node.text())))
}

fn make_regex_match_range(text: &str, m: RegexMatch) -> Range {
    let start = Position::from_byte_index(text, None, m.start() as u32);
    let end = Position::from_byte_index(text, None, m.end() as u32);
    Range::new(start, end, m.start() as u32, m.end() as u32)
}

fn metavariable_descendent(
    node: &NodeWithSource,
    context_range: Range,
    range_map: &HashMap<Range, Range>,
    context: &mut NodeCompilationContext,
    is_rhs: bool,
) -> Result<Option<Pattern>> {
    let mut cursor = node.node.walk();
    loop {
        let node = NodeWithSource::new(cursor.node(), node.source);
        if is_metavariable(&node, context.compilation.lang) {
            let name = node.text();
            if is_reserved_metavariable(name.trim(), Some(context.compilation.lang)) && !is_rhs {
                bail!("{} is a reserved metavariable name. For more information, check out the docs at https://docs.grit.io/language/patterns#metavariables.", name.trim_start_matches(context.compilation.lang.metavariable_prefix_substitute()));
            }
            let range: Range = node.range().into();
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

fn metavariable_ranges(node: &NodeWithSource, lang: &impl Language) -> Vec<Range> {
    let cursor = node.node.walk();
    traverse(CursorWrapper::new(cursor, node.source), Order::Pre)
        .flat_map(|n| {
            let child = NodeWithSource::new(n.node.clone(), node.source);
            if is_metavariable(&child, lang) {
                let range: Range = n.node.range().into();
                vec![range]
            } else {
                node_sub_variables(child, lang)
            }
        })
        .collect()
}

// assumes that metavariable substitute is 1 byte larger than the original. eg.
// len(µ) = 2 bytes, len($) = 1 byte
fn metavariable_range_mapping(
    mut ranges: Vec<Range>,
    snippet_offset: u32,
) -> HashMap<Range, Range> {
    // assumes metavariable ranges do not enclose one another
    ranges.sort_by_key(|r| r.start_byte);
    let mut byte_offset = snippet_offset;
    let mut column_offset = 0;
    let mut last_row = 0;
    let mut map = HashMap::new();
    for range in ranges.into_iter() {
        let new_row = range.start.line;
        if new_row != last_row {
            column_offset = if new_row == 1 { snippet_offset } else { 0 };
            last_row = new_row;
        }
        let start = Position::new(new_row, range.start.column - column_offset);
        let start_byte = range.start_byte - byte_offset;
        if !cfg!(target_arch = "wasm32") {
            byte_offset += 1;
            column_offset += 1;
        }
        let end = Position::new(new_row, range.end.column - column_offset);
        let end_byte = range.end_byte - byte_offset;
        let new_range = Range::new(start, end, start_byte, end_byte);
        map.insert(range, new_range);
    }
    map
}

fn node_sub_variables(node: NodeWithSource, lang: &impl Language) -> Vec<Range> {
    let mut ranges = vec![];
    if node.node.named_child_count() > 0 {
        return ranges;
    }
    let variable_regex = lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(node.source) {
        let var_range = derive_range(node.source, m);
        let start_byte = node.node.start_byte();
        let end_byte = node.node.end_byte();
        if var_range.start_byte >= start_byte && var_range.end_byte <= end_byte {
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

impl From<SnippetValues> for Pattern {
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
    range: Range,
    context_range: Range,
    range_map: &HashMap<Range, Range>,
    context: &mut NodeCompilationContext,
) -> Result<SnippetValues> {
    let name = context
        .compilation
        .lang
        .snippet_metavariable_to_grit_metavariable(name)
        .ok_or_else(|| anyhow!("metavariable |{}| not found in snippet", name))?;
    match name {
        GritMetaValue::Dots => Ok(SnippetValues::Dots),
        GritMetaValue::Underscore => Ok(SnippetValues::Underscore),
        GritMetaValue::Variable(name) => {
            let range = range_map.get(&range).ok_or_else(|| {
                anyhow!(
                    "{} not found in map {:?}",
                    range.abbreviated_debug(),
                    range_map
                        .keys()
                        .map(|k| k.abbreviated_debug())
                        .collect::<Vec<_>>()
                )
            })?;
            let column_offset = if range.start.line == 1 {
                context_range.start.column
            } else {
                0
            };
            let start = Position {
                line: range.start.line + context_range.start.line - 1,
                column: range.start.column + column_offset - 1,
            };
            let end = Position {
                line: range.end.line + context_range.start.line - 1,
                column: range.end.column + column_offset - 1,
            };
            let range = Range::new(
                start,
                end,
                range.start_byte + context_range.start_byte,
                range.end_byte + context_range.start_byte,
            );
            let var = register_variable(&name, range, context)?;
            Ok(SnippetValues::Variable(var))
        }
    }
}
