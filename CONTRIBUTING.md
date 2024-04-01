# Contributing to GritQL

Welcome! We'd love to help you contribute to GritQL.


## Welcome

We welcome contributions in the form of pull requests and issues.

Note that this codebase isn't yet extensively documented. If you get stuck, please ask for help [on Discord](https://docs.grit.io/discord).

## Development Setup

A high-level overview of tools you need to have installed:

* Rust toolchain: for compiling the codebase. You'll need [`rustc`](https://rustup.rs/) v1.74 or newer.
  * To create WASM builds, run `rustup target install wasm32-unknown-unknown`.
* C/C++ compiler. macOS: [Xcode Command Line Tools](https://download.developer.apple.com/Developer_Tools/Command_Line_Tools_for_Xcode_15.3/Command_Line_Tools_for_Xcode_15.3.dmg) via `xcode-select --install`, Linux: [gcc](https://learnubuntu.com/install-gcc/), Windows: [Microsoft Visual C++](https://visualstudio.microsoft.com/vs/features/cplusplus/).
* Emscripten: a C/C++ compiler toolchain for WASM. Install v3.1.56 with [`emsdk`](https://emscripten.org/docs/getting_started/downloads.html).
* Node.js runtime: `node`, `npm`, `npx` are used to generate parsers from `grammar.js` files. You'll need [`node`](https://nodejs.org/en/download) v18.5.0 or newer.
* Yarn package manager. You'll need [`yarn`](https://classic.yarnpkg.com/en/docs/install) (classic). Install v1.22.19 with `npm install --global yarn`.
* Tree-Sitter CLI: provides [`tree-sitter`](https://github.com/tree-sitter/tree-sitter/tree/master/cli) binary for testing grammars. Install v0.22.2 with `npm install --global tree-sitter-cli`.
* Terraform CLI. Install [`terraform`](https://developer.hashicorp.com/terraform/tutorials/aws-get-started/install-cli) with `brew tap hashicorp/tap && brew install hashicorp/tap/terraform`. 

## Building the Code

Use `git` to clone this repository into a location of your choice. 
```bash
git clone https://github.com/getgrit/gritql.git
```

Change into the cloned repository and make sure all submodules are correctly set up, including any nested submodules:
```bash
cd gritql
git submodule update --init --recursive
```

Before making any changes to the code, make sure you can run the tests and everything is initially passing:
```bash
cargo test --workspace
```

## Feature Flags

We use [feature flags](https://doc.rust-lang.org/cargo/reference/features.html) to control which parts of the codebase are compiled.

Note that some proprietary server-only integrations are hidden behind the "server" feature flag. This flag is disabled by default, and code should compile without any additions.

For major changes, we put new features should be put into the `grit_alpha` feature flag. Features that are ready for broad release should be put into the `grit_beta` feature flag. This is used for all public releases.

Features that should be tested in CI should be put into the `grit_ci` feature flag. This is used for all CI tests.

## Language Grammars

If GritQL is failing to match a code snippet, this can typically be fixed simply by adjusting the metavariable grammar for the target language.

Metavariable grammars are found under [./resources/metavariable-grammars](./resources/metavariable-grammars). Typical fixes include:
- Adding a new named field for a relevant node you want to manipulate.
- Adding a `grit_metavariable` node as a choice in the corresponding spot where you want to substitute the metavariable.
- Check [this guide](https://github.com/tree-sitter/tree-sitter/wiki/Tips-and-Tricks-for-a-grammar-author) to debug grammars generally.

After making your changes, run the [./resources/edit_grammars.mjs](./resources/edit_grammars.mjs) script to regenerate the matching grammar.

### Snippet contexts

Snippet contexts help when a snippet is a valid AST subtree, but needs to be in a larger tree to parse. For example, matching on a table name like ` $schema.$table` in SQL is not valid SQL by itself, only when surrounded by something like `SELECT x from $schema.$table` is the snippet valid.

Snippet contexts are defined by implementing the `snippet_context_strings` method in the `Language` trait. This method returns a list of strings that are used to match the snippet in the larger tree. For example, the SQL implementation returns `["SELECT 1 from ", ";"]` to match a table name in a SQL query.

## Adding a New Target Language

Note: Grit involves *two* languages:

- GritQL is [our query language](https://docs.grit.io/language/reference) for searching and transforming codebases.
- The “target language” is the language we are transforming (ex. Python). This document describes the process of adding new target languages to Grit.

Most of these steps involve iteration over a set of sample programs to get closer to correctness. The initial work for a language can typically be done in a day or two.

Here are the steps for adding a new target language:

0. Add the language as a supported language in the GritQL grammar, [like this](https://github.com/getgrit/tree-sitter-gritql/commit/ea514376a6da7bfc187c05d93e403112cae87787).
1. Find a tree sitter grammar for the language and add it as a submodule under `resources/language-submodules`.
2. Add a simple parse test in `crates/core/src/test.rs` to ensure that the grammar is working.
3. Copy the grammar file into `resources/metavariable-grammars`. This alternative grammar is used for parsing `snippets` in GritQL.
4. Patch the metavariable grammar to include  `$.grit_metavariable` anywhere we want to substitute a metavariable. This is usually at least `$identifier` and `$literal`.
    - For a snippet to match, it also needs to be a field. Often you’ll want to wrap `$thing` like: `field('thing', choice($.grit_metavariable, $thing))`
5. Add a new language implementation in `crates/core/languages/src`. This involves implementing the `Language` trait and adding a new `Language` enum variant.
6. Add `snippet_context_strings` [like this](https://github.com/getgrit/gritql/blob/main/crates/language/src/sql.rs#L52) to provide context for snippets to match in.
7. Add test cases for the language in `crates/core/src/test.rs`. This is a good time to add a few dozen test cases to ensure that the language is parsed correctly, and that the metavariable grammar is working.

### Internal steps

These steps are done in our cloud environment and are not necessary for contributors to do.

- grep for an existing language like `Sol` for solidity, and add it to all the `Language` enums you find.
    - Add the language to `apps/web/src/views/project/details.tsx`, so repos with this language don’t get an “unsupported language” warning. (5 minutes)
    - LSP target languages list: https://github.com/getgrit/rewriter/pull/7734/files#diff-f9d4f097b08d33241c5c8d15a2fbde0e37086c265ce0eba8decac20d5cd989c6R23
    - VS Code client list: https://github.com/getgrit/rewriter/blob/f992490394a4807789504f1cea6a04b934ad3b24/apps/poolish/src/lsp-client.ts
    - VS Code command palette triggers: https://github.com/getgrit/rewriter/pull/7734/files#diff-b38f1d6304993a250903310722206e6c89c58c52c2d1bd4b6fdd8f7218810570R103
- There are also `exhaustive` runtime checks that error if a switch case doesn’t handle a language, like `makeSingleLineComment`. Search for `exhaustive(lang` and fill those out too.
- Regenerate both DB/prisma types to add it to the DB schema and GraphQL types.
- Add the language to `language-selector.tsx`. Pick an icon from [https://react-icons.github.io](https://react-icons.github.io/), usually from the Simple Icons category.
