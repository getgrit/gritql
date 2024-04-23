pub mod accessor;
pub mod accumulate;
pub mod add;
pub mod after;
pub mod and;
pub mod any;
pub mod assignment;
pub mod ast_node_pattern;
pub mod before;
pub mod boolean_constant;
pub mod bubble;
pub mod call;
pub mod call_built_in;
pub mod container;
pub mod contains;
pub mod divide;
pub mod dynamic_snippet;
pub mod equal;
pub mod every;
pub mod file_pattern;
pub mod files;
pub mod float_constant;
pub mod function_definition;
pub mod functions;
pub mod r#if;
pub mod includes;
pub mod int_constant;
pub mod iter_pattern;
pub mod like;
pub mod limit;
pub mod list;
pub mod list_index;
pub mod log;
pub mod map;
pub mod r#match;
pub mod maybe;
pub mod modulo;
pub mod multiply;
pub mod not;
pub mod or;
pub mod pattern_definition;
pub mod patterns;
pub mod predicate_definition;
pub mod predicate_return;
pub mod predicates;
pub mod range;
pub mod regex;
pub mod resolved_pattern;
pub mod rewrite;
pub mod sequential;
pub mod some;
pub mod state;
pub mod step;
pub mod string_constant;
pub mod subtract;
pub mod undefined;
pub mod variable;
pub mod variable_content;
pub mod r#where;
pub mod within;

use self::{pattern_definition::PatternDefinition, state::State};
use crate::context::QueryContext;

#[cfg(feature = "grit_tracing")]
use tracing::{instrument, span};
#[cfg(feature = "grit_tracing")]
use tracing_opentelemetry::OpenTelemetrySpanExt as _;

pub const MAX_FILE_SIZE: usize = 1_000_000;

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
