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

use super::super::expression::*;
use crate::ast::model::*;
use crate::ast::validator::tests::common_tests::*;
use std::collections::{HashMap, HashSet};

#[test]
fn validate_expression_accepts_declared_variables_and_numeric_text() {
    let declared = HashSet::from(["known".to_string()]);
    let inferred = HashMap::new();

    assert!(validate_expression_with_types(&variable("known"), &declared, &inferred).is_ok());

    assert!(validate_expression_with_types(&variable("42"), &declared, &inferred).is_ok());

    assert!(validate_expression_with_types(&variable("3.14"), &declared, &inferred).is_ok());
}

#[test]
fn validate_expression_rejects_empty_and_unknown_variables() {
    let inferred = HashMap::new();

    assert!(validate_expression_with_types(&variable(" "), &HashSet::new(), &inferred).is_err());

    let declared = HashSet::from(["known".to_string()]);
    assert!(validate_expression_with_types(&variable("missing"), &declared, &inferred).is_err());
}

#[test]
fn validate_expression_checks_property_access() {
    let inferred = HashMap::new();
    let declared = HashSet::from(["n".to_string()]);

    assert!(validate_expression_with_types(&property("", "name"), &declared, &inferred).is_err());

    assert!(
        validate_expression_with_types(&property("missing", "name"), &declared, &inferred).is_err()
    );

    assert!(validate_expression_with_types(&property("n", ""), &declared, &inferred).is_err());

    assert!(validate_expression_with_types(&property("n", "name"), &declared, &inferred).is_ok());
}

#[test]
fn validate_expression_checks_element_id() {
    let inferred = HashMap::new();
    let declared = HashSet::from(["n".to_string()]);

    assert!(
        validate_expression_with_types(&function("ELEMENT_ID", vec![]), &declared, &inferred)
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &function("ELEMENT_ID", vec![variable("missing")]),
            &declared,
            &inferred
        )
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &function("ELEMENT_ID", vec![variable("n")]),
            &declared,
            &inferred
        )
            .is_ok()
    );
}

#[test]
fn validate_expression_checks_left_and_right_arguments() {
    let inferred = HashMap::new();
    let declared = HashSet::new();

    for name in ["LEFT", "RIGHT"] {
        assert!(
            validate_expression_with_types(
                &function(name, vec![string("hello")]),
                &declared,
                &inferred
            )
                .is_err()
        );

        assert!(
            validate_expression_with_types(
                &function(name, vec![string("hello"), integer(-1)]),
                &declared,
                &inferred
            )
                .is_err()
        );

        assert!(
            validate_expression_with_types(
                &function(name, vec![string("hello"), string("two")]),
                &declared,
                &inferred
            )
                .is_err()
        );

        assert!(
            validate_expression_with_types(
                &function(name, vec![string("hello"), integer(2)]),
                &declared,
                &inferred
            )
                .is_ok()
        );
    }
}

#[test]
fn validate_expression_checks_map_property_keys() {
    let inferred = HashMap::new();
    let declared = HashSet::new();

    let invalid = Expression::Map(vec![PropertyItem {
        key: " ".to_string(),
        value: integer(1),
        span: span(),
    }]);

    assert!(validate_expression_with_types(&invalid, &declared, &inferred).is_err());

    let valid = Expression::Map(vec![PropertyItem {
        key: "value".to_string(),
        value: integer(1),
        span: span(),
    }]);

    assert!(validate_expression_with_types(&valid, &declared, &inferred).is_ok());
}

#[test]
fn validate_expression_checks_path_constructor_shape() {
    let inferred = HashMap::new();
    let declared = HashSet::from(["a".to_string(), "e".to_string(), "b".to_string()]);

    assert!(
        validate_expression_with_types(&Expression::PathConstructor(vec![]), &declared, &inferred)
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &Expression::PathConstructor(vec![variable("a"), variable("e"), ]),
            &declared,
            &inferred
        )
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &Expression::PathConstructor(vec![variable("a"), integer(1), variable("b"), ]),
            &declared,
            &inferred
        )
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &Expression::PathConstructor(vec![variable("a"), variable("missing"), variable("b"), ]),
            &declared,
            &inferred
        )
            .is_err()
    );

    assert!(
        validate_expression_with_types(
            &Expression::PathConstructor(vec![variable("a"), variable("e"), variable("b"), ]),
            &declared,
            &inferred
        )
            .is_ok()
    );
}

#[test]
fn validate_when_operand_rejects_incomplete_metadata() {
    let inferred = HashMap::new();
    let declared = HashSet::from(["edge".to_string()]);

    let invalid_kinds = vec![
        WhenOperandKind::ValueTypePredicate {
            negated: false,
            target_type: None,
        },
        WhenOperandKind::NormalizedPredicate {
            negated: false,
            normal_form: Some(String::new()),
        },
        WhenOperandKind::NormalizedPredicate {
            negated: false,
            normal_form: Some("INVALID".to_string()),
        },
        WhenOperandKind::LabeledPredicate {
            negated: false,
            label_expression: None,
        },
        WhenOperandKind::LabeledPredicate {
            negated: false,
            label_expression: Some(" ".to_string()),
        },
        WhenOperandKind::SourcePredicate {
            negated: false,
            edge_reference: None,
        },
        WhenOperandKind::DestinationPredicate {
            negated: false,
            edge_reference: Some("missing".to_string()),
        },
    ];

    for kind in invalid_kinds {
        let expression = Expression::WhenOperand(WhenOperandExpression { kind, span: span() });

        assert!(validate_expression_with_types(&expression, &declared, &inferred).is_err());
    }
}

#[test]
fn validate_when_operand_accepts_complete_variants() {
    let inferred = HashMap::new();
    let declared = HashSet::from(["edge".to_string()]);

    let valid_kinds = vec![
        WhenOperandKind::NullPredicate { negated: false },
        WhenOperandKind::DirectedPredicate { negated: true },
        WhenOperandKind::ValueTypePredicate {
            negated: false,
            target_type: Some(TypeSpec::Integer),
        },
        WhenOperandKind::NormalizedPredicate {
            negated: false,
            normal_form: None,
        },
        WhenOperandKind::NormalizedPredicate {
            negated: false,
            normal_form: Some("NFC".to_string()),
        },
        WhenOperandKind::NormalizedPredicate {
            negated: false,
            normal_form: Some("NFKD".to_string()),
        },
        WhenOperandKind::LabeledPredicate {
            negated: false,
            label_expression: Some("Person".to_string()),
        },
        WhenOperandKind::SourcePredicate {
            negated: false,
            edge_reference: Some("edge".to_string()),
        },
        WhenOperandKind::DestinationPredicate {
            negated: false,
            edge_reference: Some("edge".to_string()),
        },
    ];

    for kind in valid_kinds {
        let expression = Expression::WhenOperand(WhenOperandExpression { kind, span: span() });

        assert!(validate_expression_with_types(&expression, &declared, &inferred).is_ok());
    }
}

#[test]
fn validate_unary_operator_type_checks_not_and_numeric_signs() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    assert!(
        validate_unary_operator_type(&unary(Operator::Not, boolean(true)), &declared, &inferred)
            .is_ok()
    );

    assert!(
        validate_unary_operator_type(&unary(Operator::Not, integer(1)), &declared, &inferred)
            .is_err()
    );

    assert!(
        validate_unary_operator_type(&unary(Operator::Minus, integer(1)), &declared, &inferred)
            .is_ok()
    );

    assert!(
        validate_unary_operator_type(&unary(Operator::Plus, string("one")), &declared, &inferred)
            .is_err()
    );

    assert!(
        validate_unary_operator_type(
            &unary(Operator::Minus, variable("unknown")),
            &declared,
            &inferred
        )
            .is_ok()
    );
}

#[test]
fn validate_is_predicate_types_checks_boolean_and_typed_predicates() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let valid_boolean = IsPredicateExpression {
        expression: Box::new(boolean(true)),
        negated: false,
        predicate: IsPredicateKind::True,
        span: span(),
    };
    assert!(validate_is_predicate_types(&valid_boolean, &declared, &inferred).is_ok());

    let invalid_boolean = IsPredicateExpression {
        expression: Box::new(integer(1)),
        negated: false,
        predicate: IsPredicateKind::False,
        span: span(),
    };
    assert!(validate_is_predicate_types(&invalid_boolean, &declared, &inferred).is_err());

    let valid_typed = IsPredicateExpression {
        expression: Box::new(integer(1)),
        negated: false,
        predicate: IsPredicateKind::Typed(TypeSpec::Float { precision: None }),
        span: span(),
    };
    assert!(validate_is_predicate_types(&valid_typed, &declared, &inferred).is_ok());

    let invalid_typed = IsPredicateExpression {
        expression: Box::new(boolean(true)),
        negated: false,
        predicate: IsPredicateKind::Typed(TypeSpec::Graph {
            graph_type_spec: None,
        }),
        span: span(),
    };
    assert!(validate_is_predicate_types(&invalid_typed, &declared, &inferred).is_err());

    for predicate in [IsPredicateKind::Null, IsPredicateKind::Unknown] {
        let expression = IsPredicateExpression {
            expression: Box::new(integer(1)),
            negated: false,
            predicate,
            span: span(),
        };

        assert!(validate_is_predicate_types(&expression, &declared, &inferred).is_ok());
    }
}

#[test]
fn validate_function_signatures_reject_invalid_calls() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let invalid_calls = vec![
        function_call("LOWER", vec![]),
        function_call("LOWER", vec![boolean(true)]),
        function_call("UPPER", vec![integer(1)]),
        function_call("TRIM", vec![]),
        function_call(
            "TRIM",
            vec![string("a"), string("b"), string("c"), string("d")],
        ),
        function_call("SUM", vec![]),
        function_call("SUM", vec![boolean(true)]),
        function_call("AVG", vec![string("not numeric")]),
        function_call("SIZE", vec![]),
        function_call("SIZE", vec![integer(1)]),
        function_call("CARDINALITY", vec![boolean(true)]),
        function_call("COUNT", vec![]),
        function_call("COUNT", vec![integer(1), integer(2)]),
        function_call("COALESCE", vec![]),
        function_call("COALESCE", vec![integer(1), string("one")]),
        function_call("DATE", vec![string("2026-01-01"), string("extra")]),
        function_call("CURRENT_DATE", vec![integer(1)]),
        function_call("CURRENT_TIME", vec![integer(1)]),
        function_call("CURRENT_TIMESTAMP", vec![integer(1)]),
        function_call("LOCAL_TIME", vec![integer(1), integer(2)]),
        function_call("LOCAL_DATETIME", vec![integer(1), integer(2)]),
        function_call("DURATION_BETWEEN", vec![integer(1)]),
        function_call("DURATION_BETWEEN", vec![integer(1), integer(2)]),
    ];

    for call in invalid_calls {
        assert!(
            validate_function_call_signature(&call, &declared, &inferred).is_err(),
            "expected {} to be rejected",
            call.name
        );
    }
}

#[test]
fn validate_function_signatures_accept_valid_calls() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let valid_calls = vec![
        function_call("LOWER", vec![string("HELLO")]),
        function_call("UPPER", vec![string("hello")]),
        function_call("TRIM", vec![string(" hello ")]),
        function_call("TRIM", vec![string("BOTH"), string("x"), string("xxx")]),
        function_call("SUM", vec![integer(1)]),
        function_call("AVG", vec![float(1.5)]),
        function_call("SIZE", vec![string("hello")]),
        function_call(
            "CARDINALITY",
            vec![Expression::Array(ArrayExpression {
                elements: vec![integer(1)],
                span: span(),
            })],
        ),
        function_call("COUNT", vec![integer(1)]),
        function_call("COALESCE", vec![integer(1), integer(2)]),
        function_call("DATE", vec![]),
        function_call("DATE", vec![string("2026-01-01")]),
        function_call("CURRENT_DATE", vec![]),
        function_call("CURRENT_TIME", vec![]),
        function_call("CURRENT_TIMESTAMP", vec![]),
        function_call("LOCAL_TIME", vec![]),
        function_call("LOCAL_DATETIME", vec![]),
        function_call(
            "DURATION_BETWEEN",
            vec![
                Expression::Literal(Literal::DateTime("2026-01-01".to_string())),
                Expression::Literal(Literal::DateTime("2026-01-02".to_string())),
            ],
        ),
        function_call("unknown_function", vec![integer(1)]),
    ];

    for call in valid_calls {
        assert!(
            validate_function_call_signature(&call, &declared, &inferred).is_ok(),
            "expected {} to be accepted",
            call.name
        );
    }
}

#[test]
fn validate_function_signatures_reject_nested_aggregates() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let call = function_call("SUM", vec![function("COUNT", vec![integer(1)])]);

    assert!(validate_function_call_signature(&call, &declared, &inferred).is_err());
}

#[test]
fn scalar_subquery_requires_exactly_one_return_column() {
    let valid = scalar_subquery(return_query(vec![integer(1)]), false);
    assert!(validate_scalar_subquery_projection(&valid).is_ok());

    let no_return = scalar_subquery(finish_query(), false);
    assert!(validate_scalar_subquery_projection(&no_return).is_err());

    let no_columns = scalar_subquery(return_query(vec![]), false);
    assert!(validate_scalar_subquery_projection(&no_columns).is_err());

    let two_columns = scalar_subquery(return_query(vec![integer(1), integer(2)]), false);
    assert!(validate_scalar_subquery_projection(&two_columns).is_err());

    let exists_without_return = scalar_subquery(finish_query(), true);
    assert!(validate_scalar_subquery_projection(&exists_without_return).is_ok());
}

#[test]
fn recognizes_aggregate_function_names_case_sensitively() {
    assert!(is_aggregate_function_name("COUNT"));
    assert!(is_aggregate_function_name("SUM"));
    assert!(is_aggregate_function_name("AVG"));
    assert!(!is_aggregate_function_name("LOWER"));
    assert!(!is_aggregate_function_name("count"));
}

#[test]
fn contains_aggregate_function_traverses_expression_variants() {
    let aggregate = function("SUM", vec![integer(1)]);

    let expressions = vec![
        aggregate.clone(),
        binary(integer(1), Operator::Plus, aggregate.clone()),
        Expression::Unary(unary(Operator::Minus, aggregate.clone())),
        Expression::IsPredicate(IsPredicateExpression {
            expression: Box::new(aggregate.clone()),
            negated: false,
            predicate: IsPredicateKind::Null,
            span: span(),
        }),
        Expression::Array(ArrayExpression {
            elements: vec![aggregate.clone()],
            span: span(),
        }),
        Expression::Map(vec![PropertyItem {
            key: "value".to_string(),
            value: aggregate.clone(),
            span: span(),
        }]),
        Expression::Case(simple_case(integer(1), integer(1), aggregate.clone(), None)),
        Expression::Case(searched_case(
            boolean(true),
            integer(1),
            Some(aggregate.clone()),
        )),
        Expression::Cast(CastExpression {
            expression: Box::new(aggregate.clone()),
            target_type: TypeSpec::Integer,
            span: span(),
        }),
        Expression::InlineLet(InlineLetExpression {
            variable_definitions: vec![VariableDefinition {
                variable_name: "x".to_string(),
                expression: aggregate.clone(),
                span: span(),
            }],
            evaluation_expression: Box::new(variable("x")),
            span: span(),
        }),
        Expression::WhenOperand(WhenOperandExpression {
            kind: WhenOperandKind::Comparison {
                operator: Operator::Equal,
                right: Box::new(aggregate),
            },
            span: span(),
        }),
    ];

    for expression in expressions {
        assert!(contains_aggregate_function(&expression));
    }

    assert!(!contains_aggregate_function(&integer(1)));
    assert!(!contains_aggregate_function(&Expression::Subquery(
        scalar_subquery(return_query(vec![function("SUM", vec![integer(1)])]), false, )
    )));
}

#[test]
fn comparison_type_compatibility_covers_type_families() {
    let string_type = TypeSpec::String { max_length: None };
    let bytes_type = TypeSpec::Bytes { max_length: None };
    let float_type = TypeSpec::Float { precision: None };
    let timestamp = TypeSpec::Timestamp {
        precision: None,
        with_timezone: false,
    };

    assert!(are_comparison_types_compatible(
        &TypeSpec::Integer,
        &TypeSpec::Integer
    ));
    assert!(are_comparison_types_compatible(
        &TypeSpec::Any,
        &TypeSpec::Graph {
            graph_type_spec: None,
        }
    ));
    assert!(are_comparison_types_compatible(
        &TypeSpec::Integer,
        &float_type
    ));
    assert!(are_comparison_types_compatible(&string_type, &bytes_type));
    assert!(are_comparison_types_compatible(&TypeSpec::Date, &timestamp));
    assert!(are_comparison_types_compatible(
        &TypeSpec::Boolean,
        &TypeSpec::Boolean
    ));
    assert!(are_comparison_types_compatible(
        &TypeSpec::Path,
        &TypeSpec::Path
    ));
    assert!(are_comparison_types_compatible(
        &TypeSpec::Record,
        &TypeSpec::Record
    ));
    assert!(!are_comparison_types_compatible(
        &TypeSpec::Boolean,
        &TypeSpec::Integer
    ));
}

#[test]
fn validate_binary_operator_types_reject_invalid_combinations() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let invalid = vec![
        binary_expression(integer(1), Operator::And, integer(2)),
        binary_expression(string("a"), Operator::Plus, string("b")),
        binary_expression(string("a"), Operator::Minus, string("b")),
        binary_expression(string("a"), Operator::Star, integer(2)),
        binary_expression(integer(1), Operator::Concat, integer(2)),
        binary_expression(integer(1), Operator::GreaterThan, string("one")),
        binary_expression(integer(1), Operator::Regex, string("[0-9]+")),
    ];

    for expression in invalid {
        assert!(
            validate_binary_operator_types(&expression, &declared, &inferred).is_err(),
            "expected {:?} to be rejected",
            expression.operator
        );
    }
}

#[test]
fn validate_binary_operator_types_accept_valid_combinations() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let valid = vec![
        binary_expression(boolean(true), Operator::And, boolean(false)),
        binary_expression(integer(1), Operator::Plus, integer(2)),
        binary_expression(integer(2), Operator::Minus, integer(1)),
        binary_expression(integer(2), Operator::Star, integer(3)),
        binary_expression(float(4.0), Operator::Slash, integer(2)),
        binary_expression(integer(5), Operator::Percent, integer(2)),
        binary_expression(integer(2), Operator::Caret, integer(3)),
        binary_expression(string("a"), Operator::Concat, string("b")),
        binary_expression(integer(1), Operator::LessThan, integer(2)),
        binary_expression(string("abc"), Operator::Regex, string("a.*")),
        binary_expression(integer(1), Operator::Equal, string("one")),
    ];

    for expression in valid {
        assert!(
            validate_binary_operator_types(&expression, &declared, &inferred).is_ok(),
            "expected {:?} to be accepted",
            expression.operator
        );
    }
}

#[test]
fn validate_cast_target_checks_dynamic_lists_and_incompatible_types() {
    let declared = HashSet::new();
    let inferred = HashMap::new();

    let dynamic_list = CastExpression {
        expression: Box::new(Expression::Array(ArrayExpression {
            elements: vec![variable("$value")],
            span: span(),
        })),
        target_type: list_type(TypeSpec::Integer),
        span: span(),
    };
    assert!(validate_cast_target(&dynamic_list, &declared, &inferred).is_err());

    let invalid_temporal_cast = CastExpression {
        expression: Box::new(Expression::Literal(Literal::DateTime(
            "2026-01-01".to_string(),
        ))),
        target_type: TypeSpec::Integer,
        span: span(),
    };
    assert!(validate_cast_target(&invalid_temporal_cast, &declared, &inferred).is_err());

    let valid_numeric_cast = CastExpression {
        expression: Box::new(integer(1)),
        target_type: TypeSpec::Double,
        span: span(),
    };
    assert!(validate_cast_target(&valid_numeric_cast, &declared, &inferred).is_ok());
}

#[test]
fn infer_expression_type_covers_primary_expression_variants() {
    let declared = HashSet::from(["n".to_string()]);
    let inferred = HashMap::from([
        ("typed".to_string(), TypeSpec::Integer),
        ("n".to_string(), TypeSpec::Record),
    ]);

    assert_eq!(
        infer_expression_type(&integer(1), &inferred, Some(&declared)),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        infer_expression_type(&variable("typed"), &inferred, Some(&declared)),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        infer_expression_type(&variable("100"), &inferred, Some(&declared)),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        infer_expression_type(&variable("3.14"), &inferred, Some(&declared)),
        Some(TypeSpec::Float { precision: None })
    );

    assert_eq!(
        infer_expression_type(&variable("n"), &inferred, Some(&declared)),
        Some(TypeSpec::Record)
    );

    assert_eq!(
        infer_expression_type(&variable("missing"), &inferred, Some(&declared)),
        None
    );

    assert_eq!(
        infer_expression_type(&property("n", "name"), &inferred, Some(&declared)),
        Some(TypeSpec::Any)
    );

    assert_eq!(
        infer_expression_type(&Expression::Map(vec![]), &inferred, Some(&declared)),
        Some(TypeSpec::Record)
    );

    assert_eq!(
        infer_expression_type(
            &Expression::PathConstructor(vec![]),
            &inferred,
            Some(&declared)
        ),
        Some(TypeSpec::Path)
    );
}

#[test]
fn infer_expression_type_covers_unary_and_binary_results() {
    let inferred = HashMap::new();

    assert_eq!(
        infer_expression_type(
            &Expression::Unary(unary(Operator::Not, boolean(true))),
            &inferred,
            None
        ),
        Some(TypeSpec::Boolean)
    );

    assert_eq!(
        infer_expression_type(
            &Expression::Unary(unary(Operator::Minus, integer(1))),
            &inferred,
            None
        ),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        infer_expression_type(
            &Expression::Unary(unary(Operator::Minus, string("one"))),
            &inferred,
            None
        ),
        None
    );

    assert_eq!(
        infer_expression_type(
            &binary(integer(1), Operator::Plus, float(2.0)),
            &inferred,
            None
        ),
        Some(TypeSpec::Float { precision: None })
    );

    assert_eq!(
        infer_expression_type(
            &binary(string("a"), Operator::Concat, string("b")),
            &inferred,
            None
        ),
        Some(TypeSpec::String { max_length: None })
    );

    assert_eq!(
        infer_expression_type(
            &binary(integer(1), Operator::GreaterThan, integer(0)),
            &inferred,
            None
        ),
        Some(TypeSpec::Boolean)
    );

    assert_eq!(
        infer_expression_type(
            &binary(string("a"), Operator::Star, integer(2)),
            &inferred,
            None
        ),
        None
    );
}

#[test]
fn infer_case_type_returns_single_or_union_type() {
    let inferred = HashMap::new();

    let same_type = simple_case(integer(1), integer(1), string("one"), Some(string("other")));

    assert_eq!(
        infer_case_type(&same_type, &inferred, None),
        Some(TypeSpec::String { max_length: None })
    );

    let mixed_type = searched_case(boolean(true), integer(1), Some(string("other")));

    assert_eq!(
        infer_case_type(&mixed_type, &inferred, None),
        Some(TypeSpec::Union {
            members: vec![TypeSpec::Integer, TypeSpec::String { max_length: None }, ],
        })
    );

    let empty = CaseExpression {
        case_type: CaseType::Searched(SearchedCaseExpression {
            when_branches: vec![],
            else_expression: None,
        }),
        span: span(),
    };

    assert_eq!(infer_case_type(&empty, &inferred, None), None);
}

#[test]
fn infer_function_call_type_covers_builtin_families() {
    let inferred = HashMap::new();

    let cases = vec![
        (
            function_call("COUNT", vec![integer(1)]),
            Some(TypeSpec::Integer),
        ),
        (
            function_call("PATH_LENGTH", vec![integer(1)]),
            Some(TypeSpec::Integer),
        ),
        (
            function_call("AVG", vec![integer(1)]),
            Some(TypeSpec::Float { precision: None }),
        ),
        (
            function_call("SUM", vec![integer(1)]),
            Some(TypeSpec::Integer),
        ),
        (
            function_call("LOWER", vec![string("ABC")]),
            Some(TypeSpec::String { max_length: None }),
        ),
        (function_call("CURRENT_DATE", vec![]), Some(TypeSpec::Date)),
        (
            function_call("CURRENT_TIME", vec![]),
            Some(TypeSpec::LocalTime { precision: None }),
        ),
        (
            function_call("CURRENT_TIMESTAMP", vec![]),
            Some(TypeSpec::LocalDateTime { precision: None }),
        ),
        (
            function_call("DURATION", vec![]),
            Some(TypeSpec::Duration {
                precision: None,
                qualifier: None,
            }),
        ),
        (
            function_call("ELEMENTS", vec![]),
            Some(list_type(TypeSpec::Any)),
        ),
        (function_call("unknown_function", vec![]), None),
    ];

    for (call, expected) in cases {
        assert_eq!(
            infer_function_call_type(&call, &inferred, None),
            expected,
            "unexpected type for {}",
            call.name
        );
    }
}

#[test]
fn type_from_literal_maps_every_literal_family() {
    let cases = vec![
        (
            Literal::String("value".to_string()),
            TypeSpec::String { max_length: None },
        ),
        (Literal::Integer(1), TypeSpec::Integer),
        (Literal::Float(1.5), TypeSpec::Float { precision: None }),
        (Literal::Boolean(true), TypeSpec::Boolean),
        (Literal::Null, TypeSpec::Any),
        (
            Literal::DateTime("2026-01-01".to_string()),
            TypeSpec::Timestamp {
                precision: None,
                with_timezone: false,
            },
        ),
        (
            Literal::Duration("P1D".to_string()),
            TypeSpec::Duration {
                precision: None,
                qualifier: None,
            },
        ),
        (
            Literal::TimeWindow("window".to_string()),
            TypeSpec::String { max_length: None },
        ),
        (
            Literal::Vector(vec![1.0, 2.0]),
            TypeSpec::Vector { dimension: Some(2) },
        ),
        (Literal::Parameter("$value".to_string()), TypeSpec::Any),
    ];

    for (literal, expected) in cases {
        assert_eq!(type_from_literal(&literal), expected);
    }

    assert_eq!(
        type_from_literal(&Literal::List(vec![
            Literal::Integer(1),
            Literal::Float(2.0),
        ])),
        list_type(TypeSpec::Float { precision: None })
    );

    assert_eq!(
        type_from_literal(&Literal::List(vec![])),
        list_type(TypeSpec::Any)
    );
}

#[test]
fn validate_array_element_consistency_accepts_numeric_promotion() {
    let array = ArrayExpression {
        elements: vec![integer(1), float(2.5)],
        span: span(),
    };

    assert!(validate_array_element_consistency(&array, &HashSet::new(), &HashMap::new()).is_ok());
}

#[test]
fn validate_array_element_consistency_rejects_incompatible_types() {
    let array = ArrayExpression {
        elements: vec![integer(1), string("two")],
        span: span(),
    };

    assert!(validate_array_element_consistency(&array, &HashSet::new(), &HashMap::new()).is_err());
}

#[test]
fn array_type_compatibility_covers_nested_and_dynamic_types() {
    assert!(are_array_element_types_compatible(
        &TypeSpec::Integer,
        &TypeSpec::Integer
    ));

    assert!(are_array_element_types_compatible(
        &TypeSpec::Any,
        &TypeSpec::Boolean
    ));

    assert!(are_array_element_types_compatible(
        &TypeSpec::Integer,
        &TypeSpec::Double
    ));

    assert!(are_array_element_types_compatible(
        &list_type(TypeSpec::Integer),
        &list_type(TypeSpec::Double)
    ));

    assert!(are_array_element_types_compatible(
        &list_type(TypeSpec::Integer),
        &TypeSpec::String { max_length: None }
    ));

    assert!(!are_array_element_types_compatible(
        &TypeSpec::Boolean,
        &TypeSpec::Integer
    ));
}

#[test]
fn merge_array_element_types_covers_all_merge_paths() {
    assert_eq!(
        merge_array_element_types(&TypeSpec::Boolean, &TypeSpec::Boolean),
        Some(TypeSpec::Boolean)
    );

    assert_eq!(
        merge_array_element_types(&TypeSpec::Any, &TypeSpec::Integer),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        merge_array_element_types(&TypeSpec::Integer, &TypeSpec::Any),
        Some(TypeSpec::Integer)
    );

    assert_eq!(
        merge_array_element_types(&TypeSpec::Integer, &TypeSpec::Double),
        Some(TypeSpec::Float { precision: None })
    );

    assert_eq!(
        merge_array_element_types(&list_type(TypeSpec::Integer), &list_type(TypeSpec::Double)),
        Some(list_type(TypeSpec::Float { precision: None }))
    );

    assert_eq!(
        merge_array_element_types(&TypeSpec::Boolean, &TypeSpec::Integer),
        Some(TypeSpec::Any)
    );
}

#[test]
fn normalize_union_members_flattens_and_deduplicates() {
    let result = normalize_union_members(vec![
        TypeSpec::Integer,
        TypeSpec::Union {
            members: vec![TypeSpec::Integer, TypeSpec::Boolean],
        },
        TypeSpec::Boolean,
        TypeSpec::String { max_length: None },
    ]);

    assert_eq!(
        result,
        vec![
            TypeSpec::Integer,
            TypeSpec::Boolean,
            TypeSpec::String { max_length: None },
        ]
    );
}

#[test]
fn dynamic_parameter_detection_traverses_expression_variants() {
    let parameter = variable("$value");

    let expressions = vec![
        parameter.clone(),
        binary(integer(1), Operator::Plus, parameter.clone()),
        Expression::Unary(unary(Operator::Minus, parameter.clone())),
        function("COALESCE", vec![integer(1), parameter.clone()]),
        Expression::IsPredicate(IsPredicateExpression {
            expression: Box::new(parameter.clone()),
            negated: false,
            predicate: IsPredicateKind::Null,
            span: span(),
        }),
        Expression::Array(ArrayExpression {
            elements: vec![parameter.clone()],
            span: span(),
        }),
        Expression::Map(vec![PropertyItem {
            key: "value".to_string(),
            value: parameter.clone(),
            span: span(),
        }]),
        Expression::Case(simple_case(parameter.clone(), integer(1), integer(1), None)),
        Expression::Case(searched_case(
            boolean(true),
            integer(1),
            Some(parameter.clone()),
        )),
        Expression::Cast(CastExpression {
            expression: Box::new(parameter.clone()),
            target_type: TypeSpec::String { max_length: None },
            span: span(),
        }),
        Expression::InlineLet(InlineLetExpression {
            variable_definitions: vec![VariableDefinition {
                variable_name: "x".to_string(),
                expression: parameter.clone(),
                span: span(),
            }],
            evaluation_expression: Box::new(variable("x")),
            span: span(),
        }),
        Expression::PathConstructor(vec![parameter.clone()]),
        Expression::WhenOperand(WhenOperandExpression {
            kind: WhenOperandKind::Comparison {
                operator: Operator::Equal,
                right: Box::new(parameter),
            },
            span: span(),
        }),
    ];

    for expression in expressions {
        assert!(
            contains_dynamic_parameter_reference(&expression),
            "dynamic parameter was not detected in {expression:?}"
        );
    }

    assert!(!contains_dynamic_parameter_reference(&integer(1)));
    assert!(!contains_dynamic_parameter_reference(&property(
        "n", "name"
    )));
    assert!(!contains_dynamic_parameter_reference(
        &Expression::Subquery(scalar_subquery(
            return_query(vec![variable("$inside")]),
            false,
        ))
    ));
}

#[test]
fn type_classification_helpers_cover_every_family() {
    assert!(is_float_like_type(&TypeSpec::Float { precision: None }));
    assert!(is_float_like_type(&TypeSpec::Float32));
    assert!(is_float_like_type(&TypeSpec::Real));
    assert!(is_float_like_type(&TypeSpec::Double));
    assert!(is_float_like_type(&TypeSpec::Decimal {
        precision: Some(10),
        scale: Some(2),
    }));
    assert!(!is_float_like_type(&TypeSpec::Integer));

    for numeric in [
        TypeSpec::Integer,
        TypeSpec::BigInt,
        TypeSpec::SmallInt,
        TypeSpec::Int128,
        TypeSpec::Int256,
        TypeSpec::Float { precision: None },
        TypeSpec::Float32,
        TypeSpec::Real,
        TypeSpec::Double,
        TypeSpec::Decimal {
            precision: None,
            scale: None,
        },
    ] {
        assert!(is_numeric_type(&numeric));
    }

    assert!(is_duration_type(&TypeSpec::Duration {
        precision: None,
        qualifier: None,
    }));
    assert!(is_boolean_type(&TypeSpec::Boolean));

    for temporal in [
        TypeSpec::Date,
        TypeSpec::Time {
            precision: None,
            with_timezone: false,
        },
        TypeSpec::Timestamp {
            precision: None,
            with_timezone: false,
        },
        TypeSpec::ZonedTime { precision: None },
        TypeSpec::ZonedDateTime { precision: None },
        TypeSpec::LocalTime { precision: None },
        TypeSpec::LocalDateTime { precision: None },
        TypeSpec::Duration {
            precision: None,
            qualifier: None,
        },
    ] {
        assert!(is_temporal_like_type(&temporal));
    }

    assert!(is_string_like_type(&TypeSpec::String { max_length: None }));
    assert!(is_string_like_type(&TypeSpec::Bytes { max_length: None }));
    assert!(is_concat_compatible_type(&TypeSpec::String {
        max_length: None
    }));
    assert!(is_concat_compatible_type(&TypeSpec::Path));
}

#[test]
fn numeric_promotion_prefers_float_like_types() {
    assert_eq!(
        promote_numeric_type(&TypeSpec::Integer, &TypeSpec::BigInt),
        TypeSpec::Integer
    );

    assert_eq!(
        promote_numeric_type(&TypeSpec::Integer, &TypeSpec::Double),
        TypeSpec::Float { precision: None }
    );
}

#[test]
fn cast_compatibility_covers_scalar_union_and_list_rules() {
    let string_type = TypeSpec::String { max_length: None };

    assert!(is_cast_compatible(&TypeSpec::Integer, &TypeSpec::Integer));
    assert!(is_cast_compatible(&TypeSpec::Integer, &TypeSpec::Any));
    assert!(is_cast_compatible(&TypeSpec::Boolean, &string_type));
    assert!(is_cast_compatible(&string_type, &TypeSpec::Integer));
    assert!(is_cast_compatible(&TypeSpec::Integer, &TypeSpec::Boolean));
    assert!(is_cast_compatible(
        &TypeSpec::Date,
        &TypeSpec::Timestamp {
            precision: None,
            with_timezone: false,
        }
    ));

    assert!(is_cast_compatible(
        &TypeSpec::Union {
            members: vec![TypeSpec::Integer, TypeSpec::Double],
        },
        &TypeSpec::Float { precision: None }
    ));

    assert!(is_cast_compatible(
        &TypeSpec::Integer,
        &TypeSpec::Union {
            members: vec![TypeSpec::Boolean, TypeSpec::Double, ],
        }
    ));

    assert!(is_cast_compatible(
        &list_type(TypeSpec::Integer),
        &list_type(TypeSpec::Double)
    ));

    assert!(!is_cast_compatible(
        &list_type(TypeSpec::Any),
        &list_type(TypeSpec::Integer)
    ));

    assert!(is_cast_compatible(&TypeSpec::Record, &TypeSpec::Record));
    assert!(is_cast_compatible(&TypeSpec::Path, &TypeSpec::Path));

    assert!(!is_cast_compatible(
        &TypeSpec::Boolean,
        &TypeSpec::Graph {
            graph_type_spec: None,
        }
    ));
}

#[test]
fn type_name_covers_all_type_spec_variants() {
    let cases = vec![
        (TypeSpec::Boolean, "BOOLEAN"),
        (TypeSpec::String { max_length: None }, "STRING"),
        (TypeSpec::Bytes { max_length: None }, "BYTES"),
        (
            TypeSpec::Decimal {
                precision: None,
                scale: None,
            },
            "DECIMAL",
        ),
        (TypeSpec::Integer, "INTEGER"),
        (TypeSpec::BigInt, "BIGINT"),
        (TypeSpec::SmallInt, "SMALLINT"),
        (TypeSpec::Int128, "INT128"),
        (TypeSpec::Int256, "INT256"),
        (TypeSpec::Float { precision: None }, "FLOAT"),
        (TypeSpec::Float32, "FLOAT32"),
        (TypeSpec::Real, "REAL"),
        (TypeSpec::Double, "DOUBLE"),
        (TypeSpec::Vector { dimension: None }, "VECTOR"),
        (TypeSpec::Date, "DATE"),
        (
            TypeSpec::Time {
                precision: None,
                with_timezone: false,
            },
            "TIME",
        ),
        (
            TypeSpec::Timestamp {
                precision: None,
                with_timezone: false,
            },
            "TIMESTAMP",
        ),
        (TypeSpec::ZonedTime { precision: None }, "ZONED TIME"),
        (
            TypeSpec::ZonedDateTime { precision: None },
            "ZONED DATETIME",
        ),
        (TypeSpec::LocalTime { precision: None }, "LOCAL TIME"),
        (
            TypeSpec::LocalDateTime { precision: None },
            "LOCAL DATETIME",
        ),
        (
            TypeSpec::Duration {
                precision: None,
                qualifier: None,
            },
            "DURATION",
        ),
        (TypeSpec::Reference { target_type: None }, "REFERENCE"),
        (TypeSpec::Path, "PATH"),
        (list_type(TypeSpec::Any), "LIST"),
        (TypeSpec::Record, "RECORD"),
        (
            TypeSpec::Graph {
                graph_type_spec: None,
            },
            "GRAPH",
        ),
        (TypeSpec::BindingTable, "BINDING TABLE"),
        (
            TypeSpec::Union {
                members: vec![TypeSpec::Integer],
            },
            "UNION",
        ),
        (TypeSpec::Any, "ANY"),
    ];

    for (type_spec, expected) in cases {
        assert_eq!(type_name(&type_spec), expected);
    }
}
