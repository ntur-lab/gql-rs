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

use super::*;

pub(super) fn validate_session_activity(activity: &SessionActivity) -> Result<(), GqlError> {
    if activity.commands.is_empty() {
        return Err(validation_err("Session activity must contain a command", Some(&activity.span)));
    }

    let mut saw_reset = false;
    for command in &activity.commands {
        match command {
            SessionCommand::Set { clause, span } => {
                if saw_reset {
                    return Err(validation_err("SESSION SET cannot follow SESSION RESET", Some(span)));
                }
                validate_session_set_clause(clause)?;
            }
            SessionCommand::Reset { clause, .. } => {
                saw_reset = true;
                if let Some(clause) = clause {
                    validate_session_reset_clause(clause)?;
                }
            }
        }
    }
    Ok(())
}

pub(super) fn validate_procedure_body(
    body: &ProcedureBody,
    outer_scope: &HashSet<String>,
) -> Result<HashSet<String>, GqlError> {
    let mut scope = outer_scope.clone();
    let mut local_names = HashSet::new();
    let mut local_types = HashMap::new();
    for definition in &body.definitions {
        let (name, span) = match definition {
            BindingVariableDefinition::Graph(definition) => {
                validate_graph_initializer(&definition.initializer, &scope)?;
                (&definition.name, &definition.span)
            }
            BindingVariableDefinition::BindingTable(definition) => {
                validate_table_initializer(&definition.initializer, &scope)?;
                (&definition.name, &definition.span)
            }
            BindingVariableDefinition::Value(definition) => {
                match &definition.initializer {
                    ValueInitializer::Expression(expression) => {
                        validate_expression_with_types(expression, &scope, &local_types)?;
                        if let Some(declared) = &definition.declared_type {
                            validate_initializer_type(
                                &format!("VALUE variable '{}'", definition.name),
                                expression, &declared.value_type, &declared.span, &scope, &local_types,
                            )?;
                        }
                    }
                    ValueInitializer::Graph(expression) => validate_graph_initializer(expression, &scope)?,
                    ValueInitializer::BindingTable(expression) => validate_table_initializer(expression, &scope)?,
                }
                (&definition.name, &definition.span)
            }
        };

        if name.trim().is_empty() || !local_names.insert(name) {
            return Err(validation_err(format!("Invalid or duplicate binding variable '{name}'"), Some(span)));
        }
        if let BindingVariableDefinition::Value(definition) = definition {
            let value_type = definition.declared_type.as_ref().map(|declared| declared.value_type.base.clone());
            let value_type = value_type.or_else(|| match &definition.initializer {
                ValueInitializer::Expression(expression) => infer_expression_type(expression, &local_types, Some(&scope)),
                _ => None,
            });
            if let Some(value_type) = value_type {
                local_types.insert(name.clone(), value_type);
            }
        }
        scope.insert(name.clone());
    }

    let block = &body.statements;
    let mut output = validate_statement_output(&block.first, &mut scope)?;
    for continuation in &block.continuations {
        if let Some(yield_clause) = &continuation.yield_clause {
            validate_yield_clause(yield_clause)?;
            scope.extend(yield_clause.items.iter().map(|item| {
                item.alias.as_deref().unwrap_or(&item.column_name).trim().to_owned()
            }));
        }
        output = validate_statement_output(&continuation.statement, &mut scope)?;
    }
    Ok(output)
}

fn validate_statement_output(statement: &Statement, scope: &mut HashSet<String>) -> Result<HashSet<String>, GqlError> {
    match statement {
        Statement::Query(query) => validate_query_output(query, scope),
        Statement::Nested(body) => validate_procedure_body(body, scope),
        Statement::Focused(focused) => validate_statement_output(&focused.statement, scope),
        _ => {
            validate_statement(statement, scope)?;
            Ok(HashSet::new())
        }
    }
}

fn validate_graph_initializer(expression: &GraphExpression, scope: &HashSet<String>) -> Result<(), GqlError> {
    match &expression.value {
        GraphExpressionValue::Object(object) => validate_expression(object.expression(), scope),
        _ => Ok(()),
    }
}

fn validate_table_initializer(expression: &BindingTableExpression, scope: &HashSet<String>) -> Result<(), GqlError> {
    match &expression.value {
        BindingTableExpressionValue::Object(object) => validate_expression(object.expression(), scope),
        BindingTableExpressionValue::Query(query) => validate_query(query, &mut scope.clone()),
        _ => Ok(()),
    }
}
