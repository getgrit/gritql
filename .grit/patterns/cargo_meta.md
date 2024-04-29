# Inject correct cargo metadata into the markdown file

```grit
language toml

`[package]
$props` where {
  $filename <: includes "Cargo.toml",
  $filename <: not includes or {"language-metavariables", "language-submodules" },
  any {
    and { $props <: not contains `version`, $props += `version.workspace = true` },
    and { $props <: not contains `authors`, $props += `authors.workspace = true` },
    and { $props <: not contains `description`, $props += `description.workspace = true` },
    and { $props <: not contains `documentation`, $props += `documentation.workspace = true` },
    and { $props <: not contains `homepage`, $props += `homepage.workspace = true` },
    and { $props <: not contains `license`, $props += `license = "MIT"` },
    and { $props <: not contains `publish`, $props += `publish = false` },
  }
}
```