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
mod leading_trivia_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, parse};

    fn analyze(query: &str) -> Result<(), GqlError> {
        DefaultGQLQueryAnalyzer::new().exec(query).map(|_| ())
    }

    #[test]
    fn leading_whitespace_and_comments_are_ignored() {
        for query in [
            "    RETURN 1",
            "\tRETURN 1",
            "\n\n  RETURN 1",
            "\r\n RETURN 1",
            "// comment\nRETURN 1",
            "  -- comment\n  RETURN 1",
            "/* block */ RETURN 1",
            "  /* a */ // b\n  -- c\n  RETURN 1",
            "\n    MATCH (n)\n    RETURN n\n",
            "\n    CALL db.indexes()\n    YIELD name\n    RETURN name",
        ] {
            analyze(query).unwrap_or_else(|error| panic!("query: {query:?}, error: {error}"));
            parse(query).unwrap_or_else(|error| panic!("parse: {query:?}, error: {error}"));
        }
    }

    #[test]
    fn errors_after_leading_whitespace_keep_their_source_position() {
        let error = analyze("\n\n  RETURN x").expect_err("unknown variable");
        let span = error.context().span.clone().expect("span");
        assert_eq!((span.start_line, span.start_col), (3, 9), "{error}");

        let error = analyze("   RETURN 1 +").expect_err("incomplete expression");
        assert!(matches!(error, GqlError::Parse { .. }), "{error}");
        assert!(!error.message().contains("extraneous input"), "{error}");
        let span = error.context().span.clone().expect("span");
        assert_eq!((span.start_line, span.start_col), (1, 13), "{error}");
    }

    #[test]
    fn inputs_without_statements_fail_cleanly() {
        for query in ["", "   ", "\n\t\n", "// only a comment", "  /* x */  ", "-- a\n-- b\n"] {
            let error = analyze(query).expect_err(&format!("query must fail: {query:?}"));
            assert!(matches!(error, GqlError::Parse { .. }), "query: {query:?}, error: {error}");
            assert!(error.message().contains("Empty query"), "query: {query:?}, error: {error}");
            assert!(parse(query).is_err(), "parse: {query:?}");
        }
    }
}
