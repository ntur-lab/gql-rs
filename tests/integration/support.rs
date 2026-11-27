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

/// Macro for testing SESSION statements with structured payload verification.
/// Validates parsing and that set/reset clauses are properly constructed.
#[macro_export]
macro_rules! test_session_statement {
    ($test_name:ident, $query:expr, expected_set_count: $set_count:expr, expected_reset_count: $reset_count:expr) => {
        #[test]
        fn $test_name() {
            use gql_rs::ast::{ProgramActivity, ProgramBody, SessionCommand};

            let ast = gql_rs::analyze($query).expect("SESSION statement should analyze");
            let (sets, resets) = match &ast.body {
                ProgramBody::Activity { activity: ProgramActivity::Session(session), .. } => {
                    let sets = session.commands.iter().filter(|cmd| matches!(cmd, SessionCommand::Set { .. })).count();
                    (sets, session.commands.len() - sets)
                }
                ProgramBody::CloseOnly(_) => (0, 0),
                other => panic!("Expected session program, got: {other:?}"),
            };
            assert_eq!(sets, $set_count);
            assert_eq!(resets, $reset_count);
        }
    };
}

/// Macro for testing CREATE GRAPH TYPE with nested specifications.
/// Validates parsing and that the number of element type specifications matches expectations.
#[macro_export]
macro_rules! test_graph_type_specification {
    ($test_name:ident, $query:expr, expected_elements: $element_count:expr) => {
        #[test]
        fn $test_name() {
            use gql_rs::ast::{CatalogStatement, Statement};

            let query_str = $query;
            println!("Testing CREATE GRAPH TYPE: {}", query_str);

            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            match analyzer.exec(query_str) {
                Ok(_) => {
                    if let Some(ast) = analyzer.get_ast() {
                        match ast.single_statement().expect("single catalog statement") {
                            Statement::Catalog(block) => {
                                if let Some(CatalogStatement::CreateGraphType(create_gt)) = block.statements.first() {
                                    if let gql_rs::ast::GraphTypeSourceClause::NestedSpecification {
                                        nested_graph_type_specification,
                                        ..
                                    } = &create_gt.source
                                    {
                                        let element_count =
                                            nested_graph_type_specification.element_type_specifications.len();
                                        println!("Element count: {}", element_count);

                                        assert_eq!(
                                            element_count, $element_count,
                                            "Expected {} elements, got {}",
                                            $element_count, element_count
                                        );
                                    } else {
                                        panic!("Expected nested specification in CREATE GRAPH TYPE");
                                    }
                                } else {
                                    panic!("Expected CreateGraphType statement");
                                }
                            }
                            other => {
                                panic!("Expected Catalog statement, got: {:?}", other);
                            }
                        }
                    } else {
                        panic!("No AST generated for query: {}", query_str);
                    }
                }
                Err(e) => {
                    panic!("Failed to parse CREATE GRAPH TYPE: {:?}", e);
                }
            }
        }
    };
}
