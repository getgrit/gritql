# Avoid panic

Panics should be avoided in core Grit code, because they don't support recovery.

```rust
`$x.unwrap($_)` => `$x?`
```

## Sample

```rust
let x = bar().unwrap();
```

```rust
let x = bar()?;
```
