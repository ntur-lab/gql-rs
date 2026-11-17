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
use crate::ast::model::Span;
use serde::{Deserialize, Serialize};

/// Represents the different types of data manipulation statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataStatement {
    /// INSERT statement.
    Insert(InsertStatement),
    /// SET statement.
    Set(SetStatement),
    /// MATCH ... SET statement.
    MatchSet(MatchSetStatement),
    /// REMOVE statement.
    Remove(RemoveStatement),
    /// MATCH ... REMOVE statement.
    MatchRemove(MatchRemoveStatement),
    /// DELETE statement.
    Delete(DeleteStatement),
    /// MATCH ... DELETE statement.
    MatchDelete(MatchDeleteStatement),
    /// CALL statement.
    Call(CallStatement),
}

/// Represents an INSERT statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InsertStatement {
    /// Path patterns to insert.
    pub patterns: Vec<PathPattern>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a MATCH-SET statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchSetStatement {
    /// MATCH clauses that select target bindings.
    pub match_clauses: Vec<MatchClause>,
    /// Optional WHERE condition.
    pub where_clause: Option<WhereClause>,
    /// SET items to apply.
    pub items: Vec<SetItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a SET statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SetStatement {
    /// SET items to apply.
    pub items: Vec<SetItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an item in a SET statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SetItem {
    /// Property assignment.
    Property {
        /// Target property access.
        property: PropertyAccess,
        /// Assigned value expression.
        value: Expression,
        /// Source span for the assignment.
        span: Span,
    },
    /// Variable assignment.
    Variable {
        /// Target variable name.
        variable: String,
        /// Assigned value expression.
        value: Expression,
        /// Source span for the variable assignment.
        span: Span,
    },
    /// Label assignment.
    Label {
        /// Target variable name.
        variable: String,
        /// Label expression to assign.
        labels: Expression,
        /// Source span for the label assignment.
        span: Span,
    },
}

/// Represents a MATCH-REMOVE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchRemoveStatement {
    /// MATCH clause selecting target bindings.
    pub match_clause: MatchClause,
    /// Optional WHERE condition.
    pub where_clause: Option<WhereClause>,
    /// REMOVE items to apply.
    pub remove_clause: Vec<RemoveItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a REMOVE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoveStatement {
    /// REMOVE items to apply.
    pub remove_clause: Vec<RemoveItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an item in a REMOVE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RemoveItem {
    /// Property removal.
    Property {
        /// Property access identifying the property to remove.
        property: PropertyAccess,
        /// Source range covered by this AST node.
        span: Span,
    },
    /// Label removal.
    Label {
        /// Target variable name.
        variable: String,
        /// Label expression to remove.
        labels: Expression,
        /// Source range covered by this AST node.
        span: Span,
    },
    /// Variable removal.
    Variable {
        /// Name of the variable to remove.
        name: String,
        /// Source range covered by this AST node.
        span: Span,
    },
}

/// Represents a MATCH-DELETE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchDeleteStatement {
    /// MATCH clause selecting target bindings.
    pub match_clause: MatchClause,
    /// Optional WHERE condition.
    pub where_clause: Option<WhereClause>,
    /// DELETE payload.
    pub delete_clause: DeleteClause,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the DELETE payload (target expressions and DETACH flag).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteClause {
    /// Expressions identifying values to delete.
    pub expressions: Vec<Expression>,
    /// True when DETACH is requested.
    pub detach: bool,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a DELETE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteStatement {
    /// DELETE payload.
    pub delete_clause: DeleteClause,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a NEXT statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NextStatement {
    /// Left/source statement in the chain.
    pub source_statement: Box<Statement>,
    /// Right/target statement in the chain.
    pub target_statement: Box<Statement>,
    /// Optional YIELD clause.
    pub yield_clause: Option<YieldClause>,
    /// Next chained step, if present.
    pub next: Option<Box<NextStatement>>,
    /// Source range covered by this AST node.
    pub span: Span,
}

impl NextStatement {
    /// NEXT chains inherit the strongest transaction requirement across segments.
    pub fn transaction_intent(&self) -> TransactionIntent {
        let mut intent = self.source_statement.transaction_intent();
        intent = intent.merge(self.target_statement.transaction_intent());

        if let Some(next) = &self.next {
            intent = intent.merge(next.transaction_intent());
        }

        intent
    }
}

/// Represents a YIELD clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldClause {
    /// YIELD projection items.
    pub items: Vec<YieldItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an item in a YIELD clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YieldItem {
    /// Source column name.
    pub column_name: String,
    /// Optional alias.
    pub alias: Option<String>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a MATCH clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchClause {
    /// True for OPTIONAL MATCH.
    pub optional: bool,
    /// Match payload.
    pub match_clause: MatchClauseItem,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a pattern in a MATCH clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchClauseItem {
    /// Path patterns to match.
    pub patterns: Vec<PathPattern>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a WHERE clause in a query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhereClause {
    /// WHERE condition expression.
    pub condition: Expression,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a RETURN clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnClause {
    /// DISTINCT/ALL qualifier.
    pub distinct: DistinctQualifier,
    /// Projected wildcard or explicit return items.
    pub projection: ReturnProjection,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the projection selected by a RETURN clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind", content = "items", rename_all = "snake_case")]
pub enum ReturnProjection {
    /// Projects every variable currently in scope.
    Wildcard,
    /// Projects an explicit list of expressions.
    Items(Vec<ReturnItem>),
}

impl ReturnClause {
    /// Returns explicit projection items, or `None` for a wildcard projection.
    pub fn items(&self) -> Option<&[ReturnItem]> {
        match &self.projection {
            ReturnProjection::Wildcard => None,
            ReturnProjection::Items(items) => Some(items),
        }
    }
}

/// Represents an item in a RETURN clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReturnItem {
    /// Returned expression.
    pub expression: Expression,
    /// Optional alias.
    pub alias: Option<String>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents DISTINCT/ALL modifiers for projection and set contexts in GQL.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DistinctQualifier {
    /// No qualifier.
    None,
    /// DISTINCT qualifier.
    Distinct,
    /// ALL qualifier.
    All,
}

/// Represents an ORDER BY clause in GQL.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderClause {
    /// Ordered sort specifications.
    pub items: Vec<OrderItem>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an individual order item in an ORDER BY clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrderItem {
    /// Sort key expression.
    pub expression: Expression,
    /// Sort direction.
    pub direction: OrderDirection,
    /// Optional NULLS FIRST/LAST directive.
    pub nulls_ordering: Option<NullsOrdering>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the direction of ordering in an ORDER BY clause.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum OrderDirection {
    /// ASC order.
    Ascending,
    /// DESC order.
    Descending,
}

/// Represents the nulls ordering in an ORDER BY clause.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum NullsOrdering {
    /// NULLS FIRST.
    First,
    /// NULLS LAST.
    Last,
}

/// Represents a GROUP BY clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroupClause {
    /// Optional grouping keys.
    pub keys: Option<Vec<GroupKey>>,
    /// True when grouping set is explicitly empty.
    pub is_empty_grouping_set: bool,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a grouping key in a GROUP BY clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroupKey {
    /// Grouping by binding variable.
    BindingVariable {
        /// Name of the binding variable used as the grouping key.
        name: String,
        /// Source range covered by this AST node.
        span: Span,
    },
}

/// Represents a LIMIT/OFFSET clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LimitClause {
    /// Optional LIMIT value.
    pub count: Option<PageValue>,
    /// Optional OFFSET value.
    pub offset: Option<PageValue>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a page value in a LIMIT/OFFSET clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PageValue {
    /// Numeric literal page value.
    Number {
        /// Non-negative numeric value used for pagination.
        value: usize,
        /// Source range covered by this AST node.
        span: Span,
    },
    /// Dynamic parameter page value.
    Parameter {
        /// Name of the dynamic parameter supplying the page value.
        name: String,
        /// Source range covered by this AST node.
        span: Span,
    },
}
