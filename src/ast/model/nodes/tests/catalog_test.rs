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
//   Ntur Lab S.r.l. <dev@ntur-lab.com>
//   Nicola Miotto <nicola.miotto@ntur-lab.com>,
//   Matteo Boscolo <matteo.boscolo@ntur-lab.com>

use crate::ast::*;

fn span() -> Span {
    Span {
        start_line: 1,
        start_col: 1,
        end_line: 1,
        end_col: 1,
    }
}

fn read_only_query_statement() -> Statement {
    Statement::Query(Query::Basic(BasicQuery {
        pipeline: vec![QueryStep::Let(LetStatement {
            variable_definitions: vec![],
            span: span(),
        })],
        result_statement: PrimitiveResultStatement::Finish { span: span() },
        span: span(),
    }))
}

fn write_data_statement() -> Statement {
    Statement::Data(DataStatement::Set(SetStatement {
        items: vec![],
        span: span(),
    }))
}

#[test]
fn statement_transaction_intent_and_requires_write_tx_cover_all_variants() {
    let read_only_query = read_only_query_statement();
    let write_data = write_data_statement();
    let focused_statement = Statement::Focused(FocusedStatement {
        graph_expression: GraphExpressionClause {
            kind: GraphExpressionKind::GraphReference,
            text: "social".to_string(),
            graph_reference: Some("social".to_string()),
            object_name_or_binding_variable: None,
            span: span(),
        },
        statement: Box::new(write_data_statement()),
        span: span(),
    });
    let next_statement = Statement::Next(NextStatement {
        source_statement: Box::new(read_only_query_statement()),
        target_statement: Box::new(write_data_statement()),
        yield_clause: None,
        next: None,
        span: span(),
    });
    let let_statement = Statement::Let(LetStatement {
        variable_definitions: vec![],
        span: span(),
    });
    let session_statement = Statement::Session(SessionStatement {
        kind: SessionCommandKind::Set,
        set_clauses: vec![],
        reset_clauses: vec![],
    });
    let transaction_statement = Statement::Transaction(TransactionStatement {
        kind: TransactionCommandKind::Start,
        mode: None,
        body: None,
        end_command: None,
    });
    let catalog_statement = Statement::Catalog(CatalogStatementBlock { statements: vec![] });
    let mixed_next_statement = Statement::Next(NextStatement {
        source_statement: Box::new(write_data_statement()),
        target_statement: Box::new(Statement::Catalog(CatalogStatementBlock {
            statements: vec![],
        })),
        yield_clause: None,
        next: None,
        span: span(),
    });

    assert_eq!(
        read_only_query.transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert!(!read_only_query.requires_write_tx());

    assert_eq!(
        write_data.transaction_intent(),
        TransactionIntent::DataWrite
    );
    assert!(write_data.requires_write_tx());

    assert_eq!(
        focused_statement.transaction_intent(),
        TransactionIntent::DataWrite
    );
    assert!(focused_statement.requires_write_tx());

    assert_eq!(
        next_statement.transaction_intent(),
        TransactionIntent::DataWrite
    );
    assert!(next_statement.requires_write_tx());

    assert_eq!(
        let_statement.transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert!(!let_statement.requires_write_tx());

    assert_eq!(
        session_statement.transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert!(!session_statement.requires_write_tx());

    assert_eq!(
        transaction_statement.transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert!(!transaction_statement.requires_write_tx());

    assert_eq!(
        catalog_statement.transaction_intent(),
        TransactionIntent::CatalogWrite
    );
    assert!(catalog_statement.requires_write_tx());

    assert_eq!(
        mixed_next_statement.transaction_intent(),
        TransactionIntent::DataAndCatalogWrite
    );
    assert!(mixed_next_statement.requires_write_tx());
}
