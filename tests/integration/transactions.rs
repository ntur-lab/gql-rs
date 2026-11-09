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

pub mod test_query_transaction {
    use gql_rs::{
        DefaultGQLQueryAnalyzer, GQLQueryAnalyzer,
        ast::{Query, Statement, TransactionCommandKind},
    };

    test_transaction_statement!(
        test_start_txn_read_only_typed,
        "START TRANSACTION READ ONLY",
        expected_mode: |txn: &gql_rs::ast::TransactionStatement| {
            use gql_rs::ast::TransactionMode;
            txn.mode.as_ref().map(|m| matches!(m, TransactionMode::ReadOnly)).unwrap_or(false)
        }
    );

    test_transaction_statement!(
        test_start_txn_read_write_typed,
        "START TRANSACTION READ WRITE",
        expected_mode: |txn: &gql_rs::ast::TransactionStatement| {
            use gql_rs::ast::TransactionMode;
            txn.mode.as_ref().map(|m| matches!(m, TransactionMode::ReadWrite)).unwrap_or(false)
        }
    );

    test_transaction_statement!(
        test_commit_no_mode,
        "COMMIT",
        expected_mode: |txn: &gql_rs::ast::TransactionStatement| {
            txn.mode.is_none()
        }
    );

    test_transaction_statement!(
        test_rollback_no_mode,
        "ROLLBACK",
        expected_mode: |txn: &gql_rs::ast::TransactionStatement| {
            txn.mode.is_none()
        }
    );

    #[test]
    fn test_transaction_activity_start_body_commit_preserved_in_ast() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("START TRANSACTION RETURN 1 COMMIT")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");

        let Statement::Next(outer_next) = &ast.statement else {
            panic!("expected outer NEXT statement wrapping transaction activity");
        };

        let Statement::Transaction(start_tx) = outer_next.source_statement.as_ref() else {
            panic!("expected START TRANSACTION as first statement");
        };
        assert!(matches!(start_tx.kind, TransactionCommandKind::Start));

        let Statement::Next(inner_next) = outer_next.target_statement.as_ref() else {
            panic!("expected inner NEXT chaining body and COMMIT");
        };

        let Statement::Query(Query::Basic(_)) = inner_next.source_statement.as_ref() else {
            panic!("expected query body in transaction activity");
        };

        let Statement::Transaction(commit_tx) = inner_next.target_statement.as_ref() else {
            panic!("expected COMMIT as final statement");
        };
        assert!(matches!(commit_tx.kind, TransactionCommandKind::Commit));
    }

    #[test]
    fn test_transaction_activity_body_rollback_preserved_in_ast() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("RETURN 1 ROLLBACK")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");

        let Statement::Next(next_stmt) = &ast.statement else {
            panic!("expected NEXT statement wrapping body and ROLLBACK");
        };

        let Statement::Query(Query::Basic(_)) = next_stmt.source_statement.as_ref() else {
            panic!("expected query body before rollback");
        };

        let Statement::Transaction(rollback_tx) = next_stmt.target_statement.as_ref() else {
            panic!("expected rollback transaction statement");
        };
        assert!(matches!(rollback_tx.kind, TransactionCommandKind::Rollback));
    }
}
