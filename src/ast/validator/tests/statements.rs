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

use super::super::statements::*;
use crate::ast::model::*;
use crate::ast::validator::tests::common_tests::*;

fn validate_catalog_test_statement(statement: &CatalogStatement) -> Result<(), crate::GqlError> {
    validate_catalog_statement(statement, &mut std::collections::HashSet::new())
}

#[test]
fn session_activity_enforces_nonempty_commands_and_set_before_reset() {
    use super::super::program::validate_session_activity;
    let set = SessionCommand::Set {
        clause: SessionSetClause::Schema { schema_reference: "CURRENT_SCHEMA".to_string() },
        span: span(),
    };
    let reset = SessionCommand::Reset { clause: None, span: span() };
    for (commands, valid) in [
        (vec![], false),
        (vec![set.clone(), reset.clone()], true),
        (vec![reset.clone()], true),
        (vec![reset, set], false),
    ] {
        assert_eq!(validate_session_activity(&SessionActivity { commands, span: span() }).is_ok(), valid);
    }
}

#[test]
fn session_set_clause_validates_schema_timezone_graph_and_parameter() {
    assert!(
        validate_session_set_clause(&SessionSetClause::Schema {
            schema_reference: " ".to_string(),
        })
            .is_err()
    );
    assert!(
        validate_session_set_clause(&SessionSetClause::TimeZone {
            value: SessionTimeZoneValue::Literal(" ".to_string()),
        })
            .is_err()
    );
    assert!(
        validate_session_set_clause(&SessionSetClause::Graph {
            property_graph_keyword: false,
            graph_expression: graph_expression(GraphExpressionKind::GraphReference, ""),
        })
            .is_err()
    );

    let empty_name = SessionSetClause::Parameter(Box::new(SessionSetParameterClause {
        kind: SessionSetParameterKind::Value,
        parameter_name: SessionSetParameterName {
            if_not_exists: false,
            parameter_specification: " ".to_string(),
            span: span(),
        },
        initializer: Some("1".to_string()),
        graph_initializer: None,
        value_initializer: None,
        binding_table_initializer: None,
    }));
    assert!(validate_session_set_clause(&empty_name).is_err());

    let empty_initializer = SessionSetClause::Parameter(Box::new(SessionSetParameterClause {
        kind: SessionSetParameterKind::Value,
        parameter_name: SessionSetParameterName {
            if_not_exists: false,
            parameter_specification: "$value".to_string(),
            span: span(),
        },
        initializer: Some(" ".to_string()),
        graph_initializer: None,
        value_initializer: None,
        binding_table_initializer: None,
    }));
    assert!(validate_session_set_clause(&empty_initializer).is_err());
}

#[test]
fn session_reset_parameter_requires_a_name() {
    let empty = reset(SessionResetTarget::Parameter {
        has_parameter_keyword: true,
        parameter_specification: " ".to_string(),
    });
    assert!(validate_session_reset_clause(&empty).is_err());

    let valid = reset(SessionResetTarget::Parameter {
        has_parameter_keyword: true,
        parameter_specification: "$value".to_string(),
    });
    assert!(validate_session_reset_clause(&valid).is_ok());

    for target in [
        SessionResetTarget::Schema,
        SessionResetTarget::TimeZone,
        SessionResetTarget::Graph {
            property_graph_keyword: false,
        },
    ] {
        assert!(validate_session_reset_clause(&reset(target)).is_ok());
    }
}

#[test]
fn catalog_block_and_raw_statement_cannot_be_empty() {
    assert!(validate_catalog_statement_block(&CatalogStatementBlock { statements: vec![] }, &mut std::collections::HashSet::new()).is_err());

    let raw = CatalogStatement::Raw(RawCatalogStatement {
        text: " ".to_string(),
        parse_error: None,
        span: span(),
    });
    assert!(validate_catalog_test_statement(&raw).is_err());

    let valid = CatalogStatementBlock {
        statements: vec![CatalogStatement::Raw(RawCatalogStatement {
            text: "CUSTOM CATALOG STATEMENT".to_string(),
            parse_error: None,
            span: span(),
        })],
    };
    assert!(validate_catalog_statement_block(&valid, &mut std::collections::HashSet::new()).is_ok());
}

#[test]
fn catalog_names_validate_required_components() {
    assert!(validate_catalog_schema_name(&schema("", "main")).is_err());
    assert!(validate_catalog_schema_name(&schema("/catalog", "")).is_err());
    assert!(validate_catalog_schema_name(&schema("/catalog", "main")).is_ok());

    assert!(validate_catalog_object_name(&object(None, "")).is_err());
    assert!(validate_catalog_object_name(&object(Some(""), "graph")).is_err());
    assert!(validate_catalog_object_name(&object(Some("catalog"), "graph")).is_ok());
}

#[test]
fn graph_type_clauses_validate_references_and_nested_specs() {
    let open = CreateGraphTypeClause::Open(OpenGraphTypeClause {
        typed: false,
        property_graph_keyword: false,
        graph_keyword: true,
    });
    assert!(validate_create_graph_type_clause(&open).is_ok());

    let empty_reference = CreateGraphTypeClause::Of(OfGraphTypeClause::TypedReference {
        typed: true,
        graph_type_reference: " ".to_string(),
    });
    assert!(validate_create_graph_type_clause(&empty_reference).is_err());

    assert!(
        validate_graph_type_source_clause(&GraphTypeSourceClause::CopyOf {
            with_as_keyword: false,
            graph_type_reference: " ".to_string(),
        })
            .is_err()
    );
    assert!(
        validate_graph_type_source_clause(&GraphTypeSourceClause::CopyOf {
            with_as_keyword: false,
            graph_type_reference: "base_type".to_string(),
        })
            .is_ok()
    );

    let empty_nested = NestedGraphTypeSpecification {
        element_type_specifications: vec![],
    };
    assert!(validate_nested_graph_type_specification(&empty_nested).is_err());

    let valid_nested = NestedGraphTypeSpecification {
        element_type_specifications: vec![ElementTypeSpecification::Node(NodeTypeSpecification {
            text: "NODE TYPE Person".to_string(),
            local_alias: Some("p".to_string()),
            type_name: Some("Person".to_string()),
            property_types: None,
        })],
    };
    assert!(validate_nested_graph_type_specification(&valid_nested).is_ok());
}

#[test]
fn nested_graph_type_validates_node_edge_and_property_payloads() {
    let empty_node = NestedGraphTypeSpecification {
        element_type_specifications: vec![ElementTypeSpecification::Node(NodeTypeSpecification {
            text: " ".to_string(),
            local_alias: None,
            type_name: None,
            property_types: None,
        })],
    };
    assert!(validate_nested_graph_type_specification(&empty_node).is_err());

    let empty_edge = NestedGraphTypeSpecification {
        element_type_specifications: vec![ElementTypeSpecification::Edge(EdgeTypeSpecification {
            text: " ".to_string(),
            type_name: None,
            source_alias: None,
            destination_alias: None,
            directed: None,
            property_types: None,
        })],
    };
    assert!(validate_nested_graph_type_specification(&empty_edge).is_err());

    let empty_property_name = PropertyTypesSpecification {
        properties: vec![PropertyTypeSpecification {
            name: " ".to_string(),
            typed: false,
            value_type: "STRING".to_string(),
            parsed_value_type: Some(ValueType::nullable(TypeSpec::string())),
            span: span(),
        }],
    };
    assert!(validate_property_types_specification(&empty_property_name).is_err());

    let empty_property_type = PropertyTypesSpecification {
        properties: vec![PropertyTypeSpecification {
            name: "name".to_string(),
            typed: false,
            value_type: " ".to_string(),
            parsed_value_type: None,
            span: span(),
        }],
    };
    assert!(validate_property_types_specification(&empty_property_type).is_err());
}

#[test]
fn graph_expression_validation_covers_kinds_and_delimiters() {
    assert!(validate_graph_expression_clause(&graph_expression(GraphExpressionKind::GraphReference, "",)).is_err());
    assert!(validate_graph_expression_clause(&graph_expression(GraphExpressionKind::CurrentGraph, "social",)).is_err());
    assert!(
        validate_graph_expression_clause(&graph_expression(GraphExpressionKind::GraphReference, "CURRENT_GRAPH",)).is_err()
    );
    assert!(
        validate_graph_expression_clause(&graph_expression(GraphExpressionKind::GraphReference, "graph(name)",)).is_err()
    );
    assert!(
        validate_graph_expression_clause(&graph_expression(
            GraphExpressionKind::GraphReference,
            "VARIABLE social",
        ))
            .is_err()
    );
    assert!(
        validate_graph_expression_clause(&graph_expression(
            GraphExpressionKind::ObjectNameOrBindingVariable,
            "graph[name]",
        ))
            .is_err()
    );
    assert!(validate_graph_expression_clause(&graph_expression(GraphExpressionKind::GraphReference, "social",)).is_ok());
    assert!(
        validate_graph_expression_clause(&graph_expression(GraphExpressionKind::CurrentGraph, "CURRENT_GRAPH",)).is_ok()
    );

    for text in ["(graph)", "{graph}", "[graph]"] {
        assert!(contains_structural_delimiters(text));
    }
    assert!(!contains_structural_delimiters("catalog/social"));
}

#[test]
fn catalog_create_graph_rejects_conflicting_modifiers() {
    let statement = CatalogStatement::CreateGraph(CreateGraph {
        or_replace: true,
        property_graph_keyword: false,
        if_not_exists: true,
        target: object(None, "social"),
        graph_type_clause: CreateGraphTypeClause::Open(OpenGraphTypeClause {
            typed: false,
            property_graph_keyword: false,
            graph_keyword: true,
        }),
        graph_source: None,
    });
    assert!(validate_catalog_test_statement(&statement).is_err());

    let graph_type = CatalogStatement::CreateGraphType(CreateGraphType {
        or_replace: true,
        property_graph_keyword: false,
        if_not_exists: true,
        target: object(None, "social_type"),
        source: GraphTypeSourceClause::CopyOf {
            with_as_keyword: false,
            graph_type_reference: "base".to_string(),
        },
    });
    assert!(validate_catalog_test_statement(&graph_type).is_err());
}

#[test]
fn yield_clause_validates_names_aliases_and_duplicates() {
    assert!(
        validate_yield_clause(&YieldClause {
            items: vec![],
            span: span(),
        })
            .is_err()
    );
    assert!(
        validate_yield_clause(&YieldClause {
            items: vec![yield_item("", None)],
            span: span(),
        })
            .is_err()
    );
    assert!(
        validate_yield_clause(&YieldClause {
            items: vec![yield_item("value", Some(""))],
            span: span(),
        })
            .is_err()
    );
    assert!(
        validate_yield_clause(&YieldClause {
            items: vec![yield_item("a", Some("value")), yield_item("b", Some("value"))],
            span: span(),
        })
            .is_err()
    );
    assert!(
        validate_yield_clause(&YieldClause {
            items: vec![yield_item("a", Some("one")), yield_item("b", Some("two"))],
            span: span(),
        })
            .is_ok()
    );
}

#[test]
fn next_statement_propagates_yield_names() {
    let next = NextStatement {
        source_statement: Box::new(finish_statement()),
        target_statement: Box::new(finish_statement()),
        yield_clause: Some(YieldClause {
            items: vec![yield_item("source", Some("projected"))],
            span: span(),
        }),
        next: None,
        span: span(),
    };
    let mut declared = std::collections::HashSet::new();
    assert!(validate_next_statement(&next, &mut declared).is_ok());
    assert!(declared.contains("projected"));
}
