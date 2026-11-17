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

mod syntax_error_it {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, error_handler::error::GqlError};

    #[test]
    fn analyzer_surfaces_syntax_errors_through_the_error_listener() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();

        let result = analyzer.exec("MATCH (");

        match result {
            Err(GqlError::Parse { message, context }) => {
                assert!(message.contains("Syntax error at line 1, column 7:"));
                assert_eq!(
                    context.span,
                    Some(gql_rs::Span {
                        start_line: 1,
                        start_col: 7,
                        end_line: 1,
                        end_col: 7,
                    })
                );
                assert_eq!(context.code, None);
            }
            other => panic!("Expected parse error from syntax failure, got: {:?}", other),
        }

        assert_eq!(analyzer.get_query(), None);
        assert!(analyzer.get_ast().is_none());
    }

    mod syntax_error_tests {
        use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, analyze, parse};

        fn assert_parse_position(error: &GqlError, line: isize, column: isize) {
            assert!(
                matches!(error, GqlError::Parse { .. }),
                "Expected parse error, got: {error:?}"
            );

            let span = error.context().span.as_ref().expect("Expected a source position");

            assert_eq!(span.start_line, line);
            assert_eq!(span.start_col, column);
            assert_eq!(span.end_line, line);
            assert_eq!(span.end_col, column);
        }

        #[test]
        fn syntax_error_at_eof_has_a_source_position() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();

            let error = analyzer.exec("MATCH (").unwrap_err();

            assert_parse_position(&error, 1, 7);
            assert!(error.message().contains("Syntax error at line 1, column 7:"));
            assert!(analyzer.get_query().is_none());
            assert!(analyzer.get_ast().is_none());
        }

        #[test]
        fn both_stateless_entry_points_reject_lexical_errors() {
            let query = "RETURN \u{0000}42 AS answer";

            let parse_error = parse(query).unwrap_err();
            let analysis_error = analyze(query).unwrap_err();

            assert_parse_position(&parse_error, 1, 7);
            assert_parse_position(&analysis_error, 1, 7);
        }

        #[test]
        fn lexical_error_on_second_line_has_correct_position() {
            let error = analyze("RETURN\n\u{0000}42 AS answer").unwrap_err();

            assert_parse_position(&error, 2, 0);
        }

        #[test]
        fn unicode_columns_count_characters_instead_of_utf8_bytes() {
            let prefix = "RETURN 'è😀' AS answer ";
            analyze(prefix).expect("The prefix must be valid");

            let query = format!("{prefix}\u{0000}");
            let error = analyze(&query).unwrap_err();

            assert_parse_position(&error, 1, prefix.chars().count() as isize);
        }

        #[test]
        fn lexical_failure_clears_state_and_allows_subsequent_success() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();

            analyzer.exec("RETURN 42 AS answer").unwrap();
            assert!(analyzer.get_ast().is_some());

            let error = analyzer.exec("RETURN \u{0000}42 AS answer").unwrap_err();

            assert_parse_position(&error, 1, 7);
            assert!(analyzer.get_query().is_none());
            assert!(analyzer.get_ast().is_none());

            analyzer.exec("RETURN 42 AS answer").unwrap();

            assert_eq!(analyzer.get_query(), Some("RETURN 42 AS answer"));
            assert!(analyzer.get_ast().is_some());
        }
    }
}
