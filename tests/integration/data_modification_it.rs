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

    #[test]
    fn matched_insert_preserves_terminal_return() {
        use gql_rs::ast::{PrimitiveResultStatement, Query};
        let ast = gql_rs::analyze("MATCH (n) INSERT (m) RETURN m").unwrap();
        let Statement::Next(sequence) = &ast.statement else { panic!("expected input and insert"); };
        let Statement::Next(insert_and_result) = sequence.target_statement.as_ref() else {
            panic!("expected INSERT followed by RETURN");
        };
        assert!(matches!(insert_and_result.source_statement.as_ref(), Statement::Data(DataStatement::Insert(_))));
        let Statement::Query(Query::Basic(result)) = insert_and_result.target_statement.as_ref() else {
            panic!("expected result query");
        };
        assert!(matches!(result.result_statement, PrimitiveResultStatement::Return { .. }));
        assert!(gql_rs::analyze("MATCH (n) INSERT (m) RETURN missing").is_err());
    }

    #[test]
    fn insert_never_silently_discards_following_operations() {
        for query in [
            "MATCH (n) INSERT (m) SET m.name = 'Nicola'",
            "MATCH (n) INSERT (m) INSERT (p)",
            "MATCH (n) INSERT (m) MATCH (p) RETURN p",
        ] {
            assert!(gql_rs::parse(query).is_err(), "unsupported pipeline must fail explicitly: {query}");
        }
    }

    #[test]
    fn focused_insert_keeps_graph_context_and_write_intent() {
        for query in ["USE social INSERT (m)", "USE social MATCH (n) INSERT (m) RETURN m"] {
            let ast = gql_rs::analyze(query).unwrap();
            let Statement::Focused(focused) = &ast.statement else {
                panic!("graph context must enclose the write: {query}");
            };
            assert_eq!(focused.graph_expression.text, "social");
            assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);
            assert!(serde_json::to_string(&ast).unwrap().contains("Insert"));
        }
    }


    #[test]
    fn matched_insert_preserves_input_pagination_and_explicit_next() {
        let ast = gql_rs::analyze("MATCH (n) ORDER BY n.name LIMIT 1 INSERT (m)").unwrap();
        let Statement::Next(sequence) = &ast.statement else { panic!("expected input sequence"); };
        let Statement::Query(gql_rs::ast::Query::Basic(input)) = sequence.source_statement.as_ref() else {
            panic!("expected input query");
        };
        assert!(matches!(input.pipeline.last(), Some(gql_rs::ast::QueryStep::Page(_))));
        let ast = gql_rs::analyze("MATCH (n) INSERT (m) RETURN m NEXT RETURN 1").unwrap();
        let Statement::Next(outer) = &ast.statement else { panic!("expected explicit NEXT"); };
        let Statement::Next(input) = outer.source_statement.as_ref() else { panic!("expected matched INSERT"); };
        assert!(matches!(input.target_statement.as_ref(), Statement::Next(_)));
    }

}
