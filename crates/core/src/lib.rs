#![deny(clippy::wildcard_enum_match_arm)]
pub mod binding;
pub mod compact_api;
mod effects_dependency_graph;
mod equivalence;
pub mod errors;
pub mod fs;
mod inline_snippets;
mod intervals;
mod orphan;
pub mod parse;
pub mod pattern;
mod resolve;
mod smart_insert;
mod split_snippet;
mod suppress;
mod text_unparser;
pub mod tree_sitter_serde;

#[cfg(test)]
mod test;
