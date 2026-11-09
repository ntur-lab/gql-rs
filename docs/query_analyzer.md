# Query Analyzer Guide

This document explains how `src/query_analyzer.rs` works and how to use it in
`gql-rs`.

## Purpose

The query analyzer provides a high-level API to convert a GQL query string into a validated AST.

It orchestrates three steps:

1. Lexing and parsing via ANTLR.
2. CST to AST conversion.
3. Semantic validation.

Main implementation file:

- src/query_analyzer.rs

## Core Types

### GQLQueryAnalyzer trait

The trait defines the analyzer contract:

- exec(query): parses and validates a query, returning QueryAST.
- get_query(): returns the last analyzed query text.
- get_ast(): returns the last generated AST.

This abstraction allows alternative analyzer implementations if needed.

### DefaultGQLQueryAnalyzer

Default stateful implementation with two cached fields:

- query: Option<String>
- ast: Option<QueryAST>

It stores the most recent successful analysis result.

## Execution Flow of exec

DefaultGQLQueryAnalyzer::exec follows this sequence:

1. clear()

- Resets internal state to avoid stale query/AST data.

2. Build ANTLR input pipeline

- InputStream from query text.
- GQLLexer over input.
- CommonTokenStream over lexer.
- GQLParser over token stream.
- Removes default parser listeners and attaches GqlErrorListener.

3. Parse entry rule

- Calls parser.gqlProgram().
- If the custom error listener collected syntax errors, returns the first Parse error.
- If parser returns an ANTLR error, maps it to GqlError::Parse.

4. Build AST

- Calls QueryAST::build(cst_root).
- Delegates CST to AST conversion to the internal AST builder layer.

5. Validate semantics

- Calls ast.validate().
- Fails with GqlError::Validation when semantic rules are violated.

6. Cache successful result

- Stores query and ast in analyzer state.
- Returns the AST.

## Error Model

The analyzer exposes errors through GqlResult<QueryAST>.

Typical categories:

- Parse errors
    - Syntax errors collected by GqlErrorListener.
    - ANTLR parsing failure mapped from parser result.
- Validation errors
    - Semantic inconsistencies detected by the AST validator.
- Generic errors
    - Reserved for other crate-level failures (if produced by deeper layers).

## Why a custom error listener is used

GqlErrorListener captures syntax errors with line/column context and lets the analyzer return structured Parse errors
consistently.

Practical effects:

- Explicit syntax diagnostics from ANTLR callbacks.
- Deterministic error propagation through Result.
- Clear separation between syntax failures and semantic validation failures.

## State Management Notes

The analyzer is intentionally stateful:

- get_query and get_ast expose only the latest successful run.
- `clear` is called at the start of `exec`, so failed runs do not keep stale success state.

If you need immutable/stateless behavior, create a fresh analyzer per request.

## Typical Usage

Example pattern:

1. Create DefaultGQLQueryAnalyzer.
2. Call exec with query text.
3. Use returned AST or inspect stored state with get_query/get_ast.

This is the recommended entry point for application code that does not need direct CST handling.

## Integration with Other Layers

query_analyzer.rs sits between parser infrastructure and semantic tooling:

- Grammar and generated parser: resources/GQL.g4 and src/antlr/
- AST model facade: src/ast/model/query_ast.rs
- CST to AST builder modules: src/ast/builder/
- Semantic validator modules: src/ast/validator/

In short, it is the facade layer for parse + build + validate.
