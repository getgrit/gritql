use crate::pattern::{
    built_in_functions::BuiltIns,
    function_definition::{ForeignFunctionDefinition, GritFunctionDefinition},
    pattern_definition::PatternDefinition,
    predicate_definition::PredicateDefinition,
    FileOwners,
};
use marzano_language::target_language::TargetLanguage;

pub trait Context<'a> {
    fn pattern_definitions(&self) -> &[PatternDefinition];

    fn predicate_definitions(&self) -> &[PredicateDefinition];

    fn function_definitions(&self) -> &[GritFunctionDefinition];

    fn foreign_function_definitions(&self) -> &[ForeignFunctionDefinition];

    fn ignore_limit_pattern(&self) -> bool;

    // FIXME: Don't depend on Grit's file handling in Context.
    fn files(&self) -> &FileOwners;

    fn built_ins(&self) -> &BuiltIns;

    // FIXME: This introduces a dependency on TreeSitter.
    fn language(&self) -> &TargetLanguage;

    fn name(&self) -> Option<&str>;
}
