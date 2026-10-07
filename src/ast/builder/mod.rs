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

use super::model::*;
use crate::antlr::gqlparser::*;
use crate::constants::function_names as fn_names;
use crate::constants::keywords as kw;
use crate::error_handler::error::GqlError;
use crate::types::GqlResult;
use antlr_rust::parser_rule_context::ParserRuleContext;
use antlr_rust::token::Token;
use antlr_rust::tree::ParseTree;
use std::rc::Rc;
use crate::ast::model::Span;

// Statement-level CST -> AST mapping helpers.
mod statement;
// Data statement CST -> AST mapping helpers.
mod data;
// Pattern/path CST -> AST mapping helpers.
mod pattern;
// Shared clause CST -> AST mapping helpers.
mod clause;
// Expression CST -> AST mapping helpers.
mod expression;


// Internal AST builder entrypoint.
pub(crate) struct GQLAstBuilder;

// Temporary edge atom used during simplified-path desugaring.
#[derive(Clone)]
pub(super) struct SimplifiedEdgeAtom {
    direction: EdgeDirection,
    labels: Option<Expression>,
    quantifier: Option<PathQuantifier>,
}

// Classifies top-level path expression flavor to preserve alternation semantics.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(super) enum PathExpressionFlavor {
    Single,
    Union,
    MultisetAlternation,
}

impl GQLAstBuilder {
    /// Builds a QueryAST from the CST root and dispatches top-level statement families.
    pub(crate) fn build_from_cst_root(cst_root: &GqlProgramContextAll) -> GqlResult<QueryAST> {
        if let Some(_session_close) = cst_root.sessionCloseCommand() {
            return Ok(QueryAST {
                statement: Statement::Session(SessionStatement {
                    kind: SessionCommandKind::Close,
                    set_clauses: Vec::new(),
                    reset_clauses: Vec::new(),
                }),
            });
        }

        let program_activity = cst_root
            .programActivity()
            .ok_or_else(|| Self::parse_err("CST not valid: missing programActivity".into()))?;

        if let Some(session_activity) = program_activity.sessionActivity() {
            return Ok(QueryAST {
                statement: Statement::Session(Self::build_session_command_from_context(
                    &session_activity,
                )?),
            });
        }

        let transaction_activity = program_activity.transactionActivity().ok_or_else(|| {
            Self::parse_err("Query not valid: missing transactionActivity".into())
        })?;

        let procedure_spec = match transaction_activity.procedureSpecification() {
            Some(spec) => spec,
            None => {
                return Ok(QueryAST {
                    statement: Self::build_transaction_or_catalog_statement_from_context(
                        &transaction_activity,
                    )?,
                });
            }
        };

        let statement_block = procedure_spec
            .procedureBody()
            .and_then(|b| b.statementBlock())
            .ok_or_else(|| Self::parse_err("Query not valid: missing statementBlock".into()))?;

        let mut statement = Self::build_statement_block_from_context(&statement_block)?;
        let tx_span = Self::extract_span(&*statement_block);

        if let Some(end) = transaction_activity.endTransactionCommand() {
            let end_stmt = Self::build_end_transaction_statement(&end)?;
            statement = Statement::Next(NextStatement {
                source_statement: Box::new(statement),
                target_statement: Box::new(end_stmt),
                yield_clause: None,
                next: None,
                span: tx_span.clone(),
            });
        }

        if let Some(start) = transaction_activity.startTransactionCommand() {
            let start_stmt = Self::build_start_transaction_statement(&start);
            statement = Statement::Next(NextStatement {
                source_statement: Box::new(start_stmt),
                target_statement: Box::new(statement),
                yield_clause: None,
                next: None,
                span: tx_span.clone(),
            });
        }

        Ok(QueryAST { statement })
    }

    /// Extracts a Span from a given parser context, using the start and stop tokens.
    pub(crate) fn extract_span<'input, C>(ctx: &C) -> Span 
    where
        C: ParserRuleContext<'input> + ?Sized,
    {
        let start_token = ctx.start();
        let stop_token = ctx.stop();
        let stop_text_len = std::cmp::max(0, stop_token.get_stop() - stop_token.get_start() + 1);
        
        Span {
            start_line: start_token.get_line(),
            start_col: start_token.get_column(),
            end_line: stop_token.get_line(),
            end_col: stop_token.get_column() + stop_text_len,
        }
    }

    /// Creates a parse error with a consistent format.
    pub(super) fn parse_err(message: String) -> GqlError {
        GqlError::parse(message)
    }

    /// Helper function returning parse errors in Result<T, GqlError> form.
    pub(super) fn parse_error<T>(message: impl Into<String>) -> GqlResult<T> {
        Err(Self::parse_err(message.into()))
    }
}