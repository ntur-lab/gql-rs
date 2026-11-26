# Changelog

All notable changes to this project will be documented in this file.

The format follows [Keep a Changelog](https://keepachangelog.com/en/1.1.0/)
and the project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.0.5] - 2026-11-26

### Added
- **128-bit integer literals (`Literal::BigInteger(i128)`):** Support for integer literals outside the signed 64-bit range up to signed 128-bit limits (`i128::MIN` to `i128::MAX`) across decimal, hexadecimal (`0x`), octal (`0o`), and binary (`0b`) notations (#39).
  - Literals within the `i64` range continue to use `Literal::Integer(i64)` to preserve AST and JSON compatibility.
  - Unary minus signs immediately preceding the exact magnitude of `i128::MIN` are folded directly into the literal.
  - `Literal::BigInteger` serializes as a decimal string to remain compatible with `serde_json::Value` and standard JSON consumers, while deserializing from both strings and JSON numbers.

### Fixed
- **Leading whitespace and comments:** Skip leading hidden-channel tokens (spaces, tabs, newlines, and `//`, `--`, `/* */` comments) before parsing so queries with leading trivia no longer fail with `extraneous input` while preserving original source line/column spans (#38).
- **Empty or trivia-only queries:** Reject inputs containing no GQL statements (empty, whitespace-only, or comment-only) prior to parsing with a clear `Empty query` parse error instead of triggering an `antlr-rust` error-recovery panic (#38).

### Changed
- **Breaking (AST):** Added the `Literal::BigInteger(i128)` variant to `gql_rs::ast::model::Literal`; downstream consumers matching exhaustively on `Literal` must handle this variant (#39).
- **Error message:** Empty query inputs (`""`) now return `Parse error: Empty query: the input contains no GQL statement` instead of `mismatched input '<EOF>'` (#38).

## [0.0.4]

### Added

- **Declared value types (`ValueType` and `FieldType`):** Introduced `ValueType { base, not_null }` and `FieldType` to model declared types in `CAST`, `IS [NOT] TYPED`, `WHEN ... IS TYPED`, record and binding-table fields, list elements, catalog property declarations, and typed `LET VALUE` definitions, while keeping bare `TypeSpec` for expression type inference.
- **Expanded `TypeSpec` representation:**
  - Signed and unsigned integer widths via `IntegerSize` (`Bits8`, `Bits16`, `Bits32`, `Bits64`, `Bits128`, `Bits256`, `Small`, `Regular { precision }`, `Big`).
  - Floating-point variants via `FloatSize` (`Bits16`, `Bits32`, `Bits64`, `Bits128`, `Bits256`, `Regular { precision, scale }`, `Real`, `Double`).
  - `u32` precision and scale on `TypeSpec::Decimal`.
  - `min_length` and `max_length` constraints on `TypeSpec::String` and `TypeSpec::Bytes` (mapping fixed-length `CHAR(n)` / `BINARY(n)` to `min = max = n`).
  - Open and closed reference types for `TypeSpec::Node`, `TypeSpec::Edge`, and `TypeSpec::Graph`, plus structural `fields` on `TypeSpec::Record` and `TypeSpec::BindingTable`.
  - `DurationQualifier` (`YearToMonth`, `DayToSecond`), `TypeSpec::Null`, `TypeSpec::Nothing`, `TypeSpec::PropertyValue`, and convenience constructors (`TypeSpec::integer()`, `float()`, `string()`, `bytes()`, `decimal()`, `duration()`, `record()`, `binding_table()`, `list_of()`).
- **Dedicated literal variants and numeric decoding:**
  - Added `Literal::Date`, `Literal::Time`, `Literal::Bytes(Vec<u8>)` (hex byte strings `X'...'`), and `Literal::Decimal(String)` (exact numeric literals with the `M` suffix, preserving normalized digits without precision loss).
  - Added decoding for hexadecimal (`0x`), octal (`0o`), and binary (`0b`) integers, underscore-separated numeric literals, `F`/`D` approximate float suffixes, and generalized `i64::MIN` magnitude detection across numeric expressions.
- **Centralized built-in function registry:** Added `src/ast/validator/builtins.rs` covering all 72 grammar built-in function forms across aggregate, numeric, string, byte-string, list, path, temporal, duration, graph-element, conditional, and session families, validating arity, `DISTINCT`/`ALL` modifiers, nested aggregates, structural argument constraints, and return-type inference while preserving user-defined functions and conservative `Any` behavior for unknown types.
- **Explicit delete modifier (`DeleteMode`):** Added `DeleteMode::{Unspecified, Detach, NoDetach}` to distinguish `DELETE`, `DETACH DELETE`, and `NODETACH DELETE` in standalone and pipeline statements.
- **Builder and validator diagnostics:**
  - Added `GqlError::INVALID_TYPE_PARAMETER` to reject invalid type parameters during AST construction (`min > max`, zero lengths or precision, `scale > precision`, out-of-range integer parameters, and duplicate field names in records or binding tables).
  - Added `GqlError::UNSUPPORTED_CONSTRUCT` to fail explicitly on syntactically valid constructs not yet representable in the `0.0.4` AST (such as chained property access `n.a.b` or catalog/home/current graph and table references) instead of falling back to string literals.
  - Enforced nullability and type compatibility rules for `CAST` (rejecting `CAST(NULL AS T NOT NULL)`), `NULL`/`NOTHING`/`PROPERTY VALUE` targets, and `LET VALUE x :: T = ...` initializers (including type propagation inside inline `LET ... IN ... END`).
- **Documentation:** Added `docs/builtin_functions.md` and updated the AST, builder, and validator guides to document all built-in signatures, validation limitations, and explicitly unsupported constructs.

### Changed

- **Structural CST construction:** Replaced text-based fallbacks (`build_expression_from_leaf_text`, `build_literal_from_leaf_text`, `normalize_type_token`, `extract_parenthesized_u8_list`) with structural CST mapping across expressions (`src/ast/builder/expression.rs`), clauses, statements, and declared value types (`src/ast/builder/types.rs`).
- **AST node signatures (breaking):**
  - `CastExpression::target_type`, `IsPredicateKind::Typed`, `WhenOperandKind::ValueTypePredicate`, `PropertyTypeDecl::type_spec`, `PropertyTypeSpecification::parsed_value_type`, and `TypeSpec::List::element_type` now use `ValueType` instead of `TypeSpec`.
  - `VariableDefinition` now includes `declared_type: Option<ValueType>`.
  - `DeleteClause` now uses `mode: DeleteMode` instead of a boolean `detach` flag.
  - `UNKNOWN` now builds into `Literal::Null`, `SESSION_USER` into a niladic built-in `FunctionCall`, and `GRAPH $$g` / `TABLE $$t` into `Literal::Parameter`.
- **Built-in validation coverage:** Upgraded `BYTE_LENGTH` and `OCTET_LENGTH` from `Partial` to `Complete` validation and normalized built-in, aggregate, and keyword modifier names (`NFC`/`NFD`/`NFKC`/`NFKD`, `YEAR TO MONTH`/`DAY TO SECOND`) to canonical uppercase.

### Fixed

- Prevented expressions inside function arguments, `CALL` arguments, `LET` definitions, `ORDER BY`/`RETURN` items, and `SET`/`REMOVE` clauses from silently degrading into `Literal::String` or raw text and bypassing variable or type validation (e.g., `FLOOR(x + 1)`, `FLOOR(n.price * 1.2)`, `PERCENTILE_CONT(DISTINCT n.score, 0.5)`, `LOCAL_TIME('12:30:00.5')`).
- Validated named `CALL` arguments even when the outer variable scope is empty, catching unbound variables in standalone `CALL` statements.
- Rejected all grammar aggregate functions (including `STDDEV_*`, `PERCENTILE_*`, and lowercase names) inside `WHERE` clauses.
- Fixed parsing of verbose `SIGNED`/`UNSIGNED` integer declarations (`SIGNED INTEGER8`, `UNSIGNED INTEGER`), `DURATION_BETWEEN(a, b) YEAR TO MONTH`, non-literal length arguments in `LEFT`/`RIGHT`, and nested aggregate detection for `STDDEV_*` and `PERCENTILE_*`.
- Propagated errors from invalid catalog property types into `CatalogStatement::Raw.parse_error` instead of silently discarding them.
- Fixed UTC offset detection on `TIME` and `DATETIME` literals containing surrounding whitespace.

### Removed

- Removed unused `TypeSpec::Time`, `TypeSpec::Timestamp`, and `TypeSpec::Reference` variants.

## [0.0.3] - 2026-11-17

### Added

- Explicit `ReturnProjection` AST representation for wildcard and item-based
  projections.
- Support for `RETURN *`, `RETURN ALL *`, and `RETURN DISTINCT *`.
- AST fixtures and integration coverage for `MATCH ... INSERT`, wildcard
  projections, and activities followed by `SESSION CLOSE`.

### Changed

- `ReturnClause` now stores a `projection: ReturnProjection` instead of an
  `items: Vec<ReturnItem>` field. The new `items()` helper returns explicit
  projection items or `None` for wildcard projections.
- `MATCH ... INSERT` statements are represented as an ordered query and data
  modification sequence, preserving matched bindings, graph context, and
  terminal result clauses.

### Fixed

- `MATCH ... INSERT` no longer fails during AST construction and can reuse
  variables bound by the preceding `MATCH`.
- Terminal results following `MATCH ... INSERT` are preserved.
- Program activities preceding `SESSION CLOSE` are no longer discarded.
- Transaction intent and source order are preserved when an activity is
  followed by `SESSION CLOSE`.
- The combined activity and `SESSION CLOSE` source span now ends at the close
  command.
- Wildcard projections from nested queries are propagated into the enclosing
  scope.
- Wildcard projections are handled correctly during grouping and set-operation
  validation.

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
