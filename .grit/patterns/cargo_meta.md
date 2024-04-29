# Inject correct cargo metadata into the markdown file

```grit
language toml

`[package]
$props` where {
  $filename <: includes "Cargo.toml",
  $filename <: not includes or {"language-metavariables", "language-submodules" },
  any {
    $props <: not contains `version` where $props += `version.workspace = true`
  }
}
```