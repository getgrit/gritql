//! Grit pattern definitions and matching logic
//!
//! This crate contains all the pattern definitions that are at the heart of the
//! GritQL engine. There's the [`pattern::Matcher`] trait that's implemented by
//! the patterns, which implements the matching logic.

pub mod binding;
pub mod constant;
pub mod constants;
pub mod context;
pub mod effects;
pub mod errors;
pub mod file_owners;
pub mod intervals;
pub mod pattern;
