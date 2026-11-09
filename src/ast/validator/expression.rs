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

use super::*;

/// Function to validate an expression, ensuring that all variables are declared and that the expression adheres to the semantic rules of the language. This function serves as a general entry point for validating expressions in various contexts, such as WHERE clauses, RETURN statements, and other places where expressions are used.
pub(super) fn validate_expression(
    expression: &Expression,
    declared_variables: &HashSet<String>,
) -> Result<(), GqlError> {
    let inferred_types = HashMap::new();
    validate_expression_with_types(expression, declared_variables, &inferred_types)
}

/// Function to validate an expression with type information, ensuring that all variables are declared, that the expression adheres to the semantic rules of the language, and that any type constraints are satisfied. This function is used in contexts where type inference has been performed and type information is available for variables, allowing for more precise validation of expressions.
pub(super) fn validate_expression_with_types(
    expression: &Expression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    match expression {
        Expression::Binary(binary) => {
            validate_expression_with_types(&binary.left, declared_variables, inferred_types)?;
            validate_expression_with_types(&binary.right, declared_variables, inferred_types)?;
            validate_binary_operator_types(binary, declared_variables, inferred_types)?;
        }
        Expression::Unary(unary) => {
            validate_expression_with_types(&unary.expression, declared_variables, inferred_types)?;
            validate_unary_operator_type(unary, declared_variables, inferred_types)?;
        }
        Expression::Literal(_) => {}
        Expression::Variable { name: var, span } => {
            let trimmed = var.trim();
            if trimmed.is_empty() {
                return Err(validation_err(
                    "Variable name cannot be empty".to_string(),
                    Some(&span.clone()),
                ));
            }

            if trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok() {
                return Ok(());
            }

            if !declared_variables.contains(trimmed) && !inferred_types.contains_key(trimmed) {
                return Err(validation_err(
                    format!("Unknown variable '{}'", trimmed),
                    Some(&span.clone()),
                ));
            }
        }
        Expression::PropertyAccess(property_access) => {
            if property_access.object.trim().is_empty() {
                return Err(validation_err(
                    "Property access object cannot be empty".to_string(),
                    Some(&property_access.span.clone()),
                ));
            } else if !declared_variables.contains(property_access.object.as_str()) {
                return Err(validation_err(
                    format!(
                        "Unknown variable '{}' in property access",
                        property_access.object
                    ),
                    Some(&property_access.span.clone()),
                ));
            }

            if property_access.property.trim().is_empty() {
                return Err(validation_err(
                    "Property access field cannot be empty".to_string(),
                    Some(&property_access.span.clone()),
                ));
            }
        }
        Expression::FunctionCall(function_call) => {
            let name = function_call.name.to_uppercase();

            if function_call.name.trim().is_empty() {
                return Err(validation_err(
                    "Function name cannot be empty".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }

            if name == "ELEMENT_ID" {
                if function_call.arguments.len() != 1 {
                    return Err(validation_err(
                        "ELEMENT_ID requires exactly one argument".to_string(),
                        Some(&function_call.span.clone()),
                    ));
                } else if let Some(Expression::Variable { name: var, span }) =
                    function_call.arguments.first()
                    && !declared_variables.contains(var.as_str())
                {
                    return Err(validation_err(
                        format!("ELEMENT_ID argument '{}' is not a declared variable", var),
                        Some(&span.clone()),
                    ));
                }
            }

            if name == "LEFT" || name == "RIGHT" {
                if function_call.arguments.len() != 2 {
                    return Err(validation_err(
                        format!("{} requires exactly two arguments", name),
                        Some(&function_call.span.clone()),
                    ));
                }

                let len_arg = &function_call.arguments[1];
                let ok_len = match len_arg {
                    Expression::Literal(Literal::Integer(v)) => *v >= 0,
                    Expression::Variable { name: v, span: _ } => {
                        v.parse::<i64>().map(|n| n >= 0).unwrap_or(false)
                    }
                    _ => false,
                };

                if !ok_len {
                    return Err(validation_err(
                        format!("{} second argument must be a non-negative integer", name),
                        Some(&function_call.span.clone()),
                    ));
                }
            }

            validate_function_call_signature(function_call, declared_variables, inferred_types)?;

            for arg in &function_call.arguments {
                validate_expression_with_types(arg, declared_variables, inferred_types)?;
            }
        }
        Expression::IsPredicate(predicate) => {
            validate_expression_with_types(
                &predicate.expression,
                declared_variables,
                inferred_types,
            )?;
            validate_is_predicate_types(predicate, declared_variables, inferred_types)?;
        }
        Expression::Array(array) => {
            for item in &array.elements {
                validate_expression_with_types(item, declared_variables, inferred_types)?;
            }

            validate_array_element_consistency(array, declared_variables, inferred_types)?;
        }
        Expression::Case(case_expression) => {
            validate_case_expression(case_expression, declared_variables)?;
        }
        Expression::Cast(cast_expression) => {
            validate_expression_with_types(
                &cast_expression.expression,
                declared_variables,
                inferred_types,
            )?;
            validate_cast_target(cast_expression, declared_variables, inferred_types)?;
        }
        Expression::Subquery(subquery) => {
            let mut subquery_variables = declared_variables.clone();
            validate_query(&subquery.query, &mut subquery_variables)?;

            validate_scalar_subquery_projection(subquery)?;
        }
        Expression::Map(property_items) => {
            for property in property_items {
                if property.key.trim().is_empty() {
                    return Err(validation_err(
                        "Map property key cannot be empty".to_string(),
                        Some(&property.span.clone()),
                    ));
                }
                validate_expression_with_types(
                    &property.value,
                    declared_variables,
                    inferred_types,
                )?;
            }
        }
        Expression::InlineLet(expr) => {
            let mut local_scope = declared_variables.clone();
            let mut local_types = inferred_types.clone();

            for def in &expr.variable_definitions {
                validate_expression_with_types(&def.expression, &local_scope, &local_types)?;

                let trimmed_name = def.variable_name.trim().to_string();
                if let Some(inferred) =
                    infer_expression_type(&def.expression, &local_types, Some(&local_scope))
                {
                    local_types.insert(trimmed_name.clone(), inferred);
                }

                local_scope.insert(trimmed_name);
            }
            validate_expression_with_types(
                &expr.evaluation_expression,
                &local_scope,
                &local_types,
            )?;
        }
        Expression::PathConstructor(elements) => {
            if elements.is_empty() {
                return Err(validation_err(
                    "Path constructor cannot be empty".to_string(),
                    None,
                ));
            }

            if elements.len() % 2 == 0 {
                return Err(validation_err(
                    "Path constructor must contain an odd number of elements (node, edge, node, ...)"
                        .to_string(),
                    None,
                ));
            }

            for expr in elements {
                if let Expression::Variable { name, span } = expr {
                    let trimmed = name.trim();
                    if !declared_variables.is_empty() && !declared_variables.contains(trimmed) {
                        return Err(validation_err(
                            format!(
                                "Path constructor references undeclared variable '{}'",
                                trimmed
                            ),
                            Some(&span.clone()),
                        ));
                    }
                } else {
                    return Err(validation_err(
                        "Path constructor elements must be variable references".to_string(),
                        None,
                    ));
                }

                validate_expression_with_types(expr, declared_variables, inferred_types)?;
            }
        }
        Expression::WhenOperand(operand) => match &operand.kind {
            WhenOperandKind::Comparison { right, .. } => {
                validate_expression_with_types(right, declared_variables, inferred_types)?;
            }
            WhenOperandKind::ValueTypePredicate { target_type, .. } => {
                if target_type.is_none() {
                    return Err(validation_err(
                        "ValueType WHEN operand must preserve target type".to_string(),
                        Some(&operand.span.clone()),
                    ));
                }
            }
            WhenOperandKind::NormalizedPredicate { normal_form, .. } => {
                if let Some(form) = normal_form {
                    let trimmed = form.trim();
                    if trimmed.is_empty() {
                        return Err(validation_err(
                            "Normalized WHEN operand cannot use empty normal form".to_string(),
                            Some(&operand.span.clone()),
                        ));
                    }

                    let upper = trimmed.to_ascii_uppercase();
                    let valid = matches!(upper.as_str(), "NFC" | "NFD" | "NFKC" | "NFKD");
                    if !valid {
                        return Err(validation_err(
                            format!(
                                "Normalized WHEN operand uses unsupported normal form '{}'",
                                trimmed
                            ),
                            Some(&operand.span.clone()),
                        ));
                    }
                }
            }
            WhenOperandKind::LabeledPredicate {
                label_expression, ..
            } => {
                let valid_label = label_expression
                    .as_ref()
                    .map(|x| !x.trim().is_empty())
                    .unwrap_or(false);

                if !valid_label {
                    return Err(validation_err(
                        "Labeled WHEN operand must preserve label expression".to_string(),
                        Some(&operand.span.clone()),
                    ));
                }
            }
            WhenOperandKind::SourcePredicate { edge_reference, .. }
            | WhenOperandKind::DestinationPredicate { edge_reference, .. } => {
                let edge_name = edge_reference
                    .as_ref()
                    .map(|x| x.trim().to_string())
                    .filter(|x| !x.is_empty());

                let Some(edge_name) = edge_name else {
                    return Err(validation_err(
                        "Source/Destination WHEN operand must preserve edge reference".to_string(),
                        Some(&operand.span.clone()),
                    ));
                };

                if !declared_variables.is_empty()
                    && !declared_variables.contains(edge_name.as_str())
                {
                    return Err(validation_err(
                        format!(
                            "Source/Destination WHEN operand references undeclared edge variable '{}'",
                            edge_name
                        ),
                        Some(&operand.span.clone()),
                    ));
                }
            }
            WhenOperandKind::NullPredicate { .. } | WhenOperandKind::DirectedPredicate { .. } => {}
        },
    }
    Ok(())
}

/// Function to validate a unary operator expression, ensuring that the operand is of a compatible type for the operator. This function checks the inferred type of the operand and validates it against the expected types for the specific unary operator, such as ensuring that NOT is applied to boolean or string-like types, and that PLUS/MINUS are applied to numeric types.
pub(super) fn validate_unary_operator_type(
    unary: &UnaryExpression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let Some(operand_type) =
        infer_expression_type(&unary.expression, inferred_types, Some(declared_variables))
    else {
        return Ok(());
    };

    match unary.operator {
        Operator::Not => {
            if !is_boolean_type(&operand_type)
                && !is_string_like_type(&operand_type)
                && !matches!(operand_type, TypeSpec::Any | TypeSpec::BindingTable)
            {
                return Err(validation_err(
                    format!(
                        "Unary operator NOT requires BOOLEAN operand, got {}",
                        type_name(&operand_type)
                    ),
                    Some(&unary.span.clone()),
                ));
            }
        }
        Operator::Plus | Operator::Minus
            if !is_numeric_type(&operand_type) && !matches!(operand_type, TypeSpec::Any) =>
        {
            return Err(validation_err(
                format!(
                    "Unary operator '{:?}' requires numeric operand, got {}",
                    unary.operator,
                    type_name(&operand_type)
                ),
                Some(&unary.span.clone()),
            ));
        }
        _ => {}
    }

    Ok(())
}

/// Function to validate a binary operator expression, ensuring that the left and right operands are of compatible types for the operator. This function checks the inferred types of both operands and validates them against the expected types for the specific binary operator, such as ensuring that arithmetic operators are applied to numeric types, comparison operators are applied to comparable types, and logical operators are applied to boolean or string-like types.
pub(super) fn validate_is_predicate_types(
    predicate: &IsPredicateExpression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let Some(expr_type) = infer_expression_type(
        &predicate.expression,
        inferred_types,
        Some(declared_variables),
    ) else {
        return Ok(());
    };

    match &predicate.predicate {
        IsPredicateKind::True | IsPredicateKind::False => {
            if !is_boolean_type(&expr_type) && !matches!(expr_type, TypeSpec::Any) {
                return Err(validation_err(
                    format!(
                        "IS TRUE/FALSE requires BOOLEAN expression, got {}",
                        type_name(&expr_type)
                    ),
                    Some(&predicate.span.clone()),
                ));
            }
        }
        IsPredicateKind::Unknown | IsPredicateKind::Null => {}
        IsPredicateKind::Typed(target_type) => {
            if !is_cast_compatible(&expr_type, target_type) {
                return Err(validation_err(
                    format!(
                        "IS TYPED check is incompatible: {} cannot satisfy {}",
                        type_name(&expr_type),
                        type_name(target_type)
                    ),
                    Some(&predicate.span.clone()),
                ));
            }
        }
    }

    Ok(())
}

/// Function to validate a function call signature, ensuring that the arguments provided are of the correct types for the function being called. This function checks the inferred types of the arguments and validates them against the expected types for the specific function, such as ensuring that string functions are applied to string-like types and numeric functions are applied to numeric types.
pub(super) fn validate_function_call_signature(
    function_call: &FunctionCall,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let name = function_call.name.to_ascii_uppercase();

    if is_aggregate_function_name(name.as_str())
        && function_call
            .arguments
            .iter()
            .any(contains_aggregate_function)
    {
        return Err(validation_err(
            format!(
                "Aggregate function '{}' cannot contain nested aggregate expressions",
                function_call.name
            ),
            Some(&function_call.span.clone()),
        ));
    }

    let arg_type = |idx: usize| {
        function_call
            .arguments
            .get(idx)
            .and_then(|arg| infer_expression_type(arg, inferred_types, Some(declared_variables)))
    };

    match name.as_str() {
        "LOWER" | "UPPER" => {
            if function_call.arguments.len() != 1 {
                return Err(validation_err(
                    format!("{} requires exactly one argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
            if let Some(t) = arg_type(0)
                && !is_string_like_type(&t)
                && !matches!(t, TypeSpec::Any)
            {
                return Err(validation_err(
                    format!("{} requires a string-like argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "TRIM" => {
            if !(1..=3).contains(&function_call.arguments.len()) {
                return Err(validation_err(
                    "TRIM requires one, two, or three arguments".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "SUM" | "AVG" => {
            if function_call.arguments.len() != 1 {
                return Err(validation_err(
                    format!("{} requires exactly one argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
            if let Some(t) = arg_type(0)
                && !is_numeric_type(&t)
                && !matches!(t, TypeSpec::Any)
            {
                return Err(validation_err(
                    format!("{} requires a numeric argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "SIZE" | "CARDINALITY" => {
            if function_call.arguments.len() != 1 {
                return Err(validation_err(
                    format!("{} requires exactly one argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
            if let Some(t) = arg_type(0) {
                let ok = is_string_like_type(&t)
                    || matches!(
                        t,
                        TypeSpec::List { .. }
                            | TypeSpec::Path
                            | TypeSpec::BindingTable
                            | TypeSpec::Any
                    );
                if !ok {
                    return Err(validation_err(
                        format!("{} requires list/string/path argument", name),
                        Some(&function_call.span.clone()),
                    ));
                }
            }
        }
        "COUNT" => {
            if function_call.arguments.len() != 1 {
                return Err(validation_err(
                    "COUNT requires exactly one argument".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "COALESCE" => {
            if function_call.arguments.is_empty() {
                return Err(validation_err(
                    "COALESCE requires at least one argument".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }

            let mut acc: Option<TypeSpec> = None;
            for arg in &function_call.arguments {
                let Some(t) = infer_expression_type(arg, inferred_types, Some(declared_variables))
                else {
                    continue;
                };

                if let Some(prev) = &acc
                    && !are_comparison_types_compatible(prev, &t)
                    && !matches!(prev, TypeSpec::Any)
                    && !matches!(t, TypeSpec::Any)
                {
                    return Err(validation_err(
                        "COALESCE arguments must be mutually compatible".to_string(),
                        Some(&function_call.span.clone()),
                    ));
                }

                acc = Some(t);
            }
        }
        "DATE" => {
            if function_call.arguments.len() > 1 {
                return Err(validation_err(
                    "DATE accepts at most one argument".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "CURRENT_DATE" | "CURRENT_TIME" | "CURRENT_TIMESTAMP" => {
            if !function_call.arguments.is_empty() {
                return Err(validation_err(
                    format!("{} does not accept arguments", name),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "LOCAL_TIME" | "LOCAL_DATETIME" | "ZONED_TIME" | "ZONED_DATETIME" => {
            if function_call.arguments.len() > 1 {
                return Err(validation_err(
                    format!("{} accepts at most one argument", name),
                    Some(&function_call.span.clone()),
                ));
            }
        }
        "DURATION_BETWEEN" => {
            if function_call.arguments.len() != 2 {
                return Err(validation_err(
                    "DURATION_BETWEEN requires exactly two arguments".to_string(),
                    Some(&function_call.span.clone()),
                ));
            }

            if let (Some(left), Some(right)) = (arg_type(0), arg_type(1)) {
                let temporal_ok = is_temporal_like_type(&left) && is_temporal_like_type(&right);
                if !temporal_ok && !matches!(left, TypeSpec::Any) && !matches!(right, TypeSpec::Any)
                {
                    return Err(validation_err(
                        "DURATION_BETWEEN requires temporal-compatible arguments".to_string(),
                        Some(&function_call.span.clone()),
                    ));
                }
            }
        }
        _ => {}
    }

    Ok(())
}

/// Function to validate a scalar subquery projection, ensuring that the subquery returns exactly one column and is not an EXISTS subquery. This function checks the structure of the subquery's result statement to determine its arity and validates it against the requirements for scalar subqueries, which must return a single value.
pub(super) fn validate_scalar_subquery_projection(
    subquery: &SubqueryExpression,
) -> Result<(), GqlError> {
    if subquery.is_exists {
        return Ok(());
    }

    let Some(arity) = infer_query_projection_arity(&subquery.query) else {
        return Err(validation_err(
            "Scalar subquery must terminate with RETURN".to_string(),
            Some(&subquery.span.clone()),
        ));
    };

    if arity != 1 {
        return Err(validation_err(
            format!(
                "Scalar subquery must return exactly one column, found {}",
                arity
            ),
            Some(&subquery.span.clone()),
        ));
    }

    Ok(())
}

/// Helper functions to check for aggregate functions within expressions, validate operator types, and determine type compatibility. These functions are used to enforce semantic rules related to aggregates, operator usage, and type constraints within expressions in various contexts throughout the query validation process.
pub(super) fn is_aggregate_function_name(name: &str) -> bool {
    AGGREGATE_FUNCTIONS.contains(&name)
}

/// Helper function to recursively check if an expression contains any aggregate function calls, which is used to enforce the rule that aggregate functions cannot be nested within other aggregate functions. This function traverses the expression tree and checks for any occurrences of aggregate function calls, returning true if any are found.
pub(super) fn contains_aggregate_function(expression: &Expression) -> bool {
    match expression {
        Expression::FunctionCall(func) => {
            is_aggregate_function_name(func.name.to_ascii_uppercase().as_str())
                || func.arguments.iter().any(contains_aggregate_function)
        }
        Expression::Binary(binary) => {
            contains_aggregate_function(&binary.left) || contains_aggregate_function(&binary.right)
        }
        Expression::Unary(unary) => contains_aggregate_function(&unary.expression),
        Expression::IsPredicate(pred) => contains_aggregate_function(&pred.expression),
        Expression::Array(arr) => arr.elements.iter().any(contains_aggregate_function),
        Expression::Map(items) => items
            .iter()
            .any(|item| contains_aggregate_function(&item.value)),
        Expression::Case(case_expr) => match &case_expr.case_type {
            CaseType::Simple(simple) => {
                contains_aggregate_function(&simple.test_expression)
                    || simple.when_branches.iter().any(|b| {
                        b.when_values.iter().any(contains_aggregate_function)
                            || contains_aggregate_function(&b.then_expression)
                    })
                    || simple
                        .else_expression
                        .as_ref()
                        .map(|e| contains_aggregate_function(e))
                        .unwrap_or(false)
            }
            CaseType::Searched(searched) => {
                searched.when_branches.iter().any(|b| {
                    contains_aggregate_function(&b.condition)
                        || contains_aggregate_function(&b.then_expression)
                }) || searched
                    .else_expression
                    .as_ref()
                    .map(|e| contains_aggregate_function(e))
                    .unwrap_or(false)
            }
        },
        Expression::Cast(cast_expr) => contains_aggregate_function(&cast_expr.expression),
        Expression::InlineLet(expr) => {
            expr.variable_definitions
                .iter()
                .any(|d| contains_aggregate_function(&d.expression))
                || contains_aggregate_function(&expr.evaluation_expression)
        }
        Expression::WhenOperand(operand) => match &operand.kind {
            WhenOperandKind::Comparison { right, .. } => contains_aggregate_function(right),
            _ => false,
        },
        Expression::Subquery(_) => false,
        _ => false,
    }
}

/// Helper function to determine if two types are compatible for comparison operations, such as those used in comparison operators (e.g., <, >, =). This function checks if the types are the same, if either is ANY, or if they belong to compatible categories (e.g., both numeric, both string-like, or both temporal-like), allowing for flexible comparisons while enforcing type safety where possible.
pub(super) fn are_comparison_types_compatible(left: &TypeSpec, right: &TypeSpec) -> bool {
    if left == right {
        return true;
    }

    if matches!(left, TypeSpec::Any) || matches!(right, TypeSpec::Any) {
        return true;
    }

    if is_numeric_type(left) && is_numeric_type(right) {
        return true;
    }

    if is_string_like_type(left) && is_string_like_type(right) {
        return true;
    }

    if is_temporal_like_type(left) && is_temporal_like_type(right) {
        return true;
    }

    matches!(
        (left, right),
        (TypeSpec::Boolean, TypeSpec::Boolean)
            | (TypeSpec::Path, TypeSpec::Path)
            | (TypeSpec::Record, TypeSpec::Record)
    )
}

/// Function to validate a binary operator expression, ensuring that the left and right operands are of compatible types for the operator. This function checks the inferred types of both operands and validates them against the expected types for the specific binary operator, such as ensuring that arithmetic operators are applied to numeric types, comparison operators are applied to comparable types, and logical operators are applied to boolean or string-like types.
pub(super) fn validate_binary_operator_types(
    binary: &BinaryExpression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let left_type = infer_expression_type(&binary.left, inferred_types, Some(declared_variables));
    let right_type = infer_expression_type(&binary.right, inferred_types, Some(declared_variables));

    let Some(left_type) = left_type else {
        return Ok(());
    };
    let Some(right_type) = right_type else {
        return Ok(());
    };

    match binary.operator {
        Operator::And | Operator::Or | Operator::Xor => {
            if is_string_like_type(&left_type) || is_string_like_type(&right_type) {
                return Ok(());
            }

            let left_known = !matches!(left_type, TypeSpec::Any);
            let right_known = !matches!(right_type, TypeSpec::Any);
            if left_known
                && right_known
                && (!is_boolean_type(&left_type) || !is_boolean_type(&right_type))
            {
                return Err(validation_err(
                    format!(
                        "Logical operator '{:?}' requires BOOLEAN operands, got {} and {}",
                        binary.operator,
                        type_name(&left_type),
                        type_name(&right_type)
                    ),
                    Some(&binary.span.clone()),
                ));
            }
        }
        Operator::Plus
        | Operator::Minus
        | Operator::Star
        | Operator::Slash
        | Operator::Percent
        | Operator::Caret => {
            let left_numeric = is_numeric_type(&left_type);
            let right_numeric = is_numeric_type(&right_type);
            let left_duration = is_duration_type(&left_type);
            let right_duration = is_duration_type(&right_type);

            if matches!(binary.operator, Operator::Plus)
                && is_string_like_type(&left_type)
                && is_string_like_type(&right_type)
            {
                return Err(validation_err(
                    "Operator '+' cannot concatenate strings; use '||' instead".to_string(),
                    Some(&binary.span.clone()),
                ));
            }

            match binary.operator {
                Operator::Plus | Operator::Minus => {
                    let unknown =
                        matches!(left_type, TypeSpec::Any) || matches!(right_type, TypeSpec::Any);
                    if !(left_numeric && right_numeric)
                        && !(left_duration && right_duration)
                        && !unknown
                    {
                        return Err(validation_err(
                            format!(
                                "Operator '{:?}' requires compatible numeric or duration operands, got {} and {}",
                                binary.operator,
                                type_name(&left_type),
                                type_name(&right_type)
                            ),
                            Some(&binary.span.clone()),
                        ));
                    }
                }
                _ => {
                    let unknown =
                        matches!(left_type, TypeSpec::Any) || matches!(right_type, TypeSpec::Any);
                    if !(left_numeric && right_numeric) && !unknown {
                        return Err(validation_err(
                            format!(
                                "Operator '{:?}' requires numeric operands, got {} and {}",
                                binary.operator,
                                type_name(&left_type),
                                type_name(&right_type)
                            ),
                            Some(&binary.span.clone()),
                        ));
                    }
                }
            }
        }
        Operator::Concat => {
            let unknown = matches!(left_type, TypeSpec::Any) || matches!(right_type, TypeSpec::Any);
            if (!is_concat_compatible_type(&left_type) || !is_concat_compatible_type(&right_type))
                && !unknown
            {
                return Err(validation_err(
                    format!(
                        "Operator '{:?}' requires string-like operands, got {} and {}",
                        binary.operator,
                        type_name(&left_type),
                        type_name(&right_type)
                    ),
                    Some(&binary.span.clone()),
                ));
            }
        }
        Operator::Equal | Operator::NotEqual => {}
        Operator::LessThan
        | Operator::LessEqual
        | Operator::GreaterThan
        | Operator::GreaterEqual => {
            let unknown = matches!(left_type, TypeSpec::Any) || matches!(right_type, TypeSpec::Any);
            if !are_comparison_types_compatible(&left_type, &right_type) && !unknown {
                return Err(validation_err(
                    format!(
                        "Operator '{:?}' requires comparable operands, got {} and {}",
                        binary.operator,
                        type_name(&left_type),
                        type_name(&right_type)
                    ),
                    Some(&binary.span.clone()),
                ));
            }
        }
        Operator::Regex => {
            let unknown = matches!(left_type, TypeSpec::Any) || matches!(right_type, TypeSpec::Any);
            if (!is_string_like_type(&left_type) || !is_string_like_type(&right_type)) && !unknown {
                return Err(validation_err(
                    format!(
                        "Operator '{:?}' requires string-like operands, got {} and {}",
                        binary.operator,
                        type_name(&left_type),
                        type_name(&right_type)
                    ),
                    Some(&binary.span.clone()),
                ));
            }
        }
        _ => {}
    }

    Ok(())
}

/// Function to validate a CAST expression, ensuring that the target type is compatible with the source expression's inferred type. This function checks the inferred type of the source expression and validates it against the target type specified in the CAST, enforcing rules about which types can be cast to which other types, and providing error messages when invalid casts are attempted.
pub(super) fn validate_cast_target(
    cast_expression: &CastExpression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    if let TypeSpec::List { element_type, .. } = &cast_expression.target_type
        && !matches!(element_type.as_ref(), TypeSpec::Any)
        && contains_dynamic_parameter_reference(&cast_expression.expression)
    {
        return Err(validation_err(
            "Typed LIST cast with dynamic parameters cannot be validated statically".to_string(),
            Some(&cast_expression.span.clone()),
        ));
    }

    let Some(source_type) = infer_expression_type(
        &cast_expression.expression,
        inferred_types,
        Some(declared_variables),
    ) else {
        return Ok(());
    };

    if is_cast_compatible(&source_type, &cast_expression.target_type) {
        return Ok(());
    }

    Err(validation_err(
        format!(
            "Invalid CAST target: cannot cast {} to {}",
            type_name(&source_type),
            type_name(&cast_expression.target_type)
        ),
        Some(&cast_expression.span.clone()),
    ))
}

/// Helper function to infer the type of an expression based on its structure and the types of any variables it references. This function recursively analyzes the expression tree, using the provided inferred types for variables and any declared variables to determine the resulting type of the expression, which is used for type checking and validation in various contexts throughout the query processing.
pub(super) fn infer_expression_type(
    expression: &Expression,
    inferred_types: &HashMap<String, TypeSpec>,
    declared_variables: Option<&HashSet<String>>,
) -> Option<TypeSpec> {
    match expression {
        Expression::Literal(literal) => Some(type_from_literal(literal)),
        Expression::Variable { name, span: _ } => {
            let trimmed = name.trim();
            if let Some(found) = inferred_types.get(trimmed) {
                return Some(found.clone());
            }

            if trimmed.parse::<i64>().is_ok() {
                return Some(TypeSpec::Integer);
            }
            if trimmed.parse::<f64>().is_ok() {
                return Some(TypeSpec::Float { precision: None });
            }
            if declared_variables
                .map(|vars| vars.contains(trimmed))
                .unwrap_or(false)
            {
                return Some(TypeSpec::Any);
            }
            None
        }
        Expression::PropertyAccess(property_access) => {
            let object_type = inferred_types
                .get(property_access.object.trim())
                .cloned()
                .or_else(|| {
                    if declared_variables
                        .map(|vars| vars.contains(property_access.object.trim()))
                        .unwrap_or(false)
                    {
                        Some(TypeSpec::Any)
                    } else {
                        None
                    }
                })?;

            match object_type {
                TypeSpec::Record
                | TypeSpec::Reference { .. }
                | TypeSpec::Graph { .. }
                | TypeSpec::BindingTable
                | TypeSpec::Any => Some(TypeSpec::Any),
                _ => Some(TypeSpec::Any),
            }
        }
        Expression::FunctionCall(function_call) => {
            infer_function_call_type(function_call, inferred_types, declared_variables)
        }
        Expression::IsPredicate(_) | Expression::WhenOperand(_) => Some(TypeSpec::Boolean),
        Expression::Array(array) => {
            let mut inferred_element: Option<TypeSpec> = None;
            for item in &array.elements {
                let Some(item_type) =
                    infer_expression_type(item, inferred_types, declared_variables)
                else {
                    inferred_element = None;
                    break;
                };

                inferred_element = match inferred_element {
                    None => Some(item_type),
                    Some(prev) => merge_array_element_types(&prev, &item_type),
                };
            }

            Some(TypeSpec::List {
                element_type: Box::new(inferred_element.unwrap_or(TypeSpec::Any)),
                max_length: None,
            })
        }
        Expression::Map(_) => Some(TypeSpec::Record),
        Expression::Case(case_expression) => {
            infer_case_type(case_expression, inferred_types, declared_variables)
        }
        Expression::Subquery(subquery) => {
            if subquery.is_exists {
                return Some(TypeSpec::Boolean);
            }
            if let Some(expr) = &subquery.expression {
                infer_expression_type(expr, inferred_types, declared_variables)
            } else if let Some(scalar_type) =
                infer_scalar_subquery_type(&subquery.query, inferred_types, declared_variables)
            {
                Some(scalar_type)
            } else {
                Some(TypeSpec::BindingTable)
            }
        }
        Expression::Cast(cast_expression) => Some(cast_expression.target_type.clone()),
        Expression::InlineLet(inline_let) => {
            let mut local = inferred_types.clone();
            for def in &inline_let.variable_definitions {
                if let Some(def_type) =
                    infer_expression_type(&def.expression, &local, declared_variables)
                {
                    local.insert(def.variable_name.trim().to_string(), def_type);
                }
            }
            infer_expression_type(
                &inline_let.evaluation_expression,
                &local,
                declared_variables,
            )
        }
        Expression::PathConstructor(_) => Some(TypeSpec::Path),
        Expression::Unary(unary) => {
            let operand_type =
                infer_expression_type(&unary.expression, inferred_types, declared_variables)?;
            match unary.operator {
                Operator::Not => Some(TypeSpec::Boolean),
                Operator::Plus | Operator::Minus => {
                    if is_numeric_type(&operand_type) {
                        Some(operand_type)
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
        Expression::Binary(binary) => {
            let left_type =
                infer_expression_type(&binary.left, inferred_types, declared_variables)?;
            let right_type =
                infer_expression_type(&binary.right, inferred_types, declared_variables)?;

            match binary.operator {
                Operator::And
                | Operator::Or
                | Operator::Xor
                | Operator::Equal
                | Operator::NotEqual
                | Operator::LessThan
                | Operator::LessEqual
                | Operator::GreaterThan
                | Operator::GreaterEqual
                | Operator::Regex => Some(TypeSpec::Boolean),
                Operator::Concat => Some(TypeSpec::String { max_length: None }),
                Operator::Plus | Operator::Minus => {
                    if is_duration_type(&left_type) && is_duration_type(&right_type) {
                        Some(TypeSpec::Duration {
                            precision: None,
                            qualifier: None,
                        })
                    } else if is_numeric_type(&left_type) && is_numeric_type(&right_type) {
                        Some(promote_numeric_type(&left_type, &right_type))
                    } else {
                        None
                    }
                }
                Operator::Star | Operator::Slash | Operator::Percent | Operator::Caret => {
                    if is_numeric_type(&left_type) && is_numeric_type(&right_type) {
                        Some(promote_numeric_type(&left_type, &right_type))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        }
    }
}

/// Helper function to infer the type of a scalar subquery by analyzing its structure and the types of any expressions it returns. This function checks if the subquery is a basic query with a RETURN clause that returns exactly one expression, and if so, it infers the type of that expression. For set operations, it attempts to infer the type from both sides of the operation and determines a compatible type if possible.
fn infer_scalar_subquery_type(
    query: &Query,
    inferred_types: &HashMap<String, TypeSpec>,
    declared_variables: Option<&HashSet<String>>,
) -> Option<TypeSpec> {
    match query {
        Query::Basic(basic) => {
            let PrimitiveResultStatement::Return { return_clause, .. } = &basic.result_statement
            else {
                return None;
            };

            if return_clause.items.len() != 1 {
                return None;
            }

            infer_expression_type(
                &return_clause.items[0].expression,
                inferred_types,
                declared_variables,
            )
            .or(Some(TypeSpec::Any))
        }
        Query::SetOperation(set_op) => {
            let left = infer_scalar_subquery_type(&set_op.left, inferred_types, declared_variables);
            let right =
                infer_scalar_subquery_type(&set_op.right, inferred_types, declared_variables);

            match (left, right) {
                (Some(left_type), Some(right_type)) if left_type == right_type => Some(left_type),
                (Some(_), Some(_)) => Some(TypeSpec::Any),
                (Some(left_type), None) => Some(left_type),
                (None, Some(right_type)) => Some(right_type),
                _ => None,
            }
        }
    }
}

/// Helper function to infer the resulting type of a CASE expression by analyzing the types of its branches and the ELSE clause. This function collects the inferred types from all THEN expressions and the ELSE expression (if present), normalizes them to find a common supertype, and determines if the CASE expression can be typed as a single type or if it should be treated as a union of multiple types.
pub(super) fn infer_case_type(
    case_expression: &CaseExpression,
    inferred_types: &HashMap<String, TypeSpec>,
    declared_variables: Option<&HashSet<String>>,
) -> Option<TypeSpec> {
    let mut candidates: Vec<TypeSpec> = Vec::new();

    match &case_expression.case_type {
        CaseType::Simple(simple_case) => {
            for branch in &simple_case.when_branches {
                if let Some(t) = infer_expression_type(
                    &branch.then_expression,
                    inferred_types,
                    declared_variables,
                ) {
                    candidates.push(t);
                }
            }
            if let Some(else_expr) = &simple_case.else_expression
                && let Some(t) =
                    infer_expression_type(else_expr, inferred_types, declared_variables)
            {
                candidates.push(t);
            }
        }
        CaseType::Searched(searched_case) => {
            for branch in &searched_case.when_branches {
                if let Some(t) = infer_expression_type(
                    &branch.then_expression,
                    inferred_types,
                    declared_variables,
                ) {
                    candidates.push(t);
                }
            }
            if let Some(else_expr) = &searched_case.else_expression
                && let Some(t) =
                    infer_expression_type(else_expr, inferred_types, declared_variables)
            {
                candidates.push(t);
            }
        }
    }

    if candidates.is_empty() {
        return None;
    }

    let members = normalize_union_members(candidates);
    if members.len() == 1 {
        members.first().cloned()
    } else {
        Some(TypeSpec::Union { members })
    }
}

/// Helper function to infer the return type of a function call based on its name and the types of its arguments. This function contains specific logic for built-in functions, determining their return types according to their semantics (e.g., COUNT returns INTEGER, AVG returns FLOAT, etc.), and falls back to None for functions that cannot be inferred statically.
pub(super) fn infer_function_call_type(
    function_call: &FunctionCall,
    inferred_types: &HashMap<String, TypeSpec>,
    declared_variables: Option<&HashSet<String>>,
) -> Option<TypeSpec> {
    let name = function_call.name.to_ascii_uppercase();

    if name == fn_names::COUNT
        || name == fn_names::PATH_LENGTH
        || name == "CARDINALITY"
        || name == "SIZE"
    {
        return Some(TypeSpec::Integer);
    }

    if name == fn_names::AVG {
        return Some(TypeSpec::Float { precision: None });
    }

    if name == fn_names::SUM || name == fn_names::MIN || name == fn_names::MAX {
        return function_call
            .arguments
            .first()
            .and_then(|arg| infer_expression_type(arg, inferred_types, declared_variables));
    }

    if name == fn_names::ELEMENT_ID
        || name == "LOWER"
        || name == "UPPER"
        || name == fn_names::TRIM
        || name == "LEFT"
        || name == "RIGHT"
        || name == fn_names::NORMALIZE
    {
        return Some(TypeSpec::String { max_length: None });
    }

    if name == fn_names::CURRENT_DATE || name == fn_names::DATE {
        return Some(TypeSpec::Date);
    }

    if name == fn_names::CURRENT_TIME || name == fn_names::LOCAL_TIME || name == "ZONED_TIME" {
        return Some(TypeSpec::LocalTime { precision: None });
    }

    if name == fn_names::CURRENT_TIMESTAMP
        || name == "LOCAL_TIMESTAMP"
        || name == "LOCAL_DATETIME"
        || name == fn_names::ZONED_DATETIME
    {
        return Some(TypeSpec::LocalDateTime { precision: None });
    }

    if name == fn_names::DURATION || name == fn_names::DURATION_BETWEEN {
        return Some(TypeSpec::Duration {
            precision: None,
            qualifier: None,
        });
    }

    if name == fn_names::ELEMENTS || name == fn_names::COLLECT_LIST || name == fn_names::COLLECT_SET
    {
        return Some(TypeSpec::List {
            element_type: Box::new(TypeSpec::Any),
            max_length: None,
        });
    }

    None
}

/// Helper function to determine the type of a literal value, which is used to infer the types of expressions that are literals. This function maps each variant of the Literal enum to a corresponding TypeSpec, allowing the validator to understand the types of literal expressions and use that information for type checking and inference in larger expressions.
pub(super) fn type_from_literal(literal: &Literal) -> TypeSpec {
    match literal {
        Literal::String(_) => TypeSpec::String { max_length: None },
        Literal::Integer(_) => TypeSpec::Integer,
        Literal::Float(_) => TypeSpec::Float { precision: None },
        Literal::Boolean(_) => TypeSpec::Boolean,
        Literal::Null => TypeSpec::Any,
        Literal::DateTime(_) => TypeSpec::Timestamp {
            precision: None,
            with_timezone: false,
        },
        Literal::Duration(_) => TypeSpec::Duration {
            precision: None,
            qualifier: None,
        },
        Literal::TimeWindow(_) => TypeSpec::String { max_length: None },
        Literal::Vector(values) => TypeSpec::Vector {
            dimension: Some(values.len()),
        },
        Literal::List(items) => {
            let mut inferred: Option<TypeSpec> = None;

            for item in items {
                let item_type = type_from_literal(item);
                inferred = match inferred {
                    None => Some(item_type),
                    Some(prev) => merge_array_element_types(&prev, &item_type),
                };
            }

            TypeSpec::List {
                element_type: Box::new(inferred.unwrap_or(TypeSpec::Any)),
                max_length: None,
            }
        }
        Literal::Parameter(_) => TypeSpec::Any,
    }
}

/// Function to validate the consistency of element types within an array literal, ensuring that all elements are of compatible types. This function iterates through the elements of the array, infers their types, and checks that they are compatible with each other according to the rules for array literals (e.g., all elements must be of the same type or compatible types). If incompatible types are found, it returns a validation error indicating the issue.
pub(super) fn validate_array_element_consistency(
    array: &ArrayExpression,
    declared_variables: &HashSet<String>,
    inferred_types: &HashMap<String, TypeSpec>,
) -> Result<(), GqlError> {
    let mut inferred_element: Option<TypeSpec> = None;

    for item in &array.elements {
        let Some(item_type) = infer_expression_type(item, inferred_types, Some(declared_variables))
        else {
            continue;
        };

        if let Some(prev) = &inferred_element
            && !are_array_element_types_compatible(prev, &item_type)
        {
            return Err(validation_err(
                format!(
                    "Array literal contains incompatible element types: {} and {}",
                    type_name(prev),
                    type_name(&item_type)
                ),
                Some(&array.span.clone()),
            ));
        }

        inferred_element = match inferred_element {
            None => Some(item_type),
            Some(prev) => merge_array_element_types(&prev, &item_type),
        };
    }

    Ok(())
}

/// Helper function to determine if two types are compatible for use as element types in an array literal. This function checks if the types are the same, if either is ANY, if they are both numeric types, or if they are both lists with compatible element types. This allows for flexible array literals while enforcing type consistency where possible.
pub(super) fn are_array_element_types_compatible(left: &TypeSpec, right: &TypeSpec) -> bool {
    if left == right {
        return true;
    }

    if matches!(left, TypeSpec::Any) || matches!(right, TypeSpec::Any) {
        return true;
    }

    if is_numeric_type(left) && is_numeric_type(right) {
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
        ) => are_array_element_types_compatible(l, r),
        // A nested list is a value, not a flattened sequence of outer elements.
        // Its own contents are validated recursively before this check.
        (TypeSpec::List { .. }, _) | (_, TypeSpec::List { .. }) => true,
        _ => false,
    }
}

/// Helper function to merge two types into a common type for use as the element type of an array literal. This function returns a type that is compatible with both input types, following the same rules as the compatibility check (e.g., if they are the same, return that type; if one is ANY, return the other; if both are numeric, return a promoted numeric type; if both are lists, merge their element types). This is used to determine the resulting type of an array literal when its elements have different but compatible types.
pub(super) fn merge_array_element_types(left: &TypeSpec, right: &TypeSpec) -> Option<TypeSpec> {
    if left == right {
        return Some(left.clone());
    }

    if matches!(left, TypeSpec::Any) {
        return Some(right.clone());
    }

    if matches!(right, TypeSpec::Any) {
        return Some(left.clone());
    }

    if is_numeric_type(left) && is_numeric_type(right) {
        return Some(promote_numeric_type(left, right));
    }

    match (left, right) {
        (
            TypeSpec::List {
                element_type: left_elem,
                max_length,
            },
            TypeSpec::List {
                element_type: right_elem,
                ..
            },
        ) => merge_array_element_types(left_elem, right_elem).map(|element_type| TypeSpec::List {
            element_type: Box::new(element_type),
            max_length: *max_length,
        }),
        _ => Some(TypeSpec::Any),
    }
}

/// Helper function to normalize a list of types into a flattened list of unique types, which is used when inferring the type of a CASE expression that may have multiple branches with different types. This function takes a list of candidate types, flattens any nested unions, and removes duplicates to produce a simplified list of types that can be used to determine the resulting type of the CASE expression.
pub(super) fn normalize_union_members(candidates: Vec<TypeSpec>) -> Vec<TypeSpec> {
    let mut flattened = Vec::new();

    for candidate in candidates {
        match candidate {
            TypeSpec::Union { members } => {
                for member in members {
                    if !flattened.contains(&member) {
                        flattened.push(member);
                    }
                }
            }
            other => {
                if !flattened.contains(&other) {
                    flattened.push(other);
                }
            }
        }
    }

    flattened
}

/// Helper function to check if an expression contains a reference to a dynamic parameter (i.e., a variable that starts with '$'), which is used to determine if certain validations can be performed statically or if they need to be deferred until runtime. This function recursively checks the structure of the expression for any variable references that match the criteria for dynamic parameters.
pub(super) fn contains_dynamic_parameter_reference(expression: &Expression) -> bool {
    match expression {
        Expression::Variable { name, span: _ } => name.trim().starts_with('$'),
        Expression::Binary(binary) => {
            contains_dynamic_parameter_reference(&binary.left)
                || contains_dynamic_parameter_reference(&binary.right)
        }
        Expression::Unary(unary) => contains_dynamic_parameter_reference(&unary.expression),
        Expression::PropertyAccess(_) => false,
        Expression::FunctionCall(call) => call
            .arguments
            .iter()
            .any(contains_dynamic_parameter_reference),
        Expression::IsPredicate(predicate) => {
            contains_dynamic_parameter_reference(&predicate.expression)
        }
        Expression::Array(array) => array
            .elements
            .iter()
            .any(contains_dynamic_parameter_reference),
        Expression::Map(properties) => properties
            .iter()
            .any(|item| contains_dynamic_parameter_reference(&item.value)),
        Expression::Case(case_expression) => match &case_expression.case_type {
            CaseType::Simple(simple_case) => {
                contains_dynamic_parameter_reference(&simple_case.test_expression)
                    || simple_case.when_branches.iter().any(|branch| {
                        branch
                            .when_values
                            .iter()
                            .any(contains_dynamic_parameter_reference)
                            || contains_dynamic_parameter_reference(&branch.then_expression)
                    })
                    || simple_case
                        .else_expression
                        .as_ref()
                        .map(|expr| contains_dynamic_parameter_reference(expr))
                        .unwrap_or(false)
            }
            CaseType::Searched(searched_case) => {
                searched_case.when_branches.iter().any(|branch| {
                    contains_dynamic_parameter_reference(&branch.condition)
                        || contains_dynamic_parameter_reference(&branch.then_expression)
                }) || searched_case
                    .else_expression
                    .as_ref()
                    .map(|expr| contains_dynamic_parameter_reference(expr))
                    .unwrap_or(false)
            }
        },
        Expression::Subquery(_) => false,
        Expression::Cast(cast_expression) => {
            contains_dynamic_parameter_reference(&cast_expression.expression)
        }
        Expression::InlineLet(expr) => {
            expr.variable_definitions
                .iter()
                .any(|def| contains_dynamic_parameter_reference(&def.expression))
                || contains_dynamic_parameter_reference(&expr.evaluation_expression)
        }
        Expression::PathConstructor(elements) => {
            elements.iter().any(contains_dynamic_parameter_reference)
        }
        Expression::WhenOperand(operand) => match &operand.kind {
            WhenOperandKind::Comparison { right, .. } => {
                contains_dynamic_parameter_reference(right)
            }
            _ => false,
        },
        Expression::Literal(_) => false,
    }
}

/// Helper function to determine if two types are compatible for use in a comparison operation, which is used to validate the types of operands in comparison expressions. This function checks if the types are the same, if either is ANY, if they are both numeric, if they are both temporal-like, or if they are both string-like, allowing for flexible comparisons while enforcing type compatibility where necessary.
pub(super) fn promote_numeric_type(left: &TypeSpec, right: &TypeSpec) -> TypeSpec {
    if is_float_like_type(left) || is_float_like_type(right) {
        TypeSpec::Float { precision: None }
    } else {
        TypeSpec::Integer
    }
}

/// Helper function to determine if a type is considered "float-like" for the purposes of type promotion and compatibility checks. This function checks if the type is any of the floating-point types defined in the TypeSpec enum, which is used to determine how numeric types should be promoted when combined in expressions.
pub(super) fn is_float_like_type(t: &TypeSpec) -> bool {
    matches!(
        t,
        TypeSpec::Float { .. }
            | TypeSpec::Float32
            | TypeSpec::Real
            | TypeSpec::Double
            | TypeSpec::Decimal { .. }
    )
}

/// Helper function to determine if a type is considered "numeric" for the purposes of type checking and validation. This function checks if the type is any of the numeric types defined in the TypeSpec enum, which is used to validate operations that require numeric operands and to enforce type rules in expressions that involve numbers.
pub(super) fn is_numeric_type(t: &TypeSpec) -> bool {
    matches!(
        t,
        TypeSpec::Integer
            | TypeSpec::BigInt
            | TypeSpec::SmallInt
            | TypeSpec::Int128
            | TypeSpec::Int256
            | TypeSpec::Float { .. }
            | TypeSpec::Float32
            | TypeSpec::Real
            | TypeSpec::Double
            | TypeSpec::Decimal { .. }
    )
}

/// Helper function to determine if a type is considered "duration-like" for the purposes of type checking and validation. This function checks if the type is a Duration, which is used to validate operations that involve durations and ensure that they are used correctly in expressions.
pub(super) fn is_duration_type(t: &TypeSpec) -> bool {
    matches!(t, TypeSpec::Duration { .. })
}

/// Helper function to determine if a type is considered "boolean" for the purposes of type checking and validation. This function checks if the type is Boolean, which is used to validate logical operations and ensure that boolean expressions are used correctly in conditions and predicates.
pub(super) fn is_boolean_type(t: &TypeSpec) -> bool {
    matches!(t, TypeSpec::Boolean)
}

/// Helper function to determine if a type is considered "temporal-like" for the purposes of type checking and validation. This function checks if the type is any of the temporal types defined in the TypeSpec enum (e.g., Date, Time, Timestamp, etc.), which is used to validate operations that involve temporal values and ensure that they are used correctly in expressions.
pub(super) fn is_temporal_like_type(t: &TypeSpec) -> bool {
    matches!(
        t,
        TypeSpec::Date
            | TypeSpec::Time { .. }
            | TypeSpec::Timestamp { .. }
            | TypeSpec::ZonedTime { .. }
            | TypeSpec::ZonedDateTime { .. }
            | TypeSpec::LocalTime { .. }
            | TypeSpec::LocalDateTime { .. }
            | TypeSpec::Duration { .. }
    )
}

/// Helper function to determine if a type is considered "string-like" for the purposes of type checking and validation. This function checks if the type is either String or Bytes, which is used to validate operations that involve string-like values (e.g., concatenation, regex matching) and ensure that they are used correctly in expressions.
pub(super) fn is_string_like_type(t: &TypeSpec) -> bool {
    matches!(t, TypeSpec::String { .. } | TypeSpec::Bytes { .. })
}

/// Helper function to determine if a type is compatible with the CONCAT operator, which requires string-like operands. This function checks if the type is string-like or if it is a Path (which can be implicitly converted to a string), allowing for flexible concatenation while enforcing type rules.
pub(super) fn is_concat_compatible_type(t: &TypeSpec) -> bool {
    is_string_like_type(t) || matches!(t, TypeSpec::Path)
}

/// Helper function to determine if a source type can be cast to a target type according to the rules of type compatibility and casting in the query language. This function checks various conditions for compatibility, such as if the types are the same, if either is ANY, if they are string-like, numeric, boolean, temporal-like, or if they are lists with compatible element types. This is used to validate CAST expressions and ensure that casts are only allowed when they make sense according to the type system.
pub(super) fn is_cast_compatible(source: &TypeSpec, target: &TypeSpec) -> bool {
    if source == target {
        return true;
    }

    if matches!(target, TypeSpec::Any) {
        return true;
    }

    if matches!(target, TypeSpec::String { .. }) {
        return true;
    }

    match (source, target) {
        (TypeSpec::Any, _) => true,
        (TypeSpec::Union { members }, _) => members.iter().all(|m| is_cast_compatible(m, target)),
        (_, TypeSpec::Union { members }) => members.iter().any(|m| is_cast_compatible(source, m)),
        (TypeSpec::String { .. }, t) => {
            is_numeric_type(t) || is_temporal_like_type(t) || is_boolean_type(t)
        }
        (s, t) if is_numeric_type(s) => {
            is_numeric_type(t) || is_boolean_type(t) || matches!(t, TypeSpec::String { .. })
        }
        (TypeSpec::Boolean, t) => matches!(t, TypeSpec::Boolean | TypeSpec::String { .. }),
        (s, t) if is_temporal_like_type(s) => {
            is_temporal_like_type(t) || matches!(t, TypeSpec::String { .. })
        }
        (
            TypeSpec::List {
                element_type: source_elem,
                ..
            },
            TypeSpec::List {
                element_type: target_elem,
                ..
            },
        ) => {
            if matches!(source_elem.as_ref(), TypeSpec::Any)
                && !matches!(target_elem.as_ref(), TypeSpec::Any)
            {
                return false;
            }

            is_cast_compatible(source_elem, target_elem)
        }
        (TypeSpec::Record, TypeSpec::Record) => true,
        (TypeSpec::Path, TypeSpec::Path) => true,
        _ => false,
    }
}

/// Helper function to get a human-readable name for a type, which is used in error messages and validation feedback to indicate the types of expressions and why certain operations may be invalid. This function matches each variant of the TypeSpec enum to a corresponding string representation that can be used in messages to users or developers.
pub(super) fn type_name(t: &TypeSpec) -> &'static str {
    match t {
        TypeSpec::Boolean => "BOOLEAN",
        TypeSpec::String { .. } => "STRING",
        TypeSpec::Bytes { .. } => "BYTES",
        TypeSpec::Decimal { .. } => "DECIMAL",
        TypeSpec::Integer => "INTEGER",
        TypeSpec::BigInt => "BIGINT",
        TypeSpec::SmallInt => "SMALLINT",
        TypeSpec::Int128 => "INT128",
        TypeSpec::Int256 => "INT256",
        TypeSpec::Float { .. } => "FLOAT",
        TypeSpec::Float32 => "FLOAT32",
        TypeSpec::Real => "REAL",
        TypeSpec::Double => "DOUBLE",
        TypeSpec::Vector { .. } => "VECTOR",
        TypeSpec::Date => "DATE",
        TypeSpec::Time { .. } => "TIME",
        TypeSpec::Timestamp { .. } => "TIMESTAMP",
        TypeSpec::ZonedTime { .. } => "ZONED TIME",
        TypeSpec::ZonedDateTime { .. } => "ZONED DATETIME",
        TypeSpec::LocalTime { .. } => "LOCAL TIME",
        TypeSpec::LocalDateTime { .. } => "LOCAL DATETIME",
        TypeSpec::Duration { .. } => "DURATION",
        TypeSpec::Reference { .. } => "REFERENCE",
        TypeSpec::Path => "PATH",
        TypeSpec::List { .. } => "LIST",
        TypeSpec::Record => "RECORD",
        TypeSpec::Graph { .. } => "GRAPH",
        TypeSpec::BindingTable => "BINDING TABLE",
        TypeSpec::Union { .. } => "UNION",
        TypeSpec::Any => "ANY",
    }
}

/// Function to validate a CALL clause, ensuring that the procedure name is valid, that inline procedures are used correctly, and that the arguments and YIELD items are consistent with the declared variables and the structure of the CALL. This function checks various conditions specific to CALL clauses, such as ensuring that inline CALLs do not have YIELD items, that the preserved structured statement is valid, and that the arguments are valid expressions. It also updates the set of declared variables based on the YIELD items defined in the CALL.
pub(super) fn validate_call_clause(
    call_clause: &CallClause,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    if call_clause.procedure_name.trim().is_empty() {
        return Err(validation_err(
            "CALL procedure name cannot be empty".to_string(),
            Some(&call_clause.span.clone()),
        ));
    }

    if call_clause.procedure_name == "__INLINE_CALL__" {
        if call_clause.yield_items.is_some() {
            return Err(validation_err(
                "Inline CALL cannot define YIELD items".to_string(),
                Some(&call_clause.span.clone()),
            ));
        }

        let inline_stmt = call_clause
            .inline_procedure_statement
            .as_ref()
            .ok_or_else(|| {
                validation_err(
                    "Inline CALL must preserve structured nested statement".to_string(),
                    Some(&call_clause.span.clone()),
                )
            })?;

        let mut nested_scope = declared_variables.clone();
        let mut inline_scope_vars = HashSet::new();

        for arg in &call_clause.arguments {
            if let Expression::Variable { name, span } = arg {
                let trimmed = name.trim().to_string();
                if !inline_scope_vars.insert(trimmed.clone()) {
                    return Err(validation_err(
                        format!(
                            "Inline CALL scope contains duplicate variable '{}'",
                            trimmed
                        ),
                        Some(&span.clone()),
                    ));
                }
                nested_scope.insert(trimmed);
            } else {
                return Err(validation_err(
                    "Inline CALL scope arguments must be binding variables".to_string(),
                    Some(&call_clause.span.clone()),
                ));
            }
        }

        let mut nested_ctx = ValidationContext {
            declared_variables: nested_scope,
        };
        validate_dispatch(inline_stmt, &mut nested_ctx)?;

        if let Statement::Query(query) = inline_stmt.as_ref() {
            for output_name in collect_projected_output_names(query) {
                declared_variables.insert(output_name);
            }
        }
    } else if call_clause.inline_procedure_statement.is_some() {
        return Err(validation_err(
            "Named CALL cannot preserve inline procedure statement".to_string(),
            Some(&call_clause.span.clone()),
        ));
    }

    if !declared_variables.is_empty() {
        for arg in &call_clause.arguments {
            validate_expression(arg, declared_variables)?;
        }
    }

    if let Some(yield_clause) = &call_clause.yield_items {
        validate_yield_clause(yield_clause)?;
        for item in &yield_clause.items {
            let var_to_insert = if let Some(alias) = &item.alias {
                alias.clone()
            } else {
                item.column_name.clone()
            };

            let trimmed_var = var_to_insert.trim().to_string();
            declared_variables.insert(trimmed_var);
        }
    }

    Ok(())
}
