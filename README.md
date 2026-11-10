<div align="center">

# gql-rs

**A typed parser, AST builder, and semantic validator for the Graph Query Language (GQL) in Rust.**

[![Crates.io](https://img.shields.io/crates/v/gql-rs.svg?style=flat-square)](https://crates.io/crates/gql-rs)
[![Documentation](https://img.shields.io/docsrs/gql-rs?style=flat-square)](https://docs.rs/gql-rs)
[![ISO/IEC 39075:2024](https://img.shields.io/badge/Standard-ISO%2FIEC%2039075-005B94?style=flat-square)](https://www.iso.org/standard/76120.html)
[![License: EUPL v1.2](https://img.shields.io/badge/License-EUPL_v1.2-blue.svg?style=flat-square)](LICENSE.txt)
[![Rust Version](https://img.shields.io/badge/Rust-1.90%2B-orange?style=flat-square&logo=rust)](https://www.rust-lang.org)

</div>

---


## Features

- GQL lexical and syntactic analysis based on ANTLR.
- Conversion from the ANTLR concrete syntax tree to a typed Rust AST.
- Semantic validation of variables, expressions, types, graph patterns,
  queries, catalog statements, and data-modifying statements.
- Source spans for AST nodes and validation errors.
- Serde support for AST serialization and deserialization.
- Transaction intent classification.
- Case-insensitive GQL keywords with identifier spelling preservation.
- Portable behavior coverage through the GQL Technology Compatibility Kit.

`gql-rs` parses and validates GQL queries. It does not execute queries or
connect to a graph database.

## Installation

Add the crate to your project:

```toml
[dependencies]
gql-rs = "0.0.1"
```

The Cargo package is named `gql-rs` and its Rust library is imported as
`gql_rs`.

## Quick start

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

fn main() {
    let query = "MATCH (p:Person)-[:WORKS_FOR]->(c:Company) WHERE c.name = 'Ntur Lab' RETURN p.name";

    let mut analyzer = DefaultGQLQueryAnalyzer::new();

    match analyzer.exec(query) {
        Ok(ast) => {
            println!("Valid GQL program");
            println!("AST: {ast:#?}");
            println!("Transaction intent: {:?}", ast.transaction_intent());
        }
        Err(error) => {
            eprintln!("{error}");
        }
    }
}
```

`exec` performs the complete analysis pipeline:

```text
GQL source -> lexer/parser -> CST -> typed AST -> semantic validation
```

## Analyzer state

`DefaultGQLQueryAnalyzer` stores the most recent successful query and AST:

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

fn main() {
    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    let ast = analyzer.exec("RETURN 42 AS answer")?;

    assert_eq!(analyzer.get_query(), Some("RETURN 42 AS answer"));
    assert!(analyzer.get_ast().is_some());
}
```

The analyzer clears its stored state before every execution. A failed analysis
therefore does not expose the result of an earlier successful query.

## Errors

Analysis can return:

- `GqlError::Parse` for invalid syntax or CST construction failures;
- `GqlError::Validation` for semantic errors;
- `GqlError::Generic` for crate-level fallback errors.

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

fn main() {
    let mut analyzer = DefaultGQLQueryAnalyzer::new();

    match analyzer.exec("MATCH (") {
        Err(GqlError::Parse { message, context }) => {
            eprintln!("{message}");
            eprintln!("Context: {context:?}");
        }
        Err(other) => eprintln!("{other}"),
        Ok(_) => unreachable!(),
    }
}
```

Parser diagnostics currently include line and column information in their
message. Structured context is available through `GqlError::context()` when
the originating parser or validator can provide it.

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

- `GQLQueryAnalyzer`: Trait for analyzing GQL queries and converting them to Abstract Syntax Trees (AST). DefaultGQLQueryAnalier is the default implementation.
- `QueryAST`: Root AST node for a parsed GQL program.
- `GqlError`: Represents an error that can occur during parsing and semantic analysis.
- `GqlResult<T>`: Result alias used across parser modules.

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
