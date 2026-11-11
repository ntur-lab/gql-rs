# Query Analyzer Guide

The public entry points in `src/query_analyzer.rs` provide stateless parsing, stateless semantic analysis, and stateful analysis.

All entry points are re-exported from the crate root.

## Choosing an entry point

| API | Parsing and AST construction | Semantic validation | Retained state |
|---|---|---|---|
| `parse(query)` | Yes | No | None |
| `analyze(query)` | Yes | Yes | None |
| `DefaultGQLQueryAnalyzer::exec(query)` | Yes | Yes | Query and AST copy |

Use `analyze()` when the caller needs ownership of a validated AST without retaining another copy.
Use `parse()` when semantic validation must be deferred or when inspecting the AST before validation.
Use `DefaultGQLQueryAnalyzer` when the latest successful query and AST must remain accessible through getters.

## Stateless parsing

```rust
use gql_rs::{parse, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = parse("RETURN 42 AS answer")?;

    // Optional: validate the AST separately.
    ast.validate()?;

    Ok(())
}
```

`parse()` performs these steps:

1. Creates an ANTLR input stream and lexer.
2. Removes the default lexer listeners and attaches `GqlErrorListener`.
3. Creates the token stream and parser.
4. Replaces the parser listeners with a listener sharing the same collection.
5. Calls the `gqlProgram` entry rule.
6. Returns the first collected lexical or syntactic diagnostic, if any.
7. Maps an unsuccessful ANTLR parser result to `GqlError::Parse`.
8. Builds the typed AST through `QueryAST::build()`.

ANTLR may recover from an error and return a CST. Collected diagnostics are checked before AST construction so recovery does not turn invalid input into a successful result.

The returned error is the first collected diagnostic; the API does not return a list of all diagnostics.

`parse()` does not call the semantic validator. Grammar acceptance alone also does not guarantee successful AST construction: unsupported mappings may fail, while some catalog constructs may be preserved as `CatalogStatement::Raw`.

## Stateless analysis

```rust
use gql_rs::{analyze, GqlError};

fn main() -> Result<(), GqlError> {
    let ast = analyze("RETURN 42 AS answer")?;
    println!("{ast:#?}");
    Ok(())
}
```

`analyze()` calls `parse()`, then `ast.validate()`, and returns the resulting AST.

Neither stateless function retains the query or clones the resulting AST. Stateless analysis still performs the allocations needed by parsing and AST construction.

## Stateful analysis

`GQLQueryAnalyzer` defines three methods:

- `exec(query)`: analyzes a query and returns its AST.
- `get_query()`: exposes the stored query, if any.
- `get_ast()`: exposes the stored AST, if any.

`DefaultGQLQueryAnalyzer` implements this contract with `Option<String>` and`Option<QueryAST>` fields.

```rust
use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

fn main() -> Result<(), GqlError> {
    let mut analyzer = DefaultGQLQueryAnalyzer::new();
    let ast = analyzer.exec("RETURN 42 AS answer")?;

    assert_eq!(analyzer.get_query(), Some("RETURN 42 AS answer"));
    assert!(analyzer.get_ast().is_some());

    println!("{ast:#?}");

    analyzer.clear();
    assert!(analyzer.get_query().is_none());
    assert!(analyzer.get_ast().is_none());

    Ok(())
}
```

The default implementation:

1. Clears its previous state.
2. Calls `analyze(query)`.
3. Stores the query and a clone of the successful AST.
4. Returns the original AST.

On failure, the state remains empty. A later call can succeed normally.

## Error categories

All entry points return `GqlResult<QueryAST>`.

- `GqlError::Parse` covers lexical, syntactic, and AST construction failures.
- `GqlError::Validation` covers semantic validation failures.
- `GqlError::Generic` is available for other crate-level failures.

`parse()` does not run semantic validation. `analyze()` and the default stateful analyzer do.

Errors implement `Display` and `std::error::Error`. Applications can inspect the enum variants directly or use `message()` and `context()`.

## Diagnostic positions

Lexer and parser callbacks produce `GqlError::Parse` values with:

- a human-readable message containing the reported line and column;
- `context.span = Some(span)`;
- `context.code = None`.

These spans identify a point:

```text
start_line == end_line
start_col  == end_col
```

They do not describe the full offending token. EOF also uses a zero-width span.

Lines are one-based; columns are zero-based. Listener columns count Unicode characters rather than UTF-8 bytes. Consumers using UTF-16 offsets or display columns must convert positions using the original source.

Other errors may have a source range or no position. In particular,`GqlError::parse()`, `validation()`, `validation_with_code()`, and `generic()`create errors without a known position. The `_at()` constructors retain the span supplied by the caller.

Always inspect the `Option` rather than assuming every error has a span.

## Error serialization

`GqlError` and `ErrorContext` derive Serde `Serialize` and `Deserialize`. 
Applications using JSON must add `serde_json` to their own dependencies.

```rust
use gql_rs::GqlError;

fn main() -> Result<(), serde_json::Error> {
    let error =
        GqlError::validation_with_code("Unknown variable", "E_UNKNOWN");

    let json = serde_json::to_string_pretty(&error)?;
    let restored: GqlError = serde_json::from_str(&json)?;

    assert_eq!(restored, error);
    println!("{json}");

    Ok(())
}
```

Serialization preserves the error variant, message, and context. It does not perform query parsing or semantic validation. The serialized representation remains subject to the crate's evolving API.