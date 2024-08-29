mod accessor;
mod accumulate;
mod add;
mod after;
mod and;
mod any;
mod assignment;
mod ast_node_pattern;
mod before;
mod boolean_constant;
mod bubble;
mod call;
mod call_built_in;
mod callback_pattern;
mod container;
mod contains;
mod divide;
mod dynamic_snippet;
mod equal;
mod every;
mod file_pattern;
mod files;
mod float_constant;
mod function_definition;
mod functions;
mod r#if;
mod includes;
mod int_constant;
mod iter_pattern;
mod like;
mod limit;
mod list;
mod list_index;
mod log;
mod map;
mod r#match;
mod maybe;
mod modulo;
mod multiply;
mod not;
mod or;
mod pattern_definition;
mod patterns;
mod predicate_definition;
mod predicate_return;
mod predicates;
mod range;
mod regex;
mod resolved_pattern;
mod rewrite;
mod sequential;
mod some;
mod state;
mod step;
mod string_constant;
mod subtract;
mod undefined;
mod variable;
mod variable_content;
mod r#where;
mod within;

pub use accessor::{Accessor, AccessorKey, AccessorMap};
pub use accumulate::Accumulate;
pub use add::Add;
pub use after::After;
pub use and::{And, PrAnd};
pub use any::{Any, PrAny};
pub use assignment::Assignment;
pub use ast_node_pattern::{AstLeafNodePattern, AstNodePattern};
pub use before::Before;
pub use boolean_constant::BooleanConstant;
pub use bubble::Bubble;
pub use call::{Call, PrCall};
pub use call_built_in::CallBuiltIn;
pub use callback_pattern::CallbackPattern;
pub use container::{Container, PatternOrResolved, PatternOrResolvedMut};
pub use contains::Contains;
pub use divide::Divide;
pub use dynamic_snippet::{DynamicList, DynamicPattern, DynamicSnippet, DynamicSnippetPart};
pub use equal::Equal;
pub use every::Every;
pub use file_pattern::FilePattern;
pub use files::Files;
pub use float_constant::FloatConstant;
pub use function_definition::{FunctionDefinition, GritFunctionDefinition};
pub use functions::{CallForeignFunction, CallFunction, Evaluator, FuncEvaluation, GritCall};
pub use includes::Includes;
pub use int_constant::IntConstant;
pub use iter_pattern::{PatternOrPredicate, PatternOrPredicateIterator};
pub use like::Like;
pub use limit::Limit;
pub use list::List;
pub use list_index::{to_unsigned, ContainerOrIndex, ListIndex, ListOrContainer};
pub use log::{Log, VariableInfo};
pub use map::GritMap;
pub use maybe::{Maybe, PrMaybe};
pub use modulo::Modulo;
pub use multiply::Multiply;
pub use not::{Not, PrNot};
pub use or::{Or, PrOr};
pub use pattern_definition::PatternDefinition;
pub use patterns::{CodeSnippet, Matcher, Pattern, PatternName};
pub use predicate_definition::PredicateDefinition;
pub use predicate_return::PrReturn;
pub use predicates::Predicate;
pub use r#if::{If, PrIf};
pub use r#match::Match;
pub use r#where::Where;
pub use range::{Point, Range};
pub use regex::{RegexLike, RegexPattern};
pub use resolved_pattern::ResolvedPattern;
pub use resolved_pattern::{File, JoinFn, LazyBuiltIn, ResolvedFile, ResolvedSnippet};
pub use rewrite::Rewrite;
pub use sequential::Sequential;
pub use some::Some;
pub use state::{get_top_level_effects, EffectRange, FilePtr, FileRegistry, State};
pub use step::Step;
pub use string_constant::StringConstant;
pub use subtract::Subtract;
pub use undefined::Undefined;
pub use variable::{
    get_absolute_file_name, get_file_name, is_reserved_metavariable, Variable,
    VariableSourceLocations,
};
pub use variable_content::VariableContent;
pub use within::Within;

use crate::context::QueryContext;

#[cfg(feature = "grit_tracing")]
use tracing::{instrument, span};
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

/**
 * We want both Work and State to not contain things that cannot be moved across threads.
 *
 * Even without threads, we want the ability to continue execution with a close of a State and Work.
 *
 * E.g., If a Node would contain a tree-sitter cursor, that would not be safe.
 */
pub trait Work<Q: QueryContext> {
    // it is important that any implementors of Work
    // do not compute-expensive things in execute
    // it should be stored somewhere in the struct of the implementor
    // fn execute(&self, state: &mut State) -> Vec<Match>;
    fn execute(&self, state: &mut State<Q>);
}
