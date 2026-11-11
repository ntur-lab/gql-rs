# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.2] - 2026-11-11

### Added

- Public `parse()` function for typed AST construction without semantic validation.
- Public `analyze()` function for stateless parsing and semantic validation.
- Crate-root re-export of `ErrorContext`.
- Serde serialization and deserialization for `GqlError` and `ErrorContext`.
- Regression coverage for stateless APIs, diagnostic positions, lexical
  errors, Unicode columns, and analyzer state after failure.

### Changed

- `DefaultGQLQueryAnalyzer::exec()` delegates analysis to `analyze()` while
  preserving its stateful behavior.
- Validation error constructors without a known source position now use
  `ErrorContext.span = None` instead of a default zero-valued span.
- Documentation describes the stateless APIs, diagnostic position conventions,
  and error serialization.

### Fixed

- Lexer errors are collected alongside parser errors and propagated before
  AST construction.
- Listener-generated diagnostics include structured point spans, including
  at end of input.
- Corrected the README analyzer-state example to return a `Result`.
- Aligned the README Rust version badge with the package manifest.

## [0.0.1-ga] - 2026-11-10

First release of `gql-rs`.

### Added

- ANTLR-based lexical and syntactic analysis for GQL programs.
- Conversion from the generated concrete syntax tree to a typed Rust AST.
- Semantic validation for query pipelines, graph patterns, expressions,
  data-modifying statements, catalog statements, sessions, and transactions.
- Structured parse and validation errors through `GqlError`.
- Serde serialization and deserialization for the public AST model.
- Transaction intent classification for analyzed statements.
- Unit, integration, golden AST, and portable TCK test suites.
- Contributor, parser generation, AST, validator, and test documentation.

### Known limitations

- GQL support is almost completed, see [here](docs/supported_features.md#capability-overview).
- Some catalog constructs are preserved as `CatalogStatement::Raw` when a
  dedicated typed AST representation is not available.
- The crate parses and validates GQL but does not execute queries or connect to
  a graph database.
