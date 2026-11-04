![gql-banner](https://gitlab.com/-/project/87292979/uploads/c3b5caa6be0e6bed95b88b02ae18bea4/gql-rs-banner.jpg)
# gql-rs

`gql-rs` is a Rust parser, typed AST builder, and semantic validator for the
Graph Query Language (GQL).

> **Project status:** `gql-rs` is currently an alpha release. The public API
> and AST representation may change before `1.0`.

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

`gql-rs` parses and validates GQL programs. It does not execute queries or
connect to a graph database.

## Installation

Add the crate to your project:

```toml
[dependencies]
gql-rs = "0.0.1-alpha"
```

The Cargo package is named `gql-rs` and its Rust library is imported as
`gql_rs`.

## Quick start

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

fn main() {
    let query =
        "MATCH (a:Actor)-[:ACTED_IN]->(m:Movie) RETURN a.name, m.title";

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

let mut analyzer = DefaultGQLQueryAnalyzer::new();
let ast = analyzer.exec("RETURN 42 AS answer")?;

assert_eq!(analyzer.get_query(), Some("RETURN 42 AS answer"));
assert!(analyzer.get_ast().is_some());

# Ok::<(), gql_rs::GqlError>(())
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

let mut analyzer = DefaultGQLQueryAnalyzer::new();

match analyzer.exec("MATCH (") {
    Err(GqlError::Parse { message, context }) => {
        eprintln!("{message}");
        eprintln!("Context: {context:?}");
    }
    Err(other) => eprintln!("{other}"),
    Ok(_) => unreachable!(),
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

The main entry points are:

- `DefaultGQLQueryAnalyzer`;
- `GQLQueryAnalyzer`;
- `QueryAST`;
- `GqlError`;
- `GqlResult<T>`.

The complete typed AST is re-exported from the crate root and is also available
through the public `ast` module.

## Documentation

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

See [CONTRIBUTING.md](CONTRIBUTING.md) for development requirements, test
organization, parser regeneration instructions, and the individual test suite
commands.

The project uses the Rust 2024 edition and requires Rust 1.97 or newer.

## License

Licensed under the
[European Union Public Licence 1.2](LICENSE.txt).
