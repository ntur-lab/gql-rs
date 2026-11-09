# Query AST - Structural Guide

This document describes the main parts of the AST model and the role of each section.

## File Purpose

The AST model is split in:

- src/ast/model/query_ast.rs (entry facade)
- src/ast/model/nodes/ (typed node definitions)

Logical flow:

1. The ANTLR parser produces the CST.
2. GQLAstBuilder converts CST -> QueryAST.
3. AST validator modules apply semantic rules to the AST.

## Root Node

- QueryAST
    - Contains a single field: statement of type Statement.
    - Represents a complete GQL program.

## Top-Level Layer: Statement

Statement is the main language discriminator in the AST model:

- Query: query pipelines and set operations.
- Data: DML operations (insert/set/remove/delete/call).
- Next: statement chaining with yield.
- Let: variable definitions.
- Session: session commands.
- Transaction: transaction commands.
- Catalog: block of typed DDL statements.

Fallback behavior for partially unsupported catalog constructs is represented inside Catalog as Raw catalog statements.

## Catalog/DDL Area

Main blocks:

- CatalogStatementBlock: list of CatalogStatement.
- CatalogStatement:
    - CreateSchema / DropSchema
    - CreateGraph / DropGraph
    - CreateGraphType / DropGraphType
    - Call
    - Raw (text fallback)

Supporting types:

- CatalogSchemaName, CatalogObjectName
- CreateGraphTypeClause and variants (Open / Of)
- GraphSourceClause, GraphTypeSourceClause
- GraphExpressionClause + GraphExpressionKind

This area models the schema/catalog sub-language.

## Session/Transaction/Command Area

- SessionStatement + SessionCommandKind
- TransactionStatement + TransactionCommandKind
  This separation makes top-level administrative commands explicit.

## Query Area

Main types:

- Query
    - Basic(BasicQuery)
    - SetOperation(SetOperation)
- BasicQuery
    - pipeline
    - result_statement
- QueryStep
    - Match / Let / Filter / Having / For / Page
    - Call / UseGraph / FromSubquery
- PrimitiveResultStatement
    - Finish / Return

Query completion types:

- ReturnClause, ReturnItem, DistinctQualifier
- GroupClause, GroupKey
- OrderClause, OrderItem, OrderDirection, NullsOrdering
- LimitClause, PageValue

## DML Area (DataStatement)

- DataStatement includes:
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

For MATCH/INSERT patterns:

- MatchClause, MatchClauseItem, WhereClause
- PathPattern, PatternElement, Node, Edge
- EdgeDirection, PathMode, PathSearch, PathQuantifier
- PropertyMap, PropertyItem

These types represent graph structure (nodes/edges, directions, labels, properties).

## Expression and Type System Area

Expression core:

- Expression (variable, literal, binary/unary, function call, case, cast, subquery, property access, predicates, etc.)
- Operator
- Literal
- ArrayExpression, map literals, inline LET, path constructors, and WHEN operands

Type system:

- TypeSpec
- GraphTypeSpec, VertexTypeSpec, EdgeTypeSpec
- LabelExpression, LabelTerm, LabelFactor
- PropertyTypeList, PropertyTypeDecl

This section enables advanced parsing and semantic validation (aggregations, casts, predicates, GROUP BY checks).

## Model Conventions

- Every AST node is serializable/deserializable through serde.
- Enum variants are used to keep the model extensible.
- Catalog syntax without a dedicated typed node can be preserved as
  `CatalogStatement::Raw`, including the original text and an optional builder detail.

## Transaction Intent

`QueryAST::transaction_intent()` derives one of four execution requirements:

- `ReadOnly`
- `DataWrite`
- `CatalogWrite`
- `DataAndCatalogWrite`

Query and `NEXT` trees merge the intent of their children. Data statements require a data
write, catalog statements require a catalog write, and
`QueryAST::requires_write_tx()` exposes the corresponding boolean convenience check.
