<div align="center">
  <picture>
    <source media="(prefers-color-scheme: dark)" srcset="https://raw.githubusercontent.com/getgrit/gritql/main/assets/grit-logo-darkmode.png">
    <img alt="Grit logo" src="https://raw.githubusercontent.com/getgrit/gritql/main/assets/grit-logo.png" width="40%">
  </picture>
</div>

<br>

<div align="center">

[![CI Status](https://img.shields.io/github/actions/workflow/status/getgrit/gritql/main.yaml)](https://github.com/getgrit/gritql/actions/workflows/main.yaml)
[![MIT License](https://img.shields.io/github/license/getgrit/gritql)](https://github.com/getgrit/gritql/blob/main/LICENSE)
[![Discord](https://img.shields.io/discord/1063097320771698699?logo=discord&label=discord)](https://docs.grit.io/discord)

[Playground](https://app.grit.io/studio) |
[Tutorial](https://docs.grit.io/tutorials/gritql) |
[Docs](https://docs.grit.io/language)

</div>

<hr>

GritQL is a declarative query language for searching and modifying source code.

- 📖 Start simply without learning AST details: any code snippet is a valid GritQL query
- ⚡️ Use Rust and query optimization to scale up to 10M+ line repositories
- 📦 Use Grit's built-in module system to reuse 200+ [standard patterns](https://github.com/getgrit/stdlib) or [share your own](https://docs.grit.io/guides/sharing#anchor-publishing-patterns)
- ♻️ Once you learn GritQL, you can use it to rewrite any [target language](https://docs.grit.io/language/target-languages): JavaScript/TypeScript, Python, JSON, Java, Terraform, Solidity, CSS, Markdown, YAML, Rust, Go, or SQL
- 🔧 GritQL makes it easy to include auto-fix rules for faster remediation

## Getting started

Read the [documentation](https://docs.grit.io/language/overview), [interactive tutorial](https://docs.grit.io/tutorials/gritql), or run `grit --help`.

### Installation

Install the Grit CLI:

```
curl -fsSL https://docs.grit.io/install | bash
```

### Usage

Search for all your `console.log` calls by putting the desired pattern in backticks:

```
grit apply '`console.log($_)`'
```

Replace `console.log` with `winston.log`, using `=>` to create rewrites:

```
grit apply '`console.log($msg)` => `winston.log($msg)`'
```

Save the pattern to a [`grit.yaml`](https://docs.grit.io/guides/config) file and exclude test cases in a where clause:

```
cat << 'EOF' > .grit/grit.yaml
patterns:
  - name: use_winston
    level: error
    body: |
      `console.log($msg)` => `winston.log($msg)` where {
        $msg <: not within or { `it($_, $_)`, `test($_, $_)`, `describe($_, $_)` }
      }
EOF
grit apply use_winston
```

Run `grit check` to enforce your patterns as [custom lints](https://docs.grit.io/guides/ci).

```
grit check
```

## Examples

### Remove all `console.log` calls, unless they are inside a try-catch block

```grit
`console.log($log)` => . where {
  $log <: not within `try { $_ } catch { $_ }`
}
```

### Replace a method call with a new method call

```grit
`$instance.oldMethod($args)` => `$instance.newMethod($args)` where {
  $program <: contains `$instance = new TargetClass($_)`
}
```

### More examples

Many more examples can be found in the [GritQL standard library](https://github.com/getgrit/stdlib/blob/main/.grit/patterns/).

Patterns can be combined to create complex queries, including [large refactors](https://github.com/getgrit/stdlib/blob/main/.grit/patterns/python/openai.md).

## Why GritQL?

GritQL comes from our experiences with conducting large scale refactors and migrations.

Usually, migrations start with exploratory work to figure out the scope of the problem—often using simple grep searches. These are easy to start with, but most migrations end up accumulating additional requirements like ensuring the right packages are imported and excluding cases which don’t have a viable migration path.

Eventually, any complex migration ends up being a full codemod program written with a tool like [jscodeshift](https://github.com/facebook/jscodeshift). This comes with its own problems:
- Most of the exploratory work has to be abandoned as you figure out how to represent your original regex search as an AST.
- Reading/writing a codemod requires mentally translating from AST names back to what source code actually looks like.
- Most frameworks are not composable, so you’re stuck copying patterns back and forth.
- Performance is often an afterthought, so iterating on a large codemod can be painfully slow.
- Codemod frameworks are language-specific, so if you’re hopping between multiple languages—or trying to migrate a shared API—you have to learn different frameworks.

GritQL is our attempt to develop a powerful middle ground:
- Exploratory analysis is easy: just put a code snippet in backticks and use `$metavariables` for holes you want to represent.
- Incrementally add complexity by introducing side conditions with where clauses.
- Reuse named patterns to avoid rebuilding queries, and use shared patterns from our [standard library](https://github.com/getgrit/stdlib) for common tasks like ensuring modules are imported.
- Written in Rust for maximum performance: rewrite millions of lines of code in seconds.

## Acknowledgements

GritQL uses [tree-sitter](https://github.com/tree-sitter/tree-sitter) for all language parsers and benefits greatly from the Rust ecosystem.

GritQL is released under the MIT license.

## Contributing

Contributions are welcome. To get started, check out the [**contributing guidelines**](./CONTRIBUTING.md).

You can also join us on [**Discord**](https://docs.grit.io/discord).
