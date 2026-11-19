# GQL AST Builder - Operational Guide

This document explains how the builder works when converting the ANTLR CST into the typed AST model.

## Responsibilities

GQLAstBuilder has a single responsibility:

- read the CST produced by the grammar;
- recognize valid branches;
- build consistent AST nodes;
- return structural parsing errors when required CST parts are missing.

It does not perform full semantic validation: that is handled by the validator modules.

Current implementation entrypoint:

- src/ast/builder/mod.rs

Main builder modules:

- `src/ast/builder/statement.rs`
- `src/ast/builder/data.rs`
- `src/ast/builder/pattern.rs`
- `src/ast/builder/clause.rs`
- `src/ast/builder/expression.rs`

## Entry Point

For application code, `gql_rs::parse(query)` performs lexing, parsing, and typed AST construction without invoking the semantic validator.
Use `gql_rs::analyze(query)` to include semantic validation.

The functions described below are builder implementation details.

- `build_from_cst_root(cst_root)`

High-level behavior:

1. Handles early return for simple top-level cases.
2. Reads programActivity.
3. If it is sessionActivity, builds Statement::Session.
4. If it is transactionActivity:
  - if procedureSpecification is missing, it attempts to build a top-level transaction/catalog Statement;
  - otherwise it enters the procedure body and builds the statement block.

Output: QueryAST with a single root Statement.

## Main Statement Routing

Central function:

- `build_statement_from_context`

It routes to:

- composite query -> Statement::Query
- linear data modifying -> `Statement::Data` (or query-like statement in compatibility cases)
- linear catalog modifying:
  - catalog CALL -> `CatalogStatement::Call`
  - mappable primitive catalog DDL -> `Statement::Catalog`
  - non-mappable form -> fallback `CatalogStatement::Raw` (inside `Statement::Catalog`)

This logic balances strong typing with tolerance for grammar constructs that are not yet fully supported.

## Query Pipeline

Relevant blocks:

- `build_query_from_composite_expression`
- `build_statement_block_from_context`
- `build_query_from_ambient_linear_query_statement`
- `build_query_from_focused_linear_query_statement`
- `build_pipeline_from_simple_linear_query_statement`
- `build_result_statement_from_primitive_result_statement`

In practice:

1. Composes `MATCH`/`LET`/`FILTER`/`HAVING`/`FOR`/`PAGE`/`CALL`/`USE GRAPH`/subquery branches into
   `QueryStep` nodes.
2. Closes with PrimitiveResultStatement (`RETURN`/`FINISH`).
3. Handles set operations (`UNION`/`INTERSECT`/`EXCEPT`).

## DDL/Catalog Construction

Main blocks:

- `build_ddl_statement_from_primitive_catalog`
- `build_catalog_schema_name`
- `build_catalog_object_name_from_graph_parent`
- `build_catalog_object_name_from_graph_type_parent`
- `build_of_graph_type_clause`
- `build_graph_type_source_clause`
- `build_graph_expression_clause`

Maps the catalog subtree into typed `CatalogStatement` variants.

## DML Construction

Main blocks:

- `build_data_statement_from_context`
- `build_insert_statement_from_context`
- `build_set_items_from_context` / `build_set_item_from_context`
- `build_remove_items_from_context` / `build_remove_item_from_context`
- `build_expressions_from_delete_item_list`
- `build_call_clause_from_context`

Builds `DataStatement` variants while preserving a uniform AST shape.

## Graph Pattern Construction

Main blocks:

- `build_match_clause`
- `build_path_pattern_from_context`
- `extract_path_terms` / `extract_path_terms_from_expression`
- `build_elements_from_primary` / `build_elements_from_simplified_path_context`
- `build_node_from_context`
- `build_edge_from_context`
- `build_quantifier_from_context`
- `build_label_expression_from_context`
- `build_property_map_from_context`

This part is crucial for translating nodes/edges, directions, quantifiers, and label expressions.

## Expression Construction

Key functions:

- `build_expression_from_context`
- `build_primary_expression_from_context`
- `build_case_expression_from_context`
- `build_cast_expression_from_context`
- `build_expression_from_numeric_value_expression`
- `build_value_function_from_context` and the per-family function builders
- `build_function_call_from_aggregate_context`
- `build_property_access_from_context`
- `build_expression_from_unsigned_value_specification` / `build_literal_from_context`
- `decode_unsigned_numeric_literal` / `decode_character_string_literal`
- `operator_from_context`

This is typically the largest section because it covers many grammar productions.

Expressions are always built recursively from their CST. Built-in function arguments that
the grammar keeps in dedicated rules (`numericValueExpression`, `pathValueExpression`,
temporal string and record parameters, ...) are built structurally as well, so operators,
nested function calls, variable references and spans are preserved. Token text is only
used to decode a literal or an identifier; it is never a substitute for a missing structural
representation. Built-in function names are canonical upper-case names registered in
[builtin_functions.md](builtin_functions.md).

## Value types and Cast

Declared types are built in `src/ast/builder/types.rs`. `build_value_type` maps every
alternative of the `valueType` rule by inspecting its CST accessors (never by normalizing
the source text) and returns a `ValueType`: the base `TypeSpec` plus the `NOT NULL` flag.
It is used for `CAST` targets, `IS [NOT] TYPED` predicates, `WHEN` type operands, record and
binding-table fields, list elements, catalog property types and typed variable definitions
(`LET VALUE x :: T = ...`, stored in `VariableDefinition::declared_type`).

Mapping rules:

- `NOT NULL` is kept in `ValueType::not_null`, separately from the base type. A union is
  `NOT NULL` only when every member is, because it contains null as soon as one member does.
- Integers become `TypeSpec::Integer { signed, size }`: explicit widths (`INT8` ... `INT256`,
  `INTEGER8` ..., `UINT8` ...), `SMALL`, regular (with optional binary precision) and `BIG`;
  `SIGNED`/`UNSIGNED` verbose forms are supported.
- Approximate numbers become `TypeSpec::Float { size }` (`FLOAT16` ... `FLOAT256`,
  `FLOAT[(p[, s])]`, `REAL`, `DOUBLE [PRECISION]`); `DECIMAL`/`DEC` keep precision and scale.
- `STRING`/`BYTES` keep minimum and maximum length; `CHAR(n)`/`BINARY(n)` are fixed-length
  and become `min = max = n`; `VARCHAR(n)`/`VARBINARY(n)` set the maximum.
- Temporal types map to `Date`, `ZonedDateTime`, `LocalDateTime` (also bare `TIMESTAMP`),
  `ZonedTime`, `LocalTime`; `DURATION(...)` keeps its `DurationQualifier`.
- Reference types: `Node`, `Edge` and `Graph` hold `None` for the open form (`ANY NODE`,
  `EDGE`, `ANY PROPERTY GRAPH`) and the closed type description otherwise;
  `BindingTable` and `Record` hold their `FieldType`s (`None` for `ANY RECORD`).
- `NULL` becomes `TypeSpec::Null`; `NOTHING` and `NULL NOT NULL` become `TypeSpec::Nothing`;
  `[ANY] PROPERTY VALUE` becomes `TypeSpec::PropertyValue`; `ANY [VALUE]` becomes `TypeSpec::Any`.

Type parameters violating the GQL syntax rules fail with a `GqlError::Parse` whose code is
`GqlError::INVALID_TYPE_PARAMETER`:

- a minimum length greater than the maximum length;
- a maximum, fixed or list length of 0, or a precision of 0;
- a scale greater than the precision (`DECIMAL`, `FLOAT`);
- a parameter that does not fit the target integer type;
- duplicate field names in a record or binding-table type.

## Error Handling

Final utilities:

- `parse_err`
- `parse_error`
- `unsupported_construct`

Pattern used:

- structural CST errors -> `GqlError::Parse`
- syntactically valid constructs without a faithful AST representation -> `GqlError::Parse`
  with code `GqlError::UNSUPPORTED_CONSTRUCT`, naming the construct and the follow-up work
  that tracks it (see [builtin_functions.md](builtin_functions.md#expressions-that-fail-explicitly));
- construction errors of nested expressions (function and CALL arguments, LET definitions)
  are propagated, never replaced by text or string literals;
- explicit contextual messages to simplify grammar-to-AST mapping debugging.

The public parsing pipeline checks collected lexer and parser diagnostics before invoking the builder.

Builder errors created without a known source position have
`ErrorContext.span = None`. Consumers must not assume that every
`GqlError::Parse` carries the point span produced by the ANTLR listeners.