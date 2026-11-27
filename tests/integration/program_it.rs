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

#[test]
fn preserves_mixed_definitions_and_three_explicit_continuations() {
    let ast = analyze(include_str!("../fixtures/ast/transactions/procedure_preamble_next.gql")).unwrap();
    let ProgramBody::Activity {
        activity: ProgramActivity::Transaction(TransactionActivity::WithBody {
            start: Some(start), body, end: Some(EndTransactionCommand::Commit { span: commit }), ..
        }),
        session_close: Some(close),
    } = &ast.body else {
        panic!("expected full program");
    };
    let Some(AtSchemaClause { schema: SchemaReference::Absolute { path }, .. }) = &body.at_schema else {
        panic!("expected an absolute schema");
    };
    assert_eq!(path, &["myschemas", "social"]);
    let [
        BindingVariableDefinition::Value(value),
        BindingVariableDefinition::Graph(graph),
        BindingVariableDefinition::BindingTable(table),
    ] = body.definitions.as_slice() else {
        panic!("expected ordered mixed definitions");
    };
    assert_eq!((&*value.name, &*graph.name, &*table.name), ("x", "g", "t"));
    assert!(value.declared_type.as_ref().unwrap().explicitly_typed);
    assert!(graph.property_keyword);
    assert!(table.binding_keyword);
    assert!(matches!(graph.initializer.value, GraphExpressionValue::Current { property: true }));
    let BindingTableExpressionValue::Query(query) = &table.initializer.value else {
        panic!("expected nested table query");
    };
    let Query::Nested(nested) = query.as_ref() else {
        panic!("expected full nested body");
    };
    assert_eq!(nested.body.definitions.len(), 1);

    assert_eq!(body.statements.iter().count(), 4);
    for (continuation, expected) in body.statements.continuations.iter().zip(["x", "b", "c"]) {
        assert!(matches!(continuation.statement, Statement::Query(_)));
        assert_eq!(continuation.yield_clause.as_ref().unwrap().items[0].column_name, expected);
    }
    assert_eq!(start.span.start_line, 1);
    assert_eq!(commit.start_line, 10);
    assert_eq!(close.span.end_line, 11);

    let json = serde_json::to_value(&ast).unwrap();
    let decoded: QueryAST = serde_json::from_value(json.clone()).unwrap();
    decoded.validate().unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), json);
}

#[test]
fn validates_initializers_without_leaking_nested_locals() {
    assert!(analyze("VALUE x = missing RETURN x").is_err());
    assert!(analyze("VALUE x :: BOOL = 1 RETURN x").is_err());
    assert!(analyze("VALUE x :: INT = 1 VALUE y :: BOOL = x RETURN y").is_err());
    assert!(analyze("MATCH (n) NEXT RETURN n").is_ok());
    assert!(analyze("VALUE x = y VALUE y = 1 RETURN x").is_err());
    assert!(analyze("VALUE x = 1 VALUE x = 2 RETURN x").is_err());
    assert!(analyze("BINDING TABLE t = { VALUE hidden = 1 RETURN hidden AS visible } RETURN hidden").is_err());
    assert!(analyze("CALL { VALUE hidden = 1 RETURN hidden AS visible } RETURN hidden").is_err());
    assert!(analyze(include_str!("../fixtures/ast/procedures/inline_preamble.gql")).is_ok());
}

#[test]
fn calculates_write_intent_through_nested_inline_body_and_next() {
    let ast = analyze("START TRANSACTION RETURN 1 NEXT CALL { INSERT (n:Person) FINISH } FINISH COMMIT").unwrap();
    assert_eq!(ast.transaction_intent(), TransactionIntent::DataWrite);
}

#[test]
fn preserves_catalog_and_delimited_initializer_references() {
    let ast = analyze("GRAPH g = my_schema.my_graph TABLE t = `my table` RETURN 1").unwrap();
    let ProgramBody::Activity {
        activity: ProgramActivity::Transaction(TransactionActivity::WithBody { body, .. }), ..
    } = &ast.body else {
        panic!("expected procedure body");
    };
    let [BindingVariableDefinition::Graph(graph), BindingVariableDefinition::BindingTable(table)] =
        body.definitions.as_slice() else {
            panic!("expected graph and table definitions");
        };
    let GraphExpressionValue::Catalog(reference) = &graph.initializer.value else {
        panic!("expected structured catalog graph reference");
    };
    assert_eq!(reference.parent, ["my_schema"]);
    assert_eq!(reference.name, "my_graph");
    assert!(matches!(&table.initializer.value, BindingTableExpressionValue::DelimitedName(name) if name == "my table"));
    assert!(analyze("GRAPH g = VARIABLE missing RETURN 1").is_err());
}
