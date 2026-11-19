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

//! Regression tests for structural expression construction (issue #9): expressions are built
//! from their CST instead of being reconstructed from source text, construction errors are
//! propagated, and constructs without a faithful AST representation fail explicitly.

mod structural_expression_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, ast::model::*};

    fn analyze(query: &str) -> Result<QueryAST, GqlError> {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec(query)
    }

    /// Returns the expressions projected by the final RETURN of a basic query.
    fn return_expressions(query: &str) -> Vec<Expression> {
        let ast = analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        let Statement::Query(Query::Basic(basic)) = &ast.statement else {
            panic!("expected a basic query for: {query}");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &basic.result_statement else {
            panic!("expected RETURN for: {query}");
        };
        return_clause
            .items()
            .unwrap_or_else(|| panic!("expected explicit RETURN items for: {query}"))
            .iter()
            .map(|item| item.expression.clone())
            .collect()
    }

    fn single_return_expression(query: &str) -> Expression {
        let mut expressions = return_expressions(query);
        assert_eq!(expressions.len(), 1, "query: {query}");
        expressions.remove(0)
    }

    fn function_call(expression: &Expression) -> &FunctionCall {
        match expression {
            Expression::FunctionCall(call) => call,
            other => panic!("expected function call, got {other:?}"),
        }
    }

    fn binary(expression: &Expression) -> &BinaryExpression {
        match expression {
            Expression::Binary(binary) => binary,
            other => panic!("expected binary expression, got {other:?}"),
        }
    }

    fn assert_variable(expression: &Expression, expected: &str) {
        match expression {
            Expression::Variable { name, .. } => assert_eq!(name, expected),
            other => panic!("expected variable '{expected}', got {other:?}"),
        }
    }

    fn assert_property(expression: &Expression, object: &str, property: &str) {
        match expression {
            Expression::PropertyAccess(access) => {
                assert_eq!(access.object, object);
                assert_eq!(access.property, property);
            }
            other => panic!("expected property access {object}.{property}, got {other:?}"),
        }
    }

    fn assert_unsupported(query: &str) {
        let error = analyze(query).expect_err(&format!("query must fail explicitly: {query}"));
        assert!(
            matches!(error, GqlError::Parse { .. }),
            "query: {query}, error: {error}"
        );
        assert_eq!(
            error.context().code.as_deref(),
            Some(GqlError::UNSUPPORTED_CONSTRUCT),
            "query: {query}, error: {error}"
        );
        assert!(
            error.message().starts_with("Unsupported construct:"),
            "query: {query}, error: {error}"
        );
    }

    fn assert_unknown_variable(query: &str, variable: &str) {
        let error = analyze(query).expect_err(&format!("query must fail validation: {query}"));
        assert!(
            matches!(error, GqlError::Validation { .. }),
            "query: {query}, error: {error}"
        );
        assert!(
            error
                .message()
                .contains(&format!("Unknown variable '{variable}'")),
            "query: {query}, error: {error}"
        );
    }

    /// Returns true when any string literal inside `expression` equals `text`.
    fn contains_string_literal(expression: &Expression, text: &str) -> bool {
        match expression {
            Expression::Literal(Literal::String(value)) => value == text,
            Expression::Binary(binary) => {
                contains_string_literal(&binary.left, text)
                    || contains_string_literal(&binary.right, text)
            }
            Expression::Unary(unary) => contains_string_literal(&unary.expression, text),
            Expression::FunctionCall(call) => call
                .arguments
                .iter()
                .any(|argument| contains_string_literal(argument, text)),
            Expression::Array(array) => array
                .elements
                .iter()
                .any(|element| contains_string_literal(element, text)),
            Expression::Map(items) => items
                .iter()
                .any(|item| contains_string_literal(&item.value, text)),
            _ => false,
        }
    }

    #[test]
    fn numeric_function_arguments_keep_nested_operators() {
        let floor = single_return_expression("MATCH (n) RETURN FLOOR(n.price * 1.2 + 1)");
        let call = function_call(&floor);
        assert_eq!(call.name, "FLOOR");
        let sum = binary(&call.arguments[0]);
        assert_eq!(sum.operator, Operator::Plus);
        assert!(matches!(
            *sum.right,
            Expression::Literal(Literal::Integer(1))
        ));
        let product = binary(&sum.left);
        assert_eq!(product.operator, Operator::Star);
        assert_property(&product.left, "n", "price");
        assert!(matches!(*product.right, Expression::Literal(Literal::Float(f)) if f == 1.2));

        let power = single_return_expression("MATCH (a) RETURN POWER(a.x + 1, 2 * a.y)");
        let call = function_call(&power);
        assert_eq!(call.arguments.len(), 2);
        assert_eq!(binary(&call.arguments[0]).operator, Operator::Plus);
        let exponent = binary(&call.arguments[1]);
        assert_eq!(exponent.operator, Operator::Star);
        assert_property(&exponent.right, "a", "y");

        let modulus = single_return_expression("MATCH (a) RETURN MOD(a.x - 1, -a.y)");
        let call = function_call(&modulus);
        assert_eq!(binary(&call.arguments[0]).operator, Operator::Minus);
        match &call.arguments[1] {
            Expression::Unary(unary) => {
                assert_eq!(unary.operator, Operator::Minus);
                assert_property(&unary.expression, "a", "y");
            }
            other => panic!("expected unary minus, got {other:?}"),
        }
    }

    #[test]
    fn every_numeric_argument_position_is_built_structurally() {
        for query in [
            "MATCH (x) RETURN FLOOR(x + 1)",
            "MATCH (x) RETURN CEIL(x + 1)",
            "MATCH (x) RETURN CEILING(x + 1)",
            "MATCH (x) RETURN SQRT(x + 1)",
            "MATCH (x) RETURN EXP(x + 1)",
            "MATCH (x) RETURN LN(x + 1)",
            "MATCH (x) RETURN LOG10(x + 1)",
            "MATCH (x) RETURN SIN(x + 1)",
            "MATCH (x) RETURN DEGREES(x + 1)",
            "MATCH (x) RETURN LOG(x + 1, x + 1)",
            "MATCH (x) RETURN POWER(x + 1, x + 1)",
            "MATCH (x) RETURN MOD(x + 1, x + 1)",
            "MATCH (x) RETURN LEFT('abc', x + 1)",
            "MATCH (x) RETURN RIGHT('abc', x + 1)",
            "MATCH (x) RETURN TRIM([1, 2, 3], x + 1)",
            "MATCH (x) RETURN PERCENTILE_CONT(x + 1, x + 1)",
            "MATCH (x) RETURN PERCENTILE_DISC(x + 1, x + 1)",
        ] {
            let expression = single_return_expression(query);
            let call = function_call(&expression);
            assert!(
                !contains_string_literal(&expression, "x+1"),
                "argument was reconstructed from text: {query} -> {expression:?}"
            );
            let structural = call.arguments.iter().filter(|argument| {
                matches!(argument, Expression::Binary(binary) if binary.operator == Operator::Plus)
            });
            assert!(structural.count() >= 1, "query: {query} -> {expression:?}");
        }
    }

    #[test]
    fn numeric_functions_nest_inside_numeric_arguments() {
        let expression = single_return_expression("MATCH (x) RETURN FLOOR(ABS(x) * SQRT(x))");
        let call = function_call(&expression);
        let product = binary(&call.arguments[0]);
        assert_eq!(function_call(&product.left).name, "ABS");
        assert_eq!(function_call(&product.right).name, "SQRT");
        assert_variable(&function_call(&product.right).arguments[0], "x");
    }

    #[test]
    fn argument_spans_cover_the_argument_source() {
        let expression = single_return_expression("MATCH (x) RETURN FLOOR(x + 1)");
        let call = function_call(&expression);
        let Expression::Binary(argument) = &call.arguments[0] else {
            panic!("expected binary argument");
        };
        assert_eq!(argument.span.start_line, 1);
        assert_eq!(argument.span.start_col, 23);
        assert_eq!(argument.span.end_col, 28);
    }

    #[test]
    fn binary_set_functions_preserve_set_quantifier_and_arguments() {
        let expression =
            single_return_expression("MATCH (n) RETURN PERCENTILE_CONT(DISTINCT n.score, 0.5)");
        let call = function_call(&expression);
        assert_eq!(call.name, "PERCENTILE_CONT");
        assert!(matches!(call.distinct, DistinctQualifier::Distinct));
        assert_property(&call.arguments[0], "n", "score");
        assert!(matches!(
            call.arguments[1],
            Expression::Literal(Literal::Float(f)) if f == 0.5
        ));
    }

    #[test]
    fn aggregate_names_are_canonical() {
        let expression = single_return_expression("MATCH (n) RETURN count(DISTINCT n)");
        let call = function_call(&expression);
        assert_eq!(call.name, "COUNT");
        assert!(matches!(call.distinct, DistinctQualifier::Distinct));
        assert_variable(&call.arguments[0], "n");
    }

    #[test]
    fn variable_references_inside_arguments_reach_semantic_validation() {
        for query in [
            "RETURN FLOOR(y + 1)",
            "RETURN CEIL(y * 2)",
            "RETURN SQRT(-y)",
            "RETURN POWER(2, y)",
            "RETURN MOD(y, 2)",
            "RETURN LOG(2, y)",
            "RETURN LN(y + 1)",
            "RETURN COS(y)",
            "RETURN LEFT('abc', y + 1)",
            "RETURN TRIM([1, 2], y)",
            "RETURN ELEMENTS(y)",
            "RETURN UPPER(y || 'a')",
            "RETURN DATE({yr: y})",
            "MATCH (n) RETURN PERCENTILE_CONT(y, 0.5)",
            "MATCH (n) RETURN PERCENTILE_CONT(n.score, y)",
            "CALL myProc(y)",
            "CALL myProc(FLOOR(y + 1))",
            "MATCH (n) CALL myProc(n, y + 1) RETURN n",
            "LET a = FLOOR(y + 1) RETURN a",
        ] {
            assert_unknown_variable(query, "y");
        }
    }

    #[test]
    fn construction_failures_in_call_arguments_are_propagated() {
        assert_unsupported("CALL myProc($p.name)");
        assert_unsupported("CALL myProc(FLOOR(n.a.b))");
        assert_unsupported("MATCH (n) CALL myProc(n.a.b) RETURN n");
    }

    #[test]
    fn construction_failures_in_let_definitions_are_propagated() {
        assert_unsupported("LET a = $p.name RETURN a");
        assert_unsupported("MATCH (n) LET a = n.a.b RETURN a");
        assert_unsupported("RETURN LET a = $p.name IN a END");
    }

    #[test]
    fn unsupported_expressions_never_become_successful_string_literals() {
        for query in [
            "MATCH (n) RETURN n.address.city",
            "RETURN $param.name",
            "RETURN (1 + 2).x",
            "RETURN GRAPH CURRENT_GRAPH",
            "RETURN GRAPH HOME_GRAPH",
            "RETURN GRAPH CURRENT_PROPERTY_GRAPH",
            "RETURN GRAPH cat.g",
            "RETURN GRAPH `delimited graph`",
            "RETURN TABLE /catalog_table",
            "RETURN TABLE `delimited table`",
            "MATCH (n) RETURN CASE n.a.b WHEN 1 THEN 2 END",
        ] {
            assert_unsupported(query);
        }
    }

    #[test]
    fn genuine_string_literals_are_preserved() {
        let expressions = return_expressions(
            "RETURN DATE('2024-01-31'), LOCAL_TIME('12:30:00.5'), ZONED_DATETIME('2024-01-31T12:30:00+01:00'), DURATION('P1D'), LEFT('a.b', 1), 'x+1'",
        );
        let expected = [
            ("DATE", "2024-01-31"),
            ("LOCAL_TIME", "12:30:00.5"),
            ("ZONED_DATETIME", "2024-01-31T12:30:00+01:00"),
            ("DURATION", "P1D"),
            ("LEFT", "a.b"),
        ];
        for (expression, (name, text)) in expressions.iter().zip(expected) {
            let call = function_call(expression);
            assert_eq!(call.name, name);
            assert!(
                matches!(&call.arguments[0], Expression::Literal(Literal::String(value)) if value == text),
                "{name}: {:?}",
                call.arguments[0]
            );
        }
        assert!(matches!(
            &expressions[5],
            Expression::Literal(Literal::String(value)) if value == "x+1"
        ));
    }

    #[test]
    fn temporal_constructor_records_are_built_as_maps() {
        let expression = single_return_expression("RETURN DATE({yr: 2024, mo: 1 + 1})");
        let call = function_call(&expression);
        let Expression::Map(items) = &call.arguments[0] else {
            panic!("expected record argument, got {:?}", call.arguments[0]);
        };
        assert_eq!(items.len(), 2);
        assert_eq!(items[0].key, "yr");
        assert_eq!(items[1].key, "mo");
        assert_eq!(binary(&items[1].value).operator, Operator::Plus);
    }

    #[test]
    fn temporal_and_duration_literals_use_their_literal_variants() {
        let expressions = return_expressions(
            "RETURN DATE '2024-01-31', TIME '12:30:45', DATETIME '2024-01-31T12:30:45', TIMESTAMP '2024-01-31T12:30:45', DURATION 'P1Y2M'",
        );
        assert!(matches!(&expressions[0], Expression::Literal(Literal::Date(value)) if value == "2024-01-31"));
        assert!(matches!(&expressions[1], Expression::Literal(Literal::Time(value)) if value == "12:30:45"));
        for expression in &expressions[2..4] {
            assert!(
                matches!(expression, Expression::Literal(Literal::DateTime(value)) if value == "2024-01-31T12:30:45"),
                "{expression:?}"
            );
        }
        assert!(matches!(
            &expressions[4],
            Expression::Literal(Literal::Duration(value)) if value == "P1Y2M"
        ));
    }

    #[test]
    fn numeric_literal_forms_are_decoded() {
        let expressions = return_expressions(
            "RETURN 0x1F, 0o17, 0b101, 1_000, 0x_FF, 10M, 1.5M, 2.5F, 3D, 1e3, 1.5e-1F",
        );
        let integers = [31, 15, 5, 1000, 255];
        for (expression, expected) in expressions.iter().zip(integers) {
            assert!(
                matches!(expression, Expression::Literal(Literal::Integer(value)) if *value == expected),
                "expected {expected}, got {expression:?}"
            );
        }
        let decimals = ["10", "1.5"];
        for (expression, expected) in expressions.iter().skip(integers.len()).zip(decimals) {
            assert!(
                matches!(expression, Expression::Literal(Literal::Decimal(value)) if value == expected),
                "expected {expected}M, got {expression:?}"
            );
        }
        let floats = [2.5, 3.0, 1000.0, 0.15];
        for (expression, expected) in expressions.iter().skip(integers.len() + decimals.len()).zip(floats) {
            assert!(
                matches!(expression, Expression::Literal(Literal::Float(value)) if *value == expected),
                "expected {expected}, got {expression:?}"
            );
        }
    }

    #[test]
    fn non_decimal_integer_literals_respect_the_signed_64_bit_range() {
        let error = analyze("RETURN 0x8000000000000000").expect_err("out of range");
        assert!(error.message().contains("outside the signed 64-bit range"));

        for query in [
            "RETURN -0x8000000000000000",
            "RETURN FLOOR(-9223372036854775808)",
        ] {
            let expression = single_return_expression(query);
            let value = match &expression {
                Expression::FunctionCall(call) => call.arguments[0].clone(),
                other => other.clone(),
            };
            assert!(
                matches!(value, Expression::Literal(Literal::Integer(i64::MIN))),
                "query: {query}, got {expression:?}"
            );
        }
    }

    #[test]
    fn unknown_truth_value_is_a_null_literal() {
        let expression = single_return_expression("RETURN UNKNOWN");
        assert!(matches!(expression, Expression::Literal(Literal::Null)));
    }

    #[test]
    fn session_user_is_a_niladic_builtin_call() {
        let expression = single_return_expression("RETURN SESSION_USER");
        let call = function_call(&expression);
        assert_eq!(call.name, "SESSION_USER");
        assert!(call.arguments.is_empty());
    }

    #[test]
    fn graph_and_table_parameters_are_parameter_references() {
        for query in ["RETURN GRAPH $$my_graph", "RETURN TABLE $$my_table"] {
            let expression = single_return_expression(query);
            assert!(
                matches!(&expression, Expression::Literal(Literal::Parameter(name)) if name.starts_with("$$")),
                "query: {query}, got {expression:?}"
            );
        }
    }

    #[test]
    fn graph_and_table_object_expressions_are_built_structurally() {
        let expression = single_return_expression("MATCH (g) RETURN GRAPH VARIABLE g");
        assert_variable(&expression, "g");

        let error = analyze("RETURN TABLE missing_table").expect_err("unbound binding table");
        assert!(matches!(error, GqlError::Validation { .. }), "{error}");
    }

    #[test]
    fn parenthesized_variable_property_reference_is_a_property_access() {
        let expression = single_return_expression("MATCH (n) RETURN (n).name");
        assert_property(&expression, "n", "name");
    }

    #[test]
    fn duration_between_qualifier_is_canonical() {
        for (query, expected) in [
            (
                "RETURN DURATION_BETWEEN(CURRENT_DATE, CURRENT_DATE) YEAR TO MONTH",
                "YEAR TO MONTH",
            ),
            (
                "RETURN DURATION_BETWEEN(CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) DAY TO SECOND",
                "DAY TO SECOND",
            ),
        ] {
            let expression = single_return_expression(query);
            let call = function_call(&expression);
            assert_eq!(call.arguments.len(), 3, "query: {query}");
            assert!(matches!(
                &call.arguments[2],
                Expression::Literal(Literal::String(value)) if value == expected
            ));
        }
    }
}
