<div align="center">

# gql-rs

**A typed parser, AST builder, and semantic validator for the Graph Query Language (GQL) in Rust.**

[![Crates.io](https://img.shields.io/crates/v/gql-rs.svg?style=flat-square)](https://crates.io/crates/gql-rs)
[![Documentation](https://img.shields.io/docsrs/gql-rs?style=flat-square)](https://docs.rs/gql-rs)
[![ISO/IEC 39075:2024](https://img.shields.io/badge/Standard-ISO%2FIEC%2039075-005B94?style=flat-square)](https://www.iso.org/standard/76120.html)
[![License: EUPL v1.2](https://img.shields.io/badge/License-EUPL_v1.2-blue.svg?style=flat-square)](LICENSE.txt)
[![Rust Version](https://img.shields.io/badge/Rust-1.99%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)

</div>

---


## Features

- GQL lexical and syntactic analysis based on ANTLR.
- Conversion from the ANTLR concrete syntax tree to a typed Rust AST.
- Semantic validation of variables, expressions, types, graph patterns,
  queries, catalog statements, and data-modifying statements.
- Stateless parsing and analysis through `parse()` and `analyze()`.
- Stateful analysis through `DefaultGQLQueryAnalyzer`.
- Structured source positions for lexer and parser diagnostics.
- Source spans for AST nodes and semantic errors when available.
- Serde serialization and deserialization for AST nodes and error types.
- Transaction intent classification.
- Case-insensitive GQL keywords with identifier spelling preservation.
- Portable behavior coverage through the GQL Technology Compatibility Kit.

`gql-rs` parses and validates GQL queries. It does not execute queries or
connect to a graph database.

## Installation

Add the crate to your project:

```toml
[dependencies]
gql-rs = "0.0.1-ga"
```

The Cargo package is named `gql-rs` and its Rust library is imported as
`gql_rs`.

## Quick start

Use `analyze()` to parse a query, build its typed AST, and validate its semantics:

```rust
use gql_rs::{analyze, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = analyze("MATCH (p:Person)-[:WORKS_FOR]->(c:Company) WHERE c.name = 'Ntur Lab' RETURN p.name")?;

    println!("AST: {ast:#?}");
    println!("Transaction intent: {:?}", ast.transaction_intent());

    Ok(())
}
```

Choose the entry point according to the required behavior:

| Entry point | Builds a typed AST | Validates semantics | Retains query and an AST copy |
|---|---|---|---|
| `parse(query)` | Yes | No | No |
| `analyze(query)` | Yes | Yes | No |
| `DefaultGQLQueryAnalyzer::exec(query)` | Yes | Yes | Yes |

`parse()` performs lexical analysis, syntactic analysis, and CST-to-AST conversion. It does not run the semantic validator:

```rust
use gql_rs::{parse, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = parse("MATCH (c:Company) WHERE c.name = 'Ntur Lab' RETURN c")?;

    // Run semantic validation separately when needed.
    ast.validate()?;

    Ok(())
}
```

Successful parsing does not imply semantic validity or complete support for every construct accepted by the grammar. AST construction may still reject constructs that do not have a supported representation.

`analyze()` calls `parse()` followed by `ast.validate()`. Neither stateless entry point retains the source query or clones the resulting AST.

## Analyzer state

Use `DefaultGQLQueryAnalyzer` when the most recent successful query and AST
must remain accessible:

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

`exec()` clears the stored state, calls `analyze()`, and stores the query and
a clone of the successful AST before returning the original AST.

A failed analysis leaves both getters empty, even if an earlier call succeeded.
`clear()` explicitly removes the stored query and AST.

Use `analyze()` when retaining this state is unnecessary.

## Errors

The public entry points return `GqlResult<QueryAST>`. 

Errors are represented by:

- `GqlError::Parse`: lexical, syntactic, or AST construction failures;
- `GqlError::Validation`: semantic validation failures;
- `GqlError::Generic`: other crate-level failures.

Lexer and parser listeners share an error collection. After the parser returns, the first collected diagnostic is returned before AST construction, even if ANTLR recovered and produced a CST.

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

Listener-generated diagnostics use zero-width spans: the start and end positions are identical. These identify an error location, including EOF, rather than the full extent of an offending token.

Lines are one-based and columns are zero-based. Listener columns count Unicode characters, not UTF-8 bytes; they are not display widths or UTF-16 offsets.

`ErrorContext.span` is `None` when the originating error constructor has no known position. `ErrorContext.code` is optional and is not populated by the lexer/parser listener.

`GqlError` and `ErrorContext` support Serde serialization and deserialization.

See the [query analyzer guide](docs/query_analyzer.md) for a JSON example.

## GQL support

The crate implements a growing subset of GQL syntax and semantics.

Currently covered areas include:

- queries and query composition;
- graph patterns and path patterns;
- expressions, predicates, functions, and type checks;
- data modification;
- catalog and session statements;
- transactions;
- procedure calls;
- set operations;
- ordering, grouping, and pagination.

Some syntactically valid constructs may not yet have a dedicated typed AST
representation. Catalog constructs without a specialized representation can
be preserved as `CatalogStatement::Raw`.

See the [GQL validation rules](docs/gql_validation_rules.md) and the
[test suite guide](docs/tests.md) for the currently verified behavior. A
capability overview is available in [Supported features](docs/supported_features.md).

## Public API

Main entry points are structured around dedicated traits and core types designed to handle the full lifecycle of an ISO GQL statement:
- `parse`: builds a typed AST without running semantic validation.
- `analyze`: builds and semantically validates a typed AST.
- `GQLQueryAnalyzer`: trait for stateful query analysis.
- `DefaultGQLQueryAnalyzer`: default stateful implementation.
- `QueryAST`: root AST node for a parsed GQL program.
- `GqlError`: error returned by parsing, AST construction, or validation.
- `ErrorContext`: optional source position and diagnostic code.
- `GqlResult<T>`: result alias used across the crate.

The complete typed AST is re-exported from the crate root and is also available
through the public `ast` module.

## Documentation
If you want to deep-dive on the technical details, please have a look at the following links:

- [ANTLR and grammar guide](docs/antlr_grammar.md)
- [Parser generation guide](docs/generating_parser.md)
- [AST structural guide](docs/gql_query_ast.md)
- [AST builder guide](docs/gql_ast_builder.md)
- [Validator guide](docs/gql_validator.md)
- [Validation rules](docs/gql_validation_rules.md)
- [Supported features](docs/supported_features.md)
- [Query analyzer guide](docs/query_analyzer.md)
- [Test suite guide](docs/tests.md)

[API documentation](https://docs.rs/gql-rs) is published on docs.rs.

## Development

Run the complete test suite:

```bash
cargo test
```

The project uses the Rust 2024 edition and requires Rust 1.99 or newer.

## Next Steps
One of the major pain-point is that the current dependency `antlr-rust:0.3.0-beta` it has not been updated since 2022; therefore, for future releases, the team is considering replacing it with an official implementation of [ANTLR](https://github.com/antlr/antlr4/blob/master/doc/cpp-target.md).

Others missing and partially covered features are listed [here](docs/supported_features.md#capability-overview).

## About NTUR LAB

`gql-rs` is maintained and developed by [NTUR LAB](https://ntur-lab.com).

This component is part of **Aurea**, an initiative born out of Europe to build a modern, high-performance, and open data
ecosystem. While firmly rooted in European open standards and the ethos of digital sovereignty, our technologies are
built from the ground up for the global developer community. We create open-source software meant to be adopted,
integrated, and scaled anywhere in the world.

## Contribution License Agreement

In our CLA repository we're hosting the [Contribution License Agreement (CLA)](https://github.com/ntur-lab/cla/blob/master/CONTRIBUTION-LICENSE-AGREEMENT.pdf) in PDF format; a version in Markdown format is also available [here](CONTRIBUTION-LICENSE-AGREEMENT.md).

This contributor agreement documents the rights granted by contributors to Us and because we want to protect Your right, we decided to adopt the [Harmony Contributor Agreement](https://www.harmonyagreements.org/).
Our CLA is indeed copy-pasted from it.

In order to accept your contribution, you'll have to confirm it via our automated CLA pull request verification.  
Alternatively you can [download](https://github.com/ntur-lab/cla/blob/master/CONTRIBUTION-LICENSE-AGREEMENT.pdf), sign and send it to [cla@ntur-lab.com](mailto:cla@ntur-lab.com).

> This is a legally binding document, so *please read it carefully before agreeing to it*.

The Agreement may cover one or more software projects managed by Us.

## How to Contribute

As owner, contributors and maintainers of NTUR LAB' projects, and in the interest of fostering an open and welcoming community, we pledge to respect all people who contribute through reporting issues, posting feature requests, updating documentation, submitting pull requests or patches, and other activities.

We are committed to making participation in this project a harassment-free experience for everyone, regardless of level of experience, gender, gender identity and expression, sexual orientation, disability, personal appearance, body size, race, ethnicity, age, religion, or nationality.

In that very sense, we adhere to the [Contributor Covenant](https://www.contributor-covenant.org/) code of conduct. By participating, you are expected to uphold this code.

> Please report unacceptable behavior to [code-of-conduct@ntur-lab.com](mailto:code-of-conduct@ntur-lab.com)

> If you want to contribute, please read carefully our contribution terms at [CODE_OF_CONDUCT.md](CODE-OF-CONDUCT.md) and [CONTRIBUTING.md](CONTRIBUTING.md)

## License

The repository and custom texts are made available under the European Union Public Licence v. 1.2. See the [LICENSE.txt](LICENSE.txt).

Individual governance documents incorporate or adapt third-party materials subject to their respective licenses: Contributor Covenant (CC BY 4.0), Harmony Agreements (CC BY 3.0), and Spring Data Contribution Guidelines (Apache 2.0)
