# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

No unreleased changes are currently documented.

## [0.0.1-alpha] - 2026-10-30

Initial alpha release of `gql-rs`.

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

- The public API and AST representation may change during the alpha series.
- GQL support is incomplete and does not imply full conformance with the
  complete language standard.
- Some catalog constructs are preserved as `CatalogStatement::Raw` when a
  dedicated typed AST representation is not available.
- The crate parses and validates GQL but does not execute queries or connect to
  a graph database.
- A minimum supported Rust version has not yet been established.
