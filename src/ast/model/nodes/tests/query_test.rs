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

use crate::ast::*;

fn span() -> Span {
    Span {
        start_line: 1,
        start_col: 1,
        end_line: 1,
        end_col: 1,
    }
}

fn boolean_literal(value: bool) -> Expression {
    Expression::Literal(Literal::Boolean(value))
}

fn match_step() -> QueryStep {
    QueryStep::Match(MatchClause {
        optional: false,
        match_clause: MatchClauseItem {
            patterns: vec![],
            span: span(),
        },
        span: span(),
    })
}

fn let_step() -> QueryStep {
    QueryStep::Let(LetStatement {
        variable_definitions: vec![],
        span: span(),
    })
}

fn filter_step() -> QueryStep {
    QueryStep::Filter(FilterClause {
        condition: boolean_literal(true),
        span: span(),
    })
}

fn having_step() -> QueryStep {
    QueryStep::Having(FilterClause {
        condition: boolean_literal(false),
        span: span(),
    })
}

fn for_step() -> QueryStep {
    QueryStep::For(ForClause {
        variable: "item".to_string(),
        expression: boolean_literal(true),
        ordinality_or_offset_variable: None,
        span: span(),
    })
}

fn page_step() -> QueryStep {
    QueryStep::Page(OrderByAndPageStatement {
        order_clause: None,
        limit_clause: None,
        span: span(),
    })
}

fn use_graph_step() -> QueryStep {
    QueryStep::UseGraph(GraphExpressionClause {
        kind: GraphExpressionKind::CurrentGraph,
        text: "CURRENT GRAPH".to_string(),
        graph_reference: None,
        object_name_or_binding_variable: None,
        span: span(),
    })
}

fn read_only_call_clause() -> CallClause {
    CallClause {
        procedure_name: "read_only_proc".to_string(),
        arguments: vec![],
        yield_items: None,
        is_optional: false,
        inline_procedure_body: None,
        inline_procedure_statement: None,
        span: span(),
    }
}

fn catalog_write_call_clause() -> CallClause {
    CallClause {
        procedure_name: "write_proc".to_string(),
        arguments: vec![],
        yield_items: None,
        is_optional: false,
        inline_procedure_body: None,
        inline_procedure_statement: Some(Box::new(Statement::Catalog(CatalogStatementBlock {
            statements: vec![],
        }))),
        span: span(),
    }
}

fn read_only_call_step() -> QueryStep {
    QueryStep::Call(read_only_call_clause())
}

fn catalog_write_call_step() -> QueryStep {
    QueryStep::Call(catalog_write_call_clause())
}

fn from_subquery_step() -> QueryStep {
    QueryStep::FromSubquery(Box::new(Query::Basic(BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Finish { span: span() },
        span: span(),
    })))
}

fn return_clause() -> ReturnClause {
    ReturnClause {
        distinct: DistinctQualifier::None,
        items: vec![ReturnItem {
            expression: boolean_literal(true),
            alias: None,
            span: span(),
        }],
        span: span(),
    }
}

fn read_only_finish_basic_query() -> BasicQuery {
    BasicQuery {
        pipeline: vec![],
        result_statement: PrimitiveResultStatement::Finish { span: span() },
        span: span(),
    }
}

fn read_only_return_basic_query() -> BasicQuery {
    BasicQuery {
        pipeline: vec![match_step(), let_step()],
        result_statement: PrimitiveResultStatement::Return {
            return_clause: return_clause(),
            group_clause: None,
            page_clause: None,
            span: span(),
        },
        span: span(),
    }
}

fn catalog_write_return_basic_query() -> BasicQuery {
    BasicQuery {
        pipeline: vec![catalog_write_call_step()],
        result_statement: PrimitiveResultStatement::Return {
            return_clause: return_clause(),
            group_clause: None,
            page_clause: None,
            span: span(),
        },
        span: span(),
    }
}

#[test]
fn query_and_basic_query_transaction_intent_cover_read_only_write_and_set_operations() {
    let finish_basic = read_only_finish_basic_query();
    let read_only_basic = read_only_return_basic_query();
    let write_basic = catalog_write_return_basic_query();

    assert_eq!(finish_basic.transaction_intent(), TransactionIntent::ReadOnly);
    assert_eq!(read_only_basic.transaction_intent(), TransactionIntent::ReadOnly);
    assert_eq!(write_basic.transaction_intent(), TransactionIntent::CatalogWrite);

    assert_eq!(
        Query::Basic(finish_basic.clone()).transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert_eq!(
        Query::Basic(read_only_basic.clone()).transaction_intent(),
        TransactionIntent::ReadOnly
    );
    assert_eq!(
        Query::Basic(write_basic.clone()).transaction_intent(),
        TransactionIntent::CatalogWrite
    );

    let set_operation_query = Query::SetOperation(SetOperation {
        left: Box::new(Query::Basic(read_only_return_basic_query())),
        operation: SetOperationType::Union,
        right: Box::new(Query::Basic(read_only_finish_basic_query())),
        limit_clause: None,
        order_clause: None,
        span: span(),
    });

    assert_eq!(set_operation_query.transaction_intent(), TransactionIntent::ReadOnly);
}

#[test]
fn query_step_transaction_intent_covers_all_variants() {
    let steps = vec![
        (match_step(), TransactionIntent::ReadOnly),
        (let_step(), TransactionIntent::ReadOnly),
        (filter_step(), TransactionIntent::ReadOnly),
        (having_step(), TransactionIntent::ReadOnly),
        (for_step(), TransactionIntent::ReadOnly),
        (page_step(), TransactionIntent::ReadOnly),
        (read_only_call_step(), TransactionIntent::ReadOnly),
        (catalog_write_call_step(), TransactionIntent::CatalogWrite),
        (use_graph_step(), TransactionIntent::ReadOnly),
        (from_subquery_step(), TransactionIntent::ReadOnly),
    ];

    for (step, expected) in steps {
        assert_eq!(step.transaction_intent(), expected);
    }
}

#[test]
fn call_statement_transaction_intent_delegates_to_clause() {
    let read_only_call = CallStatement {
        call_clause: read_only_call_clause(),
        span: span(),
    };
    let write_call = CallStatement {
        call_clause: catalog_write_call_clause(),
        span: span(),
    };

    assert_eq!(read_only_call.transaction_intent(), TransactionIntent::ReadOnly);
    assert_eq!(write_call.transaction_intent(), TransactionIntent::CatalogWrite);
}
