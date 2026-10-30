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

use crate::ast::*;

fn span() -> Span {
    Span {
        start_line: 1,
        start_col: 1,
        end_line: 1,
        end_col: 1,
    }
}

fn nested_next_statement() -> NextStatement {
    NextStatement {
        source_statement: Box::new(Statement::Session(SessionStatement {
            kind: SessionCommandKind::Activity,
            set_clauses: vec![],
            reset_clauses: vec![],
        })),
        target_statement: Box::new(Statement::Let(LetStatement {
            variable_definitions: vec![],
            span: span(),
        })),
        yield_clause: None,
        next: Some(Box::new(NextStatement {
            source_statement: Box::new(Statement::Query(Query::Basic(BasicQuery {
                pipeline: vec![],
                result_statement: PrimitiveResultStatement::Finish { span: span() },
                span: span(),
            }))),
            target_statement: Box::new(Statement::Catalog(CatalogStatementBlock {
                statements: vec![],
            })),
            yield_clause: None,
            next: None,
            span: span(),
        })),
        span: span(),
    }
}

#[test]
fn next_statement_transaction_intent_merges_recursive_chains() {
    assert_eq!(
        nested_next_statement().transaction_intent(),
        TransactionIntent::CatalogWrite
    );
}
