use super::{
    accessor::Accessor,
    accumulate::Accumulate,
    add::Add,
    after::After,
    and::And,
    any::Any,
    assignment::Assignment,
    before::Before,
    boolean_constant::BooleanConstant,
    bubble::Bubble,
    call::Call,
    call_built_in::CallBuiltIn,
    contains::Contains,
    divide::Divide,
    dynamic_snippet::DynamicPattern,
    every::Every,
    file_pattern::FilePattern,
    files::Files,
    float_constant::FloatConstant,
    functions::{CallForeignFunction, CallFunction},
    includes::Includes,
    int_constant::IntConstant,
    like::Like,
    limit::Limit,
    list::List,
    list_index::ListIndex,
    log::Log,
    map::GritMap,
    maybe::Maybe,
    modulo::Modulo,
    multiply::Multiply,
    not::Not,
    or::Or,
    r#if::If,
    r#where::Where,
    range::Range as PRange,
    regex::RegexPattern,
    resolved_pattern::ResolvedPattern,
    rewrite::Rewrite,
    sequential::Sequential,
    some::Some,
    string_constant::StringConstant,
    subtract::Subtract,
    undefined::Undefined,
    variable::Variable,
    within::Within,
    State,
};
use crate::{
    constants::{ABSOLUTE_PATH_INDEX, FILENAME_INDEX, GLOBAL_VARS_SCOPE_INDEX, PROGRAM_INDEX},
    context::{ExecContext, QueryContext},
    pattern::resolved_pattern::File,
};
use anyhow::{bail, Result};
use core::fmt::Debug;
use grit_util::AnalysisLogs;

pub trait Matcher<Q: QueryContext>: Debug {
    // it is important that any implementors of Pattern
    // do not compute-expensive things in execute
    // it should be stored somewhere in the struct of the implementor
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool>;

    // for the future:
    // we could speed up computation by filtering on the sort of pattern
    // here, &SortFormula is a propositional-logic formula over sorts
    // fn sort(&self) -> SortFormula;
}

pub trait PatternName {
    fn name(&self) -> &'static str;
}

#[derive(Debug, Clone)]
pub enum Pattern<Q: QueryContext> {
    AstNode(Box<Q::NodePattern>),
    List(Box<List<Q>>),
    ListIndex(Box<ListIndex<Q>>),
    Map(Box<GritMap<Q>>),
    Accessor(Box<Accessor<Q>>),
    Call(Box<Call<Q>>),
    Regex(Box<RegexPattern<Q>>),
    File(Box<FilePattern<Q>>),
    Files(Box<Files<Q>>),
    Bubble(Box<Bubble<Q>>),
    Limit(Box<Limit<Q>>),
    CallBuiltIn(Box<CallBuiltIn<Q>>),
    CallFunction(Box<CallFunction<Q>>),
    CallForeignFunction(Box<CallForeignFunction<Q>>),
    Assignment(Box<Assignment<Q>>),
    Accumulate(Box<Accumulate<Q>>),
    And(Box<And<Q>>),
    Or(Box<Or<Q>>),
    Maybe(Box<Maybe<Q>>),
    Any(Box<Any<Q>>),
    Not(Box<Not<Q>>),
    If(Box<If<Q>>),
    Undefined,
    Top,
    Bottom,
    // differentiated from top for debugging purposes.
    Underscore,
    StringConstant(StringConstant),
    AstLeafNode(Q::LeafNodePattern),
    IntConstant(IntConstant),
    FloatConstant(FloatConstant),
    BooleanConstant(BooleanConstant),
    Dynamic(DynamicPattern<Q>),
    CodeSnippet(Q::CodeSnippet),
    Variable(Variable),
    Rewrite(Box<Rewrite<Q>>),
    Log(Box<Log<Q>>),
    Range(PRange),
    Contains(Box<Contains<Q>>),
    Includes(Box<Includes<Q>>),
    Within(Box<Within<Q>>),
    After(Box<After<Q>>),
    Before(Box<Before<Q>>),
    Where(Box<Where<Q>>),
    Some(Box<Some<Q>>),
    Every(Box<Every<Q>>),
    Add(Box<Add<Q>>),
    Subtract(Box<Subtract<Q>>),
    Multiply(Box<Multiply<Q>>),
    Divide(Box<Divide<Q>>),
    Modulo(Box<Modulo<Q>>),
    Dots,
    Sequential(Sequential<Q>),
    Like(Box<Like<Q>>),
}

impl<Q: QueryContext> Pattern<Q> {
    // todo this should return a cow, but currently can't figure out lifetimes
    pub fn text<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<String> {
        Ok(
            Q::ResolvedPattern::from_pattern(self, state, context, logs)?
                .text(&state.files, context.language())?
                .to_string(),
        )
    }

    pub(crate) fn float<'a>(
        &'a self,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<f64> {
        Q::ResolvedPattern::from_pattern(self, state, context, logs)?
            .float(&state.files, context.language())
    }
}

impl<Q: QueryContext> PatternName for Pattern<Q> {
    fn name(&self) -> &'static str {
        match self {
            Pattern::AstNode(ast_node) => ast_node.name(),
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

impl<Q: QueryContext> Matcher<Q> for Pattern<Q> {
    fn execute<'a>(
        &'a self,
        binding: &Q::ResolvedPattern<'a>,
        state: &mut State<'a, Q>,
        context: &'a Q::ExecContext<'a>,
        logs: &mut AnalysisLogs,
    ) -> Result<bool> {
        if let Some(file) = binding.get_file() {
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[FILENAME_INDEX].value =
                Some(file.name(&state.files));
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[ABSOLUTE_PATH_INDEX]
                .value = Some(file.absolute_path(&state.files, context.language())?);
            state.bindings[GLOBAL_VARS_SCOPE_INDEX].back_mut().unwrap()[PROGRAM_INDEX].value =
                Some(file.binding(&state.files));
        }

        match self {
            Pattern::AstNode(ast_node) => ast_node.execute(binding, state, context, logs),
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

pub trait CodeSnippet<Q: QueryContext + 'static>: Clone + Debug + Matcher<Q> + PatternName {
    fn patterns(&self) -> impl Iterator<Item = &Pattern<Q>>;

    fn dynamic_snippet(&self) -> Option<&DynamicPattern<Q>>;
}
