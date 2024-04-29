/// Creates a new scope within the given `context`.
///
/// This is implemented as a macro instead of method to avoid capturing the
/// entire `context` instance, which would run afoul of the borrow-checking due
/// to its mutable references.
macro_rules! create_scope {
    ($context: expr, $local_vars: expr) => {{
        let scope_index = $context.vars_array.len();
        $context.vars_array.push(Vec::new());
        let context = crate::pattern_compiler::compiler::NodeCompilationContext {
            compilation: $context.compilation,
            vars: &mut $local_vars,
            vars_array: $context.vars_array,
            scope_index,
            global_vars: $context.global_vars,
            logs: $context.logs,
        };
        (scope_index, context)
    }};
}

pub(crate) mod accessor_compiler;
pub(crate) mod accumulate_compiler;
pub(crate) mod add_compiler;
pub(crate) mod after_compiler;
pub(crate) mod and_compiler;
pub(crate) mod any_compiler;
pub(crate) mod as_compiler;
pub(crate) mod assignment_compiler;
pub(crate) mod ast_node_compiler;
mod auto_wrap;
pub(crate) mod back_tick_compiler;
pub(crate) mod before_compiler;
pub(crate) mod bubble_compiler;
mod builder;
pub(crate) mod call_compiler;
pub mod compiler;
pub(crate) mod constant_compiler;
pub(crate) mod container_compiler;
pub(crate) mod contains_compiler;
pub(crate) mod divide_compiler;
pub(crate) mod equal_compiler;
pub(crate) mod every_compiler;
pub(crate) mod file_owner_compiler;
pub(crate) mod foreign_language_compiler;
pub(crate) mod function_definition_compiler;
pub(crate) mod if_compiler;
pub(crate) mod includes_compiler;
pub(crate) mod like_compiler;
pub(crate) mod limit_compiler;
pub(crate) mod list_compiler;
pub(crate) mod list_index_compiler;
pub(crate) mod log_compiler;
pub(crate) mod map_compiler;
pub(crate) mod match_compiler;
pub(crate) mod maybe_compiler;
pub(crate) mod modulo_compiler;
pub(crate) mod multiply_compiler;
mod node_compiler;
pub(crate) mod not_compiler;
pub(crate) mod or_compiler;
#[allow(clippy::module_inception)]
pub(crate) mod pattern_compiler;
pub(crate) mod pattern_definition_compiler;
pub(crate) mod predicate_compiler;
pub(crate) mod predicate_definition_compiler;
pub(crate) mod predicate_return_compiler;
pub(crate) mod range_compiler;
pub(crate) mod regex_compiler;
pub(crate) mod rewrite_compiler;
pub(crate) mod sequential_compiler;
pub(crate) mod snippet_compiler;
pub(crate) mod some_compiler;
pub(crate) mod step_compiler;
pub(crate) mod subtract_compiler;
pub(crate) mod variable_compiler;
pub(crate) mod where_compiler;
pub(crate) mod within_compiler;

pub use builder::PatternBuilder;
pub use compiler::{src_to_problem_libs, CompilationResult};
pub(crate) use node_compiler::NodeCompiler;
