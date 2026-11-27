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

#[cfg(test)]
mod big_integer_literal_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, ast::model::*};

    fn analyze(query: &str) -> Result<QueryAST, GqlError> {
        DefaultGQLQueryAnalyzer::new().exec(query)
    }

    fn return_expression(query: &str) -> Expression {
        let ast = analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        let Statement::Query(Query::Basic(basic)) = ast.single_statement().expect("single statement") else {
            panic!("expected a basic query for: {query}");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &basic.result_statement else {
            panic!("expected RETURN for: {query}");
        };
        return_clause.items().expect("explicit RETURN items")[0].expression.clone()
    }

    fn literal(query: &str) -> Literal {
        match return_expression(query) {
            Expression::Literal(literal) => literal,
            other => panic!("expected literal for {query}, got {other:?}"),
        }
    }

    fn assert_out_of_range(query: &str) {
        let error = analyze(query).expect_err(&format!("query must fail: {query}"));
        assert!(matches!(error, GqlError::Parse { .. }), "query: {query}, error: {error}");
        assert!(
            error.message().contains(&format!("outside the signed 128-bit range (maximum {})", i128::MAX)),
            "query: {query}, error: {error}"
        );
    }

    #[test]
    fn i128_maximum_keeps_its_exact_value() {
        assert!(matches!(
            literal("RETURN 170141183460469231731687303715884105727 AS huge_val"),
            Literal::BigInteger(i128::MAX)
        ));
        assert!(matches!(
            literal("RETURN 0x7FFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF_FFFF AS huge_val"),
            Literal::BigInteger(i128::MAX)
        ));
    }

    #[test]
    fn i128_minimum_is_folded_with_its_minus_sign() {
        for query in [
            "RETURN -170141183460469231731687303715884105728 AS v",
            "RETURN -0x80000000000000000000000000000000 AS v",
            "RETURN ABS(-170141183460469231731687303715884105728) AS v",
        ] {
            let value = match return_expression(query) {
                Expression::FunctionCall(call) => call.arguments[0].clone(),
                other => other,
            };
            assert!(
                matches!(value, Expression::Literal(Literal::BigInteger(i128::MIN))),
                "query: {query}, got {value:?}"
            );
        }
    }

    #[test]
    fn other_negative_big_literals_keep_the_unary_minus() {
        let expression = return_expression("RETURN -170141183460469231731687303715884105727 AS v");
        let Expression::Unary(unary) = expression else {
            panic!("expected unary minus, got {expression:?}");
        };
        assert_eq!(unary.operator, Operator::Minus);
        assert!(matches!(*unary.expression, Expression::Literal(Literal::BigInteger(i128::MAX))));
    }

    #[test]
    fn i64_range_literals_are_unchanged() {
        assert!(matches!(literal("RETURN 1 AS v"), Literal::Integer(1)));
        assert!(matches!(literal("RETURN 9223372036854775807 AS v"), Literal::Integer(i64::MAX)));
        assert!(matches!(literal("RETURN -9223372036854775808 AS v"), Literal::Integer(i64::MIN)));
        assert!(matches!(literal("RETURN 0x7FFFFFFFFFFFFFFF AS v"), Literal::Integer(i64::MAX)));

        // The serialized AST of these literals is the same as before the 128-bit support.
        let json = serde_json::to_string(&literal("RETURN 9223372036854775807 AS v")).unwrap();
        assert_eq!(json, r#"{"Integer":9223372036854775807}"#);
    }

    #[test]
    fn boundaries_between_i64_and_i128() {
        assert!(matches!(
            literal("RETURN 9223372036854775808 AS v"),
            Literal::BigInteger(value) if value == i64::MAX as i128 + 1
        ));
        let expression = return_expression("RETURN -9223372036854775809 AS v");
        let Expression::Unary(unary) = expression else {
            panic!("expected unary minus, got {expression:?}");
        };
        assert!(matches!(
            *unary.expression,
            Expression::Literal(Literal::BigInteger(value)) if value == i64::MAX as i128 + 2
        ));
    }

    #[test]
    fn literals_outside_i128_fail_with_the_limit() {
        for query in [
            "RETURN 170141183460469231731687303715884105728 AS v",
            "RETURN -170141183460469231731687303715884105729 AS v",
            "RETURN 340282366920938463463374607431768211455 AS v",
            "RETURN 0x80000000000000000000000000000000 AS v",
            "RETURN 0b1_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000_00000000 AS v",
        ] {
            assert_out_of_range(query);
        }
    }

    #[test]
    fn big_integer_literals_are_typed_as_integers() {
        for query in [
            "RETURN 170141183460469231731687303715884105727 + 1 AS v",
            "RETURN CAST(170141183460469231731687303715884105727 AS INT128) AS v",
            "RETURN ABS(-170141183460469231731687303715884105728) AS v",
            "LET VALUE x :: INT128 = 9223372036854775808 RETURN x",
        ] {
            analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        }
        let error = analyze("RETURN UPPER(9223372036854775808) AS v").expect_err("integer is not a string");
        assert!(matches!(error, GqlError::Validation { .. }), "{error}");
    }

    #[test]
    fn big_integer_literals_serialize_as_decimal_strings() {
        let max = literal("RETURN 170141183460469231731687303715884105727 AS v");
        let value = serde_json::to_value(&max).expect("serde_json::Value can hold the literal");
        assert_eq!(value, serde_json::json!({ "BigInteger": "170141183460469231731687303715884105727" }));

        let back: Literal = serde_json::from_value(value).expect("round trip");
        assert!(matches!(back, Literal::BigInteger(i128::MAX)));
        let min: Literal = serde_json::from_str(r#"{"BigInteger":"-170141183460469231731687303715884105728"}"#).unwrap();
        assert!(matches!(min, Literal::BigInteger(i128::MIN)));
        let from_number: Literal = serde_json::from_str(r#"{"BigInteger":42}"#).unwrap();
        assert!(matches!(from_number, Literal::BigInteger(42)));
    }
}
