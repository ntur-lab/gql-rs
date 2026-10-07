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

use gql_parser::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};
use gql_parser::ast::model::*;

mod expression_tests {
    use super::*;



    #[test]
    fn test_return_record_literal_maps_to_expression_map() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("RETURN {name: 'Alice', age: 42} AS rec")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected RETURN result statement");
        };

        let first_item = return_clause
            .items
            .first()
            .expect("return item should exist");

        let Expression::Map(properties) = &first_item.expression else {
            panic!("record literal must map to Expression::Map");
        };

        assert_eq!(properties.len(), 2);
        assert_eq!(properties[0].key, "name");
        assert_eq!(properties[1].key, "age");
    }

    #[test]
    fn test_cast_duration_preserves_qualifier() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (n:Event) RETURN CAST(n.dur AS DURATION(YEAR TO MONTH)) AS d")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected RETURN result statement");
        };

        let first_item = return_clause
            .items
            .first()
            .expect("return item should exist");

        let Expression::Cast(cast_expr) = &first_item.expression else {
            panic!("expected CAST expression");
        };

        let TypeSpec::Duration { qualifier, .. } = &cast_expr.target_type else {
            panic!("expected DURATION target type");
        };

        assert_eq!(qualifier.as_deref(), Some("YEAR TO MONTH"));
    }

    #[test]
    fn test_cast_graph_reference_type_populates_nested_specification() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (n) RETURN CAST(n.graph AS PROPERTY GRAPH { NODE TYPE Person, DIRECTED EDGE TYPE Knows CONNECTING (person TO person) }) AS g")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected RETURN result statement");
        };

        let first_item = return_clause
            .items
            .first()
            .expect("return item should exist");

        let Expression::Cast(cast_expr) = &first_item.expression else {
            panic!("expected CAST expression");
        };

        let TypeSpec::Graph { graph_type_spec } = &cast_expr.target_type else {
            panic!("expected GRAPH target type");
        };

        let graph_type_spec = graph_type_spec
            .as_ref()
            .expect("closed graph reference should populate graph_type_spec");

        assert_eq!(graph_type_spec.vertex_types.len(), 1);
        assert_eq!(graph_type_spec.edge_types.len(), 1);
        assert_eq!(
            graph_type_spec.vertex_types[0].identifier.as_deref(),
            Some("Person")
        );
        assert_eq!(
            graph_type_spec.edge_types[0].identifier.as_deref(),
            Some("Knows")
        );
        assert_eq!(
            graph_type_spec.edge_types[0].source_vertex.as_deref(),
            Some("person")
        );
        assert_eq!(
            graph_type_spec.edge_types[0].destination_vertex.as_deref(),
            Some("person")
        );
    }

    #[test]
    fn test_type_check_rejects_incompatible_plus_operands() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN 'abc' + true");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("requires compatible numeric or duration operands"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_string_plus_string() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec(r#"RETURN "abc" + "abc""#);

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("cannot concatenate strings; use '||' instead"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_invalid_cast_target() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST(true AS DATE)");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Invalid CAST target"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_aggregate_plus_boolean() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH (n) RETURN COUNT(n) + true");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("requires compatible numeric or duration operands"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_timestamp_cast_to_integer() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST(CURRENT_TIMESTAMP AS INTEGER)");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Invalid CAST target"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_mixed_array_cast_to_typed_list() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST([1, 2, 'a'] AS LIST<INTEGER>)");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Array literal contains incompatible element types")
                        || msg.contains("Invalid CAST target"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_accepts_homogeneous_array_cast_to_typed_list() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST([1, 2, 3] AS LIST<INTEGER>)");

        if let Err(err) = result {
            panic!("Expected successful cast for homogeneous list, got: {:?}", err);
        }
    }

    #[test]
    fn test_type_check_accepts_typed_list_cast_with_dynamic_parameter() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST([1, $x] AS LIST<INTEGER>)");

        if let Err(err) = result {
            panic!(
                "Expected typed list cast with dynamic parameter to be validated at runtime, got: {:?}",
                err
            );
        }
    }

    #[test]
    fn test_type_check_rejects_case_union_invalid_cast_target() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CAST(CASE WHEN true THEN 1 ELSE false END AS DATE)");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Invalid CAST target"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_not_on_numeric_literal() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN NOT 1");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Unary operator NOT requires BOOLEAN operand"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_case_searched_non_boolean_condition() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CASE WHEN 1 THEN 'a' ELSE 'b' END");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("CASE searched WHEN requires a BOOLEAN condition"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_case_simple_incompatible_when_value() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN CASE 1 WHEN 'a' THEN 1 ELSE 2 END");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("CASE simple WHEN value type"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_coalesce_incompatible_args() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN COALESCE(1, 'x')");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("COALESCE arguments must be mutually compatible"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_type_check_rejects_nested_aggregate_functions() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH (n) RETURN MAX(COUNT(n))");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("cannot contain nested aggregate expressions"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }
}
