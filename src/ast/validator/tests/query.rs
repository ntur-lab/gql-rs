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

use super::super::query::*;
use crate::ast::model::*;
use crate::ast::validator::tests::common_tests::*;
use std::collections::HashSet;

#[test]
fn path_term_slices_uses_elements_or_alternatives() {
    let linear = path(vec![PatternElement::Node(node(Some("n")))]);
    assert_eq!(path_pattern_term_slices(&linear).len(), 1);

    let mut alternating = path(vec![]);
    alternating.alternation = Some(PathAlternation {
        operator: PathAlternationOperator::Union,
        alternatives: vec![
            vec![PatternElement::Node(node(Some("a")))],
            vec![PatternElement::Node(node(Some("b")))],
        ],
        span: span(),
    });
    let slices = path_pattern_term_slices(&alternating);
    assert_eq!(slices.len(), 2);
    assert_eq!(slices[0].len(), 1);
}

#[test]
fn match_clause_registers_variables_and_rejects_bad_shapes() {
    let mut valid_path = path(vec![
        PatternElement::Node(node(Some("a"))),
        PatternElement::Edge(edge(Some("e"))),
        PatternElement::Node(node(Some("b"))),
    ]);
    valid_path.variable = Some("p".to_string());
    let clause = MatchClause {
        optional: false,
        match_clause: MatchClauseItem {
            patterns: vec![valid_path],
            span: span(),
        },
        span: span(),
    };
    let mut declared = HashSet::new();
    assert!(validate_match_clause(&clause, &mut declared).is_ok());
    for expected in ["p", "a", "e", "b"] {
        assert!(declared.contains(expected));
    }

    let bad_clause = MatchClause {
        optional: false,
        match_clause: MatchClauseItem {
            patterns: vec![path(vec![
                PatternElement::Edge(edge(Some("e"))),
                PatternElement::Node(node(Some("n"))),
            ])],
            span: span(),
        },
        span: span(),
    };
    assert!(validate_match_clause(&bad_clause, &mut HashSet::new()).is_err());
}

#[test]
fn path_mode_rules_reject_shortest_walk_and_self_loops() {
    let mut shortest_walk = path(vec![PatternElement::Node(node(Some("n")))]);
    shortest_walk.path_mode = Some(PathMode::Walk);
    shortest_walk.path_search = Some(PathSearch::Shortest);
    assert!(validate_path_mode_search_compatibility(&shortest_walk).is_err());

    shortest_walk.path_search = Some(PathSearch::Any);
    assert!(validate_path_mode_search_compatibility(&shortest_walk).is_ok());

    let mut self_loop = path(vec![
        PatternElement::Node(node(Some("n"))),
        PatternElement::Edge(edge(Some("e"))),
        PatternElement::Node(node(Some("n"))),
    ]);
    self_loop.path_mode = Some(PathMode::Acyclic);
    assert!(validate_path_mode_cycle_constraints(&self_loop).is_err());

    self_loop.path_mode = Some(PathMode::Trail);
    assert!(validate_path_mode_cycle_constraints(&self_loop).is_ok());
}

#[test]
fn node_edge_and_property_validation_cover_shape_errors() {
    let mut declared = HashSet::new();
    assert!(validate_node(&node(Some(" ")), &mut declared).is_err());
    assert!(validate_edge(&edge(Some(" ")), &mut declared, None).is_err());

    let mut bad_range = edge(Some("e"));
    bad_range.quantifier = Some(PathQuantifier::Range { min: 5, max: 2 });
    assert!(validate_edge_quantifier(&bad_range, None).is_err());

    let mut open = edge(Some("e"));
    open.quantifier = Some(PathQuantifier::AtLeast(1));
    assert!(validate_edge_quantifier(&open, Some(&PathMode::Simple)).is_err());
    assert!(validate_edge_quantifier(&open, Some(&PathMode::Trail)).is_ok());

    let invalid_map = PropertyMap {
        properties: vec![PropertyItem {
            key: " ".to_string(),
            value: integer(1),
            span: span(),
        }],
        span: span(),
    };
    assert!(validate_property_map(&invalid_map, &declared).is_err());

    let valid_map = PropertyMap {
        properties: vec![PropertyItem {
            key: "age".to_string(),
            value: integer(1),
            span: span(),
        }],
        span: span(),
    };
    assert!(validate_property_map(&valid_map, &declared).is_ok());
}

#[test]
fn let_statement_validates_presence_names_duplicates_and_incremental_scope() {
    let empty = LetStatement {
        variable_definitions: vec![],
        span: span(),
    };
    assert!(validate_let_statement(&empty, &mut HashSet::new()).is_err());

    let unnamed = LetStatement {
        variable_definitions: vec![VariableDefinition {
            variable_name: " ".to_string(),
            expression: integer(1),
            span: span(),
        }],
        span: span(),
    };
    assert!(validate_let_statement(&unnamed, &mut HashSet::new()).is_err());

    let duplicate = LetStatement {
        variable_definitions: vec![
            VariableDefinition {
                variable_name: "x".to_string(),
                expression: integer(1),
                span: span(),
            },
            VariableDefinition {
                variable_name: "x".to_string(),
                expression: integer(2),
                span: span(),
            },
        ],
        span: span(),
    };
    assert!(validate_let_statement(&duplicate, &mut HashSet::new()).is_err());

    let valid = LetStatement {
        variable_definitions: vec![
            VariableDefinition {
                variable_name: "x".to_string(),
                expression: integer(1),
                span: span(),
            },
            VariableDefinition {
                variable_name: "y".to_string(),
                expression: variable("x"),
                span: span(),
            },
        ],
        span: span(),
    };
    let mut declared = HashSet::new();
    assert!(validate_let_statement(&valid, &mut declared).is_ok());
    assert!(declared.contains("x") && declared.contains("y"));
}

#[test]
fn projected_output_names_prefer_aliases_and_derive_stable_names() {
    assert_eq!(derive_output_names_from_expression(&variable("value")), vec!["value"]);
    assert_eq!(
        derive_output_names_from_expression(&property("n", "name")),
        vec!["name", "n.name"]
    );
    assert!(derive_output_names_from_expression(&integer(1)).is_empty());

    let result = PrimitiveResultStatement::Return {
        return_clause: return_clause(vec![(variable("n"), Some("person")), (property("n", "name"), None)]),
        group_clause: None,
        page_clause: None,
        span: span(),
    };
    let names = collect_projected_output_names_from_result(&result);
    for expected in ["person", "name", "n.name"] {
        assert!(names.contains(expected));
    }
}

#[test]
fn return_clause_rejects_empty_and_duplicate_aliases() {
    let empty = return_clause(vec![]);
    assert!(validate_return_clause_with_aliases(&empty, &HashSet::new(), &mut HashSet::new(),).is_err());

    let bad_alias = return_clause(vec![(integer(1), Some(" "))]);
    assert!(validate_return_clause_with_aliases(&bad_alias, &HashSet::new(), &mut HashSet::new(),).is_err());

    let duplicate = return_clause(vec![(integer(1), Some("value")), (integer(2), Some("value"))]);
    assert!(validate_return_clause_with_aliases(&duplicate, &HashSet::new(), &mut HashSet::new(),).is_err());
}

#[test]
fn return_clause_accepts_wildcard_projection() {
    let wildcard = ReturnClause {
        distinct: DistinctQualifier::None,
        projection: ReturnProjection::Wildcard,
        span: span(),
    };

    assert!(validate_return_clause_with_aliases(&wildcard, &HashSet::new(), &mut HashSet::new()).is_ok());
}

#[test]
fn group_clause_validates_empty_sets_keys_and_scope() {
    let declared = HashSet::from(["n".to_string()]);
    let aliases = HashSet::from(["person".to_string()]);

    assert!(validate_group_clause(&group(None, true), &declared, &aliases).is_ok());
    assert!(validate_group_clause(&group(Some(vec!["n"]), true), &declared, &aliases).is_err());
    assert!(validate_group_clause(&group(None, false), &declared, &aliases).is_err());
    assert!(validate_group_clause(&group(Some(vec![]), false), &declared, &aliases).is_err());
    assert!(validate_group_clause(&group(Some(vec!["missing"]), false), &declared, &aliases).is_err());
    assert!(validate_group_clause(&group(Some(vec!["n"]), false), &declared, &aliases).is_ok());
    assert!(validate_group_clause(&group(Some(vec!["person"]), false), &declared, &aliases).is_ok());
}

#[test]
fn non_aggregated_return_variables_must_be_grouped() {
    let returns = return_clause(vec![
        (variable("n"), None),
        (function("COUNT", vec![variable("m")]), Some("total")),
    ]);
    assert!(validate_non_aggregated_return_variables_in_group_by(&returns, &group(Some(vec!["n"]), false),).is_ok());
    assert!(validate_non_aggregated_return_variables_in_group_by(&returns, &group(Some(vec!["other"]), false),).is_err());

    let aliased = return_clause(vec![(property("n", "name"), Some("person"))]);
    assert!(validate_non_aggregated_return_variables_in_group_by(&aliased, &group(Some(vec!["person"]), false),).is_ok());
}

#[test]
fn collect_non_aggregated_refs_traverses_composite_expressions() {
    let expression = Expression::Map(vec![PropertyItem {
        key: "value".to_string(),
        value: Expression::Array(ArrayExpression {
            elements: vec![
                variable("a"),
                Expression::Cast(CastExpression {
                    expression: Box::new(property("b", "name")),
                    target_type: TypeSpec::String { max_length: None },
                    span: span(),
                }),
                Expression::WhenOperand(WhenOperandExpression {
                    kind: WhenOperandKind::Comparison {
                        operator: Operator::Equal,
                        right: Box::new(variable("c")),
                    },
                    span: span(),
                }),
            ],
            span: span(),
        }),
        span: span(),
    }]);
    let mut refs = HashSet::new();
    collect_non_aggregated_variable_refs(&expression, false, &mut refs);
    assert_eq!(refs, HashSet::from(["a".to_string(), "b".to_string(), "c".to_string()]));

    let mut aggregate_refs = HashSet::new();
    collect_non_aggregated_variable_refs(&function("SUM", vec![variable("ignored")]), false, &mut aggregate_refs);
    assert!(aggregate_refs.is_empty());
}

#[test]
fn order_limit_and_page_values_validate_shape() {
    let empty_order = OrderClause {
        items: vec![],
        span: span(),
    };
    assert!(validate_order_clause(&empty_order, &HashSet::new(), None).is_err());

    let order = OrderClause {
        items: vec![OrderItem {
            expression: variable("alias"),
            direction: OrderDirection::Descending,
            nulls_ordering: Some(NullsOrdering::Last),
            span: span(),
        }],
        span: span(),
    };
    assert!(validate_order_clause(&order, &HashSet::new(), Some(&HashSet::from(["alias".to_string()])),).is_ok());

    let empty_limit = LimitClause {
        count: None,
        offset: None,
        span: span(),
    };
    assert!(validate_limit_clause(&empty_limit).is_err());
    assert!(
        validate_page_value(
            "LIMIT",
            &PageValue::Parameter {
                name: "".to_string(),
                span: span()
            },
        )
        .is_err()
    );
    assert!(
        validate_page_value(
            "LIMIT",
            &PageValue::Parameter {
                name: "limit".to_string(),
                span: span()
            },
        )
        .is_err()
    );
    assert!(
        validate_page_value(
            "LIMIT",
            &PageValue::Parameter {
                name: "$limit".to_string(),
                span: span()
            },
        )
        .is_ok()
    );
}

#[test]
fn case_and_predicate_validation_cover_simple_and_searched_forms() {
    let simple = CaseExpression {
        case_type: CaseType::Simple(SimpleCaseExpression {
            test_expression: Box::new(integer(1)),
            when_branches: vec![SimpleWhenBranch {
                when_values: vec![integer(1)],
                then_expression: Box::new(string("one")),
                span: span(),
            }],
            else_expression: Some(Box::new(string("other"))),
        }),
        span: span(),
    };
    assert!(validate_case_expression(&simple, &HashSet::new()).is_ok());

    let incompatible = CaseExpression {
        case_type: CaseType::Simple(SimpleCaseExpression {
            test_expression: Box::new(boolean(true)),
            when_branches: vec![SimpleWhenBranch {
                when_values: vec![integer(1)],
                then_expression: Box::new(string("bad")),
                span: span(),
            }],
            else_expression: None,
        }),
        span: span(),
    };
    assert!(validate_case_expression(&incompatible, &HashSet::new()).is_err());

    let searched = CaseExpression {
        case_type: CaseType::Searched(SearchedCaseExpression {
            when_branches: vec![SearchedWhenBranch {
                condition: Box::new(boolean(true)),
                then_expression: Box::new(integer(1)),
                span: span(),
            }],
            else_expression: Some(Box::new(integer(0))),
        }),
        span: span(),
    };
    assert!(validate_case_expression(&searched, &HashSet::new()).is_ok());
    assert!(validate_predicate_condition_type(&boolean(true), &HashSet::new(), "WHERE").is_ok());
    assert!(validate_predicate_condition_type(&integer(1), &HashSet::new(), "WHERE").is_err());
    assert!(validate_predicate_condition_type(&variable("unknown"), &HashSet::new(), "WHERE").is_ok());
}
