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

use crate::antlr::gqlparser::GqlProgramContextAll;
use crate::ast::builder::GQLAstBuilder;
use crate::ast::validator::validate_query_ast;
use crate::types::GqlResult;
use super::{Statement, TransactionIntent};
use serde::{Deserialize, Serialize};

/// Root AST node for a parsed GQL program.
///
/// The parser first produces a CST via ANTLR. Then the AST builder converts that CST
/// into this semantic model, which is used by validation and later execution layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAST {
    /// Top-level statement tree for the parsed query.
    pub statement: Statement,
}

impl QueryAST {
    /// Builds a `QueryAST` from the ANTLR CST root.
    pub(crate) fn build(cst_root: &GqlProgramContextAll) -> GqlResult<Self> {
        GQLAstBuilder::build_from_cst_root(cst_root)
    }

    /// Validates the `QueryAST`, ensuring semantic consistency and proper variable usage.
    pub fn validate(&self) -> GqlResult<()> {
        validate_query_ast(self)
    }

    /// Derives the execution transaction requirement for the root statement.
    pub fn transaction_intent(&self) -> TransactionIntent {
        self.statement.transaction_intent()
    }
}

