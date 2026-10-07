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
    /// Returns the semantic value of an identifier without source delimiters.
    pub(super) fn identifier_text(text: &str) -> String {
        let (no_escape, text) = match text.strip_prefix('@') {
            Some(text) => (true, text),
            None => (false, text),
        };
        let Some(delimiter) = text.chars().next() else {
            return String::new();
        };

        if !matches!(delimiter, '"' | '`') || !text.ends_with(delimiter) {
            return text.to_string();
        }

        let delimiter_len = delimiter.len_utf8();
        let inner = &text[delimiter_len..text.len() - delimiter_len];
        let mut chars = inner.chars().peekable();
        let mut decoded = String::with_capacity(inner.len());

        while let Some(ch) = chars.next() {
            if ch == delimiter && chars.peek() == Some(&delimiter) {
                chars.next();
                decoded.push(delimiter);
                continue;
            }

            if ch != '\\' || no_escape {
                decoded.push(ch);
                continue;
            }

            let Some(escaped) = chars.next() else {
                decoded.push(ch);
                break;
            };

            match escaped {
                '\\' | '\'' | '"' | '`' => decoded.push(escaped),
                't' => decoded.push('\t'),
                'b' => decoded.push('\u{0008}'),
                'n' => decoded.push('\n'),
                'r' => decoded.push('\r'),
                'f' => decoded.push('\u{000C}'),
                'u' | 'U' => {
                    let digits = if escaped == 'u' { 4 } else { 6 };
                    let hex: String = chars.by_ref().take(digits).collect();
                    match u32::from_str_radix(&hex, 16).ok().and_then(char::from_u32) {
                        Some(value) => decoded.push(value),
                        None => {
                            decoded.push('\\');
                            decoded.push(escaped);
                            decoded.push_str(&hex);
                        }
                    }
                }
                other => {
                    decoded.push('\\');
                    decoded.push(other);
                }
            }
        }

        decoded
    }

    /// Decodes every delimited identifier in a qualified name or identifier expression,
    /// preserving separators such as `.`, `/`, `&`, and `|`.
    pub(super) fn identifiers_text(text: &str) -> String {
        let chars: Vec<char> = text.chars().collect();
        let mut decoded = String::with_capacity(text.len());
        let mut index = 0;

        while index < chars.len() {
            let segment_start = index;
            let quote_index = if chars[index] == '@'
                && chars.get(index + 1).is_some_and(|ch| matches!(ch, '"' | '`'))
            {
                index + 1
            } else if matches!(chars[index], '"' | '`') {
                index
            } else {
                decoded.push(chars[index]);
                index += 1;
                continue;
            };

            let delimiter = chars[quote_index];
            index = quote_index + 1;
            while index < chars.len() {
                if chars[index] == '\\' && chars[segment_start] != '@' {
                    index = (index + 2).min(chars.len());
                    continue;
                }
                if chars[index] == delimiter {
                    if chars.get(index + 1) == Some(&delimiter) {
                        index += 2;
                        continue;
                    }
                    index += 1;
                    break;
                }
                index += 1;
            }

            let segment: String = chars[segment_start..index].iter().collect();
            decoded.push_str(&Self::identifier_text(&segment));
        }

        decoded
    }

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
