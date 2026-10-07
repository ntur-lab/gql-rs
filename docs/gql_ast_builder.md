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
- src/ast/builder/statement.rs
- src/ast/builder/data.rs
- src/ast/builder/pattern.rs
- src/ast/builder/clause.rs
- src/ast/builder/expression.rs

## Entry Point

- build_from_cst_root(cst_root)

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
- build_statement_from_context

It routes to:
- composite query -> Statement::Query
- linear data modifying -> Statement::Data (or query-like statement in compatibility cases)
- linear catalog modifying:
  - catalog CALL -> `CatalogStatement::Call`
  - mappable primitive catalog DDL -> Statement::Catalog
  - non-mappable form -> fallback CatalogStatement::Raw (inside Statement::Catalog)

This logic balances strong typing with tolerance for grammar constructs that are not yet fully supported.

## Query Pipeline

Relevant blocks:
- build_query_from_composite_expression
- build_statement_block_from_context
- build_query_from_ambient_linear_query_statement
- build_query_from_focused_linear_query_statement
- build_pipeline_from_simple_linear_query_statement
- build_result_statement_from_primitive_result_statement

In practice:
1. Composes MATCH/LET/FILTER/HAVING/FOR/PAGE/CALL/USE GRAPH/subquery branches into
   `QueryStep` nodes.
2. Closes with PrimitiveResultStatement (RETURN/FINISH).
3. Handles set operations (UNION/INTERSECT/EXCEPT).

## DDL/Catalog Construction

Main blocks:
- build_ddl_statement_from_primitive_catalog
- build_catalog_schema_name
- build_catalog_object_name_from_graph_parent
- build_catalog_object_name_from_graph_type_parent
- build_of_graph_type_clause
- build_graph_type_source_clause
- build_graph_expression_clause

Maps the catalog subtree into typed CatalogStatement variants.

## DML Construction

Main blocks:
- build_data_statement_from_context
- build_insert_statement_from_context
- build_set_items_from_context / build_set_item_from_context
- build_remove_items_from_context / build_remove_item_from_context
- build_expressions_from_delete_item_list
- build_call_clause_from_context

Builds DataStatement variants while preserving a uniform AST shape.

## Graph Pattern Construction

Main blocks:
- build_match_clause
- build_path_pattern_from_context
- extract_path_terms / extract_path_terms_from_expression
- build_elements_from_primary / build_elements_from_simplified_path_context
- build_node_from_context
- build_edge_from_context
- build_quantifier_from_context
- build_label_expression_from_context
- build_property_map_from_context

This part is crucial for translating nodes/edges, directions, quantifiers, and label expressions.

## Expression Construction

Key functions:
- build_expression_from_context
- build_primary_expression_from_context
- build_case_expression_from_context
- build_cast_expression_from_context
- build_function_call_from_aggregate_context
- build_property_access_from_context
- build_literal_from_context / build_literal_from_leaf_text
- operator_from_context

This is typically the largest section because it covers many grammar productions.

## TypeSpec and Cast

Key functions:
- build_type_spec_from_value_type
- build_type_spec_from_predefined
- build_type_spec_from_character_string
- build_type_spec_from_byte_string
- build_type_spec_from_numeric
- build_type_spec_from_temporal

This is where GQL types (including parameterized types) are mapped into AST TypeSpec.

## Error Handling

Final utilities:
- parse_err
- parse_error

Pattern used:
- structural CST errors -> GqlError::Parse
- explicit contextual messages to simplify grammar-to-AST mapping debugging.
