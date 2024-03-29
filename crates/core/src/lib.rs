#![deny(clippy::wildcard_enum_match_arm)]
pub mod binding;
pub mod compact_api;
pub mod context;
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

// getrandom is a deeply nested dependency used by many things eg. uuid
// to get wasm working we needed to enable a feature for this crate, so
// while we don't have a direct usage of it, we had to add it as a dependency
// and here we import it to avoid an unused dependency warning
#[cfg(feature = "wasm_core")]
use getrandom as _;

#[cfg(test)]
mod test;
