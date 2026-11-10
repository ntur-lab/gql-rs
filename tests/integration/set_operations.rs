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

mod set_operation_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

    #[test]
    fn test_set_operation_rejects_incompatible_column_types() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("RETURN 1 AS result_value UNION RETURN 'x' AS result_value");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Set operation column 1 has incompatible types"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }
}
