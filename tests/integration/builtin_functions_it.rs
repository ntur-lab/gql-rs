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

//! Query-level tests of built-in function validation (issue #19).

mod builtin_function_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

    fn analyze(query: &str) -> Result<(), GqlError> {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec(query).map(|_| ())
    }

    fn assert_accepted(queries: &[&str]) {
        for query in queries {
            analyze(query).unwrap_or_else(|error| panic!("query: {query}, error: {error}"));
        }
    }

    fn assert_rejected(queries: &[(&str, &str)]) {
        for (query, fragment) in queries {
            let error = analyze(query).expect_err(&format!("query must be rejected: {query}"));
            assert!(
                matches!(error, GqlError::Validation { .. }),
                "query: {query}, error: {error}"
            );
            assert!(
                error.message().contains(fragment),
                "query: {query}, expected '{fragment}', error: {error}"
            );
        }
    }

    #[test]
    fn known_argument_types_are_checked_per_family() {
        assert_rejected(&[
            (
                "RETURN FLOOR('a')",
                "FLOOR requires a numeric argument, got STRING",
            ),
            ("RETURN SQRT(TRUE)", "SQRT requires a numeric argument"),
            (
                "RETURN POWER(2, 'x')",
                "POWER argument 2 requires a numeric argument",
            ),
            (
                "RETURN ABS('a')",
                "ABS requires a numeric or duration argument",
            ),
            (
                "RETURN UPPER(1)",
                "UPPER requires a string-like argument, got INTEGER",
            ),
            (
                "RETURN CHAR_LENGTH(1)",
                "CHAR_LENGTH requires a string-like argument",
            ),
            (
                "RETURN BYTE_LENGTH('abc')",
                "BYTE_LENGTH requires a byte string argument",
            ),
            (
                "RETURN LEFT(1, 2)",
                "LEFT argument 1 requires a character or byte string",
            ),
            (
                "RETURN LEFT('abc', 'x')",
                "LEFT argument 2 requires a non-negative integer",
            ),
            (
                "RETURN LEFT('abc', -1)",
                "LEFT argument 2 requires a non-negative integer",
            ),
            (
                "RETURN RIGHT('abc', -(1))",
                "RIGHT argument 2 requires a non-negative integer",
            ),
            (
                "RETURN TRIM([1, 2], -1)",
                "TRIM argument 2 requires a non-negative integer",
            ),
            (
                "MATCH (n) RETURN PERCENTILE_DISC(n.x, -0.5)",
                "PERCENTILE_DISC argument 2 requires a numeric argument between 0 and 1",
            ),
            ("RETURN SIZE(1)", "SIZE requires list/string/path argument"),
            (
                "RETURN TRIM([1, 2], 'x')",
                "TRIM argument 2 requires a non-negative integer",
            ),
            ("RETURN ELEMENTS(1)", "ELEMENTS requires a path argument"),
            (
                "RETURN PATH_LENGTH('p')",
                "PATH_LENGTH requires a path argument",
            ),
            (
                "RETURN DURATION_BETWEEN(CURRENT_DATE, 1)",
                "DURATION_BETWEEN requires temporal-compatible arguments",
            ),
            (
                "RETURN DURATION_BETWEEN(CURRENT_DATE, DURATION('P1D'))",
                "DURATION_BETWEEN requires temporal-compatible arguments",
            ),
            (
                "RETURN COALESCE(1, 'a')",
                "COALESCE arguments must be mutually compatible",
            ),
            (
                "MATCH (n) RETURN SUM('a')",
                "SUM requires a numeric or duration argument",
            ),
            (
                "MATCH (n) RETURN STDDEV_POP('a')",
                "STDDEV_POP requires a numeric argument",
            ),
            (
                "MATCH (n) RETURN PERCENTILE_CONT(n.x, 2)",
                "PERCENTILE_CONT argument 2 requires a numeric argument between 0 and 1",
            ),
        ]);
    }

    #[test]
    fn valid_argument_types_are_accepted_per_family() {
        assert_accepted(&[
            "RETURN FLOOR(1.5), CEIL(1), SQRT(4.0), POWER(2, 0.5), MOD(7, 2), LOG(2, 8)",
            "RETURN SIN(0), DEGREES(1), RADIANS(180), ABS(-1), ABS(DURATION('P1D'))",
            "RETURN UPPER('a'), LOWER('A'), CHAR_LENGTH('abc'), CHARACTER_LENGTH('abc')",
            "RETURN LTRIM(' a'), RTRIM('a ', ' '), BTRIM(' a '), NORMALIZE('a', NFC)",
            "RETURN LEFT('abc', 1 + 1), RIGHT('abc', 0), TRIM(BOTH 'x' FROM 'xax')",
            "RETURN SIZE([1, 2]), CARDINALITY('abc'), TRIM([1, 2, 3], 1)",
            "MATCH p = (a)-[e]->(b) RETURN ELEMENTS(p), PATH_LENGTH(p), SIZE(ELEMENTS(p))",
            "RETURN DATE('2024-01-31'), DATE({yr: 2024}), LOCAL_TIME('12:30:00.5')",
            "RETURN DURATION_BETWEEN(CURRENT_DATE, DATE('2024-01-31')) DAY TO SECOND",
            "RETURN COALESCE(NULL, 1, 2.5), NULLIF('a', 'b'), SESSION_USER",
            "MATCH (n) RETURN SUM(n.x), AVG(DISTINCT n.x), STDDEV_SAMP(n.x), PERCENTILE_DISC(n.x, 1)",
        ]);
    }

    #[test]
    fn unknown_argument_types_are_not_rejected() {
        assert_accepted(&[
            "MATCH (n) RETURN FLOOR(n.x), UPPER(n.name), BYTE_LENGTH(n.payload)",
            "MATCH (n) RETURN LEFT(n.name, n.len), ELEMENTS(n), DATE({yr: n.ts})",
            "MATCH (n) RETURN DURATION_BETWEEN(n.t0, n.t1), COALESCE(n.a, n.b, 1)",
            "MATCH (n) RETURN PERCENTILE_CONT(n.x, n.fraction)",
            "RETURN FLOOR($value), UPPER($name)",
        ]);
    }

    #[test]
    fn aggregate_restrictions_cover_every_grammar_aggregate() {
        assert_rejected(&[
            (
                "MATCH (n) RETURN STDDEV_POP(COUNT(n))",
                "cannot contain nested aggregate expressions",
            ),
            (
                "MATCH (n) RETURN PERCENTILE_CONT(AVG(n.x), 0.5)",
                "cannot contain nested aggregate expressions",
            ),
            (
                "MATCH (n) RETURN COLLECT_LIST(MAX(n.x))",
                "cannot contain nested aggregate expressions",
            ),
            (
                "MATCH (n) WHERE STDDEV_SAMP(n.x) > 1 RETURN n",
                "is not allowed in a WHERE clause",
            ),
            (
                "MATCH (n) WHERE PERCENTILE_DISC(n.x, 0.5) > 1 RETURN n",
                "is not allowed in a WHERE clause",
            ),
        ]);
    }

    #[test]
    fn element_and_predicate_forms_are_validated() {
        assert_accepted(&[
            "MATCH (a)-[e]->(b) RETURN ELEMENT_ID(a), PROPERTY_EXISTS(a, name)",
            "MATCH (a)-[e]->(b) RETURN a IS SOURCE OF e, b IS DESTINATION OF e, e IS DIRECTED",
            "MATCH (a)-[e]->(b) RETURN a IS LABELED Person, ALL_DIFFERENT(a, b), SAME(a, b)",
            "RETURN 'a' IS NFC NORMALIZED, 'a' IS NOT nfkd NORMALIZED",
        ]);
        assert_rejected(&[
            ("MATCH (a) RETURN ELEMENT_ID(z)", "Unknown variable 'z'"),
            (
                "MATCH (a) RETURN ALL_DIFFERENT(a, z)",
                "Unknown variable 'z'",
            ),
        ]);
    }

    #[test]
    fn user_defined_function_handling_is_preserved() {
        assert_accepted(&[
            "RETURN myFunction(1, 'a', TRUE)",
            "RETURN ns.lower_case(1)",
            "MATCH (n) RETURN ns.sub.helper(n, n.x + 1)",
            "MATCH (n) RETURN COLLECT_SET(n)",
        ]);
        assert_rejected(&[
            ("RETURN myFunction(missing)", "Unknown variable 'missing'"),
            (
                "RETURN ns.helper(FLOOR(missing))",
                "Unknown variable 'missing'",
            ),
        ]);
    }

    #[test]
    fn duration_between_qualifier_is_validated() {
        assert_accepted(&[
            "RETURN DURATION_BETWEEN(CURRENT_TIMESTAMP, CURRENT_TIMESTAMP) YEAR TO MONTH",
        ]);
    }
}
