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

use super::*;
use crate::antlr::gqlparser::GqlProgramContextAll;
use crate::ast::builder::GQLAstBuilder;
use crate::ast::validator::validate_query_ast;
use crate::types::GqlResult;
use serde::{Deserialize, Serialize};

/// Root AST node for a parsed GQL program.
///
/// The parser first produces a CST via ANTLR. Then the AST builder converts that CST
/// into this semantic model, which is used by validation and later execution layers.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryAST {
    pub body: ProgramBody,
    pub span: Span,
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

    /// Returns the only statement of an unadorned program, if there is one.
    pub fn single_statement(&self) -> Option<&Statement> {
        let ProgramBody::Activity {
            activity: ProgramActivity::Transaction(TransactionActivity::WithBody {
                start: None, body, end: None, ..
            }),
            session_close: None,
        } = &self.body else {
            return None;
        };
        if body.at_schema.is_none() && body.definitions.is_empty() && body.statements.continuations.is_empty() {
            Some(&body.statements.first)
        } else {
            None
        }
    }

    /// Derives the execution transaction requirement for the complete program.
    pub fn transaction_intent(&self) -> TransactionIntent {
        match &self.body {
            ProgramBody::Activity {
                activity: ProgramActivity::Transaction(TransactionActivity::WithBody { body, .. }),
                ..
            } => body.transaction_intent(),
            _ => TransactionIntent::ReadOnly,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgramBody {
    Activity {
        activity: ProgramActivity,
        session_close: Option<SessionCloseCommand>,
    },
    CloseOnly(SessionCloseCommand),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProgramActivity {
    Session(SessionActivity),
    Transaction(TransactionActivity),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionCloseCommand {
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionActivity {
    pub commands: Vec<SessionCommand>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionCommand {
    Set { clause: SessionSetClause, span: Span },
    Reset { clause: Option<SessionResetClause>, span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionActivity {
    StartOnly(StartTransactionCommand),
    WithBody {
        start: Option<StartTransactionCommand>,
        body: ProcedureBody,
        end: Option<EndTransactionCommand>,
        span: Span,
    },
    EndOnly(EndTransactionCommand),
}

impl TransactionActivity {
    pub fn span(&self) -> &Span {
        match self {
            Self::StartOnly(command) => &command.span,
            Self::WithBody { span, .. } => span,
            Self::EndOnly(EndTransactionCommand::Commit { span } | EndTransactionCommand::Rollback { span }) => span,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartTransactionCommand {
    pub mode: Option<TransactionMode>,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EndTransactionCommand {
    Commit { span: Span },
    Rollback { span: Span },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcedureBody {
    pub at_schema: Option<AtSchemaClause>,
    pub definitions: Vec<BindingVariableDefinition>,
    pub statements: StatementBlock,
    pub span: Span,
}

impl ProcedureBody {
    pub fn transaction_intent(&self) -> TransactionIntent {
        self.definitions.iter().fold(self.statements.transaction_intent(), |intent, definition| {
            intent.merge(definition.transaction_intent())
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AtSchemaClause {
    pub schema: SchemaReference,
    pub schema_span: Span,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SchemaReference {
    Absolute { path: Vec<String> },
    Relative { parent_levels: usize, path: Vec<String> },
    Home,
    Current,
    CurrentDirectory,
    Parameter(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindingVariableDefinition {
    Graph(GraphVariableDefinition),
    BindingTable(BindingTableVariableDefinition),
    Value(ValueVariableDefinition),
}

impl BindingVariableDefinition {
    fn transaction_intent(&self) -> TransactionIntent {
        match self {
            Self::Value(definition) => definition.initializer.transaction_intent(),
            Self::Graph(definition) => definition.initializer.transaction_intent(),
            Self::BindingTable(definition) => definition.initializer.transaction_intent(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeclaredVariableType {
    pub value_type: ValueType,
    pub explicitly_typed: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphVariableDefinition {
    pub name: String,
    pub declared_type: Option<DeclaredVariableType>,
    pub initializer: GraphExpression,
    pub property_keyword: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingTableVariableDefinition {
    pub name: String,
    pub declared_type: Option<DeclaredVariableType>,
    pub initializer: BindingTableExpression,
    pub binding_keyword: bool,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValueVariableDefinition {
    pub name: String,
    pub declared_type: Option<DeclaredVariableType>,
    pub initializer: ValueInitializer,
    pub initializer_span: Span,
    pub span: Span,
}

/// A value initializer can itself contain a graph or binding-table expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ValueInitializer {
    Expression(Expression),
    Graph(GraphExpression),
    BindingTable(BindingTableExpression),
}

impl ValueInitializer {
    fn transaction_intent(&self) -> TransactionIntent {
        match self {
            Self::Expression(expression) => expression.transaction_intent(),
            Self::Graph(expression) => expression.transaction_intent(),
            Self::BindingTable(expression) => expression.transaction_intent(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectReference {
    pub schema: Option<SchemaReference>,
    pub parent: Vec<String>,
    pub name: String,
    pub span: Span,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphExpression {
    pub value: GraphExpressionValue,
    pub span: Span,
}

impl GraphExpression {
    fn transaction_intent(&self) -> TransactionIntent {
        match &self.value {
            GraphExpressionValue::Object(expression) => expression.expression().transaction_intent(),
            _ => TransactionIntent::ReadOnly,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphExpressionValue {
    Catalog(CatalogObjectReference),
    DelimitedName(String),
    Parameter(String),
    Home { property: bool },
    Current { property: bool },
    Object(ObjectExpression),
    NameOrVariable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BindingTableExpression {
    pub value: BindingTableExpressionValue,
    pub span: Span,
}

impl BindingTableExpression {
    fn transaction_intent(&self) -> TransactionIntent {
        match &self.value {
            BindingTableExpressionValue::Object(expression) => expression.expression().transaction_intent(),
            BindingTableExpressionValue::Query(query) => query.transaction_intent(),
            _ => TransactionIntent::ReadOnly,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BindingTableExpressionValue {
    Catalog(CatalogObjectReference),
    DelimitedName(String),
    Parameter(String),
    Query(Box<Query>),
    Object(ObjectExpression),
    NameOrVariable(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ObjectExpression {
    Variable(Expression),
    Parenthesized(Expression),
    Primary(Expression),
}

impl ObjectExpression {
    pub fn expression(&self) -> &Expression {
        match self {
            Self::Variable(expression) | Self::Parenthesized(expression) | Self::Primary(expression) => expression,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatementBlock {
    pub first: Statement,
    pub continuations: Vec<NextContinuation>,
    pub span: Span,
}

impl StatementBlock {
    pub fn iter(&self) -> impl Iterator<Item = &Statement> {
        std::iter::once(&self.first).chain(self.continuations.iter().map(|next| &next.statement))
    }

    pub fn last(&self) -> &Statement {
        self.continuations.last().map_or(&self.first, |next| &next.statement)
    }

    pub fn transaction_intent(&self) -> TransactionIntent {
        self.continuations.iter().fold(self.first.transaction_intent(), |intent, next| {
            intent.merge(next.statement.transaction_intent())
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextContinuation {
    pub yield_clause: Option<YieldClause>,
    pub statement: Statement,
    pub span: Span,
}
