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
use crate::ast::model::{ProcedureBody, Span};
use serde::{Deserialize, Serialize};

/// Represents a GQL query statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Query {
    /// Basic single-pipeline query.
    Basic(BasicQuery),
    /// Query in a nested procedure scope.
    Nested(NestedQuery),
    /// Set operation between two queries.
    SetOperation(SetOperation),
}

impl Query {
    /// Derives the transaction requirement needed to execute this query tree.
    pub fn transaction_intent(&self) -> TransactionIntent {
        match self {
            Query::Basic(basic) => basic.transaction_intent(),
            Query::Nested(nested) => nested.body.transaction_intent(),
            Query::SetOperation(set_op) => set_op.transaction_intent(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedQuery {
    pub body: Box<ProcedureBody>,
    pub graph_expression: Option<GraphExpressionClause>,
    pub span: Span,
}

/// Represents a basic query,
/// consisting of a sequence of query steps followed by a terminal result statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BasicQuery {
    /// Ordered query pipeline steps.
    pub pipeline: Vec<QueryStep>,
    /// Terminal result-producing statement.
    pub result_statement: PrimitiveResultStatement,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl BasicQuery {
    /// Aggregates transaction requirements from all pipeline/query parts.
    pub fn transaction_intent(&self) -> TransactionIntent {
        let mut intent = TransactionIntent::ReadOnly;

        for step in &self.pipeline {
            intent = intent.merge(step.transaction_intent());
            if matches!(intent, TransactionIntent::DataAndCatalogWrite) {
                return intent;
            }
        }

        intent.merge(self.result_statement.transaction_intent())
    }
}

/// Represents a step in the execution pipeline of a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QueryStep {
    /// MATCH step.
    Match(MatchClause),
    /// LET step.
    Let(LetStatement),
    /// FILTER step.
    Filter(FilterClause),
    /// HAVING step.
    Having(FilterClause),
    /// FOR step.
    For(ForClause),
    /// ORDER BY / LIMIT / OFFSET step.
    Page(OrderByAndPageStatement),
    /// CALL step.
    Call(CallClause),
    /// USE GRAPH step.
    UseGraph(GraphExpressionClause),
    /// Nested subquery step.
    FromSubquery(Box<Query>),
}

impl QueryStep {
    /// Returns the transaction requirement introduced by this query step.
    pub fn transaction_intent(&self) -> TransactionIntent {
        match self {
            QueryStep::Match(clause) => clause.transaction_intent(),
            QueryStep::Let(statement) => statement.transaction_intent(),
            QueryStep::Filter(filter) => filter.condition.transaction_intent(),
            QueryStep::Having(filter) => filter.condition.transaction_intent(),
            QueryStep::For(clause) => clause.expression.transaction_intent(),
            QueryStep::Page(clause) => clause.transaction_intent(),
            QueryStep::Call(call) => call.transaction_intent(),
            QueryStep::UseGraph(_) => TransactionIntent::ReadOnly,
            QueryStep::FromSubquery(query) => query.transaction_intent(),
        }
    }
}

/// Represents a terminal result statement in GQL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PrimitiveResultStatement {
    /// FINISH terminal statement.
    Finish {
        /// Source range covered by this AST node.
        span: Span,
    },
    /// RETURN terminal statement.
    Return {
        /// RETURN clause payload.
        return_clause: ReturnClause,
        /// Optional GROUP BY clause.
        group_clause: Option<GroupClause>,
        /// Optional ORDER BY / LIMIT / OFFSET clause.
        page_clause: Option<Box<OrderByAndPageStatement>>,
        /// Source range covered by this AST node.
        span: Span,
    },
}

impl PrimitiveResultStatement {
    /// Current primitive result statements are read-only by definition.
    pub fn transaction_intent(&self) -> TransactionIntent {
        match self {
            PrimitiveResultStatement::Finish { .. } => TransactionIntent::ReadOnly,
            PrimitiveResultStatement::Return { return_clause, page_clause, .. } => {
                let projection = match &return_clause.projection {
                    ReturnProjection::Wildcard => TransactionIntent::ReadOnly,
                    ReturnProjection::Items(items) => TransactionIntent::combine(items.iter().map(|item| {
                        item.expression.transaction_intent()
                    })),
                };
                projection.merge(page_clause.as_deref().map_or(
                    TransactionIntent::ReadOnly, OrderByAndPageStatement::transaction_intent,
                ))
            }
        }
    }
}

/// Represents a PAGE clause in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderByAndPageStatement {
    /// Optional ORDER BY clause.
    pub order_clause: Option<OrderClause>,
    /// Optional LIMIT/OFFSET clause.
    pub limit_clause: Option<LimitClause>,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl OrderByAndPageStatement {
    fn transaction_intent(&self) -> TransactionIntent {
        self.order_clause.as_ref().map_or(TransactionIntent::ReadOnly, OrderClause::transaction_intent)
    }
}

/// Represents a CALL statement in a data/query pipeline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStatement {
    /// CALL clause payload.
    pub call_clause: CallClause,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl CallStatement {
    /// Delegates intent derivation to inner CALL clause payload.
    pub fn transaction_intent(&self) -> TransactionIntent {
        self.call_clause.transaction_intent()
    }
}

/// Represents a CALL clause in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallClause {
    /// Called procedure name.
    pub procedure_name: String,
    /// Procedure arguments.
    pub arguments: Vec<Expression>,
    /// Optional YIELD clause.
    pub yield_items: Option<YieldClause>,
    /// True when OPTIONAL keyword is present.
    pub is_optional: bool,
    /// Complete local scope of an inline CALL.
    pub inline_procedure_body: Option<Box<ProcedureBody>>,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl CallClause {
    /// Inline procedure body can force write mode when it contains mutations.
    pub fn transaction_intent(&self) -> TransactionIntent {
        let arguments = TransactionIntent::combine(self.arguments.iter().map(Expression::transaction_intent));
        self.inline_procedure_body.as_ref().map_or(arguments, |body| arguments.merge(body.transaction_intent()))
    }
}

/// Represents a set operation,
/// combining two subqueries with a set operation
/// and optional LIMIT and ORDER BY clauses to control
/// the final result of the set operation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetOperation {
    /// Left query operand.
    pub left: Box<Query>,
    /// Set operator.
    pub operation: SetOperationType,
    /// Right query operand.
    pub right: Box<Query>,
    /// Optional top-level limit/offset.
    pub limit_clause: Option<LimitClause>,
    /// Optional top-level ordering.
    pub order_clause: Option<OrderClause>,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl SetOperation {
    /// A set operation requires the strongest mode across both branches.
    pub fn transaction_intent(&self) -> TransactionIntent {
        self.left.transaction_intent().merge(self.right.transaction_intent()).merge(
            self.order_clause.as_ref().map_or(TransactionIntent::ReadOnly, OrderClause::transaction_intent),
        )
    }
}

/// Represents the set operation variants used to combine two subqueries.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SetOperationType {
    /// UNION.
    Union,
    /// UNION ALL.
    UnionAll,
    /// UNION DISTINCT.
    UnionDistinct,
    /// INTERSECT.
    Intersect,
    /// INTERSECT ALL.
    IntersectAll,
    /// INTERSECT DISTINCT.
    IntersectDistinct,
    /// EXCEPT.
    Except,
    /// EXCEPT ALL.
    ExceptAll,
    /// EXCEPT DISTINCT.
    ExceptDistinct,
    /// OTHERWISE operator variant.
    Otherwise,
}

/// Represents a LET statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LetStatement {
    /// Variable definitions introduced by LET.
    pub variable_definitions: Vec<VariableDefinition>,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl LetStatement {
    pub(crate) fn transaction_intent(&self) -> TransactionIntent {
        TransactionIntent::combine(self.variable_definitions.iter().map(|definition| definition.expression.transaction_intent()))
    }
}

/// Represents a variable definition in a LET statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VariableDefinition {
    /// Defined variable name.
    pub variable_name: String,
    /// Value expression assigned to the variable.
    pub expression: Expression,
    /// Type declared with `LET VALUE x :: T = ...`; `None` when no type is written.
    pub declared_type: Option<ValueType>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a FILTER clause in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterClause {
    /// Boolean condition expression.
    pub condition: Expression,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a FOR clause in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ForClause {
    /// Iteration variable name.
    pub variable: String,
    /// Iterable/source expression.
    pub expression: Expression,
    /// Optional ordinality/offset variable.
    pub ordinality_or_offset_variable: Option<String>,
    /// Source range covered by this AST node.
    pub span: Span,
}
