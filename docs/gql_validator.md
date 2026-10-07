# GQL Validator - Operational Guide

This document describes the semantic validator for the GQL AST.

Current implementation is modular:
- src/ast/validator/mod.rs
- src/ast/validator/statements.rs
- src/ast/validator/data.rs
- src/ast/validator/query.rs
- src/ast/validator/expression.rs

## Purpose

The validator checks semantic consistency of the AST after parsing and builder conversion.

Examples of checks:
- declared variables versus used variables;
- consistency of RETURN/GROUP/ORDER/LIMIT clauses;
- aggregation constraints (for example, aggregates in WHERE);
- consistency of Session/Transaction/Catalog commands;
- structural validity of patterns and data operations.

## Entry Point

- validate_query_ast(ast)

Flow:
1. Initializes declared-variable scope (HashSet).
2. Dispatches by Statement type.
3. Invokes branch-specific validators.

## Top-Level Dispatch

Main handled branches:
- Statement::Query -> validate_query
- Statement::Data -> validate_data_statement
- Statement::Next -> validate_next_statement
- Statement::Let -> validate_let_statement
- Statement::Session -> validate_session_statement
- Statement::Transaction -> validate_transaction_statement
- Statement::Catalog -> validate_catalog_statement_block

The validator covers all typed top-level statement branches.

## Session and Transaction Validation

Key functions:
- validate_session_statement
- validate_transaction_statement

Typical checks:
- consistency between command kind and payload (SET/RESET/CLOSE, START/COMMIT/ROLLBACK, etc.);
- consistency between the typed command kind and its payload.

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
- function calls (including aggregates);
- case/cast/predicates;
- subqueries and call;
- numeric or parameterized pagination values.

## Variable Scope Management

General approach:
- keeps a HashSet of declared variables;
- MATCH/LET/YIELD update scope;
- expression-level checks verify scope membership.

This prevents references to variables not introduced by the query plan.

## Error Model

- Semantic errors are returned as GqlError::Validation.
- Messages are descriptive and aimed at debugging tests and edge cases.
