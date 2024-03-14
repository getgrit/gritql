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

// getrandom is a deeply nested dependency used by many things eg. uuid
// to get wasm working we needed to enable a feature for this crate, so
// while we don't have a direct usage of it, we had to add it as a dependency
// and here we import it to avoid an unused dependency warning
#[cfg(target_arch = "wasm32")]
use getrandom as _;