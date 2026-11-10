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
//! The recommended entry point is [`DefaultGQLQueryAnalyzer`], used through
//! the [`GQLQueryAnalyzer`] trait. A successful analysis returns a [`QueryAST`];
//! failures are reported as [`GqlError`] values.
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
//! - Source spans for AST nodes and validation errors.
//! - Serde support for AST serialization and deserialization.
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
//! ## Errors
//!
//! Analysis can return:
//!
//! - [`GqlError::Parse`] for invalid syntax or CST construction failures;
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
//! Parser diagnostics currently include line and column information in their
//! message. Structured context is available through `GqlError::context()` when
//! the originating parser or validator can provide it.
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
pub use error_handler::error::GqlError;
/// Public query analyzer trait and default implementation.
pub use query_analyzer::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
/// Public result alias used by crate APIs.
pub use types::GqlResult;

// Internal crate modules.
pub(crate) mod antlr;
pub(crate) mod constants;
pub(crate) mod types;