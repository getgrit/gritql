# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.5.2](https://github.com/getgrit/gritql/compare/grit-pattern-matcher-v0.5.1...grit-pattern-matcher-v0.5.2) - 2025-04-14

### Other

- increase Rust version to 1.82.0 ([#605](https://github.com/getgrit/gritql/pull/605))

## [0.5.1](https://github.com/getgrit/gritql/compare/grit-pattern-matcher-v0.5.0...grit-pattern-matcher-v0.5.1) - 2025-01-07

### Other

- use `&Path` in `FileRegistry::new_from_paths()` (#594)

## [0.5.0](https://github.com/getgrit/gritql/compare/grit-pattern-matcher-v0.4.0...grit-pattern-matcher-v0.5.0) - 2024-10-24

### Added

- call built-ins as predicate ([#557](https://github.com/getgrit/gritql/pull/557))
- optimize simple contains patterns ([#555](https://github.com/getgrit/gritql/pull/555))
- add a basic optimizer pass for contains ([#554](https://github.com/getgrit/gritql/pull/554))
- add within until ([#540](https://github.com/getgrit/gritql/pull/540))

## [0.4.0](https://github.com/getgrit/gritql/compare/grit-pattern-matcher-v0.3.0...grit-pattern-matcher-v0.4.0) - 2024-10-10

### Added

- basic builder API for SDK ([#518](https://github.com/getgrit/gritql/pull/518))
- SDK basics ([#514](https://github.com/getgrit/gritql/pull/514))
- truly lazy variables ([#512](https://github.com/getgrit/gritql/pull/512))
- add a callback pattern ([#476](https://github.com/getgrit/gritql/pull/476))
- expose insert_effect API ([#475](https://github.com/getgrit/gritql/pull/475))
- search for built-ins inside functions ([#430](https://github.com/getgrit/gritql/pull/430))
- skip warning for searches ([#428](https://github.com/getgrit/gritql/pull/428))
- add some more utils for napi ([#405](https://github.com/getgrit/gritql/pull/405))

### Fixed

- make compound snippets work with stateless compiler ([#513](https://github.com/getgrit/gritql/pull/513))
- simplify the lazy approach ([#483](https://github.com/getgrit/gritql/pull/483))
- remove min_level spread throughout the codebase ([#451](https://github.com/getgrit/gritql/pull/451))
- include universal patterns in bindings ([#431](https://github.com/getgrit/gritql/pull/431))

### Other

- remove `im` crate ([#536](https://github.com/getgrit/gritql/pull/536))
- make .index() and .scope() use accessors ([#480](https://github.com/getgrit/gritql/pull/480))
- searching for new patterns inside callback function ([#481](https://github.com/getgrit/gritql/pull/481))
- install workflow-runner via axo ([#461](https://github.com/getgrit/gritql/pull/461))
- refactor grit error ([#457](https://github.com/getgrit/gritql/pull/457))
