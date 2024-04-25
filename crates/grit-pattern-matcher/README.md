# `grit-pattern-matcher`

This crate contains all the pattern definitions that are at the heart of the
GritQL engine. There's a `Matcher` trait that's implemented by the patterns,
which implements the matching logic.

It is important this crate stays free of TreeSitter dependencies, since it is
intended to be reusable by other engines which may use their own parser
infrastructure.
