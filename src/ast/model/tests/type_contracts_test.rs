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

use crate::ast::*;
use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fmt::Debug;

fn assert_model_contract<T>()
where
    T: Clone + Debug + Serialize + DeserializeOwned,
{
}

macro_rules! assert_contracts {
    ($($model:ty),+ $(,)?) => {
        $(assert_model_contract::<$model>();)+
    };
}

#[test]
fn root_ast_types_implement_the_model_contract() {
    assert_contracts!(QueryAST, Span, TransactionIntent);
}

#[test]
fn catalog_and_session_types_implement_the_model_contract() {
    assert_contracts!(
        Statement,
        CatalogStatementBlock,
        CatalogStatement,
        RawCatalogStatement,
        CreateSchema,
        DropSchema,
        CreateGraph,
        DropGraph,
        CreateGraphType,
        DropGraphType,
        CatalogSchemaName,
        CatalogObjectName,
        CreateGraphTypeClause,
        OpenGraphTypeClause,
        OfGraphTypeClause,
        GraphSourceClause,
        GraphTypeSourceClause,
        NestedGraphTypeSpecification,
        ElementTypeSpecification,
        NodeTypeSpecification,
        EdgeTypeSpecification,
        PropertyTypesSpecification,
        PropertyTypeSpecification,
        GraphExpressionClause,
        GraphExpressionKind,
        SessionStatement,
        SessionCommandKind,
        SessionSetClause,
        SessionResetClause,
        SessionResetTarget,
        SessionResetCollectionTarget,
        SessionSetParameterClause,
        SessionSetParameterKind,
        SessionSetParameterName,
        SessionTimeZoneValue,
        TransactionStatement,
        TransactionCommandKind,
        TransactionMode,
    );
}

#[test]
fn data_statement_types_implement_the_model_contract() {
    assert_contracts!(
        DataStatement,
        InsertStatement,
        MatchSetStatement,
        SetStatement,
        SetItem,
        MatchRemoveStatement,
        RemoveStatement,
        RemoveItem,
        MatchDeleteStatement,
        DeleteClause,
        DeleteStatement,
        NextStatement,
        YieldClause,
        YieldItem,
        MatchClause,
        MatchClauseItem,
        WhereClause,
        ReturnClause,
        ReturnItem,
        DistinctQualifier,
        OrderClause,
        OrderItem,
        OrderDirection,
        NullsOrdering,
        GroupClause,
        GroupKey,
        LimitClause,
        PageValue,
    );
}

#[test]
fn expression_types_implement_the_model_contract() {
    assert_contracts!(
        Expression,
        WhenOperandExpression,
        WhenOperandKind,
        BinaryExpression,
        Operator,
        UnaryExpression,
        PropertyAccess,
        FunctionCall,
        CaseExpression,
        CaseType,
        SimpleCaseExpression,
        SimpleWhenBranch,
        SearchedCaseExpression,
        SearchedWhenBranch,
        IsPredicateExpression,
        IsPredicateKind,
        SubqueryExpression,
        CastExpression,
        InlineLetExpression,
        Literal,
        ArrayExpression,
        TypeSpec,
        GraphTypeSpec,
        VertexTypeSpec,
        EdgeTypeSpec,
        LabelExpression,
        LabelTerm,
        LabelFactor,
        PropertyTypeList,
        PropertyTypeDecl,
    );
}

#[test]
fn pattern_types_implement_the_model_contract() {
    assert_contracts!(
        PathPattern,
        PathAlternation,
        PathAlternationOperator,
        PathSearch,
        PathMode,
        PatternElement,
        Node,
        Edge,
        EdgeDirection,
        PathQuantifier,
        PropertyMap,
        PropertyItem,
    );
}

#[test]
fn query_types_implement_the_model_contract() {
    assert_contracts!(
        Query,
        BasicQuery,
        QueryStep,
        PrimitiveResultStatement,
        OrderByAndPageStatement,
        CallStatement,
        CallClause,
        SetOperation,
        SetOperationType,
        LetStatement,
        VariableDefinition,
        FilterClause,
        ForClause,
    );
}

#[test]
fn representative_expression_and_pattern_round_trip_preserves_structure() {
    let span = Span {
        start_line: 1,
        start_col: 2,
        end_line: 1,
        end_col: 20,
    };
    let pattern = PathPattern {
        variable: Some("path".to_string()),
        path_mode: Some(PathMode::Trail),
        path_search: Some(PathSearch::AnyShortest),
        simplified_pattern: None,
        alternation: None,
        elements: vec![
            PatternElement::Node(Node {
                id: Some("source".to_string()),
                labels: Some(Expression::Literal(Literal::String("Person".to_string()))),
                properties: None,
                where_clause: None,
                span: span.clone(),
            }),
            PatternElement::Edge(Edge {
                id: Some("edge".to_string()),
                labels: Some(Expression::Literal(Literal::String("KNOWS".to_string()))),
                properties: None,
                where_clause: None,
                direction: EdgeDirection::Outgoing,
                quantifier: Some(PathQuantifier::Range { min: 1, max: 3 }),
                span: span.clone(),
            }),
        ],
        span,
    };

    let json = serde_json::to_value(&pattern).expect("path pattern should serialize");
    let decoded: PathPattern = serde_json::from_value(json.clone()).expect("path pattern should deserialize");
    let round_trip = serde_json::to_value(decoded).expect("decoded path should serialize");

    assert_eq!(round_trip, json);
}
