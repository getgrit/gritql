# GritQL

[![CI Status](https://img.shields.io/github/actions/workflow/status/getgrit/tree-sitter-gritql/ci.yml)](https://github.com/getgrit/tree-sitter-gritql/actions/workflows/ci.yml)
[![MIT License](https://img.shields.io/github/license/getgrit/tree-sitter-gritql)](https://github.com/getgrit/tree-sitter-gritql/blob/main/LICENSE)
[![Discord](https://img.shields.io/discord/1063097320771698699?logo=discord&label=discord)](https://docs.grit.io/discord)

GritQL is a declarative query language for searching and modifying source code. GritQL focuses on a few areas:

- 📖 Start simply without learning AST details: any code snippet is a valid GritQL query
- ⚡️ Use Rust and query optimization to scale up to 10M+ line repositories
- 📦 Use Grit's built-in module system to reuse 200+ [standard patterns](https://github.com/getgrit/stdlib) or share your own
- ♻️ Once you learn GritQL, you can use it to rewrite any [target language](https://docs.grit.io/language/target-languages): JavaScript/TypeScript, Python, JSON, Java, Terraform, Solidity, CSS, Markdown, YAML, Rust, Go, or SQL
- 🔧 GritQL makes it easy to include auto-fix rules for faster remediation

Read the [docs](https://docs.grit.io/language) or try any query in the [studio](https://app.grit.io/studio).

## Getting started
For more, see the [documentation](https://docs.grit.io/language/overview), [interactive tutorial](https://docs.grit.io/tutorials/gritql), or run `grit --help`.

### Installation
Install the Grit CLI:

```
curl -fsSL https://docs.grit.io/install | bash
```

### Usage

Find all your `console.log` calls:
```
grit apply '`console.log($_)`'
```

Replace `console.log` with `winston.log`:
```
grit apply '`console.log($msg)` => `winston.log($msg)`'
```

Save the pattern to [`grit.yaml`](https://docs.grit.io/guides/config) file and exclude test cases:
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

## Acknowledgements

GritQL uses [tree sitter](https://github.com/tree-sitter/tree-sitter) for all language parsers and benefits greatly from the Rust ecosystem.

GritQL is released under the MIT license.

## Contributing

Contributions are welcome. To get started, check out the[**contributing guidelines**](./contributing.md).

You can also join us on [**Discord**](https://docs.grit.io/discord).
