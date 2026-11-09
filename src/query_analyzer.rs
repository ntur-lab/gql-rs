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

//! High-level interface for parsing and validating GQL source text.
//!
//! [`DefaultGQLQueryAnalyzer`] is the recommended application-facing entry
//! point. Calling [`GQLQueryAnalyzer::exec`] performs the complete pipeline:
//!
//! 1. lexical and syntactic analysis;
//! 2. CST-to-AST conversion;
//! 3. semantic validation.
//!
//! A successful execution returns a [`QueryAST`] and stores the query and AST
//! in the analyzer. The stored state is cleared before each execution, so a
//! failed analysis never exposes a previous successful result.
//!
//! # Example
//!
//! ```rust
//! use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};
//!
//! # fn main() -> Result<(), gql_rs::GqlError> {
//! let mut analyzer = DefaultGQLQueryAnalyzer::new();
//! let ast = analyzer.exec("MATCH (n) RETURN n")?;
//!
//! assert_eq!(analyzer.get_query(), Some("MATCH (n) RETURN n"));
//! println!("{ast:#?}");
//! # Ok(())
//! # }
//! ```

use crate::antlr::gqllexer::GQLLexer;
use crate::antlr::gqlparser::GQLParser;
use crate::ast::model::QueryAST;
use crate::error_handler::error::GqlError;
use crate::error_handler::error_listener::GqlErrorListener;
use crate::types::GqlResult;
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::{InputStream, Parser};

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

    /// Retrieves the last analyzed query string.
    ///
    /// # Returns
    /// * `Option<&str>` - A reference to the last query analyzed, or None if no query has been analyzed yet
    fn get_query(&self) -> Option<&str>;

    /// Retrieves the last generated Abstract Syntax Tree.
    ///
    /// # Returns
    /// * `Option<&QueryAST>` - A reference to the last AST generated, or None if no query has been analyzed yet
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
/// let query = "MATCH (a:Actor)-[:ACTED_IN]->(m:Movie) WHERE m.title = 'The Matrix' RETURN a.name";
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
        DefaultGQLQueryAnalyzer {
            query: None,
            ast: None,
        }
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
    fn exec(&mut self, query: &str) -> GqlResult<QueryAST> {
        self.clear();

        // Build the ANTLR input pipeline.
        let input = InputStream::new(query);
        let lexer = GQLLexer::new(input);
        let listener = GqlErrorListener::new();
        let error_collector = listener.errors.clone();

        let token_stream = CommonTokenStream::new(lexer);
        let mut parser = GQLParser::new(token_stream);

        // Replace the default parser error handling with structured collection.
        parser.remove_error_listeners();
        parser.add_error_listener(Box::new(listener));

        // Parse the program entry rule and inspect collected syntax errors first.
        let cst_root_result = parser.gqlProgram();

        let syntax_errors = error_collector.borrow();
        if let Some(first_err) = syntax_errors.first() {
            return Err(first_err.clone());
        }

        // Convert CST -> AST and run semantic validation on the resulting model.
        let cst_root = cst_root_result
            .map_err(|e| GqlError::parse(format!("ANTLR parsing failed: {:?}", e)))?;
        let ast = QueryAST::build(&cst_root)?;
        ast.validate()?;

        // Cache the most recent successful analysis.
        self.query = Some(query.to_string());
        self.ast = Some(ast.clone());

        Ok(ast)
    }

    fn get_query(&self) -> Option<&str> {
        self.query.as_deref()
    }

    fn get_ast(&self) -> Option<&QueryAST> {
        self.ast.as_ref()
    }
}
