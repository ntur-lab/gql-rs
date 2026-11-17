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

/// Function to validate a Query AST node, ensuring that all components of the query are semantically correct according to the GQL specification. This includes validating the structure of the query, checking for correct usage of variables, ensuring that clauses are consistent with each other, and enforcing any specific rules related to the types of queries (e.g., basic vs set operation). The function takes a mutable reference to a set of declared variables to track variable usage across the query and its subqueries.
pub(super) fn validate_query(query: &Query, declared_variables: &mut HashSet<String>) -> Result<(), GqlError> {
    validate_query_output(query, declared_variables).map(|_| ())
}

/// Validates a query and returns only the bindings exposed by its result.
pub(super) fn validate_query_output(
    query: &Query,
    declared_variables: &mut HashSet<String>,
) -> Result<HashSet<String>, GqlError> {
    match query {
        Query::Basic(basic) => validate_basic_query(basic, declared_variables),
        Query::SetOperation(set_op) => validate_set_operation_query(set_op, declared_variables),
    }
}

/// Function to validate a Statement AST node, ensuring that all components of the statement are semantically correct according to the GQL specification.
pub(super) fn validate_statement(statement: &Statement, declared_variables: &mut HashSet<String>) -> Result<(), GqlError> {
    match statement {
        Statement::Query(query) => validate_query(query, declared_variables),
        Statement::Data(data) => validate_data_statement(data, declared_variables),
        Statement::Focused(focused) => validate_statement(&focused.statement, declared_variables),
        Statement::Next(next_stmt) => validate_next_statement(next_stmt, declared_variables),
        Statement::Let(let_stmt) => validate_let_statement(let_stmt, declared_variables),
        Statement::Session(session) => validate_session_statement(session),
        Statement::Transaction(transaction) => validate_transaction_statement(transaction),
        Statement::Catalog(catalog_block) => validate_catalog_statement_block(catalog_block),
    }
}

/// Function to validate a SESSION statement, ensuring that the command kind is consistent with the presence of set/reset clauses and that each clause is semantically correct according to the GQL specification. This includes checking for empty values, ensuring that parameters are properly specified, and validating any graph expressions used in the clauses.
pub(super) fn validate_session_statement(session: &SessionStatement) -> Result<(), GqlError> {
    let has_set = !session.set_clauses.is_empty();
    let has_reset = !session.reset_clauses.is_empty();

    match session.kind {
        SessionCommandKind::Set if !has_set || has_reset => {
            return Err(validation_err(
                "Session kind SET requires only set_clauses".to_string(),
                None,
            ));
        }
        SessionCommandKind::Reset if !has_reset => {
            return Err(validation_err(
                "Session kind RESET requires reset_clauses".to_string(),
                None,
            ));
        }
        SessionCommandKind::Close if has_set || has_reset => {
            return Err(validation_err(
                "Session kind CLOSE cannot have set/reset payload".to_string(),
                None,
            ));
        }
        SessionCommandKind::Activity if !(has_set || has_reset) => {
            return Err(validation_err(
                "Session kind ACTIVITY requires at least one clause".to_string(),
                None,
            ));
        }
        _ => {}
    }

    for clause in &session.set_clauses {
        validate_session_set_clause(clause)?;
    }
    for clause in &session.reset_clauses {
        validate_session_reset_clause(clause)?;
    }

    Ok(())
}

/// Function to validate Catalog statements, ensuring that each statement in the block is semantically correct according to the GQL specification. This includes validating schema and graph names, checking for consistency in CREATE GRAPH clauses, and ensuring that raw Catalog statements are not empty.
pub(super) fn validate_catalog_statement_block(block: &CatalogStatementBlock) -> Result<(), GqlError> {
    if block.statements.is_empty() {
        return Err(validation_err("Catalog block cannot be empty".to_string(), None));
    }

    for statement in &block.statements {
        validate_catalog_statement(statement)?;
    }

    Ok(())
}

/// Function to validate a Transaction statement, ensuring that the command kind is consistent with the presence of a transaction mode and that only START transactions can carry a mode. This function enforces the rule that COMMIT, ROLLBACK, and ACTIVITY transaction commands cannot have an associated transaction mode, while START commands can optionally include a mode specification.
pub(super) fn validate_transaction_statement(transaction: &TransactionStatement) -> Result<(), GqlError> {
    match transaction.kind {
        TransactionCommandKind::Start => {}
        TransactionCommandKind::Commit | TransactionCommandKind::Rollback | TransactionCommandKind::Activity
            if transaction.mode.is_some() =>
        {
            return Err(validation_err(
                "Only START transaction can carry a transaction mode".to_string(),
                None,
            ));
        }
        _ => {}
    }

    Ok(())
}

/// Function to validate a Catalog statement, ensuring that the specific type of Catalog statement is semantically correct according to the GQL specification. This includes validating schema and graph names, checking for consistency in CREATE GRAPH clauses, and ensuring that raw Catalog statements are not empty. Each type of Catalog statement has its own set of validation rules that are enforced by this function.
pub(super) fn validate_catalog_statement(statement: &CatalogStatement) -> Result<(), GqlError> {
    match statement {
        CatalogStatement::CreateSchema(stmt) => {
            validate_catalog_schema_name(&stmt.target)?;
        }
        CatalogStatement::DropSchema(stmt) => {
            validate_catalog_schema_name(&stmt.target)?;
        }
        CatalogStatement::CreateGraph(stmt) => {
            if stmt.or_replace && stmt.if_not_exists {
                return Err(validation_err(
                    "CREATE GRAPH cannot contain both OR REPLACE and IF NOT EXISTS".to_string(),
                    Some(&stmt.target.span.clone()),
                ));
            }

            validate_catalog_object_name(&stmt.target)?;
            validate_create_graph_type_clause(&stmt.graph_type_clause)?;
            if let Some(source) = &stmt.graph_source {
                validate_graph_expression_clause(&source.graph_expression)?;
            }
        }
        CatalogStatement::DropGraph(stmt) => {
            validate_catalog_object_name(&stmt.target)?;
        }
        CatalogStatement::CreateGraphType(stmt) => {
            if stmt.or_replace && stmt.if_not_exists {
                return Err(validation_err(
                    "CREATE GRAPH TYPE cannot contain both OR REPLACE and IF NOT EXISTS".to_string(),
                    Some(&stmt.target.span.clone()),
                ));
            }

            validate_catalog_object_name(&stmt.target)?;
            validate_graph_type_source_clause(&stmt.source)?;
        }
        CatalogStatement::DropGraphType(stmt) => {
            validate_catalog_object_name(&stmt.target)?;
        }
        CatalogStatement::Raw(stmt) => {
            if stmt.text.trim().is_empty() {
                return Err(validation_err(
                    "Raw Catalog statement cannot be empty".to_string(),
                    Some(&stmt.span.clone()),
                ));
            }
        }
        CatalogStatement::Call(stmt) => {
            let mut declared_variables = HashSet::new();
            validate_call_clause(&stmt.call_clause, &mut declared_variables)?;
        }
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_catalog_schema_name(name: &CatalogSchemaName) -> Result<(), GqlError> {
    if name.absolute_directory_path.trim().is_empty() {
        return Err(validation_err(
            "Catalog schema absolute directory path cannot be empty".to_string(),
            Some(&name.span.clone()),
        ));
    }

    if name.schema_name.trim().is_empty() {
        return Err(validation_err(
            "Catalog schema name cannot be empty".to_string(),
            Some(&name.span.clone()),
        ));
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_catalog_object_name(name: &CatalogObjectName) -> Result<(), GqlError> {
    if name.name.trim().is_empty() {
        return Err(validation_err(
            "Catalog object name cannot be empty".to_string(),
            Some(&name.span.clone()),
        ));
    }

    if let Some(parent) = &name.parent
        && parent.trim().is_empty()
    {
        return Err(validation_err(
            "Catalog object parent cannot be empty".to_string(),
            Some(&name.span.clone()),
        ));
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_create_graph_type_clause(clause: &CreateGraphTypeClause) -> Result<(), GqlError> {
    match clause {
        CreateGraphTypeClause::Open(_) => Ok(()),
        CreateGraphTypeClause::Of(of_clause) => match of_clause {
            OfGraphTypeClause::LikeGraph { graph_expression } => validate_graph_expression_clause(graph_expression),
            OfGraphTypeClause::TypedReference {
                graph_type_reference, ..
            } => {
                if graph_type_reference.trim().is_empty() {
                    return Err(validation_err(
                        "OF GRAPH TYPE reference cannot be empty".to_string(),
                        None,
                    ));
                }
                Ok(())
            }
            OfGraphTypeClause::TypedNestedSpecification {
                nested_graph_type_specification,
                ..
            } => validate_nested_graph_type_specification(nested_graph_type_specification),
        },
    }
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_graph_type_source_clause(clause: &GraphTypeSourceClause) -> Result<(), GqlError> {
    match clause {
        GraphTypeSourceClause::CopyOf {
            graph_type_reference, ..
        } => {
            if graph_type_reference.trim().is_empty() {
                return Err(validation_err(
                    "COPY OF graph type reference cannot be empty".to_string(),
                    None,
                ));
            }
            Ok(())
        }
        GraphTypeSourceClause::LikeGraph { graph_expression } => validate_graph_expression_clause(graph_expression),
        GraphTypeSourceClause::NestedSpecification {
            nested_graph_type_specification,
            ..
        } => validate_nested_graph_type_specification(nested_graph_type_specification),
    }
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_session_set_clause(clause: &SessionSetClause) -> Result<(), GqlError> {
    match clause {
        SessionSetClause::Schema { schema_reference } => {
            if schema_reference.trim().is_empty() {
                return Err(validation_err(
                    "SESSION SET SCHEMA reference cannot be empty".to_string(),
                    None,
                ));
            }
        }
        SessionSetClause::Graph { graph_expression, .. } => {
            validate_graph_expression_clause(graph_expression)?;
        }
        SessionSetClause::TimeZone { value } => match value {
            SessionTimeZoneValue::Literal(text) if text.trim().is_empty() => {
                return Err(validation_err(
                    "SESSION SET TIME ZONE value cannot be empty".to_string(),
                    None,
                ));
            }
            SessionTimeZoneValue::Literal(_) => {}
        },
        SessionSetClause::Parameter(parameter) => {
            if parameter.parameter_name.parameter_specification.trim().is_empty() {
                return Err(validation_err(
                    "SESSION SET parameter specification cannot be empty".to_string(),
                    Some(&parameter.parameter_name.span.clone()),
                ));
            }

            if let Some(initializer) = &parameter.initializer
                && initializer.trim().is_empty()
            {
                return Err(validation_err(
                    "SESSION SET parameter initializer cannot be empty when present".to_string(),
                    Some(&parameter.parameter_name.span.clone()),
                ));
            }
        }
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_session_reset_clause(clause: &SessionResetClause) -> Result<(), GqlError> {
    match &clause.target {
        SessionResetTarget::ParametersOrCharacteristics { .. }
        | SessionResetTarget::Schema
        | SessionResetTarget::TimeZone => {}
        SessionResetTarget::Graph { .. } => {}
        SessionResetTarget::Parameter {
            parameter_specification,
            ..
        } => {
            if parameter_specification.trim().is_empty() {
                return Err(validation_err(
                    "SESSION RESET parameter specification cannot be empty".to_string(),
                    None,
                ));
            }
        }
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_nested_graph_type_specification(
    specification: &NestedGraphTypeSpecification,
) -> Result<(), GqlError> {
    if specification.element_type_specifications.is_empty() {
        return Err(validation_err(
            "Nested graph type specification cannot be empty".to_string(),
            None,
        ));
    }

    for element in &specification.element_type_specifications {
        match element {
            ElementTypeSpecification::Node(node) => {
                if node.text.trim().is_empty() {
                    return Err(validation_err(
                        "Node type specification cannot be empty".to_string(),
                        None,
                    ));
                }
                if let Some(property_types) = &node.property_types {
                    validate_property_types_specification(property_types)?;
                }
            }
            ElementTypeSpecification::Edge(edge) => {
                if edge.text.trim().is_empty() {
                    return Err(validation_err(
                        "Edge type specification cannot be empty".to_string(),
                        None,
                    ));
                }
                if let Some(property_types) = &edge.property_types {
                    validate_property_types_specification(property_types)?;
                }
            }
        }
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_property_types_specification(specification: &PropertyTypesSpecification) -> Result<(), GqlError> {
    for property in &specification.properties {
        if property.name.trim().is_empty() {
            return Err(validation_err(
                "Property type name cannot be empty".to_string(),
                Some(&property.span.clone()),
            ));
        }

        if property.value_type.trim().is_empty() {
            return Err(validation_err(
                "Property value type cannot be empty".to_string(),
                Some(&property.span.clone()),
            ));
        }
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_graph_expression_clause(graph_expr: &GraphExpressionClause) -> Result<(), GqlError> {
    let trimmed = graph_expr.text.trim();
    if trimmed.is_empty() {
        return Err(validation_err(
            "Graph expression text cannot be empty".to_string(),
            Some(&graph_expr.span.clone()),
        ));
    }

    if matches!(graph_expr.kind, GraphExpressionKind::CurrentGraph) {
        let upper = trimmed.to_ascii_uppercase();
        if !upper.contains("CURRENT_GRAPH") && !upper.contains("CURRENTPROPERTYGRAPH") {
            return Err(validation_err(
                "CurrentGraph kind requires CURRENT_GRAPH/CURRENT PROPERTY GRAPH text".to_string(),
                Some(&graph_expr.span.clone()),
            ));
        }
    }

    let upper = trimmed.to_ascii_uppercase();
    if !matches!(graph_expr.kind, GraphExpressionKind::CurrentGraph)
        && (upper.contains("CURRENT_GRAPH") || upper.contains("CURRENTPROPERTYGRAPH"))
    {
        return Err(validation_err(
            "Graph expression kind is inconsistent with CURRENT_GRAPH semantics".to_string(),
            Some(&graph_expr.span.clone()),
        ));
    }

    match graph_expr.kind {
        GraphExpressionKind::GraphReference => {
            if contains_structural_delimiters(trimmed) {
                return Err(validation_err(
                    "GraphReference cannot contain object-expression delimiters".to_string(),
                    Some(&graph_expr.span.clone()),
                ));
            }
            if upper.starts_with("VARIABLE ") {
                return Err(validation_err(
                    "GraphReference cannot start with VARIABLE keyword".to_string(),
                    Some(&graph_expr.span.clone()),
                ));
            }
        }
        GraphExpressionKind::ObjectNameOrBindingVariable => {
            if contains_structural_delimiters(trimmed) {
                return Err(validation_err(
                    "ObjectNameOrBindingVariable cannot contain object-expression delimiters".to_string(),
                    Some(&graph_expr.span.clone()),
                ));
            }
        }
        GraphExpressionKind::ObjectExpressionPrimary | GraphExpressionKind::CurrentGraph => {}
    }

    Ok(())
}

/// Helper function to check if a string contains any structural delimiters that would indicate it cannot be a simple graph reference. This is used in validating graph expression clauses to ensure that certain kinds of graph expressions do not contain characters that would make them invalid as simple references.
pub(super) fn contains_structural_delimiters(text: &str) -> bool {
    text.contains('(')
        || text.contains(')')
        || text.contains('{')
        || text.contains('}')
        || text.contains('[')
        || text.contains(']')
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_next_statement(
    next_stmt: &NextStatement,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    validate_statement(&next_stmt.source_statement, declared_variables)?;

    if let Some(yield_clause) = &next_stmt.yield_clause {
        validate_yield_clause(yield_clause)?;

        for item in &yield_clause.items {
            let var_to_insert = if let Some(alias) = &item.alias {
                alias.as_str()
            } else {
                item.column_name.as_str()
            };

            let trimmed_var = var_to_insert.trim();
            if !trimmed_var.is_empty() {
                declared_variables.insert(trimmed_var.to_string());
            }
        }
    }

    validate_statement(&next_stmt.target_statement, declared_variables)?;

    if let Some(next) = &next_stmt.next {
        validate_next_statement(next, declared_variables)?;
    }

    Ok(())
}

/// Function to validate a CALL clause within a Catalog statement, ensuring that the procedure name is not empty and that any provided arguments are semantically correct according to the GQL specification. This includes checking that argument expressions are valid and that any variables used in the arguments are properly declared.
pub(super) fn validate_yield_clause(yield_clause: &YieldClause) -> Result<(), GqlError> {
    if yield_clause.items.is_empty() {
        return Err(validation_err(
            "YIELD clause must contain at least one item".to_string(),
            Some(&yield_clause.span.clone()),
        ));
    }

    let mut aliases = HashSet::new();
    let mut output_names = HashSet::new();
    for item in &yield_clause.items {
        if item.column_name.trim().is_empty() {
            return Err(validation_err(
                "YIELD item name cannot be empty".to_string(),
                Some(&yield_clause.span.clone()),
            ));
        }

        let output_name = item.alias.as_deref().unwrap_or(&item.column_name).trim().to_string();

        if !output_names.insert(output_name.clone()) {
            return Err(validation_err(
                format!("Duplicate YIELD output name: {}", output_name),
                Some(&yield_clause.span.clone()),
            ));
        }

        if let Some(alias) = &item.alias {
            let a = alias.trim();
            if a.is_empty() {
                return Err(validation_err(
                    "YIELD alias cannot be empty".to_string(),
                    Some(&yield_clause.span.clone()),
                ));
            }
            if !aliases.insert(a.to_string()) {
                return Err(validation_err(
                    format!("Duplicate YIELD alias: {}", a),
                    Some(&yield_clause.span.clone()),
                ));
            }
        }
    }

    Ok(())
}
