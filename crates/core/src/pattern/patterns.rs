use super::accessor::Accessor;
use super::add::Add;
use super::before::Before;
use super::boolean_constant::BooleanConstant;
use super::compiler::{CompilationContext, ABSOLUTE_PATH_INDEX, FILENAME_INDEX, PROGRAM_INDEX};
use super::divide::Divide;
use super::dynamic_snippet::{DynamicPattern, DynamicSnippet, DynamicSnippetPart};
use super::every::Every;
use super::file_pattern::FilePattern;
use super::files::Files;
use super::float_constant::FloatConstant;
use super::functions::{CallForeignFunction, CallFunction};
use super::includes::Includes;
use super::int_constant::IntConstant;
use super::like::Like;
use super::limit::Limit;
use super::list_index::ListIndex;
use super::log::Log;
use super::map::GritMap;
use super::maybe::Maybe;
use super::modulo::Modulo;
use super::multiply::Multiply;
use super::range::Range as PRange;
use super::regex::{RegexLike, RegexPattern};
use super::resolved_pattern::ResolvedPattern;
use super::sequential::Sequential;
use super::subtract::Subtract;
use super::undefined::Undefined;
use super::variable::{is_reserved_metavariable, VariableSourceLocations, GLOBAL_VARS_SCOPE_INDEX};
use super::{
    accumulate::Accumulate, after::After, and::And, any::Any, assignment::Assignment,
    ast_node::ASTNode, bubble::Bubble, built_in_functions::CallBuiltIn, call::Call,
    code_snippet::CodeSnippet, contains::Contains, list::List, not::Not, or::Or, r#if::If,
    r#where::Where, rewrite::Rewrite, some::Some, string_constant::StringConstant,
    variable::Variable, within::Within, Node, State,
};
use marzano_util::node_with_source::NodeWithSource;
use crate::context::Context;
use crate::pattern::register_variable;
use crate::pattern::string_constant::AstLeafNode;
use anyhow::{anyhow, bail, Result};
use core::fmt::Debug;
use grit_util::{traverse, Order, AstNode};
use marzano_language::language::{Field, GritMetaValue};
use marzano_language::{language::Language, language::SnippetNode};
use marzano_util::analysis_logs::AnalysisLogs;
use marzano_util::cursor_wrapper::CursorWrapper;
use marzano_util::position::{char_index_to_byte_index, Position, Range};
use regex::Match;
use std::collections::{BTreeMap, HashMap};
use std::str;
use std::vec;

pub(crate) trait Matcher: Debug {
    // it is important that any implementors of Pattern
    // do not compute-expensive things in execute
    // it should be stored somewhere in the struct of the implementor
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool>;

    // for the future:
    // we could speed up computation by filtering on the sort of pattern
    // here, &SortFormula is a propositional-logic formula over sorts
    // fn sort(&self) -> SortFormula;
}

pub trait Name {
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub enum Pattern {
    ASTNode(Box<ASTNode>),
    List(Box<List>),
    ListIndex(Box<ListIndex>),
    Map(Box<GritMap>),
    Accessor(Box<Accessor>),
    Call(Box<Call>),
    Regex(Box<RegexPattern>),
    File(Box<FilePattern>),
    Files(Box<Files>),
    Bubble(Box<Bubble>),
    Limit(Box<Limit>),
    CallBuiltIn(Box<CallBuiltIn>),
    CallFunction(Box<CallFunction>),
    CallForeignFunction(Box<CallForeignFunction>),
    Assignment(Box<Assignment>),
    Accumulate(Box<Accumulate>),
    And(Box<And>),
    Or(Box<Or>),
    Maybe(Box<Maybe>),
    Any(Box<Any>),
    Not(Box<Not>),
    If(Box<If>),
    Undefined,
    Top,
    Bottom,
    // differentiated from top for debugging purposes.
    Underscore,
    StringConstant(StringConstant),
    AstLeafNode(AstLeafNode),
    IntConstant(IntConstant),
    FloatConstant(FloatConstant),
    BooleanConstant(BooleanConstant),
    Dynamic(DynamicPattern),
    CodeSnippet(CodeSnippet),
    Variable(Variable),
    Rewrite(Box<Rewrite>),
    Log(Box<Log>),
    Range(PRange),
    Contains(Box<Contains>),
    Includes(Box<Includes>),
    Within(Box<Within>),
    After(Box<After>),
    Before(Box<Before>),
    Where(Box<Where>),
    Some(Box<Some>),
    Every(Box<Every>),
    Add(Box<Add>),
    Subtract(Box<Subtract>),
    Multiply(Box<Multiply>),
    Divide(Box<Divide>),
    Modulo(Box<Modulo>),
    Dots,
    Sequential(Sequential),
    Like(Box<Like>),
}

fn is_metavariable(node: &Node, lang: &impl Language, text: &[u8]) -> bool {
    node.is_named()
        && (node.kind_id() == lang.metavariable_sort()
            || (lang
                .alternate_metavariable_kinds()
                .contains(&node.kind().as_ref())
                && lang
                    .exact_replaced_variable_regex()
                    .is_match(node.utf8_text(text).as_ref().unwrap())))
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

#[allow(clippy::too_many_arguments)]
fn text_to_var(
    name: &str,
    range: Range,
    context_range: Range,
    file: &str,
    lang: &impl Language,
    range_map: &HashMap<Range, Range>,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut [Vec<VariableSourceLocations>],
    scope_index: usize,
) -> Result<SnippetValues> {
    let name = lang
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
            let var = register_variable(
                &name,
                file,
                range,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )?;
            Ok(SnippetValues::Variable(var))
        }
    }
}

#[allow(clippy::too_many_arguments)]
fn metavariable_descendent(
    node: &Node,
    context_range: Range,
    file: &str,
    text: &[u8],
    lang: &impl Language,
    range_map: &HashMap<Range, Range>,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut [Vec<VariableSourceLocations>],
    scope_index: usize,
    is_rhs: bool,
) -> Result<Option<Pattern>> {
    let mut cursor = node.walk();
    loop {
        let node = cursor.node();
        if is_metavariable(&node, lang, text) {
            let name = node.utf8_text(text)?;
            if is_reserved_metavariable(name.trim(), Some(lang)) && !is_rhs {
                bail!("{} is a reserved metavariable name. For more information, check out the docs at https://docs.grit.io/language/patterns#metavariables.", name.trim_start_matches(lang.metavariable_prefix_substitute()));
            }
            let range: Range = node.range().into();
            return text_to_var(
                &name,
                range,
                context_range,
                file,
                lang,
                range_map,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )
            .map(|s| Some(s.into()));
        }
        if node.child_count() == 1 {
            cursor.goto_first_child();
        } else {
            return Ok(None);
        }
    }
}

// Transform a regex match range into a range in the original text
#[cfg(not(target_arch = "wasm32"))]
fn derive_range(text: &str, m: Match) -> Range {
    make_regex_match_range(text, m)
}

#[cfg(target_arch = "wasm32")]
fn derive_range(text: &str, m: Match) -> Range {
    let byte_range = make_regex_match_range(text, m);
    byte_range.byte_range_to_char_range(text)
}

fn make_regex_match_range(text: &str, m: Match) -> Range {
    let start = Position::from_byte_index(text, None, m.start() as u32);
    let end = Position::from_byte_index(text, None, m.end() as u32);
    Range::new(start, end, m.start() as u32, m.end() as u32)
}

fn node_sub_variables(node: Node, lang: &impl Language, text: &str) -> Vec<Range> {
    let mut ranges = vec![];
    if node.named_child_count() > 0 {
        return ranges;
    }
    let variable_regex = lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(text) {
        let var_range = derive_range(text, m);
        let start_byte = node.start_byte();
        let end_byte = node.end_byte();
        if var_range.start_byte >= start_byte && var_range.end_byte <= end_byte {
            ranges.push(var_range);
        }
    }
    ranges
}

fn metavariable_ranges(node: &Node, lang: &impl Language, text: &str) -> Vec<Range> {
    let cursor = node.walk();
    traverse(CursorWrapper::new(cursor, text), Order::Pre)
        .flat_map(|n| {
            if is_metavariable(&n.node, lang, text.as_bytes()) {
                let range: Range = n.node.range().into();
                vec![range]
            } else {
                node_sub_variables(n.node, lang, text)
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

#[allow(clippy::too_many_arguments)]
fn implicit_metavariable_regex(
    node: &Node,
    context_range: Range,
    file: &str,
    text: &[u8],
    lang: &impl Language,
    range_map: &HashMap<Range, Range>,
    vars: &mut BTreeMap<String, usize>,
    global_vars: &mut BTreeMap<String, usize>,
    vars_array: &mut [Vec<VariableSourceLocations>],
    scope_index: usize,
) -> Result<Option<RegexPattern>> {
    let range: Range = node.range().into();
    let text = String::from_utf8(text.to_vec())?;
    let offset = range.start_byte;
    let mut last = if cfg!(target_arch = "wasm32") {
        char_index_to_byte_index(offset, &text)
    } else {
        offset
    };
    let mut regex_string = String::new();
    let mut variables: Vec<Variable> = vec![];
    let capture_string = "(.*)";
    let uncapture_string = ".*";
    let variable_regex = lang.replaced_metavariable_regex();
    for m in variable_regex.find_iter(&text) {
        if last > m.start() as u32 {
            continue;
        }
        regex_string.push_str(&regex::escape(&text[last as usize..m.start()]));
        let range = derive_range(&text, m);
        last = if cfg!(target_arch = "wasm32") {
            char_index_to_byte_index(range.end_byte, &text)
        } else {
            range.end_byte
        };
        let name = m.as_str();
        let variable = text_to_var(
            name,
            range,
            context_range,
            file,
            lang,
            range_map,
            vars,
            global_vars,
            vars_array,
            scope_index,
        )?;
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
            &text[last as usize..range.end_byte as usize],
        ));
    }
    let regex = regex_string.to_string();
    let regex = RegexLike::Regex(regex);
    Ok(Some(RegexPattern::new(regex, variables)))
}

impl Pattern {
    // for now nested fields are always AssocNode
    // todo leaf nodes should be string literals for now.

    // BUG: TOP FUNCTION SHOULD CHECK THAT WE DO NOT RETURN TrimmedStringConstant
    // it should wrap TrimmedStringConstant in the appropriate ASTNode
    // cannot fix yet as other code relies on this bug
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_snippet_node<L>(
        node: SnippetNode,
        context_range: Range,
        file: &str,
        lang: &L,
        vars: &mut BTreeMap<String, usize>,
        global_vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        is_rhs: bool,
    ) -> Result<Self>
    where
        L: Language,
    {
        let context = node.context;
        let snippet_start = node.node.start_byte();
        let node = node.node;
        let ranges = metavariable_ranges(&node, lang, &context);
        let range_map = metavariable_range_mapping(ranges, snippet_start);
        #[allow(clippy::too_many_arguments)]
        fn node_to_astnode<L>(
            node: Node,
            context_range: Range,
            file: &str,
            text: &[u8],
            lang: &L,
            range_map: &HashMap<Range, Range>,
            vars: &mut BTreeMap<String, usize>,
            global_vars: &mut BTreeMap<String, usize>,
            vars_array: &mut Vec<Vec<VariableSourceLocations>>,
            scope_index: usize,
            is_rhs: bool,
        ) -> Result<Pattern>
        where
            L: Language,
        {
            let sort = node.kind_id();
            // probably safe to assume node is named, but just in case
            // maybe doesn't even matter, but is what I expect,
            // make this ann assertion?
            let node_types = lang.node_types();
            let metavariable = metavariable_descendent(
                &node,
                context_range,
                file,
                text,
                lang,
                range_map,
                vars,
                global_vars,
                vars_array,
                scope_index,
                is_rhs,
            )?;
            if let Some(metavariable) = metavariable {
                return Ok(metavariable);
            }
            if node_types[sort as usize].is_empty() {
                let content = node.utf8_text(text)?;
                if (node.named_child_count() == 0)
                    && lang.replaced_metavariable_regex().is_match(&content)
                {
                    let regex = implicit_metavariable_regex(
                        &node,
                        context_range,
                        file,
                        text,
                        lang,
                        range_map,
                        vars,
                        global_vars,
                        vars_array,
                        scope_index,
                    )?;
                    if let Some(regex) = regex {
                        return Ok(Pattern::Regex(Box::new(regex)));
                    }
                }
                return Ok(Pattern::AstLeafNode(AstLeafNode::new(
                    sort, &content, lang,
                )?));
            }
            let fields: &Vec<Field> = &node_types[sort as usize];
            let mut args = fields
                .iter()
                .filter(|field| {
                    node.child_by_field_id(field.id()).is_some()
                        // sometimes we want to be able to manually match on fields, but
                        // not have snippets include those fields, for example
                        // we don't want to match on the parenthesis of parameters
                        // by default, but we want to be able to manually check
                        // for parenthesis. see react-to-hooks for an example
                        && !lang.skip_snippet_compilation_of_field(sort, field.id())
                })
                .map(|field| {
                    let field_id = field.id();
                    let mut cursor = node.walk();
                    let mut nodes_list = node
                        .children_by_field_id(field_id, &mut cursor)
                        .filter(|n| n.is_named())
                        .map(|n| {
                            node_to_astnode(
                                n,
                                context_range,
                                file,
                                text,
                                lang,
                                range_map,
                                vars,
                                global_vars,
                                vars_array,
                                scope_index,
                                is_rhs,
                            )
                        })
                        .collect::<Result<Vec<Pattern>>>()?;

                    // TODO check if Pattern is Dots, and error at compile time,
                    // dots only makes sense in a list.
                    if !field.multiple() {    
                        if nodes_list.len() == 1 {
                            return Ok((field_id, false, nodes_list.pop().unwrap()));
                        }
                        let field_node = node.child_by_field_id(field_id).unwrap();
                        let field_node_with_source = NodeWithSource::new(field_node.clone(), str::from_utf8(text).unwrap());
                        return Ok((field_id, false,  Pattern::AstLeafNode(AstLeafNode::new(
                            field_node.kind_id(), field_node_with_source.text(), lang,
                        )?)));
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
            let mut mandatory_empty_args = fields
                .iter()
                .filter(|field| {
                    node.child_by_field_id(field.id()).is_none()
                        && lang.mandatory_empty_field(sort, field.id())
                })
                .map(|field| {
                    if field.multiple() {
                        (
                            field.id(),
                            true,
                            Pattern::List(Box::new(List::new(Vec::new()))),
                        )
                    } else {
                        (
                            field.id(),
                            false,
                            Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                                parts: vec![DynamicSnippetPart::String("".to_string())],
                            })),
                        )
                    }
                })
                .collect::<Vec<(u16, bool, Pattern)>>();
            args.append(&mut mandatory_empty_args);
            Ok(Pattern::ASTNode(Box::new(ASTNode { sort, args })))
        }
        node_to_astnode(
            node,
            context_range,
            file,
            context.as_bytes(),
            lang,
            &range_map,
            vars,
            global_vars,
            vars_array,
            scope_index,
            is_rhs,
        )
    }

    // todo this should return a cow, but currently can't figure out lifetimes
    pub fn text<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<String> {
        Ok(ResolvedPattern::from_pattern(self, state, context, logs)?
            .text(&state.files)?
            .to_string())
    }

    pub(crate) fn float<'a>(
        &'a self,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        ResolvedPattern::from_pattern(self, state, context, logs)?.float(&state.files)
    }

    // use a struct
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn from_node(
        node: &Node,
        context: &CompilationContext,
        vars: &mut BTreeMap<String, usize>,
        vars_array: &mut Vec<Vec<VariableSourceLocations>>,
        scope_index: usize,
        global_vars: &mut BTreeMap<String, usize>,
        is_rhs: bool,
        logs: &mut AnalysisLogs,
    ) -> Result<Self> {
        let kind = node.kind();
        match kind.as_ref() {
            "mulOperation" => Ok(Pattern::Multiply(Box::new(Multiply::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "divOperation" => Ok(Pattern::Divide(Box::new(Divide::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "modOperation" => Ok(Pattern::Modulo(Box::new(Modulo::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "addOperation" => Ok(Pattern::Add(Box::new(Add::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "subOperation" => Ok(Pattern::Subtract(Box::new(Subtract::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternAs" => Ok(Pattern::Where(Box::new(Where::as_from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternLimit" => Limit::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "assignmentAsPattern" => Ok(Pattern::Assignment(Box::new(Assignment::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternAccumulate" => Ok(Pattern::Accumulate(Box::new(Accumulate::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternWhere" => Ok(Pattern::Where(Box::new(Where::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternNot" => Ok(Pattern::Not(Box::new(Not::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternOr" => Or::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "patternAnd" => And::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "patternAny" => Any::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "patternMaybe" => Ok(Pattern::Maybe(Box::new(Maybe::maybe_from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternAfter" => Ok(Pattern::After(Box::new(After::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternBefore" => Ok(Pattern::Before(Box::new(Before::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternContains" => Ok(Pattern::Contains(Box::new(Contains::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "patternIncludes" => Ok(Pattern::Includes(Box::new(Includes::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "rewrite" => Ok(Pattern::Rewrite(Box::new(Rewrite::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "log" => Ok(Pattern::Log(Box::new(Log::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "range" => Ok(Pattern::Range(PRange::from_node(node, context.src)?)),
            "patternIfElse" => Ok(Pattern::If(Box::new(If::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "within" => Ok(Pattern::Within(Box::new(Within::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "bubble" => Bubble::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            ),
            "some" => Ok(Pattern::Some(Box::new(Some::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "every" => Ok(Pattern::Every(Box::new(Every::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "nodeLike" => Call::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                is_rhs,
                logs,
            ),
            "list" => Ok(Pattern::List(Box::new(List::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                is_rhs,
                logs,
            )?))),
            "listIndex" => Ok(Pattern::ListIndex(Box::new(ListIndex::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "map" => Ok(Pattern::Map(Box::new(GritMap::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                is_rhs,
                logs,
            )?))),
            "mapAccessor" => Ok(Pattern::Accessor(Box::new(Accessor::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "dot" => Ok(Pattern::Dynamic(DynamicPattern::Snippet(DynamicSnippet {
                parts: vec![DynamicSnippetPart::String("".to_string())],
            }))),
            "dotdotdot" => Ok(Pattern::Dots),
            "underscore" => Ok(Pattern::Underscore),
            "regexPattern" => RegexPattern::from_node(
                node,
                context,
                vars,
                global_vars,
                vars_array,
                scope_index,
                context.lang,
                is_rhs,
                logs,
            ),
            "variable" => Ok(Pattern::Variable(Variable::from_node(
                node,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
            )?)),
            "codeSnippet" => CodeSnippet::from_node(
                node,
                context.file,
                context.src,
                vars,
                global_vars,
                vars_array,
                scope_index,
                context.lang,
                is_rhs,
            ),
            "like" => Ok(Pattern::Like(Box::new(Like::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?))),
            "undefined" => Ok(Pattern::Undefined),
            "top" => Ok(Pattern::Top),
            "bottom" => Ok(Pattern::Bottom),
            "intConstant" => Ok(Pattern::IntConstant(IntConstant::from_node(
                node,
                context.src,
            )?)),
            "sequential" => Ok(Pattern::Sequential(Sequential::from_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)),
            "files" => Ok(Pattern::Sequential(Sequential::from_files_node(
                node,
                context,
                vars,
                vars_array,
                scope_index,
                global_vars,
                logs,
            )?)),
            "doubleConstant" => Ok(Pattern::FloatConstant(FloatConstant::from_node(
                node,
                context.src,
            )?)),
            "booleanConstant" => Ok(Pattern::BooleanConstant(BooleanConstant::from_node(
                node,
                context.src,
            )?)),
            "stringConstant" => Ok(Pattern::StringConstant(StringConstant::from_node(
                node,
                context.src,
            )?)),
            _ => bail!("unknown pattern kind: {}", kind),
        }
    }
}

impl Name for Pattern {
    fn name(&self) -> &'static str {
        match self {
            Pattern::ASTNode(ast_node) => ast_node.name(),
            Pattern::Some(some) => some.name(),
            Pattern::Every(every) => every.name(),
            Pattern::List(nodes) => nodes.name(),
            Pattern::ListIndex(index) => index.name(),
            Pattern::Map(map) => map.name(),
            Pattern::Accessor(accessor) => accessor.name(),
            Pattern::Call(pattern_call) => pattern_call.name(),
            Pattern::Regex(regex) => regex.name(),
            Pattern::File(_pattern_call) => "FILE_PATTERN",
            Pattern::Files(_) => "MULTIFILE",
            Pattern::Bubble(pattern_call) => pattern_call.name(),
            Pattern::Limit(limit) => limit.name(),
            Pattern::CallBuiltIn(built_in) => built_in.name(),
            Pattern::CallFunction(call_function) => call_function.name(),
            Pattern::CallForeignFunction(call_function) => call_function.name(),
            Pattern::Assignment(assignment) => assignment.name(),
            Pattern::Accumulate(accumulate) => accumulate.name(),
            Pattern::StringConstant(string_constant) => string_constant.name(),
            Pattern::AstLeafNode(leaf_node) => leaf_node.name(),
            Pattern::IntConstant(int_constant) => int_constant.name(),
            Pattern::FloatConstant(double_constant) => double_constant.name(),
            Pattern::BooleanConstant(boolean_constant) => boolean_constant.name(),
            Pattern::Variable(variable) => variable.name(),
            Pattern::Add(add) => add.name(),
            Pattern::Subtract(subtract) => subtract.name(),
            Pattern::Multiply(multiply) => multiply.name(),
            Pattern::Divide(divide) => divide.name(),
            Pattern::Modulo(modulo) => modulo.name(),
            Pattern::And(and) => and.name(),
            Pattern::Or(or) => or.name(),
            Pattern::Maybe(maybe) => maybe.name(),
            Pattern::Any(any) => any.name(),
            Pattern::CodeSnippet(code_snippet) => code_snippet.name(),
            Pattern::Rewrite(rewrite) => rewrite.name(),
            Pattern::Log(log) => log.name(),
            Pattern::Range(range) => range.name(),
            Pattern::Contains(contains) => contains.name(),
            Pattern::Includes(includes) => includes.name(),
            Pattern::Within(within) => within.name(),
            Pattern::After(after) => after.name(),
            Pattern::Before(before) => before.name(),
            Pattern::Where(where_) => where_.name(),
            Pattern::Undefined => "UNDEFINED",
            Pattern::Top => "TOP",
            Pattern::Underscore => "UNDERSCORE",
            Pattern::Bottom => "BOTTOM",
            Pattern::Not(not) => not.name(),
            Pattern::If(if_) => if_.name(),
            Pattern::Dots => "DOTS",
            Pattern::Dynamic(dynamic_pattern) => dynamic_pattern.name(),
            Pattern::Sequential(sequential) => sequential.name(),
            Pattern::Like(like) => like.name(),
        }
    }
}

impl Matcher for Pattern {
    fn execute<'a>(
        &'a self,
        binding: &ResolvedPattern<'a>,
        state: &mut State<'a>,
        context: &'a impl Context,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let ResolvedPattern::File(file) = &binding {
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[FILENAME_INDEX].value =
                Some(file.name(&state.files));
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[ABSOLUTE_PATH_INDEX]
                .value = Some(file.absolute_path(&state.files)?);
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[PROGRAM_INDEX].value =
                Some(file.binding(&state.files));
        }

        match self {
            Pattern::ASTNode(ast_node) => ast_node.execute(binding, state, context, logs),
            Pattern::Some(some) => some.execute(binding, state, context, logs),
            Pattern::Every(every) => every.execute(binding, state, context, logs),
            Pattern::List(patterns) => patterns.execute(binding, state, context, logs),
            Pattern::ListIndex(index) => index.execute(binding, state, context, logs),
            Pattern::Map(map) => map.execute(binding, state, context, logs),
            Pattern::Accessor(accessor) => accessor.execute(binding, state, context, logs),
            Pattern::Files(files) => files.execute(binding, state, context, logs),
            Pattern::Call(pattern_call) => pattern_call.execute(binding, state, context, logs),
            Pattern::Regex(regex) => regex.execute(binding, state, context, logs),
            Pattern::File(file_pattern) => file_pattern.execute(binding, state, context, logs),
            Pattern::Bubble(pattern_call) => pattern_call.execute(binding, state, context, logs),
            Pattern::Limit(limit) => limit.execute(binding, state, context, logs),
            Pattern::CallBuiltIn(_) => bail!("CallBuiltIn cannot be executed at the moment"),
            Pattern::CallFunction(_) => {
                bail!("CallFunction cannot be executed at the moment")
            }
            Pattern::CallForeignFunction(_) => {
                bail!("CallForeignFunction cannot be executed at the moment")
            }
            Pattern::Assignment(assignment) => assignment.execute(binding, state, context, logs),
            Pattern::Accumulate(accumulate) => accumulate.execute(binding, state, context, logs),
            Pattern::StringConstant(string_constant) => {
                string_constant.execute(binding, state, context, logs)
            }
            Pattern::AstLeafNode(leaf_node) => leaf_node.execute(binding, state, context, logs),
            Pattern::IntConstant(int_constant) => {
                int_constant.execute(binding, state, context, logs)
            }
            Pattern::FloatConstant(double_constant) => {
                double_constant.execute(binding, state, context, logs)
            }
            Pattern::BooleanConstant(boolean_constant) => {
                boolean_constant.execute(binding, state, context, logs)
            }
            Pattern::Variable(variable) => variable.execute(binding, state, context, logs),
            Pattern::Add(add) => add.execute(binding, state, context, logs),
            Pattern::Subtract(subtract) => subtract.execute(binding, state, context, logs),
            Pattern::Multiply(multiply) => multiply.execute(binding, state, context, logs),
            Pattern::Divide(divide) => divide.execute(binding, state, context, logs),
            Pattern::Modulo(modulo) => modulo.execute(binding, state, context, logs),
            Pattern::And(and) => and.execute(binding, state, context, logs),
            Pattern::Or(or) => or.execute(binding, state, context, logs),
            Pattern::Maybe(maybe) => maybe.execute(binding, state, context, logs),
            Pattern::Any(any) => any.execute(binding, state, context, logs),
            Pattern::CodeSnippet(code_snippet) => {
                code_snippet.execute(binding, state, context, logs)
            }
            Pattern::Rewrite(rewrite) => rewrite.execute(binding, state, context, logs),
            Pattern::Log(log) => log.execute(binding, state, context, logs),
            Pattern::Range(range) => range.execute(binding, state, context, logs),
            Pattern::Contains(contains) => contains.execute(binding, state, context, logs),
            Pattern::Includes(includes) => includes.execute(binding, state, context, logs),
            Pattern::Within(within) => within.execute(binding, state, context, logs),
            Pattern::After(after) => after.execute(binding, state, context, logs),
            Pattern::Before(before) => before.execute(binding, state, context, logs),
            Pattern::Where(where_) => where_.execute(binding, state, context, logs),
            Pattern::Undefined => Undefined::execute(binding, state, context, logs),
            Pattern::Top => Ok(true),
            Pattern::Underscore => Ok(true),
            Pattern::Bottom => Ok(false),
            Pattern::Not(not) => not.execute(binding, state, context, logs),
            Pattern::If(if_) => if_.execute(binding, state, context, logs),
            Pattern::Dots => bail!("Dots should only be directly within a list pattern."),
            Pattern::Dynamic(pattern) => pattern.execute(binding, state, context, logs),
            Pattern::Sequential(sequential) => sequential.execute(binding, state, context, logs),
            Pattern::Like(like) => like.execute(binding, state, context, logs),
        }
    }
}
