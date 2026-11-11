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

mod string_literal_tests {
    use gql_rs::ast::model::*;
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

    /// Parses `RETURN <literal>` and returns the value of the string literal.
    fn returned_string(literal: &str) -> String {
        let query_text = format!("RETURN {literal}");
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec(&query_text)
            .unwrap_or_else(|error| panic!("{query_text} should parse: {error:?}"));

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };
        let PrimitiveResultStatement::Return { return_clause, .. } = &query.result_statement else {
            panic!("expected RETURN result statement");
        };
        let Expression::Literal(Literal::String(value)) = &return_clause.items[0].expression else {
            panic!("expected string literal, got {:?}", return_clause.items[0].expression);
        };
        value.clone()
    }

    #[test]
    fn test_doubled_single_quote_is_one_quote() {
        assert_eq!(returned_string("'l''albero'"), "l'albero");
    }

    #[test]
    fn test_double_quotes_inside_single_quoted_string_are_kept() {
        for (literal, expected) in [
            ("'\"quoted\"'", "\"quoted\""),
            ("'He said \"hi\"'", "He said \"hi\""),
            ("'\"unclosed'", "\"unclosed"),
        ] {
            assert_eq!(returned_string(literal), expected, "literal: {literal}");
        }
    }

    #[test]
    fn test_backslash_escapes_are_decoded() {
        assert_eq!(returned_string("'He said \\\"hi\\\"'"), "He said \"hi\"");
        assert_eq!(returned_string("'a\\nb'"), "a\nb");
        assert_eq!(returned_string("'caff\\u00E8'"), "caffè");
    }

    #[test]
    fn test_no_escape_string_keeps_backslashes() {
        assert_eq!(returned_string("@'C:\\temp'"), "C:\\temp");
    }

    #[test]
    fn test_empty_string() {
        assert_eq!(returned_string("''"), "");
    }
}
