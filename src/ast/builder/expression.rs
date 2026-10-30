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

impl GQLAstBuilder {
    /// Function to build an Expression from a ValueExpressionContext, handling different types of expressions (unary, binary, function calls, predicates) and extracting the relevant components from the CST to construct the corresponding AST nodes for each type of expression. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for each type of expression. --- IGNORE ---
    pub(super) fn build_expression_from_context(
        value_expression: &ValueExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(value_expression);

        match value_expression {
            ValueExpressionContextAll::SignedExprAltContext(signed) => {
                let inner = signed.valueExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: operand missing in signed expression".into())
                })?;

                // The magnitude of i64::MIN is not a valid positive i64 literal.
                if signed.MINUS_SIGN().is_some()
                    && inner.get_text().trim_start_matches('0') == "9223372036854775808"
                {
                    return Ok(Expression::Literal(Literal::Integer(i64::MIN)));
                }
                let expression = Self::build_expression_from_context(&inner)?;
                let operator = if signed.MINUS_SIGN().is_some() {
                    Operator::Minus
                } else {
                    Operator::Plus
                };

                Ok(Expression::Unary(UnaryExpression {
                    operator,
                    expression: Box::new(expression),
                    span: span.clone(),
                }))
            }
            ValueExpressionContextAll::ConjunctiveExprAltContext(conjunctive) => {
                let expressions = conjunctive
                    .valueExpression_all()
                    .into_iter()
                    .map(|expr| Self::build_expression_from_context(&expr))
                    .collect::<GqlResult<Vec<_>>>()?;

                if expressions.len() < 2 {
                    return Self::parse_error(
                        "CST not valid: AND expression with less than 2 operands",
                    );
                }

                let mut result = expressions[0].clone();
                for expr in expressions.iter().skip(1) {
                    result = Expression::Binary(BinaryExpression {
                        left: Box::new(result),
                        operator: Operator::And,
                        right: Box::new(expr.clone()),
                        span: span.clone(),
                    });
                }
                Ok(result)
            }
            ValueExpressionContextAll::DisjunctiveExprAltContext(disjunctive) => {
                let expressions = disjunctive
                    .valueExpression_all()
                    .into_iter()
                    .map(|expr| Self::build_expression_from_context(&expr))
                    .collect::<GqlResult<Vec<_>>>()?;

                if expressions.len() < 2 {
                    return Self::parse_error(
                        "CST non valido: espressione OR con meno di 2 operandi",
                    );
                }

                let operator = if disjunctive.XOR().is_some() {
                    Operator::Xor
                } else {
                    Operator::Or
                };

                let mut result = expressions[0].clone();
                for expr in expressions.iter().skip(1) {
                    result = Expression::Binary(BinaryExpression {
                        left: Box::new(result),
                        operator: operator.clone(),
                        right: Box::new(expr.clone()),
                        span: span.clone(),
                    });
                }
                Ok(result)
            }
            ValueExpressionContextAll::NotExprAltContext(not_expr) => {
                let inner = not_expr.valueExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: operand missing in NOT expression".into())
                })?;

                let expression = Self::build_expression_from_context(&inner)?;
                Ok(Expression::Unary(UnaryExpression {
                    operator: Operator::Not,
                    expression: Box::new(expression),
                    span,
                }))
            }
            ValueExpressionContextAll::MultDivExprAltContext(mult_div) => {
                let left = mult_div
                    .valueExpression(0)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: left operand missing in multiplicative expression"
                                .into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;

                let right = mult_div
                    .valueExpression(1)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: right operand missing in multiplicative expression"
                                .into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;

                let operator = if mult_div.ASTERISK().is_some() {
                    Operator::Star
                } else {
                    Operator::Slash
                };

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    span: span.clone(),
                }))
            }

            ValueExpressionContextAll::AddSubtractExprAltContext(add_sub) => {
                let left = add_sub
                    .valueExpression(0)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: left operand missing in additive expression".into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;

                let right = add_sub
                    .valueExpression(1)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: right operand missing in additive expression".into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;

                let operator = if add_sub.PLUS_SIGN().is_some() {
                    Operator::Plus
                } else {
                    Operator::Minus
                };

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    span: span.clone(),
                }))
            }
            ValueExpressionContextAll::ConcatenationExprAltContext(concat) => {
                let expressions = concat
                    .valueExpression_all()
                    .into_iter()
                    .map(|expr| Self::build_expression_from_context(&expr))
                    .collect::<GqlResult<Vec<_>>>()?;

                if expressions.len() < 2 {
                    return Err(Self::parse_err(
                        "CST not valid: concatenation with less than 2 operands".to_string(),
                    ));
                }

                let mut result = expressions[0].clone();
                for expr in expressions.iter().skip(1) {
                    result = Expression::Binary(BinaryExpression {
                        left: Box::new(result),
                        operator: Operator::Concat,
                        right: Box::new(expr.clone()),
                        span: span.clone(),
                    });
                }
                Ok(result)
            }
            ValueExpressionContextAll::ComparisonExprAltContext(comparison) => {
                let left = comparison
                    .valueExpression(0)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: left operand missing in comparison expression".into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;
                let right = comparison
                    .valueExpression(1)
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: right operand missing in comparison expression".into(),
                        )
                    })
                    .and_then(|expression| Self::build_expression_from_context(&expression))?;
                let operator = comparison
                    .compOp()
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: compOp missing in comparison expression".into(),
                        )
                    })
                    .and_then(|comp_op| Self::operator_from_context(&comp_op))?;

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    operator,
                    right: Box::new(right),
                    span: span.clone(),
                }))
            }
            ValueExpressionContextAll::PrimaryExprAltContext(primary) => primary
                .valueExpressionPrimary()
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: valueExpressionPrimary missing in primary expression"
                            .into(),
                    )
                })
                .and_then(|value_expression_primary| {
                    Self::build_primary_expression_from_context(&value_expression_primary)
                }),
            ValueExpressionContextAll::IsNotExprAltContext(is_not) => {
                let inner = is_not.valueExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: operand missing in IS predicate".into())
                })?;
                let expression = Self::build_expression_from_context(&inner)?;

                let negated = is_not.NOT().is_some();

                let predicate = is_not
                    .truthValue()
                    .and_then(|tv| tv.BOOLEAN_LITERAL())
                    .map(|lit| {
                        if lit.get_text().eq_ignore_ascii_case("unknown") {
                            IsPredicateKind::Unknown
                        } else if lit.get_text().eq_ignore_ascii_case("true") {
                            IsPredicateKind::True
                        } else {
                            IsPredicateKind::False
                        }
                    })
                    .unwrap_or(IsPredicateKind::Unknown);

                Ok(Expression::IsPredicate(IsPredicateExpression {
                    expression: Box::new(expression),
                    negated,
                    predicate,
                    span: span.clone(),
                }))
            }
            ValueExpressionContextAll::NormalizedPredicateExprAltContext(normalized_expr) => {
                let inner = normalized_expr.valueExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: operand missing in normalized predicate".into())
                })?;

                let expression = Self::build_expression_from_context(&inner)?;
                let predicate_part =
                    normalized_expr.normalizedPredicatePart2().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: normalizedPredicatePart2 missing in normalized predicate"
                                .into(),
                        )
                    })?;

                let mut arguments = vec![expression];
                if let Some(form) = predicate_part.normalForm() {
                    arguments.push(Expression::Literal(Literal::String(form.get_text())));
                }

                let base_expr = Expression::FunctionCall(FunctionCall {
                    name: fn_names::IS_NORMALIZED.to_string(),
                    distinct: DistinctQualifier::None,
                    arguments,
                    span: span.clone(),
                });

                if predicate_part.NOT().is_some() {
                    Ok(Expression::Unary(UnaryExpression {
                        operator: Operator::Not,
                        expression: Box::new(base_expr),
                        span: span.clone(),
                    }))
                } else {
                    Ok(base_expr)
                }
            }
            ValueExpressionContextAll::PredicateExprAltContext(predicate_expr) => {
                let predicate_ctx = predicate_expr.predicate().ok_or_else(|| {
                    Self::parse_err("CST not valid: predicate missing in PredicateExprAlt".into())
                })?;

                if let Some(exists_predicate) = predicate_ctx.existsPredicate() {
                    Self::build_exists_subquery_expression_from_context(&exists_predicate)
                } else if let Some(property_exists_predicate) =
                    predicate_ctx.property_existsPredicate()
                {
                    let variable = property_exists_predicate
                        .elementVariableReference()
                        .map(|ctx| Self::identifier_text(&ctx.get_text()))
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: elementVariableReference missing in PROPERTY_EXISTS"
                                    .into(),
                            )
                        })?;

                    let property = property_exists_predicate
                        .propertyName()
                        .map(|ctx| Self::identifier_text(&ctx.get_text()))
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: propertyName missing in PROPERTY_EXISTS".into(),
                            )
                        })?;

                    Ok(Expression::FunctionCall(FunctionCall {
                        name: fn_names::PROPERTY_EXISTS.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments: vec![
                            Expression::Variable {
                                name: variable,
                                span: span.clone(),
                            },
                            Expression::Literal(Literal::String(property)),
                        ],
                        span: span.clone(),
                    }))
                } else if let Some(null_predicate) = predicate_ctx.nullPredicate() {
                    let primary = null_predicate.valueExpressionPrimary().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: valueExpressionPrimary missing in nullPredicate".into(),
                        )
                    })?;
                    let expression = Self::build_primary_expression_from_context(&primary)?;

                    let part2 = null_predicate.nullPredicatePart2().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: nullPredicatePart2 missing in nullPredicate".into(),
                        )
                    })?;
                    let negated = part2.NOT().is_some();

                    Ok(Expression::IsPredicate(IsPredicateExpression {
                        expression: Box::new(expression),
                        negated,
                        predicate: IsPredicateKind::Null,
                        span: span.clone(),
                    }))
                } else if let Some(value_type_predicate) = predicate_ctx.valueTypePredicate() {
                    let primary =
                        value_type_predicate
                            .valueExpressionPrimary()
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: valueExpressionPrimary missing in valueTypePredicate"
                                        .into(),
                                )
                            })?;
                    let expression = Self::build_primary_expression_from_context(&primary)?;

                    let part2 =
                        value_type_predicate
                            .valueTypePredicatePart2()
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: valueTypePredicatePart2 missing in valueTypePredicate"
                                        .into(),
                                )
                            })?;

                    let value_type = part2.valueType().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: valueType missing in valueTypePredicatePart2".into(),
                        )
                    })?;

                    let type_spec = Self::build_type_spec_from_value_type(&value_type)?;
                    let negated = part2.NOT().is_some();

                    Ok(Expression::IsPredicate(IsPredicateExpression {
                        expression: Box::new(expression),
                        negated,
                        predicate: IsPredicateKind::Typed(type_spec),
                        span: span.clone(),
                    }))
                } else if let Some(labeled_predicate) = predicate_ctx.labeledPredicate() {
                    let variable = labeled_predicate
                        .elementVariableReference()
                        .map(|ctx| ctx.get_text())
                        .ok_or_else(|| {
                            Self::parse_err("CST not valid: elementVariableReference missing in labeledPredicate".into())
                        })?;

                    let part2 = labeled_predicate.labeledPredicatePart2().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: labeledPredicatePart2 missing in labeledPredicate"
                                .into(),
                        )
                    })?;

                    let label_ctx = part2.labelExpression().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: labelExpression missing in labeledPredicatePart2"
                                .into(),
                        )
                    })?;
                    let label_expression = Self::build_label_expression_from_context(&label_ctx)?;

                    let base_expr = Expression::FunctionCall(FunctionCall {
                        name: fn_names::IS_LABELED.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments: vec![
                            Expression::Variable {
                                name: variable,
                                span: span.clone(),
                            },
                            label_expression,
                        ],
                        span: span.clone(),
                    });

                    let negated = part2.isLabeledOrColon().and_then(|ctx| ctx.NOT()).is_some();

                    if negated {
                        Ok(Expression::Unary(UnaryExpression {
                            operator: Operator::Not,
                            expression: Box::new(base_expr),
                            span: span.clone(),
                        }))
                    } else {
                        Ok(base_expr)
                    }
                } else if let Some(directed_predicate) = predicate_ctx.directedPredicate() {
                    let variable = directed_predicate
                        .elementVariableReference()
                        .map(|ctx| ctx.get_text())
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: elementVariableReference missing in directedPredicate"
                                    .into(),
                            )
                        })?;

                    let negated = directed_predicate
                        .directedPredicatePart2()
                        .and_then(|ctx| ctx.NOT())
                        .is_some();

                    let base_expr = Expression::FunctionCall(FunctionCall {
                        name: fn_names::IS_DIRECTED.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments: vec![Expression::Variable {
                            name: variable,
                            span: span.clone(),
                        }],
                        span: span.clone(),
                    });

                    if negated {
                        Ok(Expression::Unary(UnaryExpression {
                            operator: Operator::Not,
                            expression: Box::new(base_expr),
                            span: span.clone(),
                        }))
                    } else {
                        Ok(base_expr)
                    }
                } else if let Some(source_dest_predicate) =
                    predicate_ctx.sourceDestinationPredicate()
                {
                    let node_variable = source_dest_predicate
                        .nodeReference()
                        .and_then(|ctx| ctx.elementVariableReference())
                        .map(|ctx| ctx.get_text())
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: node reference missing in source/destination predicate"
                                    .into(),
                            )
                        })?;

                    if let Some(source_part) = source_dest_predicate.sourcePredicatePart2() {
                        let edge_variable = source_part
                            .edgeReference()
                            .and_then(|ctx| ctx.elementVariableReference())
                            .map(|ctx| ctx.get_text())
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: edge reference missing in sourcePredicatePart2"
                                        .into(),
                                )
                            })?;

                        let base_expr = Expression::FunctionCall(FunctionCall {
                            name: fn_names::IS_SOURCE_OF.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![
                                Expression::Variable {
                                    name: node_variable,
                                    span: span.clone(),
                                },
                                Expression::Variable {
                                    name: edge_variable,
                                    span: span.clone(),
                                },
                            ],
                            span: span.clone(),
                        });

                        if source_part.NOT().is_some() {
                            Ok(Expression::Unary(UnaryExpression {
                                operator: Operator::Not,
                                expression: Box::new(base_expr),
                                span: span.clone(),
                            }))
                        } else {
                            Ok(base_expr)
                        }
                    } else if let Some(destination_part) =
                        source_dest_predicate.destinationPredicatePart2()
                    {
                        let edge_variable = destination_part
                            .edgeReference()
                            .and_then(|ctx| ctx.elementVariableReference())
                            .map(|ctx| ctx.get_text())
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: edge reference missing in destinationPredicatePart2"
                                        .into(),
                                )
                            })?;

                        let base_expr = Expression::FunctionCall(FunctionCall {
                            name: fn_names::IS_DESTINATION_OF.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![
                                Expression::Variable {
                                    name: node_variable,
                                    span: span.clone(),
                                },
                                Expression::Variable {
                                    name: edge_variable,
                                    span: span.clone(),
                                },
                            ],
                            span: span.clone(),
                        });

                        if destination_part.NOT().is_some() {
                            Ok(Expression::Unary(UnaryExpression {
                                operator: Operator::Not,
                                expression: Box::new(base_expr),
                                span: span.clone(),
                            }))
                        } else {
                            Ok(base_expr)
                        }
                    } else {
                        Self::parse_error("CST not valid: unsupported source/destination predicate")
                    }
                } else if let Some(all_different_predicate) = predicate_ctx.all_differentPredicate()
                {
                    let args = all_different_predicate
                        .elementVariableReference_all()
                        .into_iter()
                        .map(|ctx| Expression::Variable {
                            name: ctx.get_text(),
                            span: span.clone(),
                        })
                        .collect::<Vec<_>>();

                    Ok(Expression::FunctionCall(FunctionCall {
                        name: fn_names::ALL_DIFFERENT.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments: args,
                        span: span.clone(),
                    }))
                } else if let Some(same_predicate) = predicate_ctx.samePredicate() {
                    let args = same_predicate
                        .elementVariableReference_all()
                        .into_iter()
                        .map(|ctx| Expression::Variable {
                            name: ctx.get_text(),
                            span: span.clone(),
                        })
                        .collect::<Vec<_>>();

                    Ok(Expression::FunctionCall(FunctionCall {
                        name: fn_names::SAME.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments: args,
                        span: span.clone(),
                    }))
                } else {
                    Self::parse_error(format!(
                        "The builder encountered a GQL Predicate it doesn't know how to map: {}",
                        predicate_ctx.get_text()
                    ))
                }
            }
            ValueExpressionContextAll::BindingTableExprAltContext(binding_table_expr) => {
                let binding_table =
                    binding_table_expr.bindingTableExpression().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: bindingTableExpression missing in BindingTableExprAlt"
                                .into(),
                        )
                    })?;

                Self::build_binding_table_expression_from_context(&binding_table)
            }
            ValueExpressionContextAll::PropertyGraphExprAltContext(property_graph_expr) => {
                let graph_expression = property_graph_expr.graphExpression().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: graphExpression missing in PropertyGraphExprAlt".into(),
                    )
                })?;

                Self::build_expression_from_leaf_text(&graph_expression.get_text(), span.clone())
            }
            ValueExpressionContextAll::ValueFunctionExprAltContext(value_fn_expr) => {
                let value_fn = value_fn_expr.valueFunction().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: valueFunction missing in ValueFunctionExprAlt".into(),
                    )
                })?;

                if let Some(char_fn) = value_fn.characterOrByteStringFunction() {
                    if let Some(fold) = char_fn.foldCharacterString() {
                        let name = if fold.UPPER().is_some() {
                            "UPPER"
                        } else {
                            "LOWER"
                        };
                        let inner = fold.valueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: valueExpression missing in foldCharacterString"
                                    .into(),
                            )
                        })?;
                        let arg = Self::build_expression_from_context(&inner)?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: name.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(sub_fn) = char_fn.subCharacterOrByteString() {
                        return Self::build_sub_character_function_from_context(&sub_fn);
                    }
                    if let Some(trim_fn) = char_fn.trimSingleCharacterOrByteString() {
                        return Self::build_trim_function_from_context(&trim_fn);
                    }

                    if let Some(trim_multi_fn) = char_fn.trimMultiCharacterCharacterString() {
                        let name = if trim_multi_fn.LTRIM().is_some() {
                            "LTRIM"
                        } else if trim_multi_fn.RTRIM().is_some() {
                            "RTRIM"
                        } else {
                            "BTRIM"
                        };

                        let source_ctx = trim_multi_fn.valueExpression(0).ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: source expression missing in trimMultiCharacterCharacterString"
                                    .into(),
                            )
                        })?;

                        let mut arguments = vec![Self::build_expression_from_context(&source_ctx)?];
                        if let Some(trim_chars_ctx) = trim_multi_fn.valueExpression(1) {
                            arguments.push(Self::build_expression_from_context(&trim_chars_ctx)?);
                        }

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: name.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments,
                            span: span.clone(),
                        }));
                    }

                    if let Some(normalize_fn) = char_fn.normalizeCharacterString() {
                        let source_ctx = normalize_fn.valueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: valueExpression missing in NORMALIZE".into(),
                            )
                        })?;

                        let mut arguments = vec![Self::build_expression_from_context(&source_ctx)?];
                        if let Some(form) = normalize_fn.normalForm() {
                            arguments.push(Expression::Literal(Literal::String(form.get_text())));
                        }

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::NORMALIZE.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments,
                            span: span.clone(),
                        }));
                    }
                }

                if let Some(list_fn) = value_fn.listValueFunction() {
                    if let Some(trim_list_fn) = list_fn.trimListFunction() {
                        let list_expr_ctx =
                            trim_list_fn.listValueExpression().ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: listValueExpression missing in TRIM(list)"
                                        .into(),
                                )
                            })?;
                        let trim_count_ctx =
                            trim_list_fn.numericValueExpression().ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: numericValueExpression missing in TRIM(list)"
                                        .into(),
                                )
                            })?;

                        let list_expr = Self::build_expression_from_context(
                            &list_expr_ctx
                                .valueExpression()
                                .ok_or_else(|| {
                                    Self::parse_err(
                                        "CST not valid: valueExpression missing in listValueExpression"
                                            .into(),
                                    )
                                })?
                                .as_ref(),
                        )?;
                        let trim_count = Self::build_expression_from_leaf_text(
                            &trim_count_ctx.get_text(),
                            span.clone(),
                        )?;

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: "TRIM".to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![list_expr, trim_count],
                            span: span.clone(),
                        }));
                    }

                    if let Some(elements_fn) = list_fn.elementsFunction() {
                        let path_expr = elements_fn.pathValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: pathValueExpression missing in ELEMENTS".into(),
                            )
                        })?;

                        let arg = Self::build_expression_from_leaf_text(
                            &path_expr.get_text(),
                            span.clone(),
                        )?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::ELEMENTS.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }
                }

                if let Some(numeric_fn) = value_fn.numericValueFunction() {
                    if let Some(cardinality_expr) = numeric_fn.cardinalityExpression() {
                        let name = if cardinality_expr.SIZE().is_some() {
                            "SIZE"
                        } else {
                            "CARDINALITY"
                        };

                        let inner = if cardinality_expr.SIZE().is_some() {
                            cardinality_expr
                                .listValueExpression()
                                .and_then(|e| e.valueExpression())
                                .ok_or_else(|| {Self::parse_err("CST not valid: valueExpression missing in listValueExpression of cardinalityExpression".to_string())})?
                        } else {
                            cardinality_expr
                                .cardinalityExpressionArgument()
                                .and_then(|a| a.valueExpression())
                                .ok_or_else(|| {Self::parse_err("CST not valid: valueExpression missing in cardinalityExpressionArgument".to_string())})?
                        };

                        let arg = Self::build_expression_from_context(&inner)?;

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: name.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    } else if let Some(length_expr) = numeric_fn.lengthExpression() {
                        if let Some(char_len) = length_expr.charLengthExpression() {
                            let name = if char_len.CHAR_LENGTH().is_some() {
                                "CHAR_LENGTH"
                            } else {
                                "CHARACTER_LENGTH"
                            };
                            let inner = char_len
                                .characterStringValueExpression()
                                .and_then(|e| e.valueExpression())
                                .ok_or_else(|| {
                                    Self::parse_err("CST not valid: valueExpression missing in charLengthExpression".into())
                                })?;
                            let arg = Self::build_expression_from_context(&inner)?;
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: name.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![arg],
                                span: span.clone(),
                            }));
                        } else if let Some(byte_len) = length_expr.byteLengthExpression() {
                            let name = if byte_len.BYTE_LENGTH().is_some() {
                                "BYTE_LENGTH"
                            } else {
                                "OCTET_LENGTH"
                            };

                            let inner = byte_len
                                .byteStringValueExpression()
                                .and_then(|e| e.valueExpression())
                                .ok_or_else(|| {
                                    Self::parse_err(
                                        "CST not valid: valueExpression missing in byteLengthExpression"
                                            .into(),
                                    )
                                })?;

                            let arg = Self::build_expression_from_context(&inner)?;
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: name.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![arg],
                                span: span.clone(),
                            }));
                        } else if let Some(path_len) = length_expr.pathLengthExpression() {
                            let inner = path_len
                                .pathValueExpression()
                                .and_then(|e| e.valueExpression())
                                .ok_or_else(|| {
                                    Self::parse_err(
                                        "CST not valid: valueExpression missing in pathLengthExpression"
                                            .into(),
                                    )
                                })?;

                            let arg = Self::build_expression_from_context(&inner)?;
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::PATH_LENGTH.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![arg],
                                span: span.clone(),
                            }));
                        } else {
                            return Self::parse_error(format!(
                                "Numeric function variant not supported: {}",
                                numeric_fn.get_text()
                            ));
                        }
                    }

                    if let Some(abs_fn) = numeric_fn.absoluteValueExpression() {
                        let value_expr = abs_fn.valueExpression().ok_or_else(|| {
                            Self::parse_err("CST not valid: valueExpression missing in ABS".into())
                        })?;
                        let arg = Self::build_expression_from_context(&value_expr)?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::ABS.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(floor_fn) = numeric_fn.floorFunction() {
                        let nve = floor_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in FLOOR".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::FLOOR.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(ceil_fn) = numeric_fn.ceilingFunction() {
                        let nve = ceil_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in CEIL".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::CEIL.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(sqrt_fn) = numeric_fn.squareRoot() {
                        let nve = sqrt_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in SQRT".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::SQRT.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(power_fn) = numeric_fn.powerFunction() {
                        let base_nve = power_fn.numericValueExpressionBase().ok_or_else(|| {
                            Self::parse_err("CST not valid: base missing in POWER".into())
                        })?;
                        let base_expr = Self::build_expression_from_leaf_text(&base_nve.numericValueExpression()
                            .ok_or_else(|| Self::parse_err("CST not valid: numericValueExpression missing in POWER base".into()))?
                            .get_text(), span.clone())?;

                        let exp_nve =
                            power_fn.numericValueExpressionExponent().ok_or_else(|| {
                                Self::parse_err("CST not valid: exponent missing in POWER".into())
                            })?;
                        let exp_expr = Self::build_expression_from_leaf_text(&exp_nve.numericValueExpression()
                            .ok_or_else(|| Self::parse_err("CST not valid: numericValueExpression missing in POWER exponent".into()))?
                            .get_text(), span.clone())?;

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::POWER.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![base_expr, exp_expr],
                            span: span.clone(),
                        }));
                    }

                    if let Some(mod_fn) = numeric_fn.modulusExpression() {
                        let div_nve = mod_fn.numericValueExpressionDividend().ok_or_else(|| {
                            Self::parse_err("CST not valid: dividend missing in MOD".into())
                        })?;
                        let dividend_expr = Self::build_expression_from_leaf_text(&div_nve.numericValueExpression()
                            .ok_or_else(|| Self::parse_err("CST not valid: numericValueExpression missing in MOD dividend".into()))?
                            .get_text(), span.clone())?;

                        let divisor_nve =
                            mod_fn.numericValueExpressionDivisor().ok_or_else(|| {
                                Self::parse_err("CST not valid: divisor missing in MOD".into())
                            })?;
                        let divisor_expr = Self::build_expression_from_leaf_text(&divisor_nve.numericValueExpression()
                            .ok_or_else(|| Self::parse_err("CST not valid: numericValueExpression missing in MOD divisor".into()))?
                            .get_text(), span.clone())?;

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::MOD.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![dividend_expr, divisor_expr],
                            span: span.clone(),
                        }));
                    }

                    if let Some(trig_fn) = numeric_fn.trigonometricFunction() {
                        let name = trig_fn
                            .trigonometricFunctionName()
                            .map(|n| n.get_text().to_uppercase())
                            .unwrap_or_else(|| "TRIG".to_string());

                        let nve = trig_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in trigonometric function"
                                    .into(),
                            )
                        })?;

                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name,
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(log_fn) = numeric_fn.generalLogarithmFunction() {
                        let base_ctx = log_fn
                            .generalLogarithmBase()
                            .and_then(|b| b.numericValueExpression())
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: generalLogarithmBase missing in LOG".into(),
                                )
                            })?;
                        let arg_ctx = log_fn
                            .generalLogarithmArgument()
                            .and_then(|a| a.numericValueExpression())
                            .ok_or_else(|| {
                                Self::parse_err(
                                    "CST not valid: generalLogarithmArgument missing in LOG".into(),
                                )
                            })?;

                        let base = Self::build_expression_from_leaf_text(
                            &base_ctx.get_text(),
                            span.clone(),
                        )?;
                        let arg = Self::build_expression_from_leaf_text(
                            &arg_ctx.get_text(),
                            span.clone(),
                        )?;

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::LOG.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![base, arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(log10_fn) = numeric_fn.commonLogarithm() {
                        let nve = log10_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in LOG10".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::LOG10.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(ln_fn) = numeric_fn.naturalLogarithm() {
                        let nve = ln_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in LN".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::LN.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }

                    if let Some(exp_fn) = numeric_fn.exponentialFunction() {
                        let nve = exp_fn.numericValueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: numericValueExpression missing in EXP".into(),
                            )
                        })?;
                        let arg =
                            Self::build_expression_from_leaf_text(&nve.get_text(), span.clone())?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::EXP.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }
                }

                if let Some(dt_sub) = value_fn.datetimeSubtraction() {
                    let params = dt_sub.datetimeSubtractionParameters().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: datetimeSubtractionParameters missing".into(),
                        )
                    })?;

                    let left_ctx = params
                        .datetimeValueExpression1()
                        .and_then(|x| x.datetimeValueExpression())
                        .and_then(|x| x.valueExpression())
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: datetimeValueExpression1 missing in DURATION_BETWEEN"
                                    .into(),
                            )
                        })?;

                    let right_ctx = params
                        .datetimeValueExpression2()
                        .and_then(|x| x.datetimeValueExpression())
                        .and_then(|x| x.valueExpression())
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: datetimeValueExpression2 missing in DURATION_BETWEEN"
                                    .into(),
                            )
                        })?;

                    let mut arguments = vec![
                        Self::build_expression_from_context(&left_ctx)?,
                        Self::build_expression_from_context(&right_ctx)?,
                    ];

                    if let Some(qualifier) = dt_sub.temporalDurationQualifier() {
                        arguments.push(Expression::Literal(Literal::String(qualifier.get_text())));
                    }

                    return Ok(Expression::FunctionCall(FunctionCall {
                        name: fn_names::DURATION_BETWEEN.to_string(),
                        distinct: DistinctQualifier::None,
                        arguments,
                        span: span.clone(),
                    }));
                }

                if let Some(duration_fn) = value_fn.durationValueFunction() {
                    if let Some(duration_ctor) = duration_fn.durationFunction() {
                        let mut arguments = Vec::new();
                        if let Some(params) = duration_ctor.durationFunctionParameters() {
                            if let Some(duration_string) = params.durationString() {
                                arguments.push(Self::build_expression_from_leaf_text(
                                    &duration_string.get_text(),
                                    span.clone(),
                                )?);
                            } else if let Some(record) = params.recordConstructor() {
                                arguments.push(Self::build_expression_from_leaf_text(
                                    &record.get_text(),
                                    span.clone(),
                                )?);
                            }
                        }

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::DURATION.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments,
                            span: span.clone(),
                        }));
                    }

                    if let Some(abs_duration) = duration_fn.absoluteValueExpression() {
                        let value_expr = abs_duration.valueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: valueExpression missing in duration ABS".into(),
                            )
                        })?;
                        let arg = Self::build_expression_from_context(&value_expr)?;
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::ABS.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: vec![arg],
                            span: span.clone(),
                        }));
                    }
                }

                if let Some(dt_fn) = value_fn.datetimeValueFunction() {
                    if let Some(date_fn) = dt_fn.dateFunction() {
                        if date_fn.CURRENT_DATE().is_some() {
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::CURRENT_DATE.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![],
                                span: span.clone(),
                            }));
                        } else {
                            let mut args = Vec::new();
                            if let Some(params) = date_fn.dateFunctionParameters() {
                                if let Some(date_str) = params.dateString() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &date_str.get_text(),
                                        span.clone(),
                                    )?);
                                } else if let Some(_record) = params.recordConstructor() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &params.get_text(),
                                        span.clone(),
                                    )?);
                                }
                            }
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::DATE.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: args,
                                span: span.clone(),
                            }));
                        }
                    }

                    if let Some(time_fn) = dt_fn.timeFunction() {
                        if time_fn.CURRENT_TIME().is_some() {
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::CURRENT_TIME.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![],
                                span: span.clone(),
                            }));
                        } else {
                            let mut args = Vec::new();
                            if let Some(params) = time_fn.timeFunctionParameters() {
                                if let Some(time_str) = params.timeString() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &time_str.get_text(),
                                        span.clone(),
                                    )?);
                                } else if let Some(_record) = params.recordConstructor() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &params.get_text(),
                                        span.clone(),
                                    )?);
                                }
                            }
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::ZONED_TIME.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: args,
                                span: span.clone(),
                            }));
                        }
                    }

                    if let Some(local_time_fn) = dt_fn.localtimeFunction() {
                        let mut args = Vec::new();
                        if let Some(params) = local_time_fn.timeFunctionParameters() {
                            if let Some(time_str) = params.timeString() {
                                args.push(Self::build_expression_from_leaf_text(
                                    &time_str.get_text(),
                                    span.clone(),
                                )?);
                            } else if let Some(_record) = params.recordConstructor() {
                                args.push(Self::build_expression_from_leaf_text(
                                    &params.get_text(),
                                    span.clone(),
                                )?);
                            }
                        }
                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: fn_names::LOCAL_TIME.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: args,
                            span: span.clone(),
                        }));
                    }

                    if let Some(datetime_fn) = dt_fn.datetimeFunction() {
                        if datetime_fn.CURRENT_TIMESTAMP().is_some() {
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::CURRENT_TIMESTAMP.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: vec![],
                                span: span.clone(),
                            }));
                        } else {
                            let mut args = Vec::new();
                            if let Some(params) = datetime_fn.datetimeFunctionParameters() {
                                if let Some(dt_str) = params.datetimeString() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &dt_str.get_text(),
                                        span.clone(),
                                    )?);
                                } else if let Some(_record) = params.recordConstructor() {
                                    args.push(Self::build_expression_from_leaf_text(
                                        &params.get_text(),
                                        span.clone(),
                                    )?);
                                }
                            }
                            return Ok(Expression::FunctionCall(FunctionCall {
                                name: fn_names::ZONED_DATETIME.to_string(),
                                distinct: DistinctQualifier::None,
                                arguments: args,
                                span: span.clone(),
                            }));
                        }
                    }

                    if let Some(local_dt_fn) = dt_fn.localdatetimeFunction() {
                        let name = if local_dt_fn.LOCAL_TIMESTAMP().is_some() {
                            "LOCAL_TIMESTAMP"
                        } else if local_dt_fn.LOCAL_DATETIME().is_some() {
                            "LOCAL_DATETIME"
                        } else {
                            return Self::parse_error(
                                "LOCAL_TIMESTAMP/LOCAL_DATETIME parsing failed",
                            );
                        };

                        let mut args = Vec::new();
                        if let Some(params) = local_dt_fn.datetimeFunctionParameters() {
                            if let Some(dt_str) = params.datetimeString() {
                                args.push(Self::build_expression_from_leaf_text(
                                    &dt_str.get_text(),
                                    span.clone(),
                                )?);
                            } else if let Some(_record) = params.recordConstructor() {
                                args.push(Self::build_expression_from_leaf_text(
                                    &params.get_text(),
                                    span.clone(),
                                )?);
                            }
                        }

                        return Ok(Expression::FunctionCall(FunctionCall {
                            name: name.to_string(),
                            distinct: DistinctQualifier::None,
                            arguments: args,
                            span: span.clone(),
                        }));
                    }
                }

                if let Some(udf) = value_fn.userDefinedFunction() {
                    let mut name_parts: Vec<String> = udf
                        .objectName_all()
                        .into_iter()
                        .map(|obj| Self::identifier_text(&obj.get_text()))
                        .collect();

                    let fn_identifier = udf.identifier().ok_or_else(|| {
                        Self::parse_err("CST not valid: identifier missing in userDefinedFunction".into())
                    })?;
                    name_parts.push(Self::identifier_text(&fn_identifier.get_text()));

                    let arguments = udf
                        .valueExpression_all()
                        .into_iter()
                        .map(|expr| Self::build_expression_from_context(&expr))
                        .collect::<GqlResult<Vec<_>>>()?;

                    return Ok(Expression::FunctionCall(FunctionCall {
                        name: name_parts.join("."),
                        distinct: DistinctQualifier::None,
                        arguments,
                        span: span.clone(),
                    }));
                }

                return Self::parse_error(format!(
                    "Type of valueFunction not supported or not yet mapped: {}",
                    value_fn.get_text()
                ));
            }
            ValueExpressionContextAll::Error(_) => {
                Self::parse_error("CST Error node: the parser found a syntax error here")
            }
        }
    }

    /// Function to build a LEFT or RIGHT function call from a SubCharacterOrByteStringContext, extracting the source string and length arguments and building the corresponding FunctionCall expression.
    pub(super) fn build_sub_character_function_from_context(
        ctx: &SubCharacterOrByteStringContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(ctx);

        let name = if ctx.LEFT().is_some() {
            "LEFT"
        } else {
            "RIGHT"
        };
        let source_ctx = ctx.valueExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: valueExpression missing in LEFT/RIGHT".into())
        })?;
        let len_ctx = ctx.stringLength().ok_or_else(|| {
            Self::parse_err("CST not valid: stringLength missing in LEFT/RIGHT".into())
        })?;

        let source = Self::build_expression_from_context(&source_ctx)?;
        let len = Self::build_expression_from_string_length(&len_ctx)?;

        Ok(Expression::FunctionCall(FunctionCall {
            name: name.to_string(),
            distinct: DistinctQualifier::None,
            arguments: vec![source, len],
            span: span.clone(),
        }))
    }

    /// Function to build a TRIM function call from a TrimSingleCharacterOrByteStringContext, extracting the source string, trim character, and specification arguments and building the corresponding FunctionCall expression.
    pub(super) fn build_trim_function_from_context(
        ctx: &TrimSingleCharacterOrByteStringContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(ctx);

        let operands = ctx
            .trimOperands()
            .ok_or_else(|| Self::parse_err("CST not valid: trimOperands missing in TRIM".into()))?;

        let source_ctx = operands
            .trimCharacterOrByteStringSource()
            .and_then(|s| s.valueExpression())
            .ok_or_else(|| Self::parse_err("CST not valid: source missing in TRIM".into()))?;

        let source = Self::build_expression_from_context(&source_ctx)?;
        let mut arguments: Vec<Expression> = Vec::new();

        if let Some(spec) = operands.trimSpecification() {
            let spec_name = if spec.LEADING().is_some() {
                "LEADING"
            } else if spec.TRAILING().is_some() {
                "TRAILING"
            } else {
                "BOTH"
            };
            arguments.push(Expression::Literal(Literal::String(spec_name.to_string())));
        }

        if let Some(trim_char_ctx) = operands
            .trimCharacterOrByteString()
            .and_then(|c| c.valueExpression())
        {
            arguments.push(Self::build_expression_from_context(&trim_char_ctx)?);
        }

        arguments.push(source);

        Ok(Expression::FunctionCall(FunctionCall {
            name: fn_names::TRIM.to_string(),
            distinct: DistinctQualifier::None,
            arguments,
            span: span.clone(),
        }))
    }

    /// Function to build an expression for string length from a StringLengthContext, extracting the numeric value expression and building the corresponding Expression.
    pub(super) fn build_expression_from_string_length(
        ctx: &StringLengthContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(ctx);
        let numeric_ctx = ctx.numericValueExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: numericValueExpression missing in stringLength".into())
        })?;

        Self::build_expression_from_leaf_text(&numeric_ctx.get_text(), span.clone())
    }

    /// Function to build a primary expression from a ValueExpressionPrimaryContext, handling various cases such as property access, literals, variable references, function calls, and more complex expressions like CASE and CAST.
    pub(super) fn build_primary_expression_from_context(
        value_expression_primary: &ValueExpressionPrimaryContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(value_expression_primary);

        if value_expression_primary.propertyName().is_some() {
            return Ok(Expression::PropertyAccess(
                Self::build_property_access_from_context(value_expression_primary)?,
            ));
        }

        if let Some(unsigned_value_specification) =
            value_expression_primary.unsignedValueSpecification()
        {
            if let Some(unsigned_literal) = unsigned_value_specification.unsignedLiteral() {
                if let Some(general_literal) = unsigned_literal.generalLiteral() {
                    if let Some(list_literal) = general_literal.listLiteral() {
                        return Self::build_array_expression_from_list_literal(&list_literal);
                    }
                    if let Some(record_literal) = general_literal.recordLiteral() {
                        return Self::build_map_expression_from_record_literal(&record_literal);
                    }
                }
            }
            return Ok(Expression::Literal(Self::build_literal_from_context(
                &unsigned_value_specification,
            )?));
        }

        if let Some(binding_variable_reference) =
            value_expression_primary.bindingVariableReference()
        {
            let name = binding_variable_reference.get_text();

            if name.eq_ignore_ascii_case("true") {
                return Ok(Expression::Literal(Literal::Boolean(true)));
            }
            if name.eq_ignore_ascii_case("false") {
                return Ok(Expression::Literal(Literal::Boolean(false)));
            }
            if name.eq_ignore_ascii_case("null") {
                return Ok(Expression::Literal(Literal::Null));
            }

            return Ok(Expression::Variable { name, span });
        }

        if let Some(cast_spec) = value_expression_primary.castSpecification() {
            return Self::build_cast_expression_from_context(&cast_spec);
        }

        if let Some(case_expr) = value_expression_primary.caseExpression() {
            return Self::build_case_expression_from_context(&case_expr);
        }

        if let Some(aggregate_function) = value_expression_primary.aggregateFunction() {
            return Self::build_function_call_from_aggregate_context(&aggregate_function);
        }

        if let Some(paren) = value_expression_primary.parenthesizedValueExpression() {
            if let Some(inner) = paren.valueExpression() {
                return Self::build_expression_from_context(&inner);
            }
        }
        if let Some(case_expr) = value_expression_primary.caseExpression() {
            if let Some(abbreviation) = case_expr.caseAbbreviation() {
                let name = if abbreviation.COALESCE().is_some() {
                    "COALESCE"
                } else {
                    "NULLIF"
                };
                let arguments = abbreviation
                    .valueExpression_all()
                    .into_iter()
                    .map(|e| Self::build_expression_from_context(&e))
                    .collect::<GqlResult<Vec<_>>>()?;
                return Ok(Expression::FunctionCall(FunctionCall {
                    name: name.to_string(),
                    distinct: DistinctQualifier::None,
                    arguments,
                    span: span.clone(),
                }));
            }
        }

        if let Some(element_id_fn) = value_expression_primary.element_idFunction() {
            let element_ref = element_id_fn.elementVariableReference().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: elementVariableReference missing in ELEMENT_ID".into(),
                )
            })?;

            return Ok(Expression::FunctionCall(FunctionCall {
                name: fn_names::ELEMENT_ID.to_string(),
                distinct: DistinctQualifier::None,
                arguments: vec![Expression::Variable {
                    name: element_ref.get_text(),
                    span: span.clone(),
                }],
                span: span.clone(),
            }));
        }
        if let Some(let_expr) = value_expression_primary.letValueExpression() {
            let list = let_expr.letVariableDefinitionList().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: missing letVariableDefinitionList in LET ... IN".into(),
                )
            })?;

            let variable_definitions = list
                .letVariableDefinition_all()
                .into_iter()
                .map(|def| Self::build_let_variable_definition_from_context(&def))
                .collect::<GqlResult<Vec<_>>>()?;

            let in_expr = let_expr.valueExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: missing valueExpression in LET ... IN".into())
            })?;

            let evaluation_expression = Box::new(Self::build_expression_from_context(&in_expr)?);

            return Ok(Expression::InlineLet(InlineLetExpression {
                variable_definitions,
                evaluation_expression,
                span: span.clone(),
            }));
        }

        if let Some(value_query_expr) = value_expression_primary.valueQueryExpression() {
            let nested = value_query_expr.nestedQuerySpecification().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: nestedQuerySpecification missing in VALUE query expression"
                        .into(),
                )
            })?;

            let query = Self::build_query_from_nested_query_spec(&nested)?;
            return Ok(Expression::Subquery(SubqueryExpression {
                query: Box::new(query),
                expression: None,
                is_exists: false,
                negated: false,
                span: span.clone(),
            }));
        }

        if let Some(path_constructor) = value_expression_primary.pathValueConstructor() {
            return Self::build_path_constructor_expression_from_context(&path_constructor);
        }

        let text_upper = value_expression_primary.get_text().to_uppercase();
        if text_upper.starts_with(kw::VALUE) || text_upper.starts_with(kw::TABLE) {
            return Self::build_expression_from_leaf_text(
                &value_expression_primary.get_text(),
                span.clone(),
            );
        }

        Self::parse_error(format!(
            "Expression primary variant not handled by builder: {}",
            value_expression_primary.get_text()
        ))
    }

    /// Function to build a CastExpression from a CastSpecificationContext, extracting the operand expression and target type specification and building the corresponding CastExpression.
    pub(super) fn build_cast_expression_from_context(
        cast_spec: &CastSpecificationContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(cast_spec);

        let operand_ctx = cast_spec
            .castOperand()
            .ok_or_else(|| Self::parse_err("CST not valid: castOperand missing in CAST".into()))?;
        let expression = if let Some(value_expr) = operand_ctx.valueExpression() {
            Self::build_expression_from_context(&value_expr)?
        } else if operand_ctx.nullLiteral().is_some() {
            Expression::Literal(Literal::Null)
        } else {
            return Self::parse_error("CST not valid: unsupported CAST operand");
        };

        let cast_target = cast_spec
            .castTarget()
            .ok_or_else(|| Self::parse_err("CST not valid: castTarget missing in CAST".into()))?;
        let value_type = cast_target.valueType().ok_or_else(|| {
            Self::parse_err("CST not valid: valueType missing in castTarget".into())
        })?;

        let target_type = Self::build_type_spec_from_value_type(&value_type)?;

        Ok(Expression::Cast(CastExpression {
            expression: Box::new(expression),
            target_type,
            span: span.clone(),
        }))
    }

    /// Function to build a TypeSpec from a ValueTypeContext, handling all the different possible value type contexts and returning the corresponding TypeSpec variant.
    pub(super) fn build_type_spec_from_value_type(
        value_type: &ValueTypeContextAll,
    ) -> GqlResult<TypeSpec> {
        match value_type {
            ValueTypeContextAll::OpenDynamicUnionTypeLabelContext(_) => Ok(TypeSpec::Any),
            ValueTypeContextAll::DynamicPropertyValueTypeLabelContext(_) => Ok(TypeSpec::Any),
            ValueTypeContextAll::ClosedDynamicUnionTypeAtl1Context(ctx) => {
                let raw_members = ctx
                    .valueType_all()
                    .into_iter()
                    .map(|inner| Self::build_type_spec_from_value_type(&inner))
                    .collect::<GqlResult<Vec<_>>>()?;

                let members = Self::flatten_union_members(raw_members);

                Ok(TypeSpec::Union { members })
            }
            ValueTypeContextAll::ClosedDynamicUnionTypeAtl2Context(ctx) => {
                let raw_members = ctx
                    .valueType_all()
                    .into_iter()
                    .map(|inner| Self::build_type_spec_from_value_type(&inner))
                    .collect::<GqlResult<Vec<_>>>()?;

                let members = Self::flatten_union_members(raw_members);

                Ok(TypeSpec::Union { members })
            }

            ValueTypeContextAll::PathValueTypeLabelContext(_) => Ok(TypeSpec::Path),
            ValueTypeContextAll::RecordTypeLabelContext(_) => Ok(TypeSpec::Record),

            ValueTypeContextAll::ListValueTypeAlt1Context(ctx) => {
                let element_ctx = ctx
                    .valueType()
                    .ok_or_else(|| Self::parse_err("LIST<T>: missing element type".into()))?;
                let element_type = Self::build_type_spec_from_value_type(&element_ctx)?;
                let max_length = ctx
                    .maxLength()
                    .map(|m| Self::parse_usize_from_ctx_text(&m.get_text()))
                    .transpose()?;
                Ok(TypeSpec::List {
                    element_type: Box::new(element_type),
                    max_length,
                })
            }
            ValueTypeContextAll::ListValueTypeAlt2Context(ctx) => {
                let element_ctx = ctx
                    .valueType()
                    .ok_or_else(|| Self::parse_err("T LIST: missing element type".into()))?;
                let element_type = Self::build_type_spec_from_value_type(&element_ctx)?;
                let max_length = ctx
                    .maxLength()
                    .map(|m| Self::parse_usize_from_ctx_text(&m.get_text()))
                    .transpose()?;
                Ok(TypeSpec::List {
                    element_type: Box::new(element_type),
                    max_length,
                })
            }
            ValueTypeContextAll::ListValueTypeAlt3Context(ctx) => {
                let max_length = ctx
                    .maxLength()
                    .map(|m| Self::parse_usize_from_ctx_text(&m.get_text()))
                    .transpose()?;
                Ok(TypeSpec::List {
                    element_type: Box::new(TypeSpec::Any),
                    max_length,
                })
            }

            ValueTypeContextAll::PredefinedTypeLabelContext(ctx) => {
                let predefined = ctx
                    .predefinedType()
                    .ok_or_else(|| Self::parse_err("predefinedType missing".into()))?;
                Self::build_type_spec_from_predefined(&predefined)
            }

            ValueTypeContextAll::Error(_) => Self::parse_error("Invalid valueType context"),
        }
    }

    /// Helper function to recursively flatten nested union members in a list of TypeSpec, ensuring that any nested unions are flattened into a single level of union members.
    pub(super) fn flatten_union_members(members: Vec<TypeSpec>) -> Vec<TypeSpec> {
        let mut flat = Vec::new();
        for member in members {
            match member {
                TypeSpec::Union { members } => flat.extend(Self::flatten_union_members(members)),
                other => flat.push(other),
            }
        }
        flat
    }

    /// Function to build a TypeSpec from a PredefinedTypeContext, handling all the different possible predefined type contexts and returning the corresponding TypeSpec variant.
    pub(super) fn build_type_spec_from_predefined(
        pre: &PredefinedTypeContextAll,
    ) -> GqlResult<TypeSpec> {
        if pre.booleanType().is_some() {
            return Ok(TypeSpec::Boolean);
        }

        if let Some(s) = pre.characterStringType() {
            return Self::build_type_spec_from_character_string(&s);
        }

        if let Some(b) = pre.byteStringType() {
            return Self::build_type_spec_from_byte_string(&b);
        }

        if let Some(n) = pre.numericType() {
            return Self::build_type_spec_from_numeric(&n);
        }

        if let Some(t) = pre.temporalType() {
            return Self::build_type_spec_from_temporal(&t);
        }

        if let Some(r) = pre.referenceValueType() {
            if r.bindingTableReferenceValueType().is_some() {
                return Ok(TypeSpec::BindingTable);
            }

            if let Some(graph_ref_type) = r.graphReferenceValueType() {
                let graph_type_spec = if let Some(closed_graph_ref_type) =
                    graph_ref_type.closedGraphReferenceValueType()
                {
                    Some(Box::new(
                        Self::build_graph_type_spec_from_closed_graph_reference(
                            &closed_graph_ref_type,
                        )?,
                    ))
                } else {
                    None
                };

                return Ok(TypeSpec::Graph { graph_type_spec });
            }

            return Ok(TypeSpec::Reference { target_type: None });
        }

        if pre.immaterialValueType().is_some() {
            return Ok(TypeSpec::Any);
        }

        Ok(TypeSpec::Any)
    }

    /// Function to build a GraphTypeSpec from a ClosedGraphReferenceValueTypeContext, extracting the nested graph type specification and building the corresponding GraphTypeSpec.
    fn build_graph_type_spec_from_closed_graph_reference(
        cst: &ClosedGraphReferenceValueTypeContextAll,
    ) -> GqlResult<GraphTypeSpec> {
        let nested = cst.nestedGraphTypeSpecification().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: missing nestedGraphTypeSpecification in closed graph reference"
                    .into(),
            )
        })?;

        Self::build_graph_type_spec_from_nested_graph_type_specification(&nested)
    }

    fn build_graph_type_spec_from_nested_graph_type_specification(
        cst: &NestedGraphTypeSpecificationContextAll,
    ) -> GqlResult<GraphTypeSpec> {
        let body = cst.graphTypeSpecificationBody().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: missing graphTypeSpecificationBody in nested graph type specification"
                    .into(),
            )
        })?;

        let element_list = body.elementTypeList().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: missing elementTypeList in graphTypeSpecificationBody".into(),
            )
        })?;

        let mut vertex_types = Vec::new();
        let mut edge_types = Vec::new();

        for element in element_list.elementTypeSpecification_all() {
            if let Some(node_spec) = element.nodeTypeSpecification() {
                vertex_types.push(Self::build_vertex_type_spec_from_context(&node_spec)?);
                continue;
            }

            if let Some(edge_spec) = element.edgeTypeSpecification() {
                edge_types.push(Self::build_edge_type_spec_from_context(&edge_spec)?);
                continue;
            }
        }

        if vertex_types.is_empty() && edge_types.is_empty() {
            return Self::parse_error(
                "Closed graph reference value type must declare at least one node or edge type",
            );
        }

        Ok(GraphTypeSpec {
            vertex_types,
            edge_types,
        })
    }

    /// Function to build a VertexTypeSpec from a NodeTypeSpecificationContext, extracting the identifier and properties of the vertex type and building the corresponding VertexTypeSpec.
    fn build_vertex_type_spec_from_context(
        cst: &NodeTypeSpecificationContextAll,
    ) -> GqlResult<VertexTypeSpec> {
        let span = Self::extract_span(cst);

        let identifier = Self::extract_node_type_identifier(cst);
        let properties = Self::extract_node_type_properties(cst)?;

        Ok(VertexTypeSpec {
            identifier,
            labels: None,
            properties,
            span: span.clone(),
        })
    }

    /// Function to build an EdgeTypeSpec from an EdgeTypeSpecificationContext, extracting the identifier, properties, and endpoints of the edge type and building the corresponding EdgeTypeSpec.
    fn build_edge_type_spec_from_context(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> GqlResult<EdgeTypeSpec> {
        let span = Self::extract_span(cst);

        let identifier = Self::extract_edge_type_identifier(cst);
        let properties = Self::extract_edge_type_properties(cst)?;
        let (source_vertex, destination_vertex) = Self::extract_edge_endpoints(cst);

        Ok(EdgeTypeSpec {
            identifier,
            labels: None,
            properties,
            source_vertex,
            destination_vertex,
            span: span.clone(),
        })
    }

    /// Function to extract the identifier of a node type from a NodeTypeSpecificationContext, checking both the pattern and phrase forms of node type specification and returning the identifier if found.
    fn extract_node_type_identifier(cst: &NodeTypeSpecificationContextAll) -> Option<String> {
        if let Some(pattern) = cst.nodeTypePattern() {
            if let Some(name) = pattern.nodeTypeName() {
                if let Some(identifier) = name.identifier() {
                    return Some(Self::identifier_text(&identifier.get_text()));
                }
            }

            if let Some(alias) = pattern.localNodeTypeAlias() {
                if let Some(identifier) = alias.regularIdentifier() {
                    return Some(Self::identifier_text(&identifier.get_text()));
                }
            }
        }

        if let Some(phrase) = cst.nodeTypePhrase() {
            if let Some(filler) = phrase.nodeTypePhraseFiller() {
                if let Some(name) = filler.nodeTypeName() {
                    if let Some(identifier) = name.identifier() {
                        return Some(Self::identifier_text(&identifier.get_text()));
                    }
                }
            }

            if let Some(alias) = phrase.localNodeTypeAlias() {
                if let Some(identifier) = alias.regularIdentifier() {
                    return Some(Self::identifier_text(&identifier.get_text()));
                }
            }
        }

        None
    }

    /// Function to extract the identifier of an edge type from an EdgeTypeSpecificationContext, checking both the pattern and phrase forms of edge type specification and returning the identifier if found.
    fn extract_edge_type_identifier(cst: &EdgeTypeSpecificationContextAll) -> Option<String> {
        if let Some(pattern) = cst.edgeTypePattern() {
            if let Some(name) = pattern.edgeTypeName() {
                if let Some(identifier) = name.identifier() {
                    return Some(identifier.get_text());
                }
            }
        }

        if let Some(phrase) = cst.edgeTypePhrase() {
            if let Some(filler) = phrase.edgeTypePhraseFiller() {
                if let Some(name) = filler.edgeTypeName() {
                    if let Some(identifier) = name.identifier() {
                        return Some(Self::identifier_text(&identifier.get_text()));
                    }
                }
            }
        }

        None
    }

    /// Function to extract the properties of a node type from a NodeTypeSpecificationContext, checking both the pattern and phrase forms of node type specification for property type declarations and returning a PropertyTypeList if found.
    fn extract_node_type_properties(
        cst: &NodeTypeSpecificationContextAll,
    ) -> GqlResult<Option<PropertyTypeList>> {
        let from_pattern = cst
            .nodeTypePattern()
            .and_then(|pattern| pattern.nodeTypeFiller())
            .and_then(|filler| filler.nodeTypeImpliedContent())
            .and_then(|content| content.nodeTypePropertyTypes())
            .and_then(|property_types| property_types.propertyTypesSpecification())
            .map(|spec| Self::build_property_type_list_from_context(&spec))
            .transpose()?;

        if from_pattern.is_some() {
            return Ok(from_pattern);
        }

        cst.nodeTypePhrase()
            .and_then(|phrase| phrase.nodeTypePhraseFiller())
            .and_then(|filler| filler.nodeTypeFiller())
            .and_then(|filler| filler.nodeTypeImpliedContent())
            .and_then(|content| content.nodeTypePropertyTypes())
            .and_then(|property_types| property_types.propertyTypesSpecification())
            .map(|spec| Self::build_property_type_list_from_context(&spec))
            .transpose()
    }

    /// Function to extract the properties of an edge type from an EdgeTypeSpecificationContext, checking both the pattern and phrase forms of edge type specification for property type declarations and returning a PropertyTypeList if found.
    fn extract_edge_type_properties(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> GqlResult<Option<PropertyTypeList>> {
        cst.edgeTypePhrase()
            .and_then(|phrase| phrase.edgeTypePhraseFiller())
            .and_then(|filler| filler.edgeTypeFiller())
            .and_then(|filler| filler.edgeTypeImpliedContent())
            .and_then(|content| content.edgeTypePropertyTypes())
            .and_then(|property_types| property_types.propertyTypesSpecification())
            .map(|spec| Self::build_property_type_list_from_context(&spec))
            .transpose()
    }

    /// Function to extract the source and destination vertex type identifiers for an edge type from an EdgeTypeSpecificationContext, checking the edge type phrase for an endpoint pair specification and returning the source and destination identifiers if found.
    fn extract_edge_endpoints(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> (Option<String>, Option<String>) {
        let Some(phrase) = cst.edgeTypePhrase() else {
            return (None, None);
        };

        let Some(endpoint_phrase) = phrase.endpointPairPhrase() else {
            return (None, None);
        };

        let Some(endpoint_pair) = endpoint_phrase.endpointPair() else {
            return (None, None);
        };

        if let Some(directed) = endpoint_pair.endpointPairDirected() {
            if let Some(pointing_right) = directed.endpointPairPointingRight() {
                let source = pointing_right
                    .sourceNodeTypeAlias()
                    .and_then(|alias| alias.regularIdentifier())
                    .map(|identifier| identifier.get_text());
                let destination = pointing_right
                    .destinationNodeTypeAlias()
                    .and_then(|alias| alias.regularIdentifier())
                    .map(|identifier| identifier.get_text());
                return (source, destination);
            }

            if let Some(pointing_left) = directed.endpointPairPointingLeft() {
                let destination = pointing_left
                    .destinationNodeTypeAlias()
                    .and_then(|alias| alias.regularIdentifier())
                    .map(|identifier| identifier.get_text());
                let source = pointing_left
                    .sourceNodeTypeAlias()
                    .and_then(|alias| alias.regularIdentifier())
                    .map(|identifier| identifier.get_text());
                return (source, destination);
            }
        }

        if let Some(undirected) = endpoint_pair.endpointPairUndirected() {
            let source = undirected
                .sourceNodeTypeAlias()
                .and_then(|alias| alias.regularIdentifier())
                .map(|identifier| identifier.get_text());
            let destination = undirected
                .destinationNodeTypeAlias()
                .and_then(|alias| alias.regularIdentifier())
                .map(|identifier| identifier.get_text());
            return (source, destination);
        }

        (None, None)
    }

    /// Helper function to build a PropertyTypeList from a PropertyTypesSpecificationContext, extracting the property type declarations and building a list of PropertyTypeDecl for the properties declared in the context.
    fn build_property_type_list_from_context(
        cst: &PropertyTypesSpecificationContextAll,
    ) -> GqlResult<PropertyTypeList> {
        let span = Self::extract_span(cst);

        let property_list = cst.propertyTypeList().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: missing propertyTypeList in propertyTypesSpecification".into(),
            )
        })?;

        let mut properties = Vec::new();

        for property in property_list.propertyType_all() {
            let name = property
                .propertyName()
                .and_then(|name| name.identifier())
                .map(|identifier| Self::identifier_text(&identifier.get_text()))
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: missing property name in propertyType".into())
                })?;

            let value_type = property
                .propertyValueType()
                .and_then(|value| value.valueType())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: missing property value type in propertyType".into(),
                    )
                })?;

            let type_spec = Self::build_type_spec_from_value_type(&value_type)?;

            properties.push(PropertyTypeDecl {
                name,
                type_spec,
                span: span.clone(),
            });
        }

        Ok(PropertyTypeList { properties })
    }

    /// Function to build a TypeSpec for character string types, extracting the max length if specified and returning a TypeSpec::String variant with the appropriate max_length.
    pub(super) fn build_type_spec_from_character_string(
        ctx: &CharacterStringTypeContextAll,
    ) -> GqlResult<TypeSpec> {
        let max_length = if let Some(fixed) = ctx.fixedLength() {
            Some(Self::parse_usize_from_ctx_text(&fixed.get_text())?)
        } else if let Some(max_len) = ctx.maxLength() {
            Some(Self::parse_usize_from_ctx_text(&max_len.get_text())?)
        } else {
            None
        };
        Ok(TypeSpec::String { max_length })
    }

    /// Function to build a TypeSpec for byte string types, extracting the max length if specified and returning a TypeSpec::Bytes variant with the appropriate max_length.
    pub(super) fn build_type_spec_from_byte_string(
        ctx: &ByteStringTypeContextAll,
    ) -> GqlResult<TypeSpec> {
        let max_length = if let Some(fixed) = ctx.fixedLength() {
            Some(Self::parse_usize_from_ctx_text(&fixed.get_text())?)
        } else if let Some(max_len) = ctx.maxLength() {
            Some(Self::parse_usize_from_ctx_text(&max_len.get_text())?)
        } else {
            None
        };
        Ok(TypeSpec::Bytes { max_length })
    }

    /// Helper function to parse a usize value from a context text, returning a GqlResult with the parsed usize or an error if parsing fails.
    pub(super) fn parse_usize_from_ctx_text(text: &str) -> GqlResult<usize> {
        text.parse::<usize>()
            .map_err(|_| Self::parse_err(format!("Invalid usize parameter: {}", text)))
    }

    /// Helper function to normalize a raw type token string by converting it to uppercase, removing whitespace and underscores, and stripping known prefixes and suffixes related to type specifications, returning the normalized type token string.
    pub(super) fn normalize_type_token(raw: &str) -> String {
        let mut t = raw
            .to_ascii_uppercase()
            .replace([' ', '\t', '\n', '\r', '_'], "");

        while let Some(stripped) = t.strip_prefix("::") {
            t = stripped.to_string();
        }
        while let Some(stripped) = t.strip_prefix("TYPED") {
            t = stripped.to_string();
        }

        if let Some(stripped) = t.strip_suffix("NOTNULL") {
            t = stripped.to_string();
        }
        if let Some(stripped) = t.strip_suffix("NULL") {
            t = stripped.to_string();
        }

        t
    }

    /// Function to build a TypeSpec for numeric types, handling various numeric type specifications such as FLOAT, DECIMAL, INTEGER, etc., and returning the corresponding TypeSpec variant with any specified parameters.
    pub(super) fn build_type_spec_from_numeric(ctx: &NumericTypeContextAll) -> GqlResult<TypeSpec> {
        let raw = ctx.get_text();
        let t = Self::normalize_type_token(&raw);

        if t == "FLOAT32" {
            return Ok(TypeSpec::Float32);
        }
        if t == "FLOAT16" || t == "FLOAT128" || t == "FLOAT256" {
            return Ok(TypeSpec::Float { precision: None });
        }
        if t == "REAL" {
            return Ok(TypeSpec::Real);
        }
        if t == "DOUBLE" || t == "DOUBLEPRECISION" || t == "FLOAT64" {
            return Ok(TypeSpec::Double);
        }

        if t.starts_with("DECIMAL") || t.starts_with("DEC") || t.starts_with("NUMERIC") {
            let nums = Self::extract_parenthesized_u8_list(&t)?;
            let precision = nums.first().copied();
            let scale = nums.get(1).copied();
            return Ok(TypeSpec::Decimal { precision, scale });
        }

        if t.starts_with("FLOAT") {
            let nums = Self::extract_parenthesized_u8_list(&t)?;
            let precision = nums.first().copied();
            return Ok(TypeSpec::Float { precision });
        }

        if t == "BIGINT" || t == "INT64" || t == "INTEGER64" || t == "UBIGINT" || t == "UINT64" {
            return Ok(TypeSpec::BigInt);
        }
        if t == "SMALLINT" || t == "INT16" || t == "INTEGER16" || t == "USMALLINT" || t == "UINT16"
        {
            return Ok(TypeSpec::SmallInt);
        }
        if t == "INT8" || t == "INTEGER8" || t == "UINT8" || t == "TINYINT" {
            return Ok(TypeSpec::Integer);
        }
        if t == "INT128" || t == "INTEGER128" || t == "UINT128" {
            return Ok(TypeSpec::Int128);
        }
        if t == "INT256" || t == "INTEGER256" || t == "UINT256" {
            return Ok(TypeSpec::Int256);
        }

        if t == "INT"
            || t == "INTEGER"
            || t == "INT32"
            || t == "INTEGER32"
            || t.starts_with("INT(")
            || t.starts_with("INTEGER(")
            || t == "UINT"
            || t == "UINT32"
            || t.starts_with("UINT(")
        {
            return Ok(TypeSpec::Integer);
        }

        Self::parse_error(format!("Unsupported numeric type in CAST: {}", raw))
    }

    /// Function to build a TypeSpec for temporal types, handling various temporal type specifications such as DATE, TIMESTAMP, TIME, DURATION, etc., and returning the corresponding TypeSpec variant with any specified parameters.
    pub(super) fn build_type_spec_from_temporal(
        ctx: &TemporalTypeContextAll,
    ) -> GqlResult<TypeSpec> {
        let raw = ctx.get_text();
        let t = Self::normalize_type_token(&raw);
        let precision = Self::extract_parenthesized_u8_list(&raw)
            .ok()
            .and_then(|values| values.first().copied());

        if t == "DATE" {
            return Ok(TypeSpec::Date);
        }

        if t == "ZONEDDATETIME" || t == "TIMESTAMPWITHTIMEZONE" {
            return Ok(TypeSpec::ZonedDateTime { precision });
        }
        if t == "LOCALDATETIME" || t == "TIMESTAMP" || t == "TIMESTAMPWITHOUTTIMEZONE" {
            return Ok(TypeSpec::LocalDateTime { precision });
        }

        if t == "ZONEDTIME" || t == "TIMEWITHTIMEZONE" {
            return Ok(TypeSpec::ZonedTime { precision });
        }
        if t == "LOCALTIME" || t == "TIME" || t == "TIMEWITHOUTTIMEZONE" {
            return Ok(TypeSpec::LocalTime { precision });
        }

        if t.starts_with("DURATION") {
            return Ok(TypeSpec::Duration {
                precision,
                qualifier: Self::extract_duration_qualifier(&t),
            });
        }

        Self::parse_error(format!("Unsupported temporal type in CAST: {}", raw))
    }

    /// Helper function to extract a list of u8 values from a parenthesized list in a type token string, returning an error if the format is invalid or if any of the values cannot be parsed as u8.
    pub(super) fn extract_parenthesized_u8_list(text: &str) -> GqlResult<Vec<u8>> {
        let start = match text.find('(') {
            Some(i) => i,
            None => return Ok(Vec::new()),
        };
        let end = text[start + 1..]
            .find(')')
            .map(|i| start + 1 + i)
            .ok_or_else(|| Self::parse_err(format!("Invalid type parameter list: {}", text)))?;

        let inner = &text[start + 1..end].trim();
        if inner.is_empty() {
            return Ok(Vec::new());
        }

        inner
            .split(',')
            .map(|p| {
                p.trim().parse::<u8>().map_err(|_| {
                    Self::parse_err(format!("Invalid numeric type parameter '{}'", p.trim()))
                })
            })
            .collect()
    }

    /// Helper function to extract a duration qualifier from a normalized temporal type token string, looking for specific qualifiers like "YEARTOMONTH" or "DAYTOSECOND" and returning the corresponding formatted qualifier string if found.
    pub(super) fn extract_duration_qualifier(normalized_temporal: &str) -> Option<String> {
        let start = normalized_temporal.find('(')?;
        let end = normalized_temporal[start + 1..]
            .find(')')
            .map(|i| start + 1 + i)?;

        let qualifier = &normalized_temporal[start + 1..end];
        match qualifier {
            "YEARTOMONTH" => Some("YEAR TO MONTH".to_string()),
            "DAYTOSECOND" => Some("DAY TO SECOND".to_string()),
            _ => None,
        }
    }

    /// Function to build a CASE expression from a CaseExpressionContext, handling both simple and searched CASE specifications, extracting the test expression, WHEN branches, and ELSE expression as needed, and returning the corresponding CaseExpression wrapped in an Expression::Case variant.
    pub(super) fn build_case_expression_from_context(
        case_expr: &CaseExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(case_expr);

        if let Some(abbreviation) = case_expr.caseAbbreviation() {
            let name = if abbreviation.COALESCE().is_some() {
                "COALESCE"
            } else {
                "NULLIF"
            };
            let arguments = abbreviation
                .valueExpression_all()
                .into_iter()
                .map(|e| Self::build_expression_from_context(&e))
                .collect::<GqlResult<Vec<_>>>()?;

            return Ok(Expression::FunctionCall(FunctionCall {
                name: name.to_string(),
                distinct: DistinctQualifier::None,
                arguments,
                span: span.clone(),
            }));
        }

        let specification = case_expr.caseSpecification().ok_or_else(|| {
            Self::parse_err("CST not valid: caseSpecification missing in caseExpression".into())
        })?;

        if let Some(simple_case) = specification.simpleCase() {
            let test_expression = simple_case
                .caseOperand()
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: caseOperand missing in simpleCase".into())
                })
                .and_then(|op| Self::build_case_operand_from_context(&op))?;

            let when_branches = simple_case
                .simpleWhenClause_all()
                .into_iter()
                .map(|when_clause| Self::build_simple_when_branch_from_context(&when_clause))
                .collect::<GqlResult<Vec<_>>>()?;

            let else_expression = if let Some(else_clause) = simple_case.elseClause() {
                let result = else_clause.result().ok_or_else(|| {
                    Self::parse_err("CST not valid: result missing in elseClause".into())
                })?;
                Some(Box::new(Self::build_result_expression_from_context(
                    &result,
                )?))
            } else {
                None
            };

            return Ok(Expression::Case(CaseExpression {
                case_type: CaseType::Simple(SimpleCaseExpression {
                    test_expression: Box::new(test_expression),
                    when_branches,
                    else_expression,
                }),
                span: span.clone(),
            }));
        }

        if let Some(searched_case) = specification.searchedCase() {
            let when_branches = searched_case
                .searchedWhenClause_all()
                .into_iter()
                .map(|when_clause| Self::build_searched_when_branch_from_context(&when_clause))
                .collect::<GqlResult<Vec<_>>>()?;

            let else_expression = if let Some(else_clause) = searched_case.elseClause() {
                let result = else_clause.result().ok_or_else(|| {
                    Self::parse_err("CST not valid: result missing in elseClause".into())
                })?;
                Some(Box::new(Self::build_result_expression_from_context(
                    &result,
                )?))
            } else {
                None
            };

            return Ok(Expression::Case(CaseExpression {
                case_type: CaseType::Searched(SearchedCaseExpression {
                    when_branches,
                    else_expression,
                }),
                span: span.clone(),
            }));
        }

        Self::parse_error("CST not valid: unsupported CASE specification")
    }

    /// Function to build an EXISTS subquery expression from an ExistsPredicateContext, extracting the MATCH statement block and any associated WHERE clauses, building the corresponding query pipeline, and returning the resulting SubqueryExpression wrapped in an Expression::Subquery variant with is_exists set to true.
    pub(super) fn build_exists_subquery_expression_from_context(
        exists_predicate: &ExistsPredicateContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(exists_predicate);

        if let Some(match_block) = exists_predicate.matchStatementBlock() {
            let match_statements = match_block.matchStatement_all();

            if match_statements.is_empty() {
                return Self::parse_error("CST not valid: matchStatementBlock is empty");
            }

            let mut pipeline = Vec::new();
            for match_statement in match_statements {
                for (graph_pattern, optional) in Self::extract_graph_patterns(&match_statement)? {
                    let match_clause = Self::build_match_clause(&graph_pattern, optional)?;
                    pipeline.push(QueryStep::Match(match_clause));

                    if let Some(where_ctx) = graph_pattern.graphPatternWhereClause() {
                        let where_clause = Self::build_where_clause_from_context(&where_ctx)?;
                        pipeline.push(QueryStep::Filter(FilterClause {
                            condition: where_clause.condition,
                            span: span.clone(),
                        }));
                    }
                }
            }

            let query = Query::Basic(BasicQuery {
                pipeline,
                result_statement: PrimitiveResultStatement::Finish { span: span.clone() },
                span: span.clone(),
            });

            return Ok(Expression::Subquery(SubqueryExpression {
                query: Box::new(query),
                expression: None,
                is_exists: true,
                negated: false,
                span: span.clone(),
            }));
        }

        Self::parse_error("CST not valid: matchStatementBlock missing in EXISTS predicate")
    }

    /// Function to build a binding table expression from a BindingTableExpressionContext, handling both the case where it contains a nested query specification and the case where it is a simple leaf expression, and returning the corresponding Expression.
    pub(super) fn build_binding_table_expression_from_context(
        binding_table: &BindingTableExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(binding_table);

        if let Some(nested) = binding_table.nestedBindingTableQuerySpecification() {
            let nested_query = nested.nestedQuerySpecification().ok_or_else(|| {
                Self::parse_err("CST not valid: nestedQuerySpecification missing".into())
            })?;

            let statement = Self::build_query_from_nested_query_spec(&nested_query)?;

            return Ok(Expression::Subquery(SubqueryExpression {
                query: Box::new(statement),
                expression: None,
                is_exists: false,
                negated: false,
                span: span.clone(),
            }));
        }

        Self::build_expression_from_leaf_text(&binding_table.get_text(), span.clone())
    }

    /// Function to build a SimpleWhenBranch from a SimpleWhenClauseContext, extracting the list of WHEN operand expressions and the THEN result expression, building the corresponding Expressions for each, and returning the resulting SimpleWhenBranch.
    pub(super) fn build_simple_when_branch_from_context(
        when_clause: &SimpleWhenClauseContextAll,
    ) -> GqlResult<SimpleWhenBranch> {
        let span = Self::extract_span(when_clause);

        let when_values = when_clause
            .whenOperandList()
            .ok_or_else(|| Self::parse_err("CST not valid: whenOperandList missing".into()))?
            .whenOperand_all()
            .into_iter()
            .map(|operand| Self::build_when_operand_from_context(&operand))
            .collect::<GqlResult<Vec<_>>>()?;

        let then_expression = when_clause
            .result()
            .ok_or_else(|| {
                Self::parse_err("CST not valid: result missing in simpleWhenClause".into())
            })
            .and_then(|result| Self::build_result_expression_from_context(&result))
            .map(Box::new)?;

        Ok(SimpleWhenBranch {
            when_values,
            then_expression,
            span: span.clone(),
        })
    }

    /// Function to build a SearchedWhenBranch from a SearchedWhenClauseContext, extracting the search condition expression and the THEN result expression, building the corresponding Expressions for each, and returning the resulting SearchedWhenBranch.
    pub(super) fn build_searched_when_branch_from_context(
        when_clause: &SearchedWhenClauseContextAll,
    ) -> GqlResult<SearchedWhenBranch> {
        let span = Self::extract_span(when_clause);

        let condition = when_clause
            .searchCondition()
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: searchCondition missing in searchedWhenClause".into(),
                )
            })?
            .booleanValueExpression()
            .and_then(|expr| expr.valueExpression())
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: booleanValueExpression missing in searchedWhenClause".into(),
                )
            })
            .and_then(|expr| Self::build_expression_from_context(&expr))
            .map(Box::new)?;

        let then_expression = when_clause
            .result()
            .ok_or_else(|| {
                Self::parse_err("CST not valid: result missing in searchedWhenClause".into())
            })
            .and_then(|result| Self::build_result_expression_from_context(&result))
            .map(Box::new)?;

        Ok(SearchedWhenBranch {
            condition,
            then_expression,
            span: span.clone(),
        })
    }

    /// Function to build a result expression from a ResultContext, handling both value expressions and NULL literals, and returning the corresponding Expression.
    pub(super) fn build_result_expression_from_context(
        result: &ResultContextAll,
    ) -> GqlResult<Expression> {
        if let Some(value_expr) = result
            .resultExpression()
            .and_then(|re| re.valueExpression())
        {
            return Self::build_expression_from_context(&value_expr);
        }

        if result.nullLiteral().is_some() {
            return Ok(Expression::Literal(Literal::Null));
        }

        Self::parse_error("CST not valid: unsupported result in CASE")
    }

    /// Function to build a case operand expression from a CaseOperandContext, handling both variable references and non-parenthesized value expression primaries, and returning the corresponding Expression.
    pub(super) fn build_case_operand_from_context(
        operand: &CaseOperandContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(operand);
        if let Some(var_ref) = operand.elementVariableReference() {
            return Ok(Expression::Variable {
                name: var_ref.get_text(),
                span: span.clone(),
            });
        }

        if let Some(primary) = operand.nonParenthesizedValueExpressionPrimary() {
            return Self::build_expression_from_non_parenthesized_primary(&primary);
        }

        Self::parse_error("CST not valid: unsupported caseOperand")
    }

    /// Function to build a WHEN operand expression from a WhenOperandContext, handling the various possible forms of WHEN operands such as comparisons, predicates, value expressions, etc., and returning the corresponding Expression.
    pub(super) fn build_when_operand_from_context(
        operand: &WhenOperandContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(operand);

        if let Some(comp_op) = operand.compOp() {
            let op = Self::operator_from_context(&comp_op)?;
            let right = operand.valueExpression().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: missing valueExpression for compOp whenOperand".into(),
                )
            })?;
            let right_expr = Self::build_expression_from_context(&right)?;
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::Comparison {
                    operator: op,
                    right: Box::new(right_expr),
                },
                span: span.clone(),
            }));
        }

        if let Some(null_part) = operand.nullPredicatePart2() {
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::NullPredicate {
                    negated: null_part.NOT().is_some(),
                },
                span: span.clone(),
            }));
        }

        if let Some(type_part) = operand.valueTypePredicatePart2() {
            let target_type = type_part
                .valueType()
                .map(|vt| Self::build_type_spec_from_value_type(&vt))
                .transpose()?;

            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::ValueTypePredicate {
                    negated: type_part.NOT().is_some(),
                    target_type,
                },
                span: span.clone(),
            }));
        }

        if let Some(norm_part) = operand.normalizedPredicatePart2() {
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::NormalizedPredicate {
                    negated: norm_part.NOT().is_some(),
                    normal_form: norm_part.normalForm().map(|nf| nf.get_text()),
                },
                span: span.clone(),
            }));
        }

        if let Some(dir_part) = operand.directedPredicatePart2() {
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::DirectedPredicate {
                    negated: dir_part.NOT().is_some(),
                },
                span: span.clone(),
            }));
        }

        if let Some(label_part) = operand.labeledPredicatePart2() {
            let negated = label_part
                .isLabeledOrColon()
                .map(|x| x.NOT().is_some())
                .unwrap_or(false);

            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::LabeledPredicate {
                    negated,
                    label_expression: label_part.labelExpression().map(|le| Self::identifiers_text(&le.get_text())),
                },
                span: span.clone(),
            }));
        }

        if let Some(source_part) = operand.sourcePredicatePart2() {
            let edge_reference = source_part
                .edgeReference()
                .and_then(|e| e.elementVariableReference())
                .map(|e| e.get_text());
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::SourcePredicate {
                    negated: source_part.NOT().is_some(),
                    edge_reference,
                },
                span: span.clone(),
            }));
        }

        if let Some(destination_part) = operand.destinationPredicatePart2() {
            let edge_reference = destination_part
                .edgeReference()
                .and_then(|e| e.elementVariableReference())
                .map(|e| e.get_text());
            return Ok(Expression::WhenOperand(WhenOperandExpression {
                kind: WhenOperandKind::DestinationPredicate {
                    negated: destination_part.NOT().is_some(),
                    edge_reference,
                },
                span: span.clone(),
            }));
        }

        if let Some(value_expr) = operand.valueExpression() {
            return Self::build_expression_from_context(&value_expr);
        }

        if let Some(primary) = operand.nonParenthesizedValueExpressionPrimary() {
            return Self::build_expression_from_non_parenthesized_primary(&primary);
        }

        Self::build_expression_from_leaf_text(&operand.get_text(), span.clone())
    }

    /// Function to build an Expression from a NonParenthesizedValueExpressionPrimaryContext, handling various special cases such as aggregate functions, unsigned value specifications, path constructors, property accesses, value queries, CASE expressions, CAST specifications, ELEMENT_ID function calls, and inline LET expressions, and returning the corresponding Expression for each case. If none of the special cases match, it falls back to building an expression from the leaf text.
    pub(super) fn build_expression_from_non_parenthesized_primary(
        primary: &NonParenthesizedValueExpressionPrimaryContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(primary);

        if let Some(binding) = primary.bindingVariableReference() {
            return Ok(Expression::Variable {
                name: binding.get_text(),
                span: span.clone(),
            });
        }

        if let Some(special) = primary.nonParenthesizedValueExpressionPrimarySpecialCase() {
            if let Some(aggregate_fn) = special.aggregateFunction() {
                return Self::build_function_call_from_aggregate_context(&aggregate_fn);
            }

            if let Some(unsigned) = special.unsignedValueSpecification() {
                return Self::build_expression_from_leaf_text(&unsigned.get_text(), span.clone());
            }

            if let Some(path_constructor) = special.pathValueConstructor() {
                return Self::build_path_constructor_expression_from_context(&path_constructor);
            }

            if let Some(base_primary) = special.valueExpressionPrimary() {
                if let Some(property_name) = special.propertyName() {
                    return Ok(Expression::PropertyAccess(PropertyAccess {
                        object: base_primary.get_text(),
                        property: Self::identifier_text(&property_name.get_text()),
                        span: span.clone(),
                    }));
                }
                return Self::build_primary_expression_from_context(&base_primary);
            }

            if let Some(value_query) = special.valueQueryExpression() {
                let nested = value_query.nestedQuerySpecification().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: nestedQuerySpecification missing in valueQueryExpression"
                            .into(),
                    )
                })?;
                let query = Self::build_query_from_nested_query_spec(&nested)?;
                return Ok(Expression::Subquery(SubqueryExpression {
                    query: Box::new(query),
                    expression: None,
                    is_exists: false,
                    negated: false,
                    span: span.clone(),
                }));
            }

            if let Some(case_expr) = special.caseExpression() {
                return Self::build_case_expression_from_context(&case_expr);
            }

            if let Some(cast_spec) = special.castSpecification() {
                return Self::build_cast_expression_from_context(&cast_spec);
            }

            if let Some(element_id_fn) = special.element_idFunction() {
                let element_ref = element_id_fn.elementVariableReference().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: elementVariableReference missing in ELEMENT_ID".into(),
                    )
                })?;

                return Ok(Expression::FunctionCall(FunctionCall {
                    name: fn_names::ELEMENT_ID.to_string(),
                    distinct: DistinctQualifier::None,
                    arguments: vec![Expression::Variable {
                        name: element_ref.get_text(),
                        span: span.clone(),
                    }],
                    span: span.clone(),
                }));
            }

            if let Some(let_expr) = special.letValueExpression() {
                let list = let_expr.letVariableDefinitionList().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: missing letVariableDefinitionList in LET ... IN".into(),
                    )
                })?;

                let variable_definitions = list
                    .letVariableDefinition_all()
                    .into_iter()
                    .map(|def| Self::build_let_variable_definition_from_context(&def))
                    .collect::<GqlResult<Vec<_>>>()?;

                let in_expr = let_expr.valueExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: missing valueExpression in LET ... IN".into())
                })?;

                let evaluation_expression =
                    Box::new(Self::build_expression_from_context(&in_expr)?);

                return Ok(Expression::InlineLet(InlineLetExpression {
                    variable_definitions,
                    evaluation_expression,
                    span: span.clone(),
                }));
            }
        }

        Self::build_expression_from_leaf_text(&primary.get_text(), span.clone())
    }

    /// Function to build a path constructor expression from a PathValueConstructorContext, extracting the sequence of path elements from the context, building the corresponding Expressions for each element, and returning the resulting Expression::PathConstructor with the list of element expressions.
    pub(super) fn build_path_constructor_expression_from_context(
        path_constructor: &PathValueConstructorContextAll,
    ) -> GqlResult<Expression> {
        let mut elements = Vec::new();

        if let Some(by_enum) = path_constructor.pathValueConstructorByEnumeration() {
            if let Some(list) = by_enum.pathElementList() {
                if let Some(start) = list.pathElementListStart() {
                    if let Some(node_ref) = start.nodeReferenceValueExpression() {
                        if let Some(prim) = node_ref.valueExpressionPrimary() {
                            elements.push(Self::build_primary_expression_from_context(&prim)?);
                        }
                    }
                }

                for step in list.pathElementListStep_all() {
                    if let Some(edge_ref) = step.edgeReferenceValueExpression() {
                        if let Some(prim) = edge_ref.valueExpressionPrimary() {
                            elements.push(Self::build_primary_expression_from_context(&prim)?);
                        }
                    }
                    if let Some(node_ref) = step.nodeReferenceValueExpression() {
                        if let Some(prim) = node_ref.valueExpressionPrimary() {
                            elements.push(Self::build_primary_expression_from_context(&prim)?);
                        }
                    }
                }
            }
        }

        Ok(Expression::PathConstructor(elements))
    }

    /// Function to build an Expression from a leaf text string, handling special cases for boolean literals, null literals, property access patterns, numeric literals, and variable references, and returning the corresponding Expression for each case. If none of the special cases match, it defaults to treating the text as a string literal.
    pub(super) fn build_expression_from_leaf_text(text: &str, span: Span) -> GqlResult<Expression> {
        let trimmed = text.trim();

        Self::validate_integer_literal_range(trimmed)?;

        if trimmed.eq_ignore_ascii_case("true") {
            return Ok(Expression::Literal(Literal::Boolean(true)));
        }
        if trimmed.eq_ignore_ascii_case("false") {
            return Ok(Expression::Literal(Literal::Boolean(false)));
        }
        if trimmed.eq_ignore_ascii_case("null") {
            return Ok(Expression::Literal(Literal::Null));
        }

        if let Ok(i) = trimmed.parse::<i64>() {
            return Ok(Expression::Literal(Literal::Integer(i)));
        }
        if let Ok(f) = trimmed.parse::<f64>() {
            return Ok(Expression::Literal(Literal::Float(f)));
        }

        if trimmed.contains('.') {
            let mut parts = trimmed.splitn(2, '.');
            let object = parts.next().unwrap_or_default().trim();
            let property = parts.next().unwrap_or_default().trim();

            if !object.is_empty() && !property.is_empty() {
                return Ok(Expression::PropertyAccess(PropertyAccess {
                    object: object.to_string(),
                    property: Self::identifier_text(property),
                    span: span.clone(),
                }));
            }
        }

        if trimmed
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '_')
            && !trimmed.is_empty()
        {
            return Ok(Expression::Variable {
                name: trimmed.to_string(),
                span: span.clone(),
            });
        }

        Ok(Expression::Literal(Self::build_literal_from_leaf_text(
            trimmed,
        )?))
    }

    /// Function to build a FunctionCall expression from an AggregateFunctionContext, handling various forms of aggregate functions such as COUNT(*), general set functions, and binary set functions, extracting the function name, distinct qualifier, and arguments as needed, and returning the corresponding Expression::FunctionCall. If the aggregate function context does not match any recognized form, it returns a parse error.
    pub(super) fn build_function_call_from_aggregate_context(
        aggregate_function: &AggregateFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(aggregate_function);

        if aggregate_function.COUNT().is_some() && aggregate_function.ASTERISK().is_some() {
            return Ok(Expression::FunctionCall(FunctionCall {
                name: fn_names::COUNT.to_string(),
                distinct: DistinctQualifier::None,
                arguments: vec![Expression::Literal(Literal::String("*".to_string()))],
                span: span.clone(),
            }));
        }

        if let Some(general_set_function) = aggregate_function.generalSetFunction() {
            let name = general_set_function
                .generalSetFunctionType()
                .map(|function_type| function_type.get_text())
                .unwrap_or_else(|| "UNKNOWN".to_string());

            let distinct = match general_set_function.setQuantifier() {
                Some(set_quantifier) if set_quantifier.DISTINCT().is_some() => {
                    DistinctQualifier::Distinct
                }
                Some(set_quantifier) if set_quantifier.ALL().is_some() => DistinctQualifier::All,
                _ => DistinctQualifier::None,
            };

            let arguments = general_set_function
                .valueExpression()
                .map(|expression| Self::build_expression_from_context(&expression))
                .transpose()?
                .map(|expression| vec![expression])
                .unwrap_or_default();

            return Ok(Expression::FunctionCall(FunctionCall {
                name,
                distinct,
                arguments,
                span: span.clone(),
            }));
        }

        if let Some(binary_set_function) = aggregate_function.binarySetFunction() {
            let name = binary_set_function
                .binarySetFunctionType()
                .map(|function_type| function_type.get_text())
                .unwrap_or_else(|| "UNKNOWN".to_string());

            let mut arguments = Vec::new();
            if let Some(left) = binary_set_function.dependentValueExpression() {
                if let Some(expr) = left.numericValueExpression() {
                    arguments.push(Expression::Literal(Self::build_literal_from_leaf_text(
                        &expr.get_text(),
                    )?));
                }
            }
            if let Some(right) = binary_set_function.independentValueExpression() {
                if let Some(expr) = right.numericValueExpression() {
                    arguments.push(Expression::Literal(Self::build_literal_from_leaf_text(
                        &expr.get_text(),
                    )?));
                }
            }

            return Ok(Expression::FunctionCall(FunctionCall {
                name,
                distinct: DistinctQualifier::None,
                arguments,
                span: span.clone(),
            }));
        }

        Self::parse_error("CST non valido: aggregateFunction non riconosciuta")
    }

    /// Function to build a PropertyAccess expression from a ValueExpressionPrimaryContext, extracting the property name and object reference from the context, and returning the corresponding PropertyAccess. If either the property name or object reference is missing, it returns a parse error indicating that the CST is not valid for a property access.
    pub(super) fn build_property_access_from_context(
        value_expression_primary: &ValueExpressionPrimaryContextAll,
    ) -> GqlResult<PropertyAccess> {
        let span = Self::extract_span(value_expression_primary);

        let property = value_expression_primary
            .propertyName()
            .map(|property_name| Self::identifier_text(&property_name.get_text()))
            .ok_or_else(|| {
                Self::parse_err("CST not valid: propertyName missing in property access".into())
            })?;

        let object = value_expression_primary
            .bindingVariableReference()
            .map(|binding_variable_reference| binding_variable_reference.get_text())
            .or_else(|| {
                value_expression_primary
                    .valueExpressionPrimary()
                    .map(|parent_primary| parent_primary.get_text())
            })
            .ok_or_else(|| {
                Self::parse_err("CST not valid: object missing in property access".into())
            })?;

        Ok(PropertyAccess {
            object,
            property,
            span: span.clone(),
        })
    }

    /// Function to build an ArrayExpression from a ListLiteralContext, extracting the list of elements from the context, building the corresponding Expressions for each element, and returning the resulting Expression::Array with the list of element expressions. If the CST structure does not match the expected format for a list literal, it returns a parse error indicating that the CST is not valid.
    pub(super) fn build_array_expression_from_list_literal(
        list_literal: &ListLiteralContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(list_literal);

        let constructor = list_literal
            .listValueConstructorByEnumeration()
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: listValueConstructorByEnumeration missing in listLiteral"
                        .into(),
                )
            })?;

        let elements = constructor
            .listElementList()
            .map(|list| {
                list.listElement_all()
                    .into_iter()
                    .map(|elem| {
                        let value = elem.valueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: valueExpression missing in listElement".into(),
                            )
                        })?;
                        Self::build_expression_from_context(&value)
                    })
                    .collect::<GqlResult<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Expression::Array(ArrayExpression {
            elements,
            span: span.clone(),
        }))
    }

    /// Function to build a MapExpression from a RecordLiteralContext, extracting the list of field specifications from the context, building the corresponding Expressions for each field value, and returning the resulting Expression::Map with the list of PropertyItems. If the CST structure does not match the expected format for a record literal, it returns a parse error indicating that the CST is not valid.
    pub(super) fn build_map_expression_from_record_literal(
        record_literal: &RecordLiteralContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(record_literal);

        let constructor = record_literal
            .recordConstructor()
            .ok_or_else(|| Self::parse_err("CST not valid: recordConstructor missing".into()))?;

        let properties = constructor
            .fieldsSpecification()
            .and_then(|spec| spec.fieldList())
            .map(|list| {
                list.field_all()
                    .into_iter()
                    .map(|field| {
                        let key =
                            field
                                .fieldName()
                                .map(|name| Self::identifier_text(&name.get_text()))
                                .ok_or_else(|| {
                                    Self::parse_err(
                                        "CST not valid: fieldName missing in record field".into(),
                                    )
                                })?;

                        let value_ctx = field.valueExpression().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: valueExpression missing in record field".into(),
                            )
                        })?;

                        let value = Self::build_expression_from_context(&value_ctx)?;
                        Ok(PropertyItem {
                            key,
                            value,
                            span: span.clone(),
                        })
                    })
                    .collect::<GqlResult<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();

        Ok(Expression::Map(properties))
    }

    /// Function to build a Literal from an UnsignedValueSpecificationContext, handling both unsigned literals and general value specifications, extracting the relevant text from the context, and returning the corresponding Literal. If the CST structure does not match the expected format for an unsigned value specification, it returns a parse error indicating that the CST is not valid.
    pub(super) fn build_literal_from_context(
        unsigned_value_specification: &UnsignedValueSpecificationContextAll,
    ) -> GqlResult<Literal> {
        if let Some(unsigned_literal) = unsigned_value_specification.unsignedLiteral() {
            return Self::build_literal_from_unsigned_literal(&unsigned_literal);
        }

        if let Some(general_value_specification) =
            unsigned_value_specification.generalValueSpecification()
        {
            return Ok(Self::build_literal_from_general_value_specification(
                &general_value_specification,
            ));
        }

        Self::parse_error(
            "CST not valid: unsignedValueSpecification without literal or general value",
        )
    }

    fn build_literal_from_unsigned_literal(
        unsigned_literal: &UnsignedLiteralContextAll,
    ) -> GqlResult<Literal> {
        if let Some(unsigned_numeric_literal) = unsigned_literal.unsignedNumericLiteral() {
            return Self::build_literal_from_leaf_text(
                &unsigned_numeric_literal.get_text(),
            );
        }

        if let Some(general_literal) = unsigned_literal.generalLiteral() {
            return Self::build_literal_from_general_literal(&general_literal);
        }

        Self::parse_error("CST not valid: unsignedLiteral without supported literal")
    }

    fn build_literal_from_general_literal(general_literal: &GeneralLiteralContextAll) -> GqlResult<Literal> {
        if let Some(boolean_literal) = general_literal.BOOLEAN_LITERAL() {
            return Ok(Literal::Boolean(boolean_literal.get_text().eq_ignore_ascii_case("true")));
        }

        if let Some(character_string_literal) = general_literal.characterStringLiteral() {
            return Self::build_literal_from_leaf_text(&character_string_literal.get_text());
        }

        if general_literal.nullLiteral().is_some() {
            return Ok(Literal::Null);
        }

        Ok(Literal::String(general_literal.get_text()))
    }

    fn build_literal_from_general_value_specification(
        general_value_specification: &GeneralValueSpecificationContextAll,
    ) -> Literal {
        if general_value_specification
            .dynamicParameterSpecification()
            .is_some()
        {
            return Literal::Parameter(general_value_specification.get_text());
        }

        Literal::String(general_value_specification.get_text())
    }

    /// Rejects integer syntax outside i64 without reinterpreting it as floating point.
    pub(super) fn validate_integer_literal_range(text: &str) -> GqlResult<()> {
        let digits = text.strip_prefix('-').or_else(|| text.strip_prefix('+')).unwrap_or(text);
        if !digits.is_empty() && digits.bytes().all(|byte| byte.is_ascii_digit())
            && text.parse::<i64>().is_err()
        {
            return Self::parse_error(format!(
                "Integer literal '{}' is outside the signed 64-bit range", text
            ));
        }
        Ok(())
    }

    /// Function to build a Literal from a leaf text string, handling various cases such as quoted strings, numeric literals, boolean literals, null literals, and defaulting to treating the text as a string literal if none of the other cases match.
    pub(super) fn build_literal_from_leaf_text(text: &str) -> GqlResult<Literal> {
        let trimmed = text.trim();
        if let Some(value) = Self::string_literal_text(trimmed) {
            Literal::String(value)
        if let Some(value) = Self::string_literal_text(trimmed) {
            Literal::String(value)
        Self::validate_integer_literal_range(trimmed)?;
        Ok(if (trimmed.starts_with('\'') && trimmed.ends_with('\''))
            || (trimmed.starts_with('"') && trimmed.ends_with('"'))
        {
            // Remove exactly the enclosing pair: quotes at the edges of the
            // content are data, and doubled delimiters represent one quote.
            let inner = &trimmed[1..trimmed.len() - 1];
            let value = if trimmed.starts_with('\'') {
                inner.replace("''", "'")
            } else {
                inner.replace("\"\"", "\"")
            };
            Literal::String(value)
        } else if let Ok(i) = trimmed.parse::<i64>() {
            Literal::Integer(i)
        } else if let Ok(f) = trimmed.parse::<f64>() {
            Literal::Float(f)
        } else if trimmed.eq_ignore_ascii_case("true") {
            Literal::Boolean(true)
        } else if trimmed.eq_ignore_ascii_case("false") {
            Literal::Boolean(false)
        } else if trimmed.eq_ignore_ascii_case("null") {
            Literal::Null
        } else {
            Literal::String(trimmed.to_string())
        })
    }

    /// Helper function to map a CompOpContext to the corresponding Operator enum variant, checking for each possible operator type in the context and returning the appropriate Operator. If the context does not match any recognized comparison operator, it returns a parse error indicating that the CST is not valid for a comparison operator.
    pub(super) fn operator_from_context(comp_op: &CompOpContextAll) -> GqlResult<Operator> {
        if comp_op.EQUALS_OPERATOR().is_some() {
            Ok(Operator::Equal)
        } else if comp_op.NOT_EQUALS_OPERATOR().is_some() {
            Ok(Operator::NotEqual)
        } else if comp_op.LESS_THAN_OR_EQUALS_OPERATOR().is_some() {
            Ok(Operator::LessEqual)
        } else if comp_op.GREATER_THAN_OR_EQUALS_OPERATOR().is_some() {
            Ok(Operator::GreaterEqual)
        } else if comp_op.LEFT_ANGLE_BRACKET().is_some() {
            Ok(Operator::LessThan)
        } else if comp_op.RIGHT_ANGLE_BRACKET().is_some() {
            Ok(Operator::GreaterThan)
        } else {
            Self::parse_error("CST not valid: unsupported comparison operator")
        }
    }
}
