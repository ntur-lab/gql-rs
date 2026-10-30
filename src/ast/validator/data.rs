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

use super::*;

/// Function to validate data modification statements, ensuring that they adhere to the semantic rules of the language, such as correct variable usage, valid patterns, and proper use of expressions. It dispatches to specific validation functions based on the type of data statement being validated.
pub(super) fn validate_data_statement(
    data: &DataStatement,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    match data {
        DataStatement::Insert(insert) => validate_insert_statement(insert, declared_variables),
        DataStatement::Set(set) => validate_set_items(&set.items, declared_variables),
        DataStatement::MatchSet(match_set) => {
            for clause in &match_set.match_clauses {
                validate_match_clause(clause, declared_variables)?;
            }

            if let Some(where_clause) = &match_set.where_clause {
                validate_expression(&where_clause.condition, declared_variables)?;
            }
            validate_set_items(&match_set.items, declared_variables)
        }
        DataStatement::Remove(remove) => {
            validate_remove_items(&remove.remove_clause, declared_variables)
        }
        DataStatement::MatchRemove(match_remove) => {
            validate_match_clause(&match_remove.match_clause, declared_variables)?;
            if let Some(where_clause) = &match_remove.where_clause {
                validate_expression(&where_clause.condition, declared_variables)?;
            }
            validate_remove_items(&match_remove.remove_clause, declared_variables)
        }
        DataStatement::Delete(delete) => {
            for expr in &delete.delete_clause.expressions {
                validate_expression(expr, declared_variables)?;
            }
            Ok(())
        }
        DataStatement::MatchDelete(match_delete) => {
            validate_match_clause(&match_delete.match_clause, declared_variables)?;
            if let Some(where_clause) = &match_delete.where_clause {
                validate_expression(&where_clause.condition, declared_variables)?;
            }
            for expr in &match_delete.delete_clause.expressions {
                validate_expression(expr, declared_variables)?;
            }
            Ok(())
        }
        DataStatement::Call(call) => {
            validate_call_clause(&call.call_clause, declared_variables)?;
            Ok(())
        }
    }
}

/// Function to validate an INSERT statement, ensuring that it contains valid path patterns and that the patterns adhere to the semantic rules of the language, such as starting and ending with a node and containing valid elements.
pub(super) fn validate_insert_statement(
    insert: &InsertStatement,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    if insert.patterns.is_empty() {
        return Err(validation_err(
            "INSERT must contain at least one path pattern".to_string(),
            Some(&insert.span.clone()),
        ));
    }

    for pattern in &insert.patterns {
        validate_insert_pattern(pattern, declared_variables)?;
    }

    Ok(())
}

/// Function to validate a INSERT path pattern, ensuring that it is not empty, starts and ends with a node, and that all elements within the pattern are valid according to the semantic rules of the language.
pub(super) fn validate_insert_pattern(
    pattern: &PathPattern,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    let term_slices = path_pattern_term_slices(pattern);

    if term_slices.is_empty() {
        return Err(validation_err(
            "INSERT path pattern cannot be empty".to_string(),
            Some(&pattern.span.clone()),
        ));
    }

    if let Some(path_variable) = &pattern.variable {
        let trimmed = path_variable.trim();
        if trimmed.is_empty() {
            return Err(validation_err(
                "INSERT path variable cannot be empty".to_string(),
                Some(&pattern.span.clone()),
            ));
        }
        declared_variables.insert(trimmed.to_string());
    }

    validate_path_mode_search_compatibility(pattern)?;
    validate_path_mode_cycle_constraints(pattern)?;

    for term_elements in term_slices {
        if term_elements.is_empty() {
            return Err(validation_err(
                "INSERT path pattern cannot be empty".to_string(),
                Some(&pattern.span.clone()),
            ));
        }

        for (idx, element) in term_elements.iter().enumerate() {
            match element {
                PatternElement::Node(node) => {
                    validate_node(node, declared_variables)?;
                }
                PatternElement::Edge(edge) => {
                    if idx == 0 && term_elements.len() > 1 {
                        return Err(validation_err(
                            "INSERT path pattern cannot start with an edge".to_string(),
                            Some(&pattern.span.clone()),
                        ));
                    }
                    validate_edge(edge, declared_variables, pattern.path_mode.as_ref())?;
                }
            }
        }

        if term_elements.len() > 1 && !matches!(term_elements.last(), Some(PatternElement::Node(_)))
        {
            return Err(validation_err(
                "INSERT path pattern must end with a node".to_string(),
                Some(&pattern.span.clone()),
            ));
        }
    }

    Ok(())
}

/// Function to validate Set operations, ensuring that the left and right branches of the operation produce compatible results in terms of arity, projected aliases, and column types, and that any specified LIMIT or ORDER BY clauses are valid according to the semantic rules of the language.
pub(super) fn validate_set_operation_query(
    set_op: &SetOperation,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    let mut left_scope = declared_variables.clone();
    validate_query(&set_op.left, &mut left_scope)?;

    let mut right_scope = declared_variables.clone();
    validate_query(&set_op.right, &mut right_scope)?;

    if !query_produces_return_rows(&set_op.left) || !query_produces_return_rows(&set_op.right) {
        return Err(validation_err(
            "Set operation branches must terminate with RETURN, not FINISH".to_string(),
            Some(&set_op.span.clone()),
        ));
    }

    let left_arity = infer_query_projection_arity(&set_op.left);
    let right_arity = infer_query_projection_arity(&set_op.right);
    if let (Some(left), Some(right)) = (left_arity, right_arity)
        && left != right
    {
        return Err(validation_err(
            format!(
                "Set operation branches must project the same number of columns (left={}, right={})",
                left, right
            ),
            Some(&set_op.span.clone()),
        ));
    }

    let left_signature = infer_query_projection_signature(&set_op.left);
    let right_signature = infer_query_projection_signature(&set_op.right);
    if let (Some(left_sig), Some(right_sig)) = (left_signature, right_signature)
        && left_sig != right_sig
    {
        return Err(validation_err(
            format!(
                "Set operation branches must expose compatible projected aliases (left={:?}, right={:?})",
                left_sig, right_sig
            ),
            Some(&set_op.span.clone()),
        ));
    }

    let left_types = infer_query_projection_types(&set_op.left);
    let right_types = infer_query_projection_types(&set_op.right);
    if let (Some(left_types), Some(right_types)) = (left_types, right_types) {
        for (idx, (left, right)) in left_types.iter().zip(right_types.iter()).enumerate() {
            let Some(left) = left else {
                continue;
            };
            let Some(right) = right else {
                continue;
            };

            if !are_set_operation_column_types_compatible(left, right) {
                return Err(validation_err(
                    format!(
                        "Set operation column {} has incompatible types: {} vs {}",
                        idx + 1,
                        type_name(left),
                        type_name(right)
                    ),
                    Some(&set_op.span.clone()),
                ));
            }
        }
    }

    if let Some(limit_clause) = &set_op.limit_clause {
        validate_limit_clause(limit_clause)?;
    }

    if let Some(order_clause) = &set_op.order_clause {
        let merged_scope: HashSet<String> = left_scope.union(&right_scope).cloned().collect();
        validate_order_clause(order_clause, &merged_scope, None)?;
    }

    Ok(())
}

/// Function to validate projection in procedure calls, ensuring that the projection items are valid expressions, that any aliases are properly declared and do not conflict with existing variables, and that the overall structure of the projection adheres to the semantic rules of the language.
pub(super) fn query_produces_return_rows(query: &Query) -> bool {
    match query {
        Query::Basic(basic) => {
            matches!(
                basic.result_statement,
                PrimitiveResultStatement::Return { .. }
            )
        }
        Query::SetOperation(set_op) => {
            query_produces_return_rows(&set_op.left) && query_produces_return_rows(&set_op.right)
        }
    }
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_query_projection_arity(query: &Query) -> Option<usize> {
    match query {
        Query::Basic(basic) => infer_result_projection_arity(&basic.result_statement),
        Query::SetOperation(set_op) => {
            let left = infer_query_projection_arity(&set_op.left);
            let right = infer_query_projection_arity(&set_op.right);
            match (left, right) {
                (Some(l), Some(r)) if l == r => Some(l),
                _ => None,
            }
        }
    }
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_result_projection_arity(result: &PrimitiveResultStatement) -> Option<usize> {
    let PrimitiveResultStatement::Return { return_clause, .. } = result else {
        return None;
    };

    if return_clause.items.iter().any(|item| {
        matches!(
            &item.expression,
            Expression::Literal(Literal::String(s)) if s.trim() == "*"
        )
    }) {
        return None;
    }

    Some(return_clause.items.len())
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_query_projection_signature(query: &Query) -> Option<Vec<String>> {
    match query {
        Query::Basic(basic) => infer_result_projection_signature(&basic.result_statement),
        Query::SetOperation(set_op) => {
            let left = infer_query_projection_signature(&set_op.left);
            let right = infer_query_projection_signature(&set_op.right);
            match (left, right) {
                (Some(l), Some(r)) if l == r => Some(l),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_result_projection_signature(
    result: &PrimitiveResultStatement,
) -> Option<Vec<String>> {
    let PrimitiveResultStatement::Return { return_clause, .. } = result else {
        return None;
    };

    let mut signature = Vec::with_capacity(return_clause.items.len());
    for item in &return_clause.items {
        let Some(alias) = &item.alias else {
            return None;
        };

        let trimmed = alias.trim();
        if trimmed.is_empty() {
            return None;
        }

        signature.push(trimmed.to_string());
    }

    Some(signature)
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_query_projection_types(query: &Query) -> Option<Vec<Option<TypeSpec>>> {
    match query {
        Query::Basic(basic) => infer_result_projection_types(&basic.result_statement),
        Query::SetOperation(set_op) => {
            let left = infer_query_projection_types(&set_op.left);
            let right = infer_query_projection_types(&set_op.right);

            match (left, right) {
                (Some(l), Some(r)) if l.len() == r.len() => Some(
                    l.into_iter()
                        .zip(r)
                        .map(|(lt, rt)| match (lt, rt) {
                            (Some(l), Some(r))
                                if are_set_operation_column_types_compatible(&l, &r) =>
                            {
                                Some(l)
                            }
                            (Some(l), None) => Some(l),
                            (None, Some(r)) => Some(r),
                            _ => None,
                        })
                        .collect(),
                ),
                (Some(l), None) => Some(l),
                (None, Some(r)) => Some(r),
                _ => None,
            }
        }
    }
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn infer_result_projection_types(
    result: &PrimitiveResultStatement,
) -> Option<Vec<Option<TypeSpec>>> {
    let PrimitiveResultStatement::Return { return_clause, .. } = result else {
        return None;
    };

    let inferred = HashMap::new();
    let types = return_clause
        .items
        .iter()
        .map(|item| infer_expression_type(&item.expression, &inferred, None))
        .collect();
    Some(types)
}

/// Helper function to determine if two column types are compatible for set operations, following the rules of the language which allow for certain implicit conversions and compatibility between types, such as allowing ANY to be compatible with any type, and allowing compatible scalar types to be considered compatible for set operations.
pub(super) fn are_set_operation_column_types_compatible(left: &TypeSpec, right: &TypeSpec) -> bool {
    if matches!(left, TypeSpec::Any) || matches!(right, TypeSpec::Any) {
        return true;
    }

    if are_comparison_types_compatible(left, right) {
        return true;
    }

    match (left, right) {
        (
            TypeSpec::List {
                element_type: l, ..
            },
            TypeSpec::List {
                element_type: r, ..
            },
        ) => are_set_operation_column_types_compatible(l, r),
        (TypeSpec::Union { members }, other) | (other, TypeSpec::Union { members }) => members
            .iter()
            .all(|m| are_set_operation_column_types_compatible(m, other)),
        _ => false,
    }
}

/// Function to validate REMOVE items, ensuring that any variables referenced in the REMOVE clause are properly declared and that the structure of the REMOVE items adheres to the semantic rules of the language.
pub(super) fn validate_remove_items(
    items: &[RemoveItem],
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    for item in items {
        validate_remove_item(item, declared_variables)?;
    }
    Ok(())
}

/// Function to validate a single REMOVE item, ensuring that any variables referenced in the REMOVE clause are properly declared and that the structure of the REMOVE item adheres to the semantic rules of the language.
pub(super) fn validate_remove_item(
    item: &RemoveItem,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    match item {
        RemoveItem::Property { property, span } => {
            if !declared_variables.contains(&property.object)
                && !property.object.starts_with('$')
                && !declared_variables.is_empty()
            {
                return Err(validation_err(
                    format!(
                        "Variable {} not declared in REMOVE property",
                        property.object
                    ),
                    Some(&span.clone()),
                ));
            }
            Ok(())
        }
        RemoveItem::Label {
            variable,
            labels,
            span,
        } => {
            if !declared_variables.contains(variable)
                && !variable.starts_with('$')
                && !declared_variables.is_empty()
            {
                return Err(validation_err(
                    format!("Variable {} not declared in REMOVE label", variable),
                    Some(&span.clone()),
                ));
            }
            validate_expression(labels, declared_variables)
        }
        RemoveItem::Variable {
            name: variable,
            span,
        } => {
            if !declared_variables.contains(variable)
                && !variable.starts_with('$')
                && !declared_variables.is_empty()
            {
                return Err(validation_err(
                    format!("Variable {} not declared in REMOVE variable", variable),
                    Some(&span.clone()),
                ));
            }
            Ok(())
        }
    }
}

/// Function to validate SET items, ensuring that any variables referenced in the SET clause are properly declared, that the expressions being assigned are valid, and that the structure of the SET items adheres to the semantic rules of the language.
pub(super) fn validate_set_items(
    items: &[SetItem],
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    for item in items {
        validate_set_item(item, declared_variables)?;
    }
    Ok(())
}

/// Function to validate a single SET item, ensuring that any variables referenced in the SET clause are properly declared, that the expressions being assigned are valid, and that the structure of the SET item adheres to the semantic rules of the language. This includes validating property assignments, variable assignments, and label assignments, and ensuring that the assigned expressions are compatible with the expected types for their targets.
pub(super) fn validate_set_item(
    item: &SetItem,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    match item {
        SetItem::Property {
            property,
            value,
            span,
        } => {
            if !declared_variables.contains(&property.object) && !property.object.starts_with('$') {
                return Err(validation_err(
                    format!("Variable {} not declared in SET property", property.object),
                    Some(&span.clone()),
                ));
            }

            validate_expression(value, declared_variables)?;

            let inferred = HashMap::new();
            if let Some(value_type) =
                infer_expression_type(value, &inferred, Some(declared_variables))
                && matches!(value_type, TypeSpec::BindingTable | TypeSpec::Graph { .. })
            {
                return Err(validation_err(
                    "SET property cannot assign binding-table/graph values".to_string(),
                    Some(&span.clone()),
                ));
            }

            Ok(())
        }
        SetItem::Label {
            variable,
            labels,
            span,
        } => {
            if !declared_variables.contains(variable) && !variable.starts_with('$') {
                return Err(validation_err(
                    format!("Variable {} not declared in SET label", variable),
                    Some(&span.clone()),
                ));
            }

            validate_expression(labels, declared_variables)?;

            let inferred = HashMap::new();
            if let Some(label_type) =
                infer_expression_type(labels, &inferred, Some(declared_variables))
                && !is_string_like_type(&label_type)
                && !matches!(label_type, TypeSpec::Any | TypeSpec::List { .. })
            {
                return Err(validation_err(
                    "SET label expects string-like or list-like expression".to_string(),
                    Some(&span.clone()),
                ));
            }

            Ok(())
        }
        SetItem::Variable {
            variable,
            value,
            span,
        } => {
            if !declared_variables.contains(variable) && !variable.starts_with('$') {
                return Err(validation_err(
                    format!("Variable {} not declared in SET variable", variable),
                    Some(&span.clone()),
                ));
            }

            validate_expression(value, declared_variables)?;

            let inferred = HashMap::new();
            if let Some(value_type) =
                infer_expression_type(value, &inferred, Some(declared_variables))
                && matches!(value_type, TypeSpec::BindingTable)
            {
                return Err(validation_err(
                    "SET variable cannot assign binding-table values".to_string(),
                    Some(&span.clone()),
                ));
            }

            Ok(())
        }
    }
}

/// Helper functions to infer the arity, projected aliases, and column types of a query's result set, which are used for validating set operations and procedure call projections. These functions analyze the structure of the query and its result statements to determine the characteristics of the projected output.
pub(super) fn check_no_aggregates_in_where(expression: &Expression) -> Result<(), GqlError> {
    match expression {
        Expression::FunctionCall(func) => {
            let name = func.name.to_uppercase();
            if AGGREGATE_FUNCTIONS.contains(&name.as_str()) {
                return Err(validation_err(
                    format!(
                        "Aggregate function '{}' is not allowed in a WHERE clause.",
                        func.name
                    ),
                    Some(&func.span.clone()),
                ));
            }
            for arg in &func.arguments {
                check_no_aggregates_in_where(arg)?;
            }
        }
        Expression::Binary(binary) => {
            check_no_aggregates_in_where(&binary.left)?;
            check_no_aggregates_in_where(&binary.right)?;
        }
        Expression::Unary(unary) => {
            check_no_aggregates_in_where(&unary.expression)?;
        }
        Expression::IsPredicate(pred) => {
            check_no_aggregates_in_where(&pred.expression)?;
        }
        _ => {}
    }
    Ok(())
}
