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

use crate::ast::model::*;
use crate::constants::{AGGREGATE_FUNCTIONS, function_names as fn_names};
use crate::error_handler::error::GqlError;
use std::collections::{HashMap, HashSet};

// Internal crate modules.
mod data;
mod expression;
mod query;
mod statements;
#[cfg(test)]
mod tests;

use data::*;
use expression::*;
use query::*;
use statements::*;

/// Shared mutable validation context propagated across statement traversal.
pub(super) struct ValidationContext {
    pub(super) declared_variables: HashSet<String>,
}

impl ValidationContext {
    pub(super) fn new() -> Self {
        Self {
            declared_variables: HashSet::new(),
        }
    }
}

/// Validates the complete QueryAST semantic invariants.
pub fn validate_query_ast(ast: &QueryAST) -> Result<(), GqlError> {
    let mut ctx = ValidationContext::new();

    validate_dispatch(&ast.statement, &mut ctx)
}

/// Dispatches validation to the correct statement-specific routine.
pub(super) fn validate_dispatch(
    statement: &Statement,
    ctx: &mut ValidationContext,
) -> Result<(), GqlError> {
    // Each top-level statement family has its own semantic validation path.
    match statement {
        Statement::Query(query) => validate_query(query, &mut ctx.declared_variables),
        Statement::Data(data) => validate_data_statement(data, &mut ctx.declared_variables),
        Statement::Focused(focused) => validate_dispatch(&focused.statement, ctx),
        Statement::Next(next_stmt) => {
            validate_next_statement(next_stmt, &mut ctx.declared_variables)
        }
        Statement::Let(let_stmt) => validate_let_statement(let_stmt, &mut ctx.declared_variables),
        Statement::Session(session) => validate_session_statement(session),
        Statement::Transaction(transaction) => validate_transaction_statement(transaction),
        Statement::Catalog(catalog_block) => validate_catalog_statement_block(catalog_block),
    }
}

/// Helper to create a validation error with explicit span context.
pub(super) fn validation_err(message: impl Into<String>, span: Option<&Span>) -> GqlError {
    GqlError::validation_at(message, span.cloned().unwrap_or_default())
}
