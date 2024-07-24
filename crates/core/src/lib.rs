#![deny(clippy::wildcard_enum_match_arm)]
pub mod analysis;
pub mod api;
pub mod ast_node;
pub mod built_in_functions;
mod clean;
pub mod compact_api;
pub mod constants;
mod equivalence;
mod foreign_function_definition;
pub mod fs;
mod inline_snippets;
mod limits;
pub mod marzano_binding;
pub mod marzano_code_snippet;
pub mod marzano_context;
pub mod marzano_resolved_pattern;
mod optimizer;
pub mod parse;
mod paths;
pub mod pattern_compiler;
pub mod problem;
mod smart_insert;
mod split_snippet;
mod suppress;
mod text_unparser;
pub mod tree_sitter_serde;
mod variables;

// getrandom is a deeply nested dependency used by many things eg. uuid
// to get wasm working we needed to enable a feature for this crate, so
// while we don't have a direct usage of it, we had to add it as a dependency
// and here we import it to avoid an unused dependency warning
#[cfg(feature = "wasm_core")]
use getrandom as _;
#[cfg(test)]
mod test_notebooks;

#[cfg(test)]
mod test;
#[cfg(test)]
mod test_files;
#[cfg(any(test, feature = "test_utils"))]
pub mod test_utils;
mod error;
