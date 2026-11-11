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

mod stateless_api_it {
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError, analyze, parse};

    #[test]
    fn parse_builds_ast_without_running_semantic_validation() {
        let query = "MATCH (n:User) SET undefined_node.prop = 42";

        let ast = parse(query).expect("Syntactically valid query must produce an AST");

        assert!(
            matches!(ast.validate(), Err(GqlError::Validation { .. })),
            "Explicit validation must reject the undefined variable"
        );

        assert!(
            matches!(analyze(query), Err(GqlError::Validation { .. })),
            "Full analysis must reject the undefined variable"
        );
    }

    #[test]
    fn parse_still_rejects_invalid_syntax() {
        assert!(matches!(parse("MATCH ("), Err(GqlError::Parse { .. })));
    }

    #[test]
    fn stateless_and_stateful_analysis_produce_equivalent_asts() {
        let query = "RETURN 42 AS answer";

        let stateless_ast = analyze(query).unwrap();

        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let stateful_ast = analyzer.exec(query).unwrap();

        let expected = serde_json::to_value(&stateless_ast).unwrap();

        assert_eq!(serde_json::to_value(&stateful_ast).unwrap(), expected);

        assert_eq!(serde_json::to_value(analyzer.get_ast().unwrap()).unwrap(), expected);

        assert_eq!(analyzer.get_query(), Some(query));
    }
}
