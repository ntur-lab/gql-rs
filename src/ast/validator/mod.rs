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

use crate::ast::model::*;
use crate::constants::function_names as fn_names;
use crate::error_handler::error::GqlError;
use std::collections::{HashMap, HashSet};

// Internal crate modules.
pub(crate) mod builtins;
mod data;
mod expression;
mod query;
mod program;
mod statements;
#[cfg(test)]
mod tests;

use builtins::*;
use data::*;
use expression::*;
use query::*;
use program::*;
use statements::*;

/// Validates the complete QueryAST semantic invariants.
pub fn validate_query_ast(ast: &QueryAST) -> Result<(), GqlError> {
    if let ProgramBody::Activity { activity, .. } = &ast.body {
        match activity {
            ProgramActivity::Session(session) => validate_session_activity(session),
            ProgramActivity::Transaction(TransactionActivity::WithBody { body, .. }) => {
                validate_procedure_body(body, &HashSet::new()).map(|_| ())
            }
            ProgramActivity::Transaction(_) => Ok(()),
        }
    } else {
        Ok(())
    }
}

/// Helper to create a validation error with explicit span context.
pub(super) fn validation_err(message: impl Into<String>, span: Option<&Span>) -> GqlError {
    GqlError::validation_at(message, span.cloned().unwrap_or_default())
}
