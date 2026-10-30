# Supported GQL Features

`gql-rs` is under active development. The crate implements a growing subset of
GQL syntax and semantic validation; it does not currently claim complete
conformance with the full language standard.

This page is a high-level capability overview. The test suites are the source
of truth for verified behavior, while
[GQL Validator - Enforced Rules](gql_validation_rules.md) documents the
semantic checks implemented by the validator.

## Status legend

- **Covered**: representative syntax, AST construction, and validation behavior
  are exercised by the current test suites.
- **Partial**: useful forms are implemented, but some grammar alternatives are
  not mapped to dedicated AST nodes or do not have complete semantic checks.
- **Not provided**: the capability is intentionally outside this crate.

## Capability overview

| Area | Status | Current coverage |
| --- | --- | --- |
| Lexing and parsing | Covered | ANTLR-based parsing from GQL source to a concrete syntax tree. |
| Typed AST construction | Partial | Major query, data, pattern, session, transaction, and catalog structures are modeled. Some grammar alternatives remain unmapped. |
| Query pipelines | Covered | `MATCH`, `LET`, `FILTER`, `FOR`, `CALL`, graph selection, subqueries, result statements, ordering, grouping, and pagination. |
| Query composition | Covered | Chained statements, `NEXT`, and the tested focused and nested query forms. |
| Set operations | Covered | `UNION`, `INTERSECT`, and `EXCEPT`, including inferable result compatibility checks. |
| Graph patterns | Covered | Node and edge patterns, directions, labels, properties, path modes, searches, quantifiers, and tested simplified path forms. |
| Expressions | Partial | Literals, variables, properties, predicates, operators, arrays, maps, `CASE`, casts, path constructors, and the tested built-in functions. Some function and expression alternatives remain unmapped. |
| Type validation | Partial | Deterministic incompatibilities are rejected when types can be inferred. Ambiguous values fall back to the conservative `Any` behavior. |
| Data modification | Covered | Tested `INSERT`, `SET`, `REMOVE`, and `DELETE` forms, including match-qualified variants. |
| Catalog and DDL | Partial | Common schema, graph, and graph-type statements have typed nodes. Other accepted catalog constructs may use `CatalogStatement::Raw`. |
| Session commands | Covered | Tested `SET`, `RESET`, `CLOSE`, and binding-related forms. |
| Transactions | Covered | Tested start, commit, rollback, and transaction access modes. |
| Procedure calls | Partial | Named calls, arguments, `YIELD`, inline calls, and tested query-pipeline forms are supported; not every grammar alternative is mapped. |
| AST serialization | Covered | Public AST nodes support Serde serialization and deserialization. |
| Transaction intent | Covered | Statements are classified as read-only, data-writing, catalog-writing, or both. |
| Query execution | Not provided | The crate does not execute GQL or connect to a graph database. |

## Important compatibility notes

### Grammar acceptance and typed AST coverage differ

The grammar can recognize syntax for which the builder does not yet provide a
dedicated typed representation. Depending on the construct, analysis either
returns a structured error or preserves an accepted catalog statement as
`CatalogStatement::Raw`.

### Semantic validation is intentionally conservative

The validator rejects deterministic semantic incompatibilities. When a type
cannot be inferred reliably, it uses `Any` and avoids reporting errors that
could be false positives without runtime schema or catalog information.

### Alpha API stability

The public API, AST variants, serialized representation, and validation rules
may change before version `1.0`. Consumers should pin an exact alpha version if
they depend on the serialized AST shape.

## Verification

Verified behavior is divided across:

- unit tests for AST and error contracts;
- integration tests for parser, builder, and validator interaction;
- golden JSON fixtures for complete AST conformance;
- portable Gherkin scenarios in the GQL Technology Compatibility Kit.

See the [test suite guide](tests.md) for the commands and contribution rules
for each layer.
