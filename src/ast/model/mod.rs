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

//! Public data model for the GQL abstract syntax tree.
//!
//! # Structure
//!
//! A [`QueryAST`] contains one root [`Statement`]. The model is divided into:
//!
//! - queries and query pipelines: [`Query`], [`BasicQuery`], [`QueryStep`];
//! - data modification: [`DataStatement`];
//! - catalog operations: [`CatalogStatement`];
//! - expressions and types: [`Expression`], [`TypeSpec`];
//! - graph patterns: [`PathPattern`], [`Node`], [`Edge`];
//! - session and transaction commands.
//!
//! # Source locations
//!
//! AST nodes use [`Span`] to identify their position in the original GQL
//! source. A zero-valued span means that a precise location was not available.
//!
//! # Serialization stability
//!
//! The model supports Serde serialization and deserialization. Because the
//! crate is in alpha, the serialized AST representation may change between
//! releases.

mod query_ast;
mod nodes;
#[cfg(test)]
mod tests;

pub use query_ast::*;
pub use nodes::*;
use serde::{Deserialize, Serialize};

/// This struct represents a span in the source code, 
/// with start and end line and column information. 
/// It is used to track the location of AST nodes for error reporting and debugging purposes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Span {
    /// One-based line containing the first source character.
    pub start_line: isize,
    /// Zero-based column containing the first source character.
    pub start_col: isize,
    /// One-based line containing the final source character.
    pub end_line: isize,
    /// Zero-based column following the final source character.
    pub end_col: isize,
}

impl Default for Span {
    fn default() -> Self {
        Self {
            start_line: 0,
            start_col: 0,
            end_line: 0,
            end_col: 0,
        }
    }
}
