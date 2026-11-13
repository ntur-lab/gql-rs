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

/// Function to validate a query, ensuring that all components of the query are structurally and semantically correct according to the rules of the query language. This function recursively validates each part of the query, including MATCH clauses, LET statements, FILTER conditions, HAVING conditions, FOR clauses, pagination clauses, CALL clauses, and subqueries. It also manages the scope of declared variables to ensure that all references are valid and that there are no naming conflicts.
pub(super) fn validate_basic_query(basic: &BasicQuery, declared_variables: &mut HashSet<String>) -> Result<(), GqlError> {
    for step in &basic.pipeline {
        match step {
            QueryStep::Match(match_clause) => {
                validate_match_clause(match_clause, declared_variables)?;
            }
            QueryStep::Let(let_stmt) => {
                validate_let_statement(let_stmt, declared_variables)?;
            }
            QueryStep::Filter(filter_clause) => {
                validate_expression(&filter_clause.condition, declared_variables)?;
                validate_predicate_condition_type(&filter_clause.condition, declared_variables, "WHERE/FILTER")?;
                check_no_aggregates_in_where(&filter_clause.condition)?;
            }
            QueryStep::Having(having_clause) => {
                validate_expression(&having_clause.condition, declared_variables)?;
                validate_predicate_condition_type(&having_clause.condition, declared_variables, "HAVING")?;
            }
            QueryStep::For(for_clause) => {
                let var_name = for_clause.variable.trim();
                if var_name.is_empty() {
                    return Err(validation_err(
                        "FOR variable name cannot be empty".to_string(),
                        Some(&for_clause.span.clone()),
                    ));
                }
                validate_expression(&for_clause.expression, declared_variables)?;
                declared_variables.insert(var_name.to_string());

                if let Some(ordinality_or_offset_var) = &for_clause.ordinality_or_offset_variable {
                    let ord_var = ordinality_or_offset_var.trim();
                    if ord_var.is_empty() {
                        return Err(validation_err(
                            "FOR ordinality/offset variable name cannot be empty".to_string(),
                            Some(&for_clause.span.clone()),
                        ));
                    }
                    declared_variables.insert(ord_var.to_string());
                }
            }
            QueryStep::Page(page_stmt) => {
                if let Some(order_clause) = &page_stmt.order_clause {
                    validate_order_clause(order_clause, declared_variables, None)?;
                }
                if let Some(limit_clause) = &page_stmt.limit_clause {
                    validate_limit_clause(limit_clause)?;
                }
            }
            QueryStep::Call(call_clause) => {
                validate_call_clause(call_clause, declared_variables)?;
            }
            QueryStep::UseGraph(graph_expr) => {
                validate_graph_expression_clause(graph_expr)?;
            }
            QueryStep::FromSubquery(from_query) => {
                let mut nested_scope = declared_variables.clone();
                validate_query(from_query, &mut nested_scope)?;

                for output_name in collect_projected_output_names(from_query) {
                    declared_variables.insert(output_name);
                }
            }
        }
    }

    match &basic.result_statement {
        PrimitiveResultStatement::Finish { span: _ } => {}
        PrimitiveResultStatement::Return {
            return_clause,
            group_clause,
            page_clause,
            span: _,
        } => {
            let mut return_aliases = HashSet::new();

            validate_return_clause_with_aliases(return_clause, declared_variables, &mut return_aliases)?;

            if let Some(group) = group_clause {
                validate_group_clause(group, declared_variables, &return_aliases)?;
                validate_non_aggregated_return_variables_in_group_by(return_clause, group)?;
            }

            if let Some(page) = page_clause {
                if let Some(order_clause) = &page.order_clause {
                    validate_order_clause(order_clause, declared_variables, Some(&return_aliases))?;
                }
                if let Some(limit_clause) = &page.limit_clause {
                    validate_limit_clause(limit_clause)?;
                }
            }
        }
    }

    Ok(())
}

/// Function to validate a MATCH clause, ensuring that all patterns and their components are structurally and semantically correct. This includes checking that path patterns are well-formed, that any variables used are properly declared and do not conflict with each other, and that any mode-specific constraints on patterns are respected. The function also manages the scope of declared variables to ensure that they are available for subsequent query steps.
pub(super) fn validate_match_clause(
    clause: &MatchClause,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    for pattern in &clause.match_clause.patterns {
        let term_slices = path_pattern_term_slices(pattern);
        if term_slices.is_empty() {
            return Err(validation_err(
                "Path pattern cannot be empty".to_string(),
                Some(&pattern.span.clone()),
            ));
        }

        if let Some(path_variable) = &pattern.variable {
            let trimmed = path_variable.trim();
            if trimmed.is_empty() {
                return Err(validation_err(
                    "Path variable cannot be empty".to_string(),
                    Some(&pattern.span.clone()),
                ));
            } else {
                declared_variables.insert(trimmed.to_string());
            }
        }

        validate_path_mode_search_compatibility(pattern)?;
        validate_path_mode_cycle_constraints(pattern)?;

        for term_elements in term_slices {
            if term_elements.is_empty() {
                return Err(validation_err(
                    "Path pattern cannot be empty".to_string(),
                    Some(&pattern.span.clone()),
                ));
            }

            for (idx, element) in term_elements.iter().enumerate() {
                match element {
                    PatternElement::Node(node) => validate_node(node, declared_variables)?,
                    PatternElement::Edge(edge) => {
                        if idx == 0 && term_elements.len() > 1 {
                            return Err(validation_err(
                                "Path pattern cannot start with an edge".to_string(),
                                Some(&pattern.span.clone()),
                            ));
                        }
                        validate_edge(edge, declared_variables, pattern.path_mode.as_ref())?;
                    }
                }
            }

            if term_elements.len() > 1 && !matches!(term_elements.last(), Some(PatternElement::Node(_))) {
                return Err(validation_err(
                    "Path pattern must end with a node".to_string(),
                    Some(&pattern.span.clone()),
                ));
            }
        }
    }
    Ok(())
}

/// Helper function to extract slices of pattern elements for each alternative in a path pattern, or a single slice if there is no alternation. This allows validation logic to uniformly process each linear sequence of pattern elements regardless of whether the original pattern was defined with alternatives or not.
pub(super) fn path_pattern_term_slices(pattern: &PathPattern) -> Vec<&[PatternElement]> {
    if let Some(alternation) = &pattern.alternation {
        return alternation
            .alternatives
            .iter()
            .map(|elements| elements.as_slice())
            .collect();
    }

    vec![pattern.elements.as_slice()]
}

/// Function to validate that the specified path search strategy is compatible with the path mode used in a pattern. For example, certain shortest path search strategies may not be valid when used with a WALK path mode, and this function checks for such incompatibilities and returns appropriate validation errors if they are detected.
pub(super) fn validate_path_mode_search_compatibility(pattern: &PathPattern) -> Result<(), GqlError> {
    let Some(path_search) = &pattern.path_search else {
        return Ok(());
    };
    let Some(path_mode) = &pattern.path_mode else {
        return Ok(());
    };

    let is_shortest_search = matches!(
        path_search,
        PathSearch::Shortest
            | PathSearch::AllShortest
            | PathSearch::AnyShortest
            | PathSearch::CountedShortestPaths { .. }
            | PathSearch::CountedShortestGroups { .. }
    );

    if is_shortest_search && matches!(path_mode, PathMode::Walk) {
        return Err(validation_err(
            "Path mode WALK is not compatible with SHORTEST path search".to_string(),
            Some(&pattern.span.clone()),
        ));
    }

    Ok(())
}

/// Function to validate that path patterns with SIMPLE or ACYCLIC modes do not contain immediate self-loops, which would violate the constraints of those modes. This function iterates through the pattern elements and checks for any occurrences of a node being connected to itself via an edge, and returns a validation error if such a case is found.
pub(super) fn validate_path_mode_cycle_constraints(pattern: &PathPattern) -> Result<(), GqlError> {
    let Some(path_mode) = &pattern.path_mode else {
        return Ok(());
    };

    if !matches!(path_mode, PathMode::Simple | PathMode::Acyclic) {
        return Ok(());
    }

    for term_elements in path_pattern_term_slices(pattern) {
        for triple in term_elements.windows(3) {
            let (PatternElement::Node(left_node), PatternElement::Edge(_), PatternElement::Node(right_node)) =
                (&triple[0], &triple[1], &triple[2])
            else {
                continue;
            };

            let (Some(left_id), Some(right_id)) = (&left_node.id, &right_node.id) else {
                continue;
            };

            let left_trimmed = left_id.trim();
            let right_trimmed = right_id.trim();
            if !left_trimmed.is_empty() && left_trimmed == right_trimmed {
                return Err(validation_err(
                    format!(
                        "Path mode {:?} does not allow immediate self-loop on node '{}'",
                        path_mode, left_trimmed
                    ),
                    Some(&pattern.span.clone()),
                ));
            }
        }
    }

    Ok(())
}

/// Function to collect output names projected by a query for outer-scope propagation.
pub(super) fn collect_projected_output_names(query: &Query) -> HashSet<String> {
    match query {
        Query::Basic(basic) => collect_projected_output_names_from_result(&basic.result_statement),
        Query::SetOperation(set_op) => {
            let left = collect_projected_output_names(&set_op.left);
            if left.is_empty() {
                collect_projected_output_names(&set_op.right)
            } else {
                left
            }
        }
    }
}

/// Helper function to extract projected names from a terminal RETURN result statement.
pub(super) fn collect_projected_output_names_from_result(result: &PrimitiveResultStatement) -> HashSet<String> {
    let mut names = HashSet::new();

    let PrimitiveResultStatement::Return { return_clause, .. } = result else {
        return names;
    };

    let ReturnProjection::Items(items) = &return_clause.projection else {
        return names;
    };

    for item in items {
        if let Some(alias) = &item.alias {
            let trimmed = alias.trim();
            if !trimmed.is_empty() {
                names.insert(trimmed.to_string());
                continue;
            }
        }

        names.extend(derive_output_names_from_expression(&item.expression));
    }

    names
}

/// Helper function to derive a stable output name from a projection expression when possible.
pub(super) fn derive_output_names_from_expression(expression: &Expression) -> Vec<String> {
    match expression {
        Expression::Variable { name, span: _ } => {
            let trimmed = name.trim();
            if trimmed.is_empty() {
                Vec::new()
            } else {
                vec![trimmed.to_string()]
            }
        }
        Expression::PropertyAccess(prop) => {
            let trimmed = prop.property.trim();
            if trimmed.is_empty() {
                Vec::new()
            } else {
                let mut out = Vec::new();
                let object = prop.object.trim();
                let property = prop.property.trim();
                if !property.is_empty() {
                    out.push(property.to_string());
                }
                if !object.is_empty() && !property.is_empty() {
                    out.push(format!("{}.{}", object, property));
                }
                out
            }
        }
        _ => Vec::new(),
    }
}

/// Function to validate node pattern content and register node variables into scope.
pub(super) fn validate_node(node: &Node, declared_variables: &mut HashSet<String>) -> Result<(), GqlError> {
    if let Some(id) = &node.id {
        let trimmed = id.trim();
        if trimmed.is_empty() {
            return Err(validation_err(
                "Node variable cannot be empty".to_string(),
                Some(&node.span.clone()),
            ));
        } else {
            declared_variables.insert(trimmed.to_string());
        }
    }

    if let Some(labels) = &node.labels {
        validate_expression(labels, declared_variables)?;
    }

    if let Some(properties) = &node.properties {
        validate_property_map(properties, declared_variables)?;
    }

    if let Some(where_clause) = &node.where_clause {
        validate_expression(where_clause, declared_variables)?;
        validate_predicate_condition_type(where_clause, declared_variables, "Node WHERE clause")?;
        check_no_aggregates_in_where(where_clause)?;
    }
    Ok(())
}
/// Function to validate LET definitions and updates variable scope incrementally.
pub(super) fn validate_let_statement(
    let_statement: &LetStatement,
    declared_variables: &mut HashSet<String>,
) -> Result<(), GqlError> {
    if let_statement.variable_definitions.is_empty() {
        return Err(validation_err(
            "LET statement must contain at least one variable definition".to_string(),
            Some(&let_statement.span.clone()),
        ));
    }

    let mut local_names = HashSet::new();

    for def in &let_statement.variable_definitions {
        let name = def.variable_name.trim();

        if name.is_empty() {
            return Err(validation_err(
                "LET variable name cannot be empty".to_string(),
                Some(&let_statement.span.clone()),
            ));
        }

        if !local_names.insert(name.to_string()) {
            return Err(validation_err(
                format!("Duplicate LET variable in same clause: {}", name),
                Some(&let_statement.span.clone()),
            ));
        }

        validate_expression(&def.expression, declared_variables)?;
        declared_variables.insert(name.to_string());
    }

    Ok(())
}

/// Function to validate edge pattern content and register edge variables into scope.
pub(super) fn validate_edge(
    edge: &Edge,
    declared_variables: &mut HashSet<String>,
    path_mode: Option<&PathMode>,
) -> Result<(), GqlError> {
    validate_edge_quantifier(edge, path_mode)?;

    if let Some(id) = &edge.id {
        let trimmed = id.trim();
        if trimmed.is_empty() {
            return Err(validation_err(
                "Edge variable cannot be empty".to_string(),
                Some(&edge.span.clone()),
            ));
        } else {
            declared_variables.insert(trimmed.to_string());
        }
    }

    if let Some(labels) = &edge.labels {
        validate_expression(labels, declared_variables)?;
    }

    if let Some(properties) = &edge.properties {
        validate_property_map(properties, declared_variables)?;
    }

    if let Some(where_clause) = &edge.where_clause {
        validate_expression(where_clause, declared_variables)?;
        validate_predicate_condition_type(where_clause, declared_variables, "Edge WHERE clause")?;
        check_no_aggregates_in_where(where_clause)?;
    }
    Ok(())
}

/// Function to validate edge quantifier bounds and mode-specific restrictions.
pub(super) fn validate_edge_quantifier(edge: &Edge, path_mode: Option<&PathMode>) -> Result<(), GqlError> {
    let Some(quantifier) = &edge.quantifier else {
        return Ok(());
    };

    if let PathQuantifier::Range { min, max } = quantifier
        && min > max
    {
        return Err(validation_err(
            format!(
                "Invalid path quantifier range: lower bound {} is greater than upper bound {}",
                min, max
            ),
            Some(&edge.span.clone()),
        ));
    }

    if matches!(path_mode, Some(PathMode::Simple | PathMode::Acyclic)) {
        let is_open = match quantifier {
            PathQuantifier::AtLeast(_) => true,
            PathQuantifier::Range { max, .. } => *max == u32::MAX,
            _ => false,
        };

        if is_open {
            return Err(validation_err(
                "SIMPLE/ACYCLIC path modes do not allow open quantifiers (*, +, or unbounded ranges)".to_string(),
                Some(&edge.span.clone()),
            ));
        }
    }

    Ok(())
}

/// Function to validate key/value pairs of a property map expression.
pub(super) fn validate_property_map(
    property_map: &PropertyMap,
    declared_variables: &HashSet<String>,
) -> Result<(), GqlError> {
    for prop in &property_map.properties {
        if prop.key.trim().is_empty() {
            return Err(validation_err(
                "Property key cannot be empty".to_string(),
                Some(&prop.span.clone()),
            ));
        }
        validate_expression(&prop.value, declared_variables)?;
    }
    Ok(())
}

/// Function to validate RETURN items and enforce alias uniqueness rules.
pub(super) fn validate_return_clause_with_aliases(
    return_clause: &ReturnClause,
    declared_variables: &HashSet<String>,
    return_aliases: &mut HashSet<String>,
) -> Result<(), GqlError> {
    let ReturnProjection::Items(items) = &return_clause.projection else {
        return Ok(());
    };

    if items.is_empty() {
        return Err(validation_err(
            "RETURN clause must contain at least one item".to_string(),
            Some(&return_clause.span.clone()),
        ));
    }

    for item in items {
        validate_expression(&item.expression, declared_variables)?;

        if let Some(alias) = &item.alias {
            let trimmed = alias.trim();
            if trimmed.is_empty() {
                return Err(validation_err(
                    "RETURN alias cannot be empty".to_string(),
                    Some(&return_clause.span.clone()),
                ));
            } else if !return_aliases.insert(trimmed.to_string()) {
                return Err(validation_err(
                    format!("Duplicate RETURN alias: {}", trimmed),
                    Some(&return_clause.span.clone()),
                ));
            }
        }
    }
    Ok(())
}

/// Function to validate GROUP BY clause shape and referenced grouping keys.
pub(super) fn validate_group_clause(
    group_clause: &GroupClause,
    declared_variables: &HashSet<String>,
    return_aliases: &HashSet<String>,
) -> Result<(), GqlError> {
    if group_clause.is_empty_grouping_set {
        if let Some(keys) = &group_clause.keys
            && !keys.is_empty()
        {
            return Err(validation_err(
                "GROUP BY () cannot contain grouping keys".to_string(),
                Some(&group_clause.span.clone()),
            ));
        }
        return Ok(());
    }

    let Some(keys) = &group_clause.keys else {
        return Err(validation_err(
            "GROUP BY must contain at least one key or be GROUP BY ()".to_string(),
            Some(&group_clause.span.clone()),
        ));
    };

    if keys.is_empty() {
        return Err(validation_err(
            "GROUP BY keys cannot be empty unless GROUP BY () is used".to_string(),
            Some(&group_clause.span.clone()),
        ));
    }

    for key in keys {
        match key {
            GroupKey::BindingVariable { name, span } => {
                let trimmed = name.trim();
                if trimmed.is_empty() {
                    return Err(validation_err("GROUP BY key cannot be empty", Some(span)));
                } else if !declared_variables.contains(trimmed) && !return_aliases.contains(trimmed) {
                    return Err(validation_err(
                        format!("Unknown variable '{}' in GROUP BY", trimmed),
                        Some(span),
                    ));
                }
            }
        }
    }
    Ok(())
}

/// Function to ensure non-aggregated RETURN variables are covered by GROUP BY.
pub(super) fn validate_non_aggregated_return_variables_in_group_by(
    return_clause: &ReturnClause,
    group_clause: &GroupClause,
) -> Result<(), GqlError> {
    let grouped_keys: HashSet<&str> = group_clause
        .keys
        .as_ref()
        .map(|keys| {
            keys.iter()
                .map(|key| match key {
                    GroupKey::BindingVariable { name, span: _ } => name.trim(),
                })
                .filter(|name| !name.is_empty())
                .collect::<HashSet<_>>()
        })
        .unwrap_or_default();

    let ReturnProjection::Items(items) = &return_clause.projection else {
        return Ok(());
    };

    for item in items {
        let mut non_aggregated_refs = HashSet::new();
        collect_non_aggregated_variable_refs(&item.expression, false, &mut non_aggregated_refs);

        if non_aggregated_refs.is_empty() {
            continue;
        }

        let alias_is_grouped = item
            .alias
            .as_ref()
            .map(|alias| grouped_keys.contains(alias.trim()))
            .unwrap_or(false);

        if alias_is_grouped {
            continue;
        }

        for var in non_aggregated_refs {
            if !grouped_keys.contains(var.as_str()) {
                return Err(validation_err(
                    format!("Non-aggregated variable '{}' in RETURN must appear in GROUP BY", var),
                    Some(&return_clause.span.clone()),
                ));
            }
        }
    }

    Ok(())
}

/// Helper function to collect variable references that are outside aggregate function boundaries.
pub(super) fn collect_non_aggregated_variable_refs(
    expression: &Expression,
    inside_function: bool,
    refs: &mut HashSet<String>,
) {
    match expression {
        Expression::Binary(binary) => {
            collect_non_aggregated_variable_refs(&binary.left, inside_function, refs);
            collect_non_aggregated_variable_refs(&binary.right, inside_function, refs);
        }
        Expression::Unary(unary) => {
            collect_non_aggregated_variable_refs(&unary.expression, inside_function, refs);
        }
        Expression::Variable { name, span: _ } => {
            if !inside_function {
                let trimmed = name.trim();
                if !trimmed.is_empty() {
                    refs.insert(trimmed.to_string());
                }
            }
        }
        Expression::PropertyAccess(property_access) => {
            if !inside_function {
                let obj = property_access.object.trim();
                if !obj.is_empty() {
                    refs.insert(obj.to_string());
                }
            }
        }
        Expression::FunctionCall(_) => {}
        Expression::IsPredicate(predicate) => {
            collect_non_aggregated_variable_refs(&predicate.expression, inside_function, refs);
        }
        Expression::Array(array) => {
            for item in &array.elements {
                collect_non_aggregated_variable_refs(item, inside_function, refs);
            }
        }
        Expression::Case(case_expression) => {
            collect_non_aggregated_variable_refs_in_case(case_expression, inside_function, refs);
        }
        Expression::Cast(cast_expression) => {
            collect_non_aggregated_variable_refs(&cast_expression.expression, inside_function, refs);
        }
        Expression::InlineLet(inline_let) => {
            for def in &inline_let.variable_definitions {
                collect_non_aggregated_variable_refs(&def.expression, inside_function, refs);
            }
            collect_non_aggregated_variable_refs(&inline_let.evaluation_expression, inside_function, refs);
        }
        Expression::Subquery(_) => {}
        Expression::Literal(_) => {}
        Expression::PathConstructor(elements) => {
            for expr in elements {
                collect_non_aggregated_variable_refs(expr, inside_function, refs);
            }
        }
        Expression::WhenOperand(operand) => {
            if let WhenOperandKind::Comparison { right, .. } = &operand.kind {
                collect_non_aggregated_variable_refs(right, inside_function, refs);
            }
        }
        Expression::Map(property_items) => {
            for property in property_items {
                collect_non_aggregated_variable_refs(&property.value, inside_function, refs);
            }
        }
    }
}

/// Function to validate ORDER BY items against declared variables and return aliases.
pub(super) fn validate_order_clause(
    order_clause: &OrderClause,
    declared_variables: &HashSet<String>,
    return_aliases: Option<&HashSet<String>>,
) -> Result<(), GqlError> {
    if order_clause.items.is_empty() {
        return Err(validation_err(
            "ORDER BY must contain at least one sort item".to_string(),
            Some(&order_clause.span.clone()),
        ));
    }

    for item in &order_clause.items {
        if let Expression::Variable { name, span: _ } = &item.expression {
            let var_name = name.trim();
            if declared_variables.contains(var_name)
                || return_aliases
                    .map(|aliases| aliases.contains(var_name))
                    .unwrap_or(false)
            {
                continue;
            }
        }

        validate_expression(&item.expression, declared_variables)?;

        let inferred = HashMap::new();
        if let Some(order_type) = infer_expression_type(&item.expression, &inferred, Some(declared_variables)) {
            let sortable = is_numeric_type(&order_type)
                || is_string_like_type(&order_type)
                || is_temporal_like_type(&order_type)
                || is_boolean_type(&order_type)
                || matches!(order_type, TypeSpec::Any);
            if !sortable {
                return Err(validation_err(
                    format!("ORDER BY expression has non-sortable type {}", type_name(&order_type)),
                    Some(&item.span.clone()),
                ));
            }
        }
    }
    Ok(())
}

/// Function to validate LIMIT/OFFSET presence and internal page values.
pub(super) fn validate_limit_clause(limit_clause: &LimitClause) -> Result<(), GqlError> {
    if limit_clause.count.is_none() && limit_clause.offset.is_none() {
        return Err(validation_err(
            "LIMIT/OFFSET clause cannot be empty".to_string(),
            Some(&limit_clause.span.clone()),
        ));
    }

    if let Some(count) = &limit_clause.count {
        validate_page_value("LIMIT", count)?;
    }

    if let Some(offset) = &limit_clause.offset {
        validate_page_value("OFFSET/SKIP", offset)?;
    }
    Ok(())
}

/// Function to validate a page value used in LIMIT/OFFSET clauses.
pub(super) fn validate_page_value(name: &str, value: &PageValue) -> Result<(), GqlError> {
    match value {
        PageValue::Number { value: _, span: _ } => {}
        PageValue::Parameter { name: param, span } => {
            let p = param.trim();
            if p.is_empty() {
                return Err(validation_err(
                    format!("{} parameter cannot be empty", name),
                    Some(span),
                ));
            } else if !p.starts_with('$') {
                return Err(validation_err(
                    format!("{} parameter must start with '$'", name),
                    Some(span),
                ));
            }
        }
    }
    Ok(())
}

/// Function to validate CASE branches and optional ELSE expressions.
pub(super) fn validate_case_expression(
    case_expression: &CaseExpression,
    declared_variables: &HashSet<String>,
) -> Result<(), GqlError> {
    let inferred = HashMap::new();

    match &case_expression.case_type {
        CaseType::Simple(simple_case) => {
            validate_expression(&simple_case.test_expression, declared_variables)?;
            let test_type = infer_expression_type(&simple_case.test_expression, &inferred, Some(declared_variables));

            for branch in &simple_case.when_branches {
                for when_value in &branch.when_values {
                    validate_expression(when_value, declared_variables)?;
                    if let (Some(t), Some(w)) = (
                        test_type.clone(),
                        infer_expression_type(when_value, &inferred, Some(declared_variables)),
                    ) && !are_comparison_types_compatible(&t, &w)
                    {
                        return Err(validation_err(
                            format!(
                                "CASE simple WHEN value type {} is incompatible with test expression type {}",
                                type_name(&w),
                                type_name(&t)
                            ),
                            Some(&branch.span.clone()),
                        ));
                    }
                }
                validate_expression(&branch.then_expression, declared_variables)?;
            }

            if let Some(else_expr) = &simple_case.else_expression {
                validate_expression(else_expr, declared_variables)?;
            }
        }
        CaseType::Searched(searched_case) => {
            for branch in &searched_case.when_branches {
                validate_expression(&branch.condition, declared_variables)?;
                validate_predicate_condition_type(&branch.condition, declared_variables, "CASE searched WHEN")?;
                validate_expression(&branch.then_expression, declared_variables)?;
            }

            if let Some(else_expr) = &searched_case.else_expression {
                validate_expression(else_expr, declared_variables)?;
            }
        }
    }

    Ok(())
}

/// Function to validate that a predicate condition expression evaluates to a BOOLEAN type or compatible type, and to provide context-specific error messages if the condition is invalid. This function is used to ensure that conditions in WHERE/FILTER and HAVING clauses are semantically correct and will not lead to runtime errors due to type incompatibilities.
pub(super) fn validate_predicate_condition_type(
    expression: &Expression,
    declared_variables: &HashSet<String>,
    context_name: &str,
) -> Result<(), GqlError> {
    let inferred = HashMap::new();
    let Some(cond_type) = infer_expression_type(expression, &inferred, Some(declared_variables)) else {
        return Ok(());
    };

    if !is_boolean_type(&cond_type) && !matches!(cond_type, TypeSpec::Any | TypeSpec::BindingTable) {
        return Err(validation_err(
            format!(
                "{} requires a BOOLEAN condition, got {}",
                context_name,
                type_name(&cond_type)
            ),
            None,
        ));
    }

    Ok(())
}

/// Helper function to collect non-aggregated variable references from CASE expression trees.
pub(super) fn collect_non_aggregated_variable_refs_in_case(
    case_expression: &CaseExpression,
    inside_function: bool,
    refs: &mut HashSet<String>,
) {
    match &case_expression.case_type {
        CaseType::Simple(simple_case) => {
            collect_non_aggregated_variable_refs(&simple_case.test_expression, inside_function, refs);

            for branch in &simple_case.when_branches {
                for when_value in &branch.when_values {
                    collect_non_aggregated_variable_refs(when_value, inside_function, refs);
                }
                collect_non_aggregated_variable_refs(&branch.then_expression, inside_function, refs);
            }

            if let Some(else_expr) = &simple_case.else_expression {
                collect_non_aggregated_variable_refs(else_expr, inside_function, refs);
            }
        }
        CaseType::Searched(searched_case) => {
            for branch in &searched_case.when_branches {
                collect_non_aggregated_variable_refs(&branch.condition, inside_function, refs);
                collect_non_aggregated_variable_refs(&branch.then_expression, inside_function, refs);
            }

            if let Some(else_expr) = &searched_case.else_expression {
                collect_non_aggregated_variable_refs(else_expr, inside_function, refs);
            }
        }
    }
}
