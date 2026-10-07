# ANTLR and Grammar Guide

This document describes how ANTLR is used in gql-parser and how the grammar in `resources/GQL.g4` is organized.

## Where the grammar lives

- Grammar source file: resources/GQL.g4
- Generated ANTLR modules: `src/antlr/`
  - `gqllexer.rs`
  - `gqlparser.rs`
  - `gqllistener.rs`
  - `gqlvisitor.rs`
- Internal module declaration: `src/antlr/mod.rs`

Important: generated files in src/antlr are derived from GQL.g4 and should not be manually edited unless strictly necessary.

For the exact regeneration command, automation script, review checklist, and
troubleshooting steps, see
[Updating the GQL grammar and generated Rust parser](generating_parser.md).

## ANTLR role in the crate

ANTLR is responsible for lexical and syntactic analysis.

Parsing pipeline in this project:
1. Input query text
2. Lexer and parser generated from GQL.g4
3. CST generation
4. CST to AST conversion in GQLAstBuilder
5. Semantic validation in the AST validator modules

So ANTLR stops at syntax and CST production; semantic checks happen later.

## Grammar structure and style

At the top of GQL.g4:
- grammar name: GQL
- parser rules use lower-camel-case names
- lexer rules use upper-case names
- case-insensitive keywords are composed from the `A` through `Z` fragments
- Rule names follow an ISO-like naming style with numbered section comments.

The grammar is a combined grammar (parser and lexer rules in one file).

The explicit letter fragments are required by the ANTLR 4.8 Rust-target
generator currently used by the project. That generator does not support the
newer `caseInsensitive` grammar option.

## High-level grammar structure

The grammar is organized in macro areas, marked by section comments.

Main flow starts from:
- gqlProgram
- programActivity
- sessionActivity or transactionActivity

Then procedure-based statements are parsed through:
- procedureSpecification
- procedureBody
- statementBlock
- statement

Top-level statement categories:
- compositeQueryStatement
- linearCatalogModifyingStatement
- linearDataModifyingStatement

These grammar branches are routed into the available top-level `Statement` variants by the
builder; the correspondence is not one-to-one for every grammar rule.

## Key parser areas in GQL.g4

### Session and Transaction

Relevant rules include:
- sessionSetCommand, sessionResetCommand, sessionCloseCommand
- startTransactionCommand, commitCommand, rollbackCommand

These are reflected in AST SessionStatement and TransactionStatement kinds.

### Catalog / DDL

Relevant rules include:
- createSchemaStatement, dropSchemaStatement
- createGraphStatement, dropGraphStatement
- createGraphTypeStatement, dropGraphTypeStatement
- callCatalogModifyingProcedureStatement

These feed typed catalog AST nodes. Catalog forms without a dedicated typed representation
are preserved as `CatalogStatement::Raw`; catalog procedure calls use
`CatalogStatement::Call`.

### Data-modifying statements

Relevant rules include:
- insertStatement
- setStatement
- removeStatement
- deleteStatement
- callDataModifyingProcedureStatement

These are mapped into `DataStatement` variants when the current builder has a corresponding
AST representation.

### Query language core

Relevant rules include:
- compositeQueryStatement, compositeQueryExpression
- linearQueryStatement, simpleQueryStatement
- matchStatement, filterStatement, letStatement, forStatement
- returnStatement, selectStatement
- orderByClause, groupByClause, limitClause, offsetClause

These produce Query, QueryStep and result-related AST nodes.

### Graph pattern and path language

Relevant rules include:
- graphPattern
- pathPattern
- simplifiedPathPatternExpression
- graphPatternQuantifier
- labelExpression

These define the shape of MATCH and INSERT graph patterns.

### Expressions, predicates, and types

Relevant rules include:
- valueExpression, booleanValueExpression, numericValueExpression
- predicate family (exists/null/type/directed/source-destination/etc.)
- caseExpression, castSpecification, aggregateFunction
- valueType and related type rules

These feed Expression and TypeSpec AST construction.

## Mapping between grammar and implementation

Implementation layers:
- Syntax/CST: `resources/GQL.g4` + `src/antlr/*`
- CST to AST: `src/ast/builder/*`
- Semantic validation: `src/ast/validator/*`

A practical rule of thumb:
- Add or change a grammar rule -> update builder mapping -> update validator rules -> add tests.

## How to evolve the grammar safely

Recommended workflow:
1. Update GQL.g4
2. Regenerate ANTLR parser artifacts by following
   [the generation guide](generating_parser.md)
3. Update builder functions for new/changed CST branches
4. Update validator checks if semantics changed
5. Run tests

Minimum verification commands:
```sh
cargo check
cargo test
```
