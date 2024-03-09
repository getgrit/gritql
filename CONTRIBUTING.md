# Contributing to GritQL

Welcome! We'd love to help you contribute to GritQL.


## Welcome

We welcome contributions in the form of pull requests and issues.

Note that this codebase isn't yet extensively documented. If you get stuck, please ask for help [on Discord](https://docs.grit.io/discord).

## Language Grammars

If GritQL is failing to match a code snippet, this can typically be fixed simply by adjusting the metavariable grammar for the target language.

Metavariable grammars are found under [./resources/metavariable-grammars](./resources/metavariable-grammars). Typical fixes include:
- Adding a new named field for a relevant node you want to manipulate.
- Adding a `grit_metavariable` node as a choice in the corresponding spot where you want to substitute the metavariable.

After making your changes, run the [./resources/edit_grammars.mjs](./resources/edit_grammars.mjs) script to regenerate the matching grammar.

## Feature Flags

We use [feature flags](https://doc.rust-lang.org/cargo/reference/features.html) to control which parts of the codebase are compiled.

Note that some proprietary server-only integrations are hidden behind the "server" feature flag. This flag is disabled by default and code should compile without any additions.

For major changes, we put new features should be put into the `grit_alpha` feature flag. Features that are ready for broad release should be put into the `grit_beta` feature flag. This is used for all public releases.

Features that should be tested in CI should be put into the `grit_ci` feature flag. This is used for all CI tests.
