// Copyright (c) 2026 NTUR LAB S.r.l.
//
// This file is part of NTURDB.
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

//! # gql-rs
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
//! # Example
//!
//! ```rust
//! use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
//!
//! # fn main() -> Result<(), gql_rs::GqlError> {
//! let mut analyzer = DefaultGQLQueryAnalyzer::new();
//! let ast = analyzer.exec("MATCH (n:Person) RETURN n")?;
//!
//! println!("{ast:#?}");
//! # Ok(())
//! # }
//! ```

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
