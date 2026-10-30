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

mod query_composition_tests {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};

    #[test]
    fn test_value_subquery_rejects_multi_column_projection() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH (n) RETURN VALUE { MATCH (m) RETURN m.x, m.y }");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("Scalar subquery must return exactly one column"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }
}
