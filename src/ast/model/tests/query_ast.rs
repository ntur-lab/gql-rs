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

use crate::ast::model::*;

fn finish_statement() -> Statement {
    Statement::Query(Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Finish { span: Span::default() },
        span: Span::default(),
    }))
}

fn data_statement() -> Statement {
    Statement::Data(DataStatement::Set(SetStatement {
        items: vec![],
        span: Span::default(),
    }))
}

fn catalog_statement() -> Statement {
    Statement::Catalog(CatalogStatementBlock {
        statements: vec![CatalogStatement::CreateSchema(CreateSchema {
            if_not_exists: false,
            target: CatalogSchemaName {
                absolute_directory_path: "/catalog".to_string(),
                schema_name: "social".to_string(),
                span: Span::default(),
            },
        })],
    })
}

fn program(first: Statement, continuations: Vec<NextContinuation>) -> QueryAST {
    let span = Span::default();
    QueryAST {
        body: ProgramBody::Activity {
            activity: ProgramActivity::Transaction(TransactionActivity::WithBody {
                start: None,
                body: ProcedureBody {
                    at_schema: None,
                    definitions: vec![],
                    statements: StatementBlock { first, continuations, span: span.clone() },
                    span: span.clone(),
                },
                end: None,
                span: span.clone(),
            }),
            session_close: None,
        },
        span,
    }
}

#[test]
fn validate_accepts_valid_query() {
    let ast = program(finish_statement(), vec![]);

    assert!(ast.validate().is_ok());
}

#[test]
fn validate_rejects_unknown_return_variable() {
    let ast = program(Statement::Query(Query::Basic(BasicQuery {
            pipeline: vec![],
            result_statement: PrimitiveResultStatement::Return {
                return_clause: ReturnClause {
                    distinct: DistinctQualifier::None,
                    projection: ReturnProjection::Items(vec![ReturnItem {
                        expression: Expression::Variable {
                            name: "missing".to_string(),
                            span: Span::default(),
                        },
                        alias: None,
                        span: Span::default(),
                    }]),
                    span: Span::default(),
                },
                group_clause: None,
                page_clause: None,
                span: Span::default(),
            },
            span: Span::default(),
        })), vec![]);

    let error = ast
        .validate()
        .expect_err("an undeclared RETURN variable must fail validation");

    assert!(
        matches!(error, crate::error_handler::error::GqlError::Validation { .. }),
        "expected a validation error, got {error:?}"
    );
    assert!(
        error.to_string().contains("missing"),
        "error must identify the unknown variable: {error}"
    );
}

#[test]
fn validate_rejects_empty_catalog_block() {
    let ast = program(Statement::Catalog(CatalogStatementBlock { statements: vec![] }), vec![]);

    let error = ast.validate().expect_err("an empty catalog block must fail validation");

    assert!(
        error.to_string().contains("Catalog block cannot be empty"),
        "unexpected error: {error}"
    );
}

#[test]
fn transaction_intent_returns_read_only_for_query() {
    let ast = program(finish_statement(), vec![]);

    assert_eq!(ast.transaction_intent(), TransactionIntent::ReadOnly);
}

#[test]
fn transaction_intent_returns_data_write_for_data_statement() {
    let ast = program(data_statement(), vec![]);

    assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);
}

#[test]
fn transaction_intent_returns_catalog_write_for_catalog_statement() {
    let ast = program(catalog_statement(), vec![]);

    assert_eq!(ast.transaction_intent(), TransactionIntent::CatalogWrite);
}

#[test]
fn transaction_intent_combines_data_and_catalog_writes() {
    let ast = program(data_statement(), vec![NextContinuation {
        statement: catalog_statement(),
        yield_clause: None,
        span: Span::default(),
    }]);

    assert_eq!(ast.transaction_intent(), TransactionIntent::DataAndCatalogWrite);
}

#[test]
fn transaction_intent_preserves_intent_through_focused_statement() {
    let ast = program(Statement::Focused(FocusedStatement {
            graph_expression: GraphExpressionClause {
                kind: GraphExpressionKind::GraphReference,
                text: "social".to_string(),
                graph_reference: Some("social".to_string()),
                object_name_or_binding_variable: None,
                span: Span::default(),
            },
            statement: Box::new(data_statement()),
            span: Span::default(),
        }), vec![]);

    assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);
}
