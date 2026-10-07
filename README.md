# gql-parser

`gql-parser` is a Rust crate for parsing and semantic validation of GQL programs.

The crate transforms GQL text into a typed AST, using ANTLR for lexical and syntactic
analysis and a dedicated validation phase for the semantic rules implemented by the crate.

## Goal

Provide a clear and reusable pipeline:

1. GQL textual query
2. CST (ANTLR)
3. AST (`QueryAST`)
4. Semantic validation (`validate_query_ast`)

This separation makes the crate suitable for both query execution and static analysis, tooling, and tests.

## Crate Architecture

- `resources/GQL.g4`
  - Source GQL grammar.
- `src/antlr/`
  - ANTLR-generated lexer/parser.
- `src/ast/`
  - AST model (`model/`).
  - CST -> AST builder modules (`builder/`).
  - Semantic validator modules (`validator/`).
- `src/query_analyzer.rs`
  - High-level entry point with the `GQLQueryAnalyzer` trait and `DefaultGQLQueryAnalyzer` implementation.
- `src/error_handler/`
  - Error types (`GqlError`).
- `tests/`
  - Unit tests for AST model types.
  - Integration tests for parser, AST builder, validator, golden AST fixtures, and TCK.

## Public API

The following symbols are re-exported from the crate root:

- `QueryAST`
- `GQLQueryAnalyzer`
- `DefaultGQLQueryAnalyzer`
- `GqlError`
- `GqlResult<T>`

The full AST model is also available through the public `ast` module re-exports.

## Documentation

Detailed technical documentation is available in `docs/`:

- [ANTLR and Grammar Guide](docs/antlr_grammar.md)
- [Parser Generation Guide](docs/generating_parser.md)
- [AST Structural Guide](docs/gql_query_ast.md)
- [AST Builder Operational Guide](docs/gql_ast_builder.md)
- [Validator Operational Guide](docs/gql_validator.md)
- [Validator Rules Guide](docs/gql_validation_rules.md)
- [Query Analyzer Guide](docs/query_analyzer.md)
- [Test Suite Guide](docs/tests.md)

## Quick Example

```rust
use gql_parser::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

fn main() {
    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    let query = "MATCH (a:Actor)-[:ACTED_IN]->(m:Movie) RETURN a.name, m.title";

    match analyzer.exec(query) {
        Ok(ast) => {
            println!("Valid query");
            println!("AST: {:?}", ast);
        }
        Err(err) => {
            eprintln!("Error: {}", err);
        }
    }
}
```

## Build and Test

From the repository root:

```bash
cargo test
```

Run the three suites independently:

```bash
cargo test --test gql_tck
cargo test --lib --test unit
cargo test --test integration --test ast_conformance
```

To build only the crate:

```bash
cargo build
```

The grammar accepts keywords case-insensitively while preserving the original spelling of
identifiers and literal values. The implementation does not normalize the complete input.

## License

Licensed under the [European Union Public Licence 1.2](LICENSE.txt).
