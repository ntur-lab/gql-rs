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

//! Public entry points for parsing and validating GQL source.
//!
//! - [`parse`] performs lexical analysis, parsing, and typed AST construction.
//! - [`analyze`] additionally runs semantic validation.
//! - [`DefaultGQLQueryAnalyzer`] retains the latest successful query and an AST
//!   copy through the [`GQLQueryAnalyzer`] interface.
//!
//! Both stateless functions return an owned [`QueryAST`] without retaining
//! the query or cloning the resulting AST.
//!
//! Lexer and parser listeners share a diagnostic collection. The first
//! collected error is returned before AST construction, even when ANTLR
//! recovers and produces a CST.
//!
//! # Example
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

use crate::antlr::gqllexer::GQLLexer;
use crate::antlr::gqlparser::GQLParser;
use crate::ast::model::QueryAST;
use crate::error_handler::error::GqlError;
use crate::error_handler::error_listener::GqlErrorListener;
use crate::types::GqlResult;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream, Parser};
use std::rc::Rc;

/// Parses GQL source and builds a typed AST without semantic validation.
///
/// The first collected lexical or syntactic diagnostic is returned after
/// parsing, before AST construction. AST construction errors are propagated.
/// Constructs accepted by the grammar may still fail if the AST builder
/// does not support them.
///
/// This function does not retain the query or clone the resulting AST.
///
/// # Example
///
/// ```
/// let ast = gql_rs::parse("RETURN 42 AS answer")?;
/// # Ok::<(), gql_rs::GqlError>(())
/// ```
pub fn parse(query: &str) -> GqlResult<QueryAST> {
    let input = InputStream::new(query);
    let mut lexer = GQLLexer::new(input);

    let listener = GqlErrorListener::new();
    let error_collector = Rc::clone(&listener.errors);

    lexer.remove_error_listeners();
    lexer.add_error_listener(Box::new(listener.clone()));

    let token_stream = CommonTokenStream::new(lexer);
    let mut parser = GQLParser::new(token_stream);

    parser.remove_error_listeners();
    parser.add_error_listener(Box::new(listener));

    let cst_root_result = parser.gqlProgram();

    // Recovery may produce a CST despite lexical or syntactic errors.
    // Do not turn such a CST into a successful result.
    let first_error = {
        let errors = error_collector.borrow();
        errors.first().cloned()
    };

    if let Some(error) = first_error {
        return Err(error);
    }

    let cst_root = cst_root_result.map_err(|error| GqlError::parse(format!("ANTLR parsing failed: {:?}", error)))?;

    QueryAST::build(&cst_root)
}

/// Parses GQL source, builds its typed AST, and validates its semantics.
///
/// This function does not retain the query or clone the resulting AST.
///
/// # Example
///
/// ```
/// let ast = gql_rs::analyze("RETURN 42 AS answer")?;
/// # Ok::<(), gql_rs::GqlError>(())
/// ```
pub fn analyze(query: &str) -> GqlResult<QueryAST> {
    let ast = parse(query)?;
    ast.validate()?;
    Ok(ast)
}

/// Trait for analyzing GQL queries and converting them to Abstract Syntax Trees (AST).
/// This trait defines the interface for query analysis functionality, allowing implementations
/// to provide different strategies for parsing and validating GQL query strings.
pub trait GQLQueryAnalyzer {
    /// Executes analysis on a GQL query string, parsing it into an Abstract Syntax Tree (AST).
    ///
    /// # Arguments
    /// * `query` - A string slice containing the GQL query to analyze
    ///
    /// # Returns
    /// * `GqlResult<QueryAST>` - The resulting Abstract Syntax Tree if successful, or a GqlError on failure
    fn exec(&mut self, query: &str) -> GqlResult<QueryAST>;

    /// Retrieves the stored query, if any.
    ///
    /// The default implementation returns `None` before its first successful
    /// analysis, after `clear()`, and after a failed execution.
    fn get_query(&self) -> Option<&str>;

    /// Retrieves the stored AST, if any.
    ///
    /// The default implementation returns `None` before its first successful
    /// analysis, after `clear()`, and after a failed execution.
    fn get_ast(&self) -> Option<&QueryAST>;
}

/// Default implementation of GQLQueryAnalyzer that stores the query string and resulting AST
/// in internal state after parsing and validation.
///
/// # Example
/// ```
/// use gql_rs::{GQLQueryAnalyzer, DefaultGQLQueryAnalyzer};
///
/// let mut analyzer = DefaultGQLQueryAnalyzer::new();
/// let query = "MATCH (p:Person)-[:WORKS_FOR]->(c:Company) WHERE c.name = 'Ntur Lab' RETURN p.name";
/// match analyzer.exec(query) {
///     Ok(ast) => {
///         println!("Query parsed successfully!");
///         println!("Original query: {}", analyzer.get_query().unwrap());
///         println!("AST: {:?}", analyzer.get_ast().unwrap());
///     }
///     Err(e) => {
///         eprintln!("Failed to parse query: {:?}", e);
///     }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct DefaultGQLQueryAnalyzer {
    /// The last query string that was analyzed
    query: Option<String>,
    /// The AST resulting from the analysis of the query
    ast: Option<QueryAST>,
}

impl DefaultGQLQueryAnalyzer {
    /// Creates a new instance of DefaultGQLQueryAnalyzer with empty state.
    ///
    /// # Returns
    /// * `Self` - A new analyzer instance ready to process queries
    pub fn new() -> Self {
        DefaultGQLQueryAnalyzer { query: None, ast: None }
    }

    /// Clears the internal state of the analyzer, removing any stored query and AST.
    /// This can be useful to reset the analyzer before processing a new query.
    pub fn clear(&mut self) {
        self.query = None;
        self.ast = None;
    }
}

impl Default for DefaultGQLQueryAnalyzer {
    fn default() -> Self {
        Self::new()
    }
}

impl GQLQueryAnalyzer for DefaultGQLQueryAnalyzer {
    /// See [GQLQueryAnalyzer::exec]
    fn exec(&mut self, query: &str) -> GqlResult<QueryAST> {
        self.clear();

        let ast = analyze(query)?;

        self.query = Some(query.to_string());
        self.ast = Some(ast.clone());

        Ok(ast)
    }

    /// See [GQLQueryAnalyzer::get_query]
    fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    /// See [GQLQueryAnalyzer::get_ast]
    fn get_ast(&self) -> Option<&QueryAST> {
        self.ast.as_ref()
    }
}
