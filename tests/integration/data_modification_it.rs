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

mod data_modification_it {
    use gql_rs::ast::{DataStatement, PatternElement, Statement, TransactionIntent};
    use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

    #[test]
    fn match_insert_reuses_matched_node_bindings() {
        let query = "MATCH (a:Person), (b:Person)\n\
                     WHERE a.name = 'P1' AND b.name = 'P3'\n\
                     INSERT (a)-[:LIKES]->(b)";
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer.exec(query).expect("MATCH ... INSERT should analyze");

        let ast = analyzer.get_ast().expect("analysis should produce an AST");
        let Statement::Next(sequence) = &ast.statement else {
            panic!("expected MATCH input feeding INSERT, got {:?}", ast.statement);
        };

        let Statement::Query(gql_rs::ast::Query::Basic(input)) = sequence.source_statement.as_ref() else {
            panic!("expected MATCH query as INSERT input");
        };
        assert!(matches!(input.pipeline.as_slice(), [gql_rs::ast::QueryStep::Match(_), gql_rs::ast::QueryStep::Filter(_)]));

        let Statement::Data(DataStatement::Insert(insert)) = sequence.target_statement.as_ref() else {
            panic!("expected INSERT after MATCH input");
        };
        assert_eq!(insert.patterns.len(), 1);

        let elements = &insert.patterns[0].elements;
        assert_eq!(elements.len(), 3);
        assert!(matches!(&elements[0], PatternElement::Node(node) if node.id.as_deref() == Some("a")));
        assert!(matches!(&elements[2], PatternElement::Node(node) if node.id.as_deref() == Some("b")));
        assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);

        let serialized = serde_json::to_value(ast).expect("AST should serialize");
        assert!(serialized["statement"]["Next"]["source_statement"]["Query"].is_object());
        assert!(serialized["statement"]["Next"]["target_statement"]["Data"]["Insert"].is_object());
    }
}
