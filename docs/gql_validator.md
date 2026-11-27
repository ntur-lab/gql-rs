# GQL Validator - Operational Guide

This document describes the semantic validator for the GQL AST.

Current implementation is modular:

- `src/ast/validator/mod.rs`
- `src/ast/validator/statements.rs`
- `src/ast/validator/data.rs`
- `src/ast/validator/query.rs`
- `src/ast/validator/expression.rs`

## Purpose

The validator checks semantic consistency of the AST after parsing and builder conversion.

Examples of checks:

- declared variables versus used variables;
- consistency of `RETURN`/`GROUP`/`ORDER`/`LIMIT` clauses;
- aggregation constraints (for example, aggregates in `WHERE`);
- consistency of Session/Transaction/Catalog commands;
- structural validity of patterns and data operations.

## Entry Point
Application code can call `QueryAST::validate()` on an existing AST.
Both `gql_rs::analyze(query)` and `DefaultGQLQueryAnalyzer::exec(query)`invoke validation automatically; `gql_rs::parse(query)` does not.

The internal validation entry point is:
- `validate_query_ast(ast)`

The validator dispatches on the program activity. A standalone session close or
transaction boundary carries no procedure body to validate. Session activities
validate SET and RESET commands in source order, including a bare RESET.

Each procedure body starts a local scope. Initializers are validated in declaration
order, before adding the new variable to the scope. The statement block then
validates its first statement and each explicit `NEXT` continuation once, adding
its `YIELD` bindings before the following statement. Nested queries, nested
data procedures, and inline calls reuse this traversal. A nested body's local
definitions stay in that body.

## Catalog/DDL Validation

Key functions:

- validate_catalog_statement_block
- validate_catalog_statement
- validate_catalog_schema_name
- validate_catalog_object_name
- validate_create_graph_type_clause
- validate_graph_type_source_clause
- validate_graph_expression_clause

Typical checks:

- non-empty block;
- valid schema/object names;
- illegal combinations (for example, OR REPLACE + IF NOT EXISTS in the same statement);
- consistency of graph type/source clauses.

## Query Validation

Key functions:

- validate_query
- validate_basic_query
- validate_set_operation_query
- validate_match_clause
- validate_return_clause_with_aliases
- validate_group_clause
- validate_order_clause
- validate_limit_clause

Important checks:

- used variables must be declared in the current scope;
- non-empty RETURN, consistent/unique aliases;
- GROUP BY consistency with non-aggregated variables in RETURN;
- semantically valid ORDER BY and LIMIT/OFFSET.

## Aggregations and GROUP BY

Key functions:

- check_no_aggregates_in_where
- validate_non_aggregated_return_variables_in_group_by
- collect_non_aggregated_variable_refs
- collect_non_aggregated_variable_refs_in_case

Main rules:

- no aggregate functions in WHERE/FILTER;
- when GROUP BY is present, non-aggregated variables in RETURN must be grouped or equivalent to valid aliases.

## DML Validation

Key functions:

- validate_data_statement
- validate_insert_statement / validate_insert_pattern
- validate_set_items / validate_set_item
- validate_remove_items / validate_remove_item

Typical checks:

- baseline correctness of set/remove/delete operations;
- consistent property access;
- variable usage consistent with scope.

## Expression Validation

Key functions:

- validate_expression
- validate_case_expression
- validate_call_clause
- validate_page_value

Covers:

- binary/unary expressions;
- function calls (including aggregates), dispatched through the built-in function registry
  (`builtins.rs`, see [builtin_functions.md](builtin_functions.md));
- case/cast/predicates;
- subqueries and call;
- numeric or parameterized pagination values.

## Variable Scope Management

General approach:

- keeps a HashSet of declared variables;
- `MATCH`/`LET`/`YIELD` update scope;
- expression-level checks verify scope membership.

This prevents references to variables not introduced by the query plan.

## Error Model

Semantic errors are returned as `GqlError::Validation`.

`ErrorContext` contains an optional source span and an optional diagnostic code.
Constructors without position information use `span: None`; `validation_at()` stores the supplied span.

Applications should inspect the variant and structured context instead of extracting positions from the human-readable message.

`GqlError` and `ErrorContext` support Serde serialization and deserialization.

See the [query analyzer guide](query_analyzer.md) for examples.
