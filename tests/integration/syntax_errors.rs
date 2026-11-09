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

mod syntax_error_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, error_handler::error::GqlError};

    #[test]
    fn analyzer_surfaces_syntax_errors_through_the_error_listener() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();

        let result = analyzer.exec("MATCH (");

        match result {
            Err(GqlError::Parse { message, context }) => {
                assert!(message.contains("Syntax error at line 1, column 7:"));
                assert_eq!(context.span, None);
                assert_eq!(context.code, None);
            }
            other => panic!("Expected parse error from syntax failure, got: {:?}", other),
        }

        assert_eq!(analyzer.get_query(), None);
        assert!(analyzer.get_ast().is_none());
    }
}
