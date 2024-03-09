---
level: info
---

# Avoid panic

Panics should be avoided in core Grit code. Instead, use `?` to propagate errors.

```grit
language rust

file($name, $body) where {
  $name <: includes "marzano/lsp",
  $body <: contains `$foo.unwrap()` => `$foo?`,
}
```

## Sample

```rust
// @filename: marzano/lsp/foo.rs
let x = bar().unwrap();
```

```rust
// @filename: marzano/lsp/foo.rs
let x = bar()?;
```
