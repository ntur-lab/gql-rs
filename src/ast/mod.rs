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

//! Typed abstract syntax tree for analyzed GQL programs.
//!
//! The root type is [`QueryAST`]. Its [`Statement`] value identifies the
//! top-level statement category:
//!
//! - [`Statement::Query`] for query pipelines and set operations;
//! - [`Statement::Data`] for data-modifying statements;
//! - [`Statement::Catalog`] for catalog and DDL statements;
//! - session and transaction variants for administrative commands.
//!
//! AST nodes preserve source spans where available and support Serde
//! serialization and deserialization.
//!
//! Application code normally obtains an AST through
//! [`crate::DefaultGQLQueryAnalyzer`] rather than constructing one directly.

/// Public types that compose the GQL abstract syntax tree.
pub mod model;

// Internal crate modules.
// Internal CST -> AST builder layer.
pub(crate) mod builder;
// Internal semantic validation layer.
pub(crate) mod validator;


pub use model::*;
