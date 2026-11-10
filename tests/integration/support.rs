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
            use gql_rs::ast::Statement;

            let query_str = $query;
            println!("Testing SESSION statement: {}", query_str);

            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            match analyzer.exec(query_str) {
                Ok(_) => {
                    if let Some(ast) = analyzer.get_ast() {
                        match &ast.statement {
                            Statement::Session(session) => {
                                println!("Session kind: {:?}", session.kind);
                                println!("Set clauses count: {}", session.set_clauses.len());
                                println!("Reset clauses count: {}", session.reset_clauses.len());

                                assert_eq!(
                                    session.set_clauses.len(),
                                    $set_count,
                                    "Expected {} set clauses, got {}",
                                    $set_count,
                                    session.set_clauses.len()
                                );
                                assert_eq!(
                                    session.reset_clauses.len(),
                                    $reset_count,
                                    "Expected {} reset clauses, got {}",
                                    $reset_count,
                                    session.reset_clauses.len()
                                );
                            }
                            other => {
                                panic!("Expected Session statement, got: {:?}", other);
                            }
                        }
                    } else {
                        panic!("No AST generated for query: {}", query_str);
                    }
                }
                Err(e) => {
                    panic!("Failed to parse SESSION statement: {:?}", e);
                }
            }
        }
    };
}

/// Macro for testing TRANSACTION statements with mode verification.
/// Validates parsing and that the transaction mode matches expected criteria.
#[macro_export]
macro_rules! test_transaction_statement {
    ($test_name:ident, $query:expr, expected_mode: $mode_check:expr) => {
        #[test]
        fn $test_name() {
            use gql_rs::ast::Statement;

            let query_str = $query;
            println!("Testing TRANSACTION statement: {}", query_str);

            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            match analyzer.exec(query_str) {
                Ok(_) => {
                    if let Some(ast) = analyzer.get_ast() {
                        match &ast.statement {
                            Statement::Transaction(transaction) => {
                                println!("Transaction kind: {:?}", transaction.kind);
                                println!("Transaction mode: {:?}", transaction.mode);

                                if !$mode_check(transaction) {
                                    panic!("Transaction mode check failed for: {}", query_str);
                                }
                            }
                            other => {
                                panic!("Expected Transaction statement, got: {:?}", other);
                            }
                        }
                    } else {
                        panic!("No AST generated for query: {}", query_str);
                    }
                }
                Err(e) => {
                    panic!("Failed to parse TRANSACTION statement: {:?}", e);
                }
            }
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
            use gql_rs::ast::{Statement, CatalogStatement};

            let query_str = $query;
            println!("Testing CREATE GRAPH TYPE: {}", query_str);

            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            match analyzer.exec(query_str) {
                Ok(_) => {
                    if let Some(ast) = analyzer.get_ast() {
                        match &ast.statement {
                            Statement::Catalog(block) => {
                                if let Some(CatalogStatement::CreateGraphType(create_gt)) =
                                    block.statements.first()
                                {
                                    if let gql_rs::ast::GraphTypeSourceClause::NestedSpecification {
                                        nested_graph_type_specification,
                                        ..
                                    } = &create_gt.source
                                    {
                                        let element_count = nested_graph_type_specification
                                            .element_type_specifications
                                            .len();
                                        println!("Element count: {}", element_count);

                                        assert_eq!(
                                            element_count,
                                            $element_count,
                                            "Expected {} elements, got {}",
                                            $element_count,
                                            element_count
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
