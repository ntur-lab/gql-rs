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

use gql_rs::{analyze, ast::*};

fn transaction(ast: &QueryAST) -> (&TransactionActivity, Option<&SessionCloseCommand>) {
    let ProgramBody::Activity { activity: ProgramActivity::Transaction(activity), session_close } = &ast.body else {
        panic!("expected transaction activity");
    };
    (activity, session_close.as_ref())
}

#[test]
fn standalone_transaction_commands_preserve_their_modes_and_spans() {
    for (source, expected_mode) in [
        ("START TRANSACTION READ ONLY", TransactionMode::ReadOnly),
        ("START TRANSACTION READ WRITE", TransactionMode::ReadWrite),
    ] {
        let ast = analyze(source).unwrap();
        let (TransactionActivity::StartOnly(start), None) = transaction(&ast) else {
            panic!("expected standalone START");
        };
        assert_eq!(std::mem::discriminant(start.mode.as_ref().unwrap()), std::mem::discriminant(&expected_mode));
        assert_eq!(start.span, ast.span);
    }

    let commit = analyze("COMMIT").unwrap();
    let rollback = analyze("ROLLBACK").unwrap();
    assert!(matches!(transaction(&commit), (TransactionActivity::EndOnly(EndTransactionCommand::Commit { .. }), None)));
    assert!(matches!(transaction(&rollback), (TransactionActivity::EndOnly(EndTransactionCommand::Rollback { .. }), None)));
}

#[test]
fn test_transaction_activity_start_body_commit_preserved_in_ast() {
    let ast = analyze("START TRANSACTION RETURN 1 COMMIT").unwrap();
    let (TransactionActivity::WithBody { start: Some(start), body, end: Some(EndTransactionCommand::Commit { span }), .. }, None) =
        transaction(&ast) else {
            panic!("expected transaction body with START and COMMIT");
        };
    assert!(matches!(body.statements.first, Statement::Query(Query::Basic(_))));
    assert!(body.statements.continuations.is_empty());
    assert!(start.span.end_col < body.span.start_col);
    assert!(body.span.end_col < span.start_col);
    assert!(ast.single_statement().is_none());
}

#[test]
fn test_transaction_activity_body_rollback_preserved_in_ast() {
    let ast = analyze("RETURN 1 ROLLBACK").unwrap();
    assert!(matches!(transaction(&ast),
        (TransactionActivity::WithBody { start: None, end: Some(EndTransactionCommand::Rollback { .. }), .. }, None)
    ));
    assert!(analyze("START TRANSACTION COMMIT").is_err());
    assert!(analyze("").is_err());
}

#[test]
fn test_write_transaction_activity_followed_by_close_preserves_both_nodes_and_intent() {
    let ast = analyze("START TRANSACTION MATCH (n) SET n.active = TRUE FINISH COMMIT SESSION CLOSE").unwrap();
    let (TransactionActivity::WithBody { end: Some(EndTransactionCommand::Commit { .. }), .. }, Some(close)) =
        transaction(&ast) else {
            panic!("expected transaction body with COMMIT and trailing SESSION CLOSE");
        };
    assert_eq!(close.span.end_col, ast.span.end_col);
    assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);
}
