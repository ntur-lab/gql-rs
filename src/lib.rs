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

/// This module contains the error handling logic for the GQL parser.
pub mod error_handler;
/// This module defines the AST layer for representing GQL queries.
pub mod ast;
/// This module contains the query analyzer logic for the GQL parser.
pub mod query_analyzer;


/// Public error type returned by parsing and validation flows.
pub use error_handler::error::GqlError;
/// Public result alias used by crate APIs.
pub use types::GqlResult;
/// Flattened public AST surface (includes `QueryAST` and all AST nodes).
pub use ast::*;
/// Public query analyzer trait and default implementation.
pub use query_analyzer::{GQLQueryAnalyzer, DefaultGQLQueryAnalyzer};


// Internal crate modules.
pub(crate) mod types;
pub(crate) mod constants;
pub(crate) mod antlr;
