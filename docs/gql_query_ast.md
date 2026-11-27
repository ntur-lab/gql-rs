# Query AST - Structural Guide

This document describes the main parts of the AST model and the role of each section.

## File Purpose

The AST model is split in:

- `src/ast/model/query_ast.rs` (entry facade)
- `src/ast/model/nodes/` (typed node definitions)

Logical flow:

1. The ANTLR parser produces the CST.
2. GQLAstBuilder converts CST -> `QueryAST`.
3. AST validator modules apply semantic rules to the AST.

The public `parse()` function performs steps 1 and 2.
`analyze()` also performs step 3. An AST returned by `parse()` has not passed semantic validation.

Applications can invoke `QueryAST::validate()` separately.
Deserializing an AST through Serde does not automatically run this validation either.

## Program and statement blocks

`QueryAST` exposes `body: ProgramBody` and the program `span`. An activity is either
a `SessionActivity` or a `TransactionActivity`, followed by an optional `SESSION CLOSE`.
A standalone `SESSION CLOSE` has its own root variant.

`TransactionActivity` distinguishes a standalone start command, a body with optional
start and end commands, and a standalone end command. The body is a `ProcedureBody`.
This type holds the optional schema context, ordered local definitions, and a
`StatementBlock`. It is also used for nested queries, nested data procedures, and
inline calls.

A `StatementBlock` has one initial statement and a list of explicit `NextContinuation`
nodes. Each continuation carries its own optional `YIELD`, target statement, and span.
Currently, `Statement::Next` remains in use for legacy DML pipeline composition (#8).

### Migrating from 0.0.x

- The signatures of `parse` and `analyze` still return `QueryAST`.
- Use `ast.single_statement()` for a program consisting of one statement without a
  preamble, explicit transaction commands, continuations, or session closure. For
  general execution, match `ast.body`, then traverse the `ProcedureBody` and its
  statement block in source order. Handle transaction and session commands independently.
- Match `Query::Nested` for a nested query with its own procedure body. Nested
  data procedures use `Statement::Nested`. An inline call now carries its
  structured body in `CallClause::inline_procedure_body`.
- Serialized programs now have `body` and `span` at the root. For persisted
  0.0.x ASTs, reparse the original GQL to produce the new representation.
  Older ASTs can lack context that cannot be recovered from JSON alone.

## Catalog/DDL Area

Main blocks:

- `CatalogStatementBlock`: list of `CatalogStatement`.
- `CatalogStatement`:
  - `CreateSchema` / `DropSchema`
  - `CreateGraph` / `DropGraph`
  - `CreateGraphType` / `DropGraphType`
  - `Call`
  - `Raw` (text fallback)

Supporting types:

- `CatalogSchemaName`, `CatalogObjectName`
- `CreateGraphTypeClause` and variants (Open / Of)
- `GraphSourceClause`, `GraphTypeSourceClause`
- `GraphExpressionClause` + `GraphExpressionKind`

This area models the schema/catalog sub-language.

## Session and transaction commands

`SessionActivity` retains ordered SET and RESET commands with individual spans.
`StartTransactionCommand` retains its access mode and span. The remaining
command payload work is tracked by #17.

## Query Area

Main types:

- `Query`
  - Basic(BasicQuery)
  - SetOperation(SetOperation)
- `BasicQuery`
  - pipeline
  - result_statement
- `QueryStep`
  - Match / Let / Filter / Having / For / Page
  - Call / UseGraph / FromSubquery
- `PrimitiveResultStatement`
  - Finish / Return

Query completion types:

- `ReturnClause`, `ReturnItem`, `DistinctQualifier`
- `GroupClause`, `GroupKey`
- `OrderClause`, `OrderItem`, `OrderDirection`, `NullsOrdering`
- `LimitClause`, `PageValue`

## DML Area (DataStatement)

- `DataStatement` includes:
  - Insert, Set, MatchSet
  - Remove, MatchRemove
  - Delete, MatchDelete
  - Call

Dedicated types:

- InsertStatement, SetStatement, SetItem
- RemoveStatement, RemoveItem
- DeleteStatement, DeleteClause
- CallStatement, CallClause

## Pattern Matching Area

For `MATCH`/`INSERT` patterns:

- MatchClause, MatchClauseItem, WhereClause
- PathPattern, PatternElement, Node, Edge
- EdgeDirection, PathMode, PathSearch, PathQuantifier
- PropertyMap, PropertyItem

These types represent graph structure (nodes/edges, directions, labels, properties).

## Expression and Type System Area

Expression core:

- Expression (variable, literal, binary/unary, function call, case, cast, subquery, property access, predicates, etc.)
- Operator
- Literal (string, integer, float, exact decimal, bytes, boolean, null, date, time, datetime, duration, ...)
- ArrayExpression, map literals, inline LET, path constructors, and WHEN operands

Type system:

- ValueType (declared type: base `TypeSpec` plus `not_null`), FieldType
- TypeSpec, IntegerSize, FloatSize, DurationQualifier
- GraphTypeSpec, VertexTypeSpec, EdgeTypeSpec
- LabelExpression, LabelTerm, LabelFactor
- PropertyTypeList, PropertyTypeDecl

This section enables advanced parsing and semantic validation (aggregations, casts, predicates, GROUP BY checks).

## Model Conventions

- Every AST node is serializable/deserializable through `serde`.
- Enum variants are used to keep the model extensible.
- Catalog syntax without a dedicated typed node can be preserved as
  `CatalogStatement::Raw`, including the original text and an optional builder detail.

AST source ranges and listener diagnostic points serve different purposes.
Lexer/parser diagnostics use a zero-width `Span` to identify the error position, including EOF. Error contexts use `None` when no position is known; this does not change the representation of spans stored directly in AST nodes.

## Transaction Intent

`QueryAST::transaction_intent()` derives one of four execution requirements:

- `ReadOnly`
- `DataWrite`
- `CatalogWrite`
- `DataAndCatalogWrite`

The intent combines requirements across the procedure body, including continuations
and nested bodies. A data statement contributes a data write, and a catalog statement
contributes a catalog write. Use `ast.transaction_intent().requires_write_tx()`
for a boolean check.
