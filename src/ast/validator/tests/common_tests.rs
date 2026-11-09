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
//   Ntur Lab S.r.l. <dev@ntur-lab.com>
//   Nicola Miotto <nicola.miotto@ntur-lab.com>,
//   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

use crate::{BasicQuery, BinaryExpression, CaseExpression, CaseType, CatalogObjectName, CatalogSchemaName, DistinctQualifier, Edge, EdgeDirection, Expression, FunctionCall, GraphExpressionClause, GraphExpressionKind, GroupClause, GroupKey, Literal, Node, Operator, PathPattern, PatternElement, PrimitiveResultStatement, PropertyAccess, Query, ReturnClause, ReturnItem, SearchedCaseExpression, SearchedWhenBranch, SessionResetClause, SessionResetTarget, SetOperation, SetOperationType, SimpleCaseExpression, SimpleWhenBranch, Span, Statement, SubqueryExpression, TypeSpec, UnaryExpression, YieldItem};

pub(crate) fn span() -> Span {
    Span::default()
}

pub(crate) fn integer(value: i64) -> Expression {
    Expression::Literal(Literal::Integer(value))
}

pub(crate) fn string(value: &str) -> Expression {
    Expression::Literal(Literal::String(value.to_string()))
}

pub(crate) fn variable(name: &str) -> Expression {
    Expression::Variable {
        name: name.to_string(),
        span: span(),
    }
}

pub(crate) fn function(name: &str, arguments: Vec<Expression>) -> Expression {
    Expression::FunctionCall(FunctionCall {
        name: name.to_string(),
        distinct: DistinctQualifier::None,
        arguments,
        span: span(),
    })
}

pub(crate) fn node(id: Option<&str>) -> Node {
    Node {
        id: id.map(str::to_string),
        labels: None,
        properties: None,
        where_clause: None,
        span: span(),
    }
}

pub(crate) fn edge(id: Option<&str>) -> Edge {
    Edge {
        id: id.map(str::to_string),
        labels: None,
        properties: None,
        where_clause: None,
        direction: EdgeDirection::Outgoing,
        quantifier: None,
        span: span(),
    }
}

pub(crate) fn path(elements: Vec<PatternElement>) -> PathPattern {
    PathPattern {
        variable: None,
        path_mode: None,
        path_search: None,
        simplified_pattern: None,
        alternation: None,
        elements,
        span: span(),
    }
}

pub(crate) fn return_result(items: Vec<(Expression, Option<&str>)>) -> PrimitiveResultStatement {
    PrimitiveResultStatement::Return {
        return_clause: ReturnClause {
            distinct: DistinctQualifier::None,
            items: items
                .into_iter()
                .map(|(expression, alias)| ReturnItem {
                    expression,
                    alias: alias.map(str::to_string),
                    span: span(),
                })
                .collect(),
            span: span(),
        },
        group_clause: None,
        page_clause: None,
        span: span(),
    }
}

pub(crate) fn basic_query(result_statement: PrimitiveResultStatement) -> Query {
    Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement,
        span: span(),
    })
}

pub(crate) fn set_operation(left: Query, right: Query) -> SetOperation {
    SetOperation {
        left: Box::new(left),
        operation: SetOperationType::Union,
        right: Box::new(right),
        limit_clause: None,
        order_clause: None,
        span: span(),
    }
}

pub(crate) fn float(value: f64) -> Expression {
    Expression::Literal(Literal::Float(value))
}


pub(crate) fn boolean(value: bool) -> Expression {
    Expression::Literal(Literal::Boolean(value))
}


pub(crate) fn property(object: &str, name: &str) -> Expression {
    Expression::PropertyAccess(PropertyAccess {
        object: object.to_string(),
        property: name.to_string(),
        span: span(),
    })
}

pub(crate) fn function_call(name: &str, arguments: Vec<Expression>) -> FunctionCall {
    FunctionCall {
        name: name.to_string(),
        distinct: DistinctQualifier::None,
        arguments,
        span: span(),
    }
}

pub(crate) fn binary_expression(left: Expression, operator: Operator, right: Expression) -> BinaryExpression {
    BinaryExpression {
        left: Box::new(left),
        operator,
        right: Box::new(right),
        span: span(),
    }
}

pub(crate) fn binary(left: Expression, operator: Operator, right: Expression) -> Expression {
    Expression::Binary(binary_expression(left, operator, right))
}

pub(crate) fn unary(operator: Operator, expression: Expression) -> UnaryExpression {
    UnaryExpression {
        operator,
        expression: Box::new(expression),
        span: span(),
    }
}

pub(crate) fn list_type(element_type: TypeSpec) -> TypeSpec {
    TypeSpec::List {
        element_type: Box::new(element_type),
        max_length: None,
    }
}

pub(crate) fn return_query(expressions: Vec<Expression>) -> Query {
    Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Return {
            return_clause: ReturnClause {
                distinct: DistinctQualifier::None,
                items: expressions
                    .into_iter()
                    .map(|expression| ReturnItem {
                        expression,
                        alias: None,
                        span: span(),
                    })
                    .collect(),
                span: span(),
            },
            group_clause: None,
            page_clause: None,
            span: span(),
        },
        span: span(),
    })
}

pub(crate) fn finish_query() -> Query {
    Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Finish { span: span() },
        span: span(),
    })
}

pub(crate) fn scalar_subquery(query: Query, is_exists: bool) -> SubqueryExpression {
    SubqueryExpression {
        query: Box::new(query),
        expression: None,
        is_exists,
        negated: false,
        span: span(),
    }
}

pub(crate) fn simple_case(test_expression: Expression, when_value: Expression, then_expression: Expression, else_expression: Option<Expression>) -> CaseExpression {
    CaseExpression {
        case_type: CaseType::Simple(SimpleCaseExpression {
            test_expression: Box::new(test_expression),
            when_branches: vec![SimpleWhenBranch {
                when_values: vec![when_value],
                then_expression: Box::new(then_expression),
                span: span(),
            }],
            else_expression: else_expression.map(Box::new),
        }),
        span: span(),
    }
}

pub(crate) fn searched_case(condition: Expression, then_expression: Expression, else_expression: Option<Expression>) -> CaseExpression {
    CaseExpression {
        case_type: CaseType::Searched(SearchedCaseExpression {
            when_branches: vec![SearchedWhenBranch {
                condition: Box::new(condition),
                then_expression: Box::new(then_expression),
                span: span(),
            }],
            else_expression: else_expression.map(Box::new),
        }),
        span: span(),
    }
}

pub(crate) fn return_clause(items: Vec<(Expression, Option<&str>)>) -> ReturnClause {
    ReturnClause {
        distinct: DistinctQualifier::None,
        items: items
            .into_iter()
            .map(|(expression, alias)| ReturnItem {
                expression,
                alias: alias.map(str::to_string),
                span: span(),
            })
            .collect(),
        span: span(),
    }
}

pub(crate) fn group(keys: Option<Vec<&str>>, is_empty_grouping_set: bool) -> GroupClause {
    GroupClause {
        keys: keys.map(|names| {
            names
                .into_iter()
                .map(|name| GroupKey::BindingVariable {
                    name: name.to_string(),
                    span: span(),
                })
                .collect()
        }),
        is_empty_grouping_set,
        span: span(),
    }
}

pub(crate) fn graph_expression(kind: GraphExpressionKind, text: &str) -> GraphExpressionClause {
    GraphExpressionClause {
        kind,
        text: text.to_string(),
        graph_reference: None,
        object_name_or_binding_variable: None,
        span: span(),
    }
}

pub(crate) fn schema(path: &str, name: &str) -> CatalogSchemaName {
    CatalogSchemaName {
        absolute_directory_path: path.to_string(),
        schema_name: name.to_string(),
        span: span(),
    }
}

pub(crate) fn object(parent: Option<&str>, name: &str) -> CatalogObjectName {
    CatalogObjectName {
        parent: parent.map(str::to_string),
        name: name.to_string(),
        span: span(),
    }
}

pub(crate) fn reset(target: SessionResetTarget) -> SessionResetClause {
    SessionResetClause { target }
}

pub(crate) fn yield_item(column_name: &str, alias: Option<&str>) -> YieldItem {
    YieldItem {
        column_name: column_name.to_string(),
        alias: alias.map(str::to_string),
        span: span(),
    }
}

pub(crate) fn finish_statement() -> Statement {
    Statement::Query(Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Finish { span: span() },
        span: span(),
    }))
}
