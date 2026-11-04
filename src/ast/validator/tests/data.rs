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

use super::super::data::*;
use crate::ast::model::*;
use std::collections::HashSet;
use crate::ast::validator::tests::common_tests::*;

#[test]
fn insert_requires_at_least_one_pattern() {
    let insert = InsertStatement {
        patterns: vec![],
        span: span(),
    };

    assert!(validate_insert_statement(&insert, &mut HashSet::new()).is_err());
}

#[test]
fn insert_registers_path_and_element_variables() {
    let mut pattern = path(vec![
        PatternElement::Node(node(Some("a"))),
        PatternElement::Edge(edge(Some("e"))),
        PatternElement::Node(node(Some("b"))),
    ]);
    pattern.variable = Some("p".to_string());

    let insert = InsertStatement {
        patterns: vec![pattern],
        span: span(),
    };
    let mut declared = HashSet::new();

    assert!(validate_insert_statement(&insert, &mut declared).is_ok());
    for expected in ["p", "a", "e", "b"] {
        assert!(declared.contains(expected));
    }
}

#[test]
fn insert_rejects_empty_path_variable_and_invalid_shape() {
    let mut empty_variable = path(vec![PatternElement::Node(node(Some("n")))]);
    empty_variable.variable = Some(" ".to_string());
    assert!(validate_insert_pattern(&empty_variable, &mut HashSet::new()).is_err());

    let starts_with_edge = path(vec![
        PatternElement::Edge(edge(Some("e"))),
        PatternElement::Node(node(Some("n"))),
    ]);
    assert!(validate_insert_pattern(&starts_with_edge, &mut HashSet::new()).is_err());

    let ends_with_edge = path(vec![
        PatternElement::Node(node(Some("n"))),
        PatternElement::Edge(edge(Some("e"))),
    ]);
    assert!(validate_insert_pattern(&ends_with_edge, &mut HashSet::new()).is_err());
}

#[test]
fn projection_helpers_cover_return_finish_and_wildcard_results() {
    let result = return_result(vec![
        (integer(1), Some("one")),
        (string("two"), Some("two")),
    ]);
    let query = basic_query(result.clone());

    assert!(query_produces_return_rows(&query));
    assert_eq!(infer_result_projection_arity(&result), Some(2));
    assert_eq!(infer_query_projection_arity(&query), Some(2));
    assert_eq!(
        infer_result_projection_signature(&result),
        Some(vec!["one".to_string(), "two".to_string()])
    );
    assert_eq!(
        infer_result_projection_types(&result),
        Some(vec![
            Some(TypeSpec::Integer),
            Some(TypeSpec::String { max_length: None }),
        ])
    );

    let finish = PrimitiveResultStatement::Finish { span: span() };
    let finish_query = basic_query(finish.clone());
    assert!(!query_produces_return_rows(&finish_query));
    assert_eq!(infer_result_projection_arity(&finish), None);
    assert_eq!(infer_result_projection_signature(&finish), None);
    assert_eq!(infer_result_projection_types(&finish), None);

    let wildcard = return_result(vec![(string("*"), None)]);
    assert_eq!(infer_result_projection_arity(&wildcard), None);
    assert_eq!(infer_result_projection_signature(&wildcard), None);
}

#[test]
fn set_operation_projection_helpers_merge_compatible_branches() {
    let left = basic_query(return_result(vec![(integer(1), Some("value"))]));
    let right = basic_query(return_result(vec![(Expression::Literal(Literal::Float(2.0)), Some("value"))]));
    let query = Query::SetOperation(set_operation(left, right));

    assert!(query_produces_return_rows(&query));
    assert_eq!(infer_query_projection_arity(&query), Some(1));
    assert_eq!(
        infer_query_projection_signature(&query),
        Some(vec!["value".to_string()])
    );
    assert_eq!(
        infer_query_projection_types(&query),
        Some(vec![Some(TypeSpec::Integer)])
    );
}

#[test]
fn validate_set_operation_rejects_finish_arity_alias_and_type_mismatches() {
    let finish = basic_query(PrimitiveResultStatement::Finish { span: span() });
    let one_column = basic_query(return_result(vec![(integer(1), Some("value"))]));
    assert!(validate_set_operation_query(
        &set_operation(finish, one_column.clone()),
        &mut HashSet::new(),
    )
    .is_err());

    let two_columns = basic_query(return_result(vec![
        (integer(1), Some("value")),
        (integer(2), Some("other")),
    ]));
    assert!(validate_set_operation_query(
        &set_operation(one_column.clone(), two_columns),
        &mut HashSet::new(),
    )
    .is_err());

    let different_alias = basic_query(return_result(vec![(integer(1), Some("other"))]));
    assert!(validate_set_operation_query(
        &set_operation(one_column.clone(), different_alias),
        &mut HashSet::new(),
    )
    .is_err());

    let incompatible_type = basic_query(return_result(vec![(
        Expression::Literal(Literal::Boolean(true)),
        Some("value"),
    )]));
    assert!(validate_set_operation_query(
        &set_operation(one_column, incompatible_type),
        &mut HashSet::new(),
    )
    .is_err());
}

#[test]
fn set_operation_type_compatibility_covers_any_list_and_union() {
    assert!(are_set_operation_column_types_compatible(
        &TypeSpec::Any,
        &TypeSpec::Graph { graph_type_spec: None },
    ));
    assert!(are_set_operation_column_types_compatible(
        &TypeSpec::Integer,
        &TypeSpec::Double,
    ));
    assert!(are_set_operation_column_types_compatible(
        &TypeSpec::List {
            element_type: Box::new(TypeSpec::Integer),
            max_length: None,
        },
        &TypeSpec::List {
            element_type: Box::new(TypeSpec::Double),
            max_length: None,
        },
    ));
    assert!(are_set_operation_column_types_compatible(
        &TypeSpec::Union {
            members: vec![TypeSpec::Integer, TypeSpec::Double],
        },
        &TypeSpec::Integer,
    ));
    assert!(!are_set_operation_column_types_compatible(
        &TypeSpec::Boolean,
        &TypeSpec::Graph { graph_type_spec: None },
    ));
}

#[test]
fn remove_item_validates_property_label_and_variable_scope() {
    let mut declared = HashSet::from(["n".to_string()]);

    let valid_property = RemoveItem::Property {
        property: PropertyAccess {
            object: "n".to_string(),
            property: "temporary".to_string(),
            span: span(),
        },
        span: span(),
    };
    assert!(validate_remove_item(&valid_property, &mut declared).is_ok());

    let unknown_property = RemoveItem::Property {
        property: PropertyAccess {
            object: "missing".to_string(),
            property: "temporary".to_string(),
            span: span(),
        },
        span: span(),
    };
    assert!(validate_remove_item(&unknown_property, &mut declared).is_err());

    let valid_label = RemoveItem::Label {
        variable: "n".to_string(),
        labels: string("Deprecated"),
        span: span(),
    };
    assert!(validate_remove_item(&valid_label, &mut declared).is_ok());

    let unknown_variable = RemoveItem::Variable {
        name: "missing".to_string(),
        span: span(),
    };
    assert!(validate_remove_item(&unknown_variable, &mut declared).is_err());

    let dynamic_variable = RemoveItem::Variable {
        name: "$dynamic".to_string(),
        span: span(),
    };
    assert!(validate_remove_item(&dynamic_variable, &mut declared).is_ok());
}

#[test]
fn set_item_validates_all_target_variants() {
    let mut declared = HashSet::from(["n".to_string()]);

    let property_item = SetItem::Property {
        property: PropertyAccess {
            object: "n".to_string(),
            property: "age".to_string(),
            span: span(),
        },
        value: integer(42),
        span: span(),
    };
    assert!(validate_set_item(&property_item, &mut declared).is_ok());

    let unknown_property = SetItem::Property {
        property: PropertyAccess {
            object: "missing".to_string(),
            property: "age".to_string(),
            span: span(),
        },
        value: integer(42),
        span: span(),
    };
    assert!(validate_set_item(&unknown_property, &mut declared).is_err());

    let valid_label = SetItem::Label {
        variable: "n".to_string(),
        labels: string("Employee"),
        span: span(),
    };
    assert!(validate_set_item(&valid_label, &mut declared).is_ok());

    let invalid_label_type = SetItem::Label {
        variable: "n".to_string(),
        labels: integer(1),
        span: span(),
    };
    assert!(validate_set_item(&invalid_label_type, &mut declared).is_err());

    let valid_variable = SetItem::Variable {
        variable: "n".to_string(),
        value: integer(1),
        span: span(),
    };
    assert!(validate_set_item(&valid_variable, &mut declared).is_ok());

    let unknown_variable = SetItem::Variable {
        variable: "missing".to_string(),
        value: integer(1),
        span: span(),
    };
    assert!(validate_set_item(&unknown_variable, &mut declared).is_err());
}

#[test]
fn aggregate_detection_in_where_traverses_nested_expressions() {
    assert!(check_no_aggregates_in_where(&integer(1)).is_ok());
    assert!(check_no_aggregates_in_where(&function("LOWER", vec![string("A")])).is_ok());
    assert!(check_no_aggregates_in_where(&function("SUM", vec![integer(1)])).is_err());

    let nested_binary = Expression::Binary(BinaryExpression {
        left: Box::new(integer(1)),
        operator: Operator::GreaterThan,
        right: Box::new(function("COUNT", vec![integer(1)])),
        span: span(),
    });
    assert!(check_no_aggregates_in_where(&nested_binary).is_err());

    let nested_unary = Expression::Unary(UnaryExpression {
        operator: Operator::Not,
        expression: Box::new(function("AVG", vec![integer(1)])),
        span: span(),
    });
    assert!(check_no_aggregates_in_where(&nested_unary).is_err());

    let nested_predicate = Expression::IsPredicate(IsPredicateExpression {
        expression: Box::new(function("MAX", vec![integer(1)])),
        negated: false,
        predicate: IsPredicateKind::Null,
        span: span(),
    });
    assert!(check_no_aggregates_in_where(&nested_predicate).is_err());
}
