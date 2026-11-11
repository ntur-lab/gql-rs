// Copyright (c) 2026 NTUR LAB S.r.l. (https://ntur-lab.com)
//
// This file is part of gql-rs (https://ntur-lab.com/gql-rs)
//
// Licensed under the EUPL, Version 1.2 or - as soon they will be approved by
// the European Commission - subsequent versions of the EUPL (the "Licence");
// You may not use this work except in compliance with the Licence.
// You may obtain a copy of the Licence at:
//
// https://joinup.ec.europa.eu/software/page/eupl
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the Licence is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the Licence for the specific language governing permissions and
// limitations under the Licence.
//
// Authors:
//   Ntur Lab S.r.l. <dev@ntur-lab.com>,
//   Nicola Miotto <nicola.miotto@ntur-lab.com>,
//   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

//! # gql-rs
//!
//! **A typed parser, AST builder, and semantic validator for the Graph Query Language (GQL) in Rust.**
//!
//! `gql-rs` parses GQL source text, builds a typed abstract syntax tree, and
//! applies semantic validation to the resulting model.
//!
//! Use [`analyze`] to build and semantically validate an AST without retaining
//! analyzer state. Use [`parse`] to build an AST without semantic validation.
//!
//! [`DefaultGQLQueryAnalyzer`], used through [`GQLQueryAnalyzer`], additionally
//! retains the latest successful query and an AST copy.
//!
//! Successful calls return a [`QueryAST`]; failures are reported as [`GqlError`].
//!
//! The crate does not execute queries or connect to a graph database. Its
//! public API and AST representation are still evolving.
//!
//! ## Features
//!
//! - GQL lexical and syntactic analysis based on ANTLR.
//! - Conversion from the ANTLR concrete syntax tree to a typed Rust AST.
//! - Semantic validation of variables, expressions, types, graph patterns,
//!   queries, catalog statements, and data-modifying statements.
//! - Stateless parsing and analysis through [`parse`] and [`analyze`].
//! - Structured positions for lexer and parser diagnostics.
//! - Source spans for AST nodes and semantic errors when available.
//! - Serde support for AST nodes, [`GqlError`], and [`ErrorContext`].
//! - Transaction intent classification.
//! - Case-insensitive GQL keywords with identifier spelling preservation.
//! - Portable behavior coverage through the GQL Technology Compatibility Kit.
//!
//! ## Quick start
//!
//! ```rust
//! use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
//!
//! fn main() {
//!     let query = "MATCH (p:Person)-[:WORKS_FOR]->(c:Company) WHERE c.name = 'Ntur Lab' RETURN p.name";
//!
//!     let mut analyzer = DefaultGQLQueryAnalyzer::new();
//!
//!     match analyzer.exec(query) {
//!         Ok(ast) => {
//!             println!("Valid GQL program");
//!             println!("AST: {ast:#?}");
//!             println!("Transaction intent: {:?}", ast.transaction_intent());
//!         }
//!         Err(error) => {
//!             eprintln!("{error}");
//!         }
//!     }
//! }
//! ```
//!
//! `exec` performs the complete analysis pipeline:
//!
//! ```text
//! GQL source -> lexer/parser -> CST -> typed AST -> semantic validation
//! ```
//!
//! ## Stateless entry points
//!
//! ```rust
//! use gql_rs::{analyze, parse, GqlError};
//!
//! fn main() -> Result<(), GqlError> {
//!     let validated_ast = analyze("RETURN 42 AS answer")?;
//!     println!("{validated_ast:#?}");
//!
//!     let ast = parse("RETURN 42 AS answer")?;
//!     ast.validate()?;
//!
//!     Ok(())
//! }
//! ```
//!
//! Neither function retains the query or clones the resulting AST.
//! `parse` still rejects lexical, syntactic, and AST construction failures.
//! A successful parse alone does not establish semantic validity.
//!
//! ## Errors
//!
//! Analysis can return:
//!
//! - [`GqlError::Parse`] for lexical, syntactic, or AST construction failures;
//! - [`GqlError::Validation`] for semantic errors;
//! - [`GqlError::Generic`] for crate-level fallback errors.
//!
//! ```rust
//! use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};
//!
//! fn main() {
//!     let mut analyzer = DefaultGQLQueryAnalyzer::new();
//!
//!     match analyzer.exec("MATCH (") {
//!         Err(GqlError::Parse { message, context }) => {
//!             eprintln!("{message}");
//!             eprintln!("Context: {context:?}");
//!         }
//!         Err(other) => eprintln!("{other}"),
//!         Ok(_) => unreachable!(),
//!     }
//! }
//! ```
//!
//! Lexer and parser diagnostics include a structured zero-width span:
//! the start and end positions are identical, including at EOF.
//! Lines are one-based and columns are zero-based Unicode character offsets.
//! Columns are not UTF-8 byte offsets, UTF-16 offsets, or display widths.
//!
//! Errors without a known position have `context().span == None`.
//! Diagnostic codes are optional. [`GqlError`] and [`ErrorContext`] support
//! Serde serialization and deserialization.
//!
//! ## GQL support
//!
//! The crate implements a growing subset of GQL syntax and semantics.
//! Currently covered areas include:
//!
//! - queries and query composition;
//! - graph patterns and path patterns;
//! - expressions, predicates, functions, and type checks;
//! - data modification;
//! - catalog and session statements;
//! - transactions;
//! - procedure calls;
//! - set operations;
//! - ordering, grouping, and pagination.
//!
//! Some syntactically valid constructs may not yet have a dedicated typed AST
//! representation. Catalog constructs without a specialized representation can
//! be preserved as `CatalogStatement::Raw`.
//!
//! ## About NTUR LAB
//!
//! `gql-rs` is maintained and developed by [NTUR LAB](https://ntur-lab.com).
//!
//! This component is part of **Aurea**, an initiative born out of Europe to build a modern, high-performance, and open data
//! ecosystem. While firmly rooted in European open standards and the ethos of digital sovereignty, our technologies are
//! built from the ground up for the global developer community. We create open-source software meant to be adopted,
//! integrated, and scaled anywhere in the world.

/// This module defines the AST layer for representing GQL queries.
pub mod ast;
/// This module contains the error handling logic for the GQL parser.
pub mod error_handler;
/// This module contains the query analyzer logic for the GQL parser.
pub mod query_analyzer;

/// Flattened public AST surface (includes `QueryAST` and all AST nodes).
pub use ast::*;
/// Public error type returned by parsing and validation flows.
pub use error_handler::error::{ErrorContext, GqlError};
/// Public stateless entry points, query analyzer trait, and default implementation.
pub use query_analyzer::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, analyze, parse};
/// Public result alias used by crate APIs.
pub use types::GqlResult;

// Internal crate modules.
pub(crate) mod antlr;
pub(crate) mod constants;
pub(crate) mod types;
