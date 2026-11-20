<div align="center">

# gql-rs

**A typed parser, AST builder, and semantic validator for the Graph Query Language (GQL) in Rust.**

[![Crates.io](https://img.shields.io/crates/v/gql-rs.svg?style=flat-square)](https://crates.io/crates/gql-rs)
[![Documentation](https://img.shields.io/docsrs/gql-rs?style=flat-square)](https://docs.rs/gql-rs)
[![Quality gate status](https://sonarcloud.io/api/project_badges/measure?project=ntur-lab_gql-rs&metric=alert_status)](https://sonarcloud.io/summary/new_code?id=ntur-lab_gql-rs)
[![ISO/IEC 39075:2024](https://img.shields.io/badge/Standard-ISO%2FIEC%2039075-005B94?style=flat-square)](https://www.iso.org/standard/76120.html)
[![License: EUPL v1.2](https://img.shields.io/badge/License-EUPL_v1.2-blue.svg?style=flat-square)](LICENSE.txt)
[![Rust Version](https://img.shields.io/badge/Rust-1.99%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)

</div>

---

`gql-rs` parses and validates GQL queries. It does not execute queries or connect to a graph database.

## Features

- **Lexical & syntactic analysis:** ANTLR-based GQL parser with case-insensitive keywords, preserved identifier spelling, and structured error positions.
- **Typed AST:** Structural conversion from the ANTLR concrete syntax tree into a typed Rust AST covering queries, graph patterns, expressions, declared value types, DML, and catalog/session statements.
- **Semantic validation:** Static checks for variable scopes, expressions, type compatibility, built-in function signatures, graph patterns, and query clauses.
- **Stateless & stateful APIs:** One-shot parsing/analysis via `parse()` and `analyze()`, or stateful session tracking via `DefaultGQLQueryAnalyzer`.
- **Diagnostics & serialization:** Source spans on AST nodes and errors, optional diagnostic codes, transaction intent classification, and full Serde support.
- **Compatibility:** Verified against the GQL Technology Compatibility Kit (TCK).

## Installation

Add the crate to your `Cargo.toml`:

```toml
[dependencies]
gql-rs = "0.0.4"
```

The Cargo package is named `gql-rs` and its Rust library is imported as `gql_rs`.

## Usage

Choose the entry point that matches your workflow:

| Entry point | Builds typed AST | Validates semantics | Retains query & AST copy |
| :--- | :---: | :---: | :---: |
| `parse(query)` | Yes | No | No |
| `analyze(query)` | Yes | Yes | No |
| `DefaultGQLQueryAnalyzer::exec(query)` | Yes | Yes | Yes |

### Stateless analysis

Use `analyze()` to parse a query, build its typed AST, and validate its semantics in one step:

```rust
use gql_rs::{analyze, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = analyze("MATCH (p:Person)-[:WORKS_FOR]->(c:Company) WHERE c.name = 'Ntur Lab' RETURN p.name")?;

    println!("AST: {ast:#?}");
    println!("Transaction intent: {:?}", ast.transaction_intent());

    Ok(())
}
```

If you need to inspect or serialize the AST before validating it, use `parse()` and call `ast.validate()` separately:

```rust
use gql_rs::{parse, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = parse("MATCH (c:Company) WHERE c.name = 'Ntur Lab' RETURN c")?;

    // Run semantic validation separately when needed.
    ast.validate()?;

    Ok(())
}
```

### Stateful analysis

Use `DefaultGQLQueryAnalyzer` when you want to retain the most recent successful query and AST:

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

fn main() -> Result<(), GqlError> {
    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    let ast = analyzer.exec("RETURN 42 AS value")?;

    assert_eq!(analyzer.get_query(), Some("RETURN 42 AS value"));
    assert!(analyzer.get_ast().is_some());

    println!("{ast:#?}");

    Ok(())
}
```

`exec()` clears any previously stored state, runs `analyze()`, and retains the query together with a clone of the AST on success. If analysis fails, both getters remain empty. Call `clear()` to reset the stored state explicitly.

## GQL support

`gql-rs` implements a growing subset of ISO/IEC 39075 GQL syntax and semantics, including:

- queries, set operations (`UNION`, `INTERSECT`, `EXCEPT`), and query composition (`NEXT`);
- graph patterns and path patterns;
- declared value types (`ValueType` / `TypeSpec`), expressions, predicates, and built-in functions;
- data modification (`INSERT`, `SET`, `REMOVE`, `DELETE`);
- catalog DDL, session commands, transactions, and procedure calls;
- ordering, grouping, and pagination.

Syntactically valid constructs without a dedicated AST representation either fail explicitly during AST construction (`GqlError::UNSUPPORTED_CONSTRUCT`) or, for unmapped catalog statements, fall back to `CatalogStatement::Raw`.

See [Supported features](docs/supported_features.md), [Built-in functions](docs/builtin_functions.md), and the [GQL validation rules](docs/gql_validation_rules.md) for detailed coverage.

## Errors

All public entry points return `GqlResult<QueryAST>`, with errors categorized into:

- `GqlError::Parse`: lexical, syntactic, or AST construction failures;
- `GqlError::Validation`: semantic validation failures;
- `GqlError::Generic`: other crate-level failures.

```rust
use gql_rs::{analyze, GqlError};

fn main() {
    match analyze("MATCH (") {
        Err(GqlError::Parse { message, context }) => {
            eprintln!("{message}");

            if let Some(span) = context.span {
                eprintln!(
                    "Error at line {}, column {}",
                    span.start_line, span.start_col
                );
            }
        }
        Err(other) => eprintln!("{other}"),
        Ok(_) => unreachable!(),
    }
}
```

Diagnostic details:

- **Spans:** Lines are 1-based and columns are 0-based Unicode character offsets (not UTF-8 byte offsets). Lexer/parser listener errors use zero-width spans (`start == end`), while `ErrorContext.span` is `None` when no source position is available.
- **Diagnostic codes:** `ErrorContext.code` is optional. It is `None` for listener-generated syntax errors, and populated for specific AST construction errors such as `GqlError::UNSUPPORTED_CONSTRUCT` and `GqlError::INVALID_TYPE_PARAMETER`.
- **Serialization:** Both `GqlError` and `ErrorContext` implement Serde `Serialize` and `Deserialize` (see the [query analyzer guide](docs/query_analyzer.md) for a JSON example).

## Public API

Core types and traits are re-exported at the crate root, while the full AST node hierarchy lives in `gql_rs::ast`:

- `parse` / `analyze`: stateless entry points.
- `GQLQueryAnalyzer` / `DefaultGQLQueryAnalyzer`: trait and default implementation for stateful analysis.
- `QueryAST`: root AST node representing a parsed GQL program.
- `GqlError` / `ErrorContext` / `GqlResult<T>`: error types and result alias used across the crate.

## Documentation

Full API documentation is published on [docs.rs](https://docs.rs/gql-rs). Additional guides and specifications are available in `docs/`:

**Architecture & Guides**
- [Query analyzer guide](docs/query_analyzer.md)
- [AST structural guide](docs/gql_query_ast.md)
- [AST builder guide](docs/gql_ast_builder.md)
- [Validator guide](docs/gql_validator.md)
- [ANTLR and grammar guide](docs/antlr_grammar.md)
- [Parser generation guide](docs/generating_parser.md)
- [Test suite guide](docs/tests.md)

**Language & Feature Reference**
- [Supported features](docs/supported_features.md)
- [Validation rules](docs/gql_validation_rules.md)
- [Built-in functions](docs/builtin_functions.md)

## Development & Roadmap

The project uses the Rust 2024 edition and requires **Rust 1.99 or newer**. Run the complete test suite with:

```bash
cargo test
```

**Next steps:** One of the main pain points is that the current dependency `antlr-rust:0.3.0-beta` has not been updated since 2022; for future releases, the team is evaluating replacing it with an official [ANTLR4](https://github.com/antlr/antlr4/blob/master/doc/cpp-target.md) target. Other planned language and semantic features are tracked in [Supported features](docs/supported_features.md#capability-overview).

## About NTUR LAB

`gql-rs` is maintained and developed by [NTUR LAB](https://ntur-lab.com) as part of **Aurea**, an initiative to build a modern, high-performance, open data ecosystem rooted in European open standards and digital sovereignty for the global developer community.

## Contributing & CLA

We welcome contributions and adhere to the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct. Please read [CONTRIBUTING.md](CONTRIBUTING.md) and [CODE_OF_CONDUCT.md](CODE-OF-CONDUCT.md) before submitting a pull request, and report any unacceptable behavior to [code-of-conduct@ntur-lab.com](mailto:code-of-conduct@ntur-lab.com).

To accept your contribution, you must agree to our [Contribution License Agreement (CLA)](https://github.com/ntur-lab/cla/blob/master/CONTRIBUTION-LICENSE-AGREEMENT.pdf) (also available in [Markdown](CONTRIBUTION-LICENSE-AGREEMENT.md)). You can confirm it via the automated CLA check on your pull request, or sign and send the PDF to [cla@ntur-lab.com](mailto:cla@ntur-lab.com).

## License

This repository and custom texts are licensed under the **European Union Public Licence v. 1.2** ([LICENSE.txt](LICENSE.txt)).

Individual governance documents incorporate or adapt third-party materials under their respective licenses: Contributor Covenant (CC BY 4.0), Harmony Agreements (CC BY 3.0), and Spring Data Contribution Guidelines (Apache 2.0).