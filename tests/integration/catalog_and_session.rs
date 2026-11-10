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

mod catalog_and_session_tests {
    use gql_rs::{
        ast::{CatalogStatement, Statement}, DefaultGQLQueryAnalyzer,
        GQLQueryAnalyzer,
    };

    test_session_statement!(
        test_session_set_schema_struct,
        "SESSION SET SCHEMA CURRENT_SCHEMA",
        expected_set_count: 1,
        expected_reset_count: 0
    );

    test_session_statement!(
        test_session_set_graph_struct,
        "SESSION SET GRAPH $g = my_graph",
        expected_set_count: 1,
        expected_reset_count: 0
    );

    test_session_statement!(
        test_session_set_timezone_struct,
        "SESSION SET TIME ZONE 'America/New_York'",
        expected_set_count: 1,
        expected_reset_count: 0
    );

    test_session_statement!(
        test_session_set_parameter_value,
        "SESSION SET VALUE $v = 42",
        expected_set_count: 1,
        expected_reset_count: 0
    );

    test_session_statement!(
        test_session_reset_schema_struct,
        "SESSION RESET SCHEMA",
        expected_set_count: 0,
        expected_reset_count: 1
    );

    test_session_statement!(
        test_session_reset_graph_struct,
        "SESSION RESET GRAPH",
        expected_set_count: 0,
        expected_reset_count: 1
    );

    test_session_statement!(
        test_session_reset_timezone_struct,
        "SESSION RESET TIME ZONE",
        expected_set_count: 0,
        expected_reset_count: 1
    );

    test_session_statement!(
        test_session_reset_all_parameters_struct,
        "SESSION RESET ALL PARAMETERS",
        expected_set_count: 0,
        expected_reset_count: 1
    );

    test_session_statement!(
        test_session_mixed_set_reset,
        "SESSION SET GRAPH CURRENT_GRAPH SESSION RESET SCHEMA",
        expected_set_count: 1,
        expected_reset_count: 1
    );

    test_session_statement!(
        test_session_close_struct,
        "SESSION CLOSE",
        expected_set_count: 0,
        expected_reset_count: 0
    );

    test_graph_type_specification!(
        test_create_graph_type_single_node,
        "CREATE GRAPH TYPE simple { NODE TYPE Person }",
        expected_elements: 1
    );

    test_graph_type_specification!(
        test_create_graph_type_multiple_nodes,
        "CREATE GRAPH TYPE social { NODE TYPE Person, NODE TYPE Company }",
        expected_elements: 2
    );

    #[test]
    fn test_catalog_create_schema_is_typed_not_raw() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE SCHEMA /myschemas/foo")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let first = block
            .statements
            .first()
            .expect("catalog block should contain one statement");

        assert!(
            matches!(first, CatalogStatement::CreateSchema(_)),
            "expected typed CreateSchema, got {:?}",
            first
        );
    }

    #[test]
    fn test_catalog_create_graph_type_like_is_typed_not_raw() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH TYPE my_type LIKE CURRENT_GRAPH")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let first = block
            .statements
            .first()
            .expect("catalog block should contain one statement");

        assert!(
            matches!(first, CatalogStatement::CreateGraphType(_)),
            "expected typed CreateGraphType, got {:?}",
            first
        );
    }

    #[test]
    fn test_create_graph_of_typed_reference_maps_to_of_typed_reference_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH social_graph TYPED my_graph_type")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let Some(CatalogStatement::CreateGraph(create_graph)) = block.statements.first() else {
            panic!("expected CreateGraph statement");
        };

        assert!(matches!(
            &create_graph.graph_type_clause,
            gql_rs::ast::CreateGraphTypeClause::Of(
                gql_rs::ast::OfGraphTypeClause::TypedReference { .. }
            )
        ));
    }

    #[test]
    fn test_create_graph_type_copy_of_maps_to_copy_of_source_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH TYPE my_graph_type COPY OF base_graph_type")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let Some(CatalogStatement::CreateGraphType(create_graph_type)) = block.statements.first()
        else {
            panic!("expected CreateGraphType statement");
        };

        assert!(matches!(
            &create_graph_type.source,
            gql_rs::ast::GraphTypeSourceClause::CopyOf { .. }
        ));
    }

    #[test]
    fn test_create_graph_type_like_maps_to_like_source_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH TYPE my_graph_type LIKE CURRENT_GRAPH")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let Some(CatalogStatement::CreateGraphType(create_graph_type)) = block.statements.first()
        else {
            panic!("expected CreateGraphType statement");
        };

        assert!(matches!(
            &create_graph_type.source,
            gql_rs::ast::GraphTypeSourceClause::LikeGraph { .. }
        ));
    }

    #[test]
    fn test_create_graph_type_nested_spec_maps_to_nested_source_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH TYPE my_graph_type { NODE TYPE Person }")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let Some(CatalogStatement::CreateGraphType(create_graph_type)) = block.statements.first()
        else {
            panic!("expected CreateGraphType statement");
        };

        assert!(matches!(
            &create_graph_type.source,
            gql_rs::ast::GraphTypeSourceClause::NestedSpecification { .. }
        ));
    }

    #[test]
    fn test_create_graph_type_as_nested_spec_maps_to_nested_source_clause() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("CREATE GRAPH TYPE my_graph_type AS { NODE TYPE Person }")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Catalog(block) = &ast.statement else {
            panic!("expected catalog statement block");
        };

        let Some(CatalogStatement::CreateGraphType(create_graph_type)) = block.statements.first()
        else {
            panic!("expected CreateGraphType statement");
        };

        assert!(matches!(
            &create_graph_type.source,
            gql_rs::ast::GraphTypeSourceClause::NestedSpecification { .. }
        ));
    }
}
