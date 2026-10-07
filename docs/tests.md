# Test suite guide

The repository separates tests by responsibility. Query acceptance, AST model behaviour,
and exact AST construction are different contracts and use different suites.

## Unit tests

Public model unit tests live under `tests/unit` and do not invoke the lexer or parser. They
cover AST model contracts, serialization, source spans, transaction intent, and error
values. Internal library tests also include the parser-rule coverage check.

```text
cargo test --lib --test unit
```

## Integration tests

Integration tests live under `tests/integration`. They exercise the public analyzer across
the lexer, parser, CST-to-AST builder, and semantic validator. Modules are grouped by public
language capability: catalog and session statements, expressions, graph patterns, query
composition, set operations, transactions, and structured syntax errors.

An integration test must assert a concrete AST field or variant, a structured error, or a
semantic outcome. Query acceptance by itself belongs to the TCK.

```text
cargo test --test integration
```

## Full AST conformance fixtures

The conformance runner lives at `tests/integration/fixtures/ast_conformance.rs`. Golden
fixtures live under `tests/integration/fixtures/ast`; each case pairs a `.gql` input with an
adjacent `.ast.json` containing the complete expected AST, including source spans.

```text
cargo test --test ast_conformance
```

After an intentional AST contract change, regenerate and then review every JSON difference:

```text
UPDATE_AST_FIXTURES=1 cargo test --test ast_conformance
```

A normal test run never modifies fixtures.

## Portable GQL TCK

The Gherkin scenarios under `tests/integration/tck/features` validate public GQL behaviour
without depending on Rust AST types. This allows the corpus to be adapted for other GQL
implementations.

```text
cargo test --test gql_tck
```

The Cucumber adapter provides typed step definitions and isolated scenario state. Scenarios
tagged `@pending` are excluded by the runner. The internal library test included by
`cargo test --lib --test unit` also walks the generated ANTLR syntax trees and checks the
reachable parser rules against the TCK corpus.

## Complete verification

Run everything, including doctests:

```text
cargo test
```

The three CI-equivalent commands are:

```text
cargo test --test gql_tck
cargo test --lib --test unit
cargo test --test integration --test ast_conformance
```

## Adding tests

Use the smallest appropriate layer:

1. Put portable syntax or semantic behaviour in the TCK.
2. Add an integration test when exact Rust AST fields or typed errors matter.
3. Add a golden fixture when the complete AST is the contract.
4. Add a unit test for behaviour implemented directly by a model or error type.

Do not add duplicate acceptance-only Rust tests; the TCK is canonical for those cases.
