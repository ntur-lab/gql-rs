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

mod expression_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, ast::model::*};
    #[test]
    fn rejects_integer_literals_outside_signed_64_bit_range() {
        for query in [
            "RETURN 9223372036854775808 AS a",
            "RETURN -9223372036854775809 AS a",
            "RETURN +9223372036854775808 AS a",
            "RETURN [9223372036854775808] AS a",
            "RETURN {a: 9223372036854775808} AS a",
            "RETURN 9999999999999999999999999999999999999999 AS a",
        ] {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            let error = analyzer
                .exec(query)
                .expect_err("out-of-range integer must fail");
            assert!(
                error
                    .to_string()
                    .contains("outside the signed 64-bit range"),
                "query: {query}, error: {error}"
            );
        }
    }

    #[test]
    fn accepts_integer_boundaries_and_floating_point_syntax() {
        for query in [
            "RETURN 9223372036854775807 AS a",
            "RETURN -9223372036854775808 AS a",
            "RETURN -009223372036854775808 AS a",
            "RETURN 3.14 AS a, 1e3 AS b",
            "RETURN 9223372036854775808.0 AS a",
            "RETURN '9223372036854775808' AS a",
        ] {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec(query)
                .unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        }
    }

    #[test]
    fn test_string_literals_decode_doubled_delimiters_and_preserve_content() {
        for (literal, expected) in [
            ("'it''s'", "it's"),
            ("''", ""),
            ("''''", "'"),
            ("'''quoted'''", "'quoted'"),
            ("'one''''two'", "one''two"),
            ("'  Zoë 北京 🙂 it''s  '", "  Zoë 北京 🙂 it's  "),
            ("'\"quoted\"'", "\"quoted\""),
            ("\"'quoted'\"", "'quoted'"),
            ("\"say \"\"hello\"\"\"", "say \"hello\""),
        ] {
            let query_text = format!("RETURN {literal} AS a");
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec(&query_text)
                .expect("string query should parse");

            let ast = analyzer.get_ast().expect("AST must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };
            let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement
            else {
                panic!("expected RETURN result statement");
            };
            let Expression::Literal(Literal::String(value)) = &return_clause.items[0].expression
            else {
                panic!("expected string literal for {query_text}");
            };
            assert_eq!(value, expected, "query: {query_text}");
        }
    }

    #[test]
    fn accepts_null_and_nested_lists_without_flattening_the_ast() {
        for query in [
            "RETURN [1, null, [2, 3]] AS l",
            "RETURN [[2, 3], null, 1] AS l",
            "RETURN [1, [], [[2]]] AS l",
        ] {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec(query)
                .unwrap_or_else(|error| panic!("{query}: {error}"));
            let ast = analyzer.get_ast().unwrap();
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected query")
            };
            let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement
            else {
                panic!("expected RETURN");
            };
            let Expression::Array(array) = &return_clause.items[0].expression else {
                panic!("expected list")
            };
            assert_eq!(array.elements.len(), 3);
            assert!(
                array
                    .elements
                    .iter()
                    .any(|item| matches!(item, Expression::Array(_)))
            );
        }
    }

    #[test]
    fn rejects_nested_list_when_cast_to_scalar_element_list() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let error = analyzer
            .exec("RETURN CAST([1, null, [2, 3]] AS LIST<INTEGER>)")
            .expect_err("a nested list is not an integer element");
        assert!(matches!(error, GqlError::Validation { .. }));
    }

    #[test]
    fn test_delimited_property_names_are_unquoted_in_property_access() {
        for (query_text, expected_property) in [
            ("MATCH (n) RETURN n.\"year\"", "year"),
            ("MATCH (n) RETURN n.`year`", "year"),
            ("MATCH (n) RETURN n.\"release\"\"year\"", "release\"year"),
            ("MATCH (n) RETURN n.`release``year`", "release`year"),
            ("MATCH (n) RETURN n.\"release\\u005Fyear\"", "release_year"),
        ] {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer.exec(query_text).expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement
            else {
                panic!("expected RETURN result statement");
            };

            let Expression::PropertyAccess(property_access) = &return_clause.items[0].expression
            else {
                panic!("expected property access expression");
            };

            assert_eq!(
                property_access.property, expected_property,
                "query: {query_text}"
            );
        }
    }

    #[test]
    fn test_delimited_property_names_are_unquoted_in_data_modification() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (n) SET n.\"year\" = 2020")
            .expect("SET query should parse");

        let ast = analyzer.get_ast().expect("SET AST must be available");
        let Statement::Data(DataStatement::MatchSet(statement)) = &ast.statement else {
            panic!("expected MATCH SET statement");
        };
        let SetItem::Property { property, .. } = &statement.items[0] else {
            panic!("expected SET property item");
        };
        assert_eq!(property.property, "year");

        analyzer
            .exec("MATCH (n) REMOVE n.`year`")
            .expect("REMOVE query should parse");

        let ast = analyzer.get_ast().expect("REMOVE AST must be available");
        let Statement::Data(DataStatement::MatchRemove(statement)) = &ast.statement else {
            panic!("expected MATCH REMOVE statement");
        };
        let RemoveItem::Property { property, .. } = &statement.remove_clause[0] else {
            panic!("expected REMOVE property item");
        };
        assert_eq!(property.property, "year");
    }

    #[test]
    fn test_delimited_record_fields_and_return_aliases_are_unquoted() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("RETURN {\"year\": 2020} AS `result`")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("AST must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected RETURN result statement");
        };
        assert_eq!(return_clause.items[0].alias.as_deref(), Some("result"));
        let Expression::Map(properties) = &return_clause.items[0].expression else {
            panic!("expected record literal");
        };
        assert_eq!(properties[0].key, "year");
    }

    #[test]
    fn test_delimited_pattern_labels_and_property_keys_are_unquoted() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (n:\"select\" {`year`: 2020}) RETURN n")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("AST must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };
        let QueryStep::Match(match_clause) = &query.pipeline[0] else {
            panic!("expected MATCH query step");
        };
        let PatternElement::Node(node) = &match_clause.match_clause.patterns[0].elements[0] else {
            panic!("expected node pattern");
        };
        let Some(Expression::Literal(Literal::String(label))) = &node.labels else {
            panic!("expected literal node label");
        };
        assert_eq!(label, "select");
        assert_eq!(
            node.properties
                .as_ref()
                .expect("properties must exist")
                .properties[0]
                .key,
            "year"
        );
    }

    #[test]
    fn test_delimited_qualified_procedure_names_are_unquoted() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CALL \"db\".`labels`()")
            .expect("CALL query should parse");

        let ast = analyzer.get_ast().expect("AST must be available");
        match &ast.statement {
            Statement::Query(Query::Basic(query)) => {
                let QueryStep::Call(call) = &query.pipeline[0] else {
                    panic!("expected CALL query step");
                };
                assert_eq!(call.procedure_name, "db.labels");
            }
            Statement::Data(DataStatement::Call(statement)) => {
                assert_eq!(statement.call_clause.procedure_name, "db.labels");
            }
            Statement::Catalog(block) => {
                let CatalogStatement::Call(statement) = &block.statements[0] else {
                    panic!("expected catalog CALL statement");
                };
                assert_eq!(statement.call_clause.procedure_name, "db.labels");
            }
            other => panic!("expected CALL statement, got: {other:?}"),
        }
    }

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
            panic!(
                "Expected successful cast for homogeneous list, got: {:?}",
                err
            );
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

    #[test]
    fn test_validator_rejects_subtraction_on_incompatible_operands() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("RETURN true - 10")
            .expect_err("subtraction between boolean and integer must fail validation");

        match err {
            GqlError::Validation { message, .. } => {
                assert!(
                    message.contains("requires compatible numeric or duration operands"),
                    "unexpected error message: {message}"
                );
            }
            other => panic!("expected Validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_math_functions_map_to_function_call() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec("RETURN ABS(-5), CEIL(4.2), FLOOR(4.8), SQRT(16.0)").expect("query should parse");
        let ast = analyzer.get_ast().expect("AST must be available");

        let Statement::Query(Query::Basic(query)) = &ast.statement else { panic!("expected query"); };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else { panic!("expected return"); };

        let expected_names = ["ABS", "CEIL", "FLOOR", "SQRT"];
        for (i, expected_name) in expected_names.iter().enumerate() {
            let Expression::FunctionCall(call) = &return_clause.items[i].expression else {
                panic!("item {i} must be FunctionCall");
            };
            assert_eq!(call.name, *expected_name);
        }
    }

    #[test]
    fn test_arithmetic_subtraction_builds_binary_minus() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("RETURN 20 - 5")
            .expect("query should parse");
        let ast = analyzer.get_ast().expect("AST must be available");

        let Statement::Query(Query::Basic(query)) = &ast.statement else { panic!("expected query"); };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else { panic!("expected return"); };
        let Expression::Binary(binary) = &return_clause.items[0].expression else {
            panic!("expected binary minus expression");
        };
        assert_eq!(binary.operator, Operator::Minus);
    }

    #[test]
    fn test_validator_rejects_aggregate_in_where_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("MATCH (n:Person) WHERE sum(n.age) > 50 RETURN n")
            .expect_err("aggregate in WHERE should be rejected");

        match err {
            GqlError::Validation { message, .. } => {
                assert!(
                    message.to_lowercase().contains("aggregate"),
                    "expected aggregate restriction error, got: {message}"
                );
            }
            other => panic!("expected Validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_modulus_function_builds_function_call() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec("RETURN MOD(10, 3)").expect("query should parse");
        let ast = analyzer.get_ast().expect("AST must be available");

        let Statement::Query(Query::Basic(query)) = &ast.statement else { panic!("expected query"); };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else { panic!("expected return"); };
        let Expression::FunctionCall(call) = &return_clause.items[0].expression else {
            panic!("expected function call for MOD");
        };
        assert_eq!(call.name, "MOD");
        assert_eq!(call.arguments.len(), 2);
    }

    #[test]
    fn test_trim_with_explicit_spec_builds_call() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec("RETURN trim(BOTH 'x' FROM 'xxhellowxx')").expect("query should parse");
        let ast = analyzer.get_ast().expect("AST must be available");

        let Statement::Query(Query::Basic(query)) = &ast.statement else { panic!("expected query"); };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else { panic!("expected return"); };
        let Expression::FunctionCall(call) = &return_clause.items[0].expression else {
            panic!("expected FunctionCall for TRIM");
        };
        assert_eq!(call.name, "TRIM");
        assert_eq!(call.arguments.len(), 3);
    }

    #[test]
    fn test_validator_rejects_set_on_undefined_variable() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("MATCH (n:User) SET undefined_node.prop = 42")
            .expect_err("SET on unknown variable must fail validation");

        match err {
            GqlError::Validation { message, .. } => {
                assert!(
                    message.contains("undefined_node") || message.contains("scope"),
                    "expected variable scope error, got: {message}"
                );
            }
            other => panic!("expected Validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_searched_case_expression_branches_in_ast() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("RETURN CASE WHEN 1 > 0 THEN 'pos' WHEN 1 < 0 THEN 'neg' ELSE 'zero' END AS res")
            .expect("query should parse");
        let ast = analyzer.get_ast().expect("AST must be available");

        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected query");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected return");
        };
        let Expression::Case(case_expr) = &return_clause.items[0].expression else {
            panic!("expected Case expression");
        };

        assert!(
            matches!(case_expr.case_type, CaseType::Searched(_)),
            "expected CaseType::Searched, got: {:?}",
            case_expr.case_type
        );
    }

    #[test]
    fn test_validator_rejects_empty_or_multi_projection_in_scalar_subquery() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();

        let err = analyzer
            .exec("MATCH (n) RETURN VALUE { MATCH (m) }")
            .expect_err("scalar subquery without return must fail");

        assert!(matches!(err, GqlError::Validation { .. } | GqlError::Parse { .. }));
    }

    #[test]
    fn test_validator_rejects_negative_limit_or_offset() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("MATCH (n) RETURN n LIMIT -5")
            .expect_err("negative limit must fail");

        assert!(matches!(err, GqlError::Validation { .. } | GqlError::Parse { .. }));
    }

    #[test]
    fn test_validator_rejects_count_asterisk_multiplied_by_string() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("MATCH (n) RETURN COUNT(*) * 'invalid'")
            .expect_err("cannot multiply COUNT(*) by string");

        assert!(matches!(err, GqlError::Validation { .. }));
    }

    #[test]
    fn test_validator_rejects_remove_label_on_undefined_node() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let err = analyzer
            .exec("MATCH (a) REMOVE undefined_node:Admin")
            .expect_err("REMOVE label on unknown variable must fail");

        assert!(matches!(err, GqlError::Validation { .. }));
    }
}
