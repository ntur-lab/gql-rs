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
                if signed.MINUS_SIGN().is_some() && Self::is_i64_min_magnitude(&inner.get_text()) {
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

                let truth_value = is_not
                    .truthValue()
                    .and_then(|tv| tv.BOOLEAN_LITERAL())
                    .ok_or_else(|| {
                        Self::parse_err("CST not valid: truth value missing in IS predicate".into())
                    })?
                    .get_text();
                let predicate = if truth_value.eq_ignore_ascii_case("true") {
                    IsPredicateKind::True
                } else if truth_value.eq_ignore_ascii_case("false") {
                    IsPredicateKind::False
                } else {
                    IsPredicateKind::Unknown
                };

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
                    // The normal form is a keyword modifier; its token text is the decoded value.
                    arguments.push(Expression::Literal(Literal::String(
                        form.get_text().to_ascii_uppercase(),
                    )));
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

                Self::build_graph_expression_from_context(&graph_expression)
            }
            ValueExpressionContextAll::ValueFunctionExprAltContext(value_fn_expr) => {
                let value_fn = value_fn_expr.valueFunction().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: valueFunction missing in ValueFunctionExprAlt".into(),
                    )
                })?;

                Self::build_value_function_from_context(&value_fn)
            }
            ValueExpressionContextAll::Error(_) => {
                Self::parse_error("CST Error node: the parser found a syntax error here")
            }
        }
    }

    /// Builds a function call node with no DISTINCT/ALL qualifier.
    fn builtin_call(name: &str, arguments: Vec<Expression>, span: Span) -> Expression {
        Expression::FunctionCall(FunctionCall {
            name: name.to_string(),
            distinct: DistinctQualifier::None,
            arguments,
            span,
        })
    }

    /// Builds an Expression from a NumericValueExpressionContext. The grammar keeps numeric
    /// arguments of built-in functions in a dedicated rule hierarchy, so signs, operators,
    /// primaries and nested numeric functions are built recursively here instead of being
    /// reconstructed from source text.
    pub(super) fn build_expression_from_numeric_value_expression(
        ctx: &NumericValueExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(ctx);

        if let Some(primary) = ctx.valueExpressionPrimary() {
            return Self::build_primary_expression_from_context(&primary);
        }

        if let Some(function) = ctx.numericValueFunction() {
            return Self::build_numeric_value_function_from_context(&function);
        }

        let operands = ctx.numericValueExpression_all();
        match operands.as_slice() {
            [operand] => {
                let operator = if ctx.MINUS_SIGN().is_some() {
                    Operator::Minus
                } else if ctx.PLUS_SIGN().is_some() {
                    Operator::Plus
                } else {
                    return Self::parse_error(
                        "CST not valid: sign missing in signed numeric expression",
                    );
                };

                // The magnitude of i64::MIN is not a valid positive i64 literal.
                if operator == Operator::Minus && Self::is_i64_min_magnitude(&operand.get_text())
                {
                    return Ok(Expression::Literal(Literal::Integer(i64::MIN)));
                }

                let expression = Self::build_expression_from_numeric_value_expression(operand)?;
                Ok(Expression::Unary(UnaryExpression {
                    operator,
                    expression: Box::new(expression),
                    span,
                }))
            }
            [left, right] => {
                let operator = if ctx.ASTERISK().is_some() {
                    Operator::Star
                } else if ctx.SOLIDUS().is_some() {
                    Operator::Slash
                } else if ctx.PLUS_SIGN().is_some() {
                    Operator::Plus
                } else if ctx.MINUS_SIGN().is_some() {
                    Operator::Minus
                } else {
                    return Self::parse_error(
                        "CST not valid: operator missing in binary numeric expression",
                    );
                };

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(Self::build_expression_from_numeric_value_expression(left)?),
                    operator,
                    right: Box::new(Self::build_expression_from_numeric_value_expression(right)?),
                    span,
                }))
            }
            _ => Self::parse_error("CST not valid: unsupported numericValueExpression shape"),
        }
    }

    /// Builds the argument held by an optional NumericValueExpressionContext, reporting `what`
    /// when the CST does not contain it.
    fn required_numeric_argument(
        ctx: Option<Rc<NumericValueExpressionContextAll>>,
        what: &str,
    ) -> GqlResult<Expression> {
        let ctx = ctx.ok_or_else(|| {
            Self::parse_err(format!("CST not valid: numericValueExpression missing in {what}"))
        })?;
        Self::build_expression_from_numeric_value_expression(&ctx)
    }

    /// Builds an Expression from a ValueFunctionContext, dispatching to the dedicated builder of
    /// every built-in function family admitted by the grammar and to the user-defined function
    /// path.
    pub(super) fn build_value_function_from_context(
        value_fn: &ValueFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(value_fn);

        if let Some(numeric_fn) = value_fn.numericValueFunction() {
            return Self::build_numeric_value_function_from_context(&numeric_fn);
        }

        if let Some(dt_sub) = value_fn.datetimeSubtraction() {
            return Self::build_datetime_subtraction_from_context(&dt_sub);
        }

        if let Some(dt_fn) = value_fn.datetimeValueFunction() {
            return Self::build_datetime_value_function_from_context(&dt_fn);
        }

        if let Some(duration_fn) = value_fn.durationValueFunction() {
            return Self::build_duration_value_function_from_context(&duration_fn);
        }

        if let Some(char_fn) = value_fn.characterOrByteStringFunction() {
            return Self::build_character_or_byte_string_function_from_context(&char_fn);
        }

        if let Some(list_fn) = value_fn.listValueFunction() {
            return Self::build_list_value_function_from_context(&list_fn);
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

            return Ok(Self::builtin_call(&name_parts.join("."), arguments, span));
        }

        Self::parse_error(format!(
            "Type of valueFunction not supported or not yet mapped: {}",
            value_fn.get_text()
        ))
    }

    /// Builds the character and byte string functions (UPPER/LOWER, LEFT/RIGHT, TRIM,
    /// LTRIM/RTRIM/BTRIM, NORMALIZE).
    fn build_character_or_byte_string_function_from_context(
        char_fn: &CharacterOrByteStringFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(char_fn);

        if let Some(fold) = char_fn.foldCharacterString() {
            let name = if fold.UPPER().is_some() {
                fn_names::UPPER
            } else {
                fn_names::LOWER
            };
            let inner = fold.valueExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: valueExpression missing in foldCharacterString".into())
            })?;
            let arg = Self::build_expression_from_context(&inner)?;
            return Ok(Self::builtin_call(name, vec![arg], span));
        }

        if let Some(sub_fn) = char_fn.subCharacterOrByteString() {
            return Self::build_sub_character_function_from_context(&sub_fn);
        }

        if let Some(trim_fn) = char_fn.trimSingleCharacterOrByteString() {
            return Self::build_trim_function_from_context(&trim_fn);
        }

        if let Some(trim_multi_fn) = char_fn.trimMultiCharacterCharacterString() {
            let name = if trim_multi_fn.LTRIM().is_some() {
                fn_names::LTRIM
            } else if trim_multi_fn.RTRIM().is_some() {
                fn_names::RTRIM
            } else {
                fn_names::BTRIM
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

            return Ok(Self::builtin_call(name, arguments, span));
        }

        if let Some(normalize_fn) = char_fn.normalizeCharacterString() {
            let source_ctx = normalize_fn.valueExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: valueExpression missing in NORMALIZE".into())
            })?;

            let mut arguments = vec![Self::build_expression_from_context(&source_ctx)?];
            if let Some(form) = normalize_fn.normalForm() {
                // The normal form is a keyword modifier; its token text is the decoded value.
                arguments.push(Expression::Literal(Literal::String(
                    form.get_text().to_ascii_uppercase(),
                )));
            }

            return Ok(Self::builtin_call(fn_names::NORMALIZE, arguments, span));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported character or byte string function: {}",
            char_fn.get_text()
        ))
    }

    /// Builds the list functions TRIM(list, count) and ELEMENTS(path).
    fn build_list_value_function_from_context(
        list_fn: &ListValueFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(list_fn);

        if let Some(trim_list_fn) = list_fn.trimListFunction() {
            let list_expr = trim_list_fn
                .listValueExpression()
                .and_then(|list| list.valueExpression())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: listValueExpression missing in TRIM(list)".into())
                })
                .and_then(|expr| Self::build_expression_from_context(&expr))?;
            let trim_count = Self::required_numeric_argument(
                trim_list_fn.numericValueExpression(),
                "TRIM(list)",
            )?;

            return Ok(Self::builtin_call(
                fn_names::TRIM,
                vec![list_expr, trim_count],
                span,
            ));
        }

        if let Some(elements_fn) = list_fn.elementsFunction() {
            let arg = elements_fn
                .pathValueExpression()
                .and_then(|path| path.valueExpression())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: pathValueExpression missing in ELEMENTS".into())
                })
                .and_then(|expr| Self::build_expression_from_context(&expr))?;

            return Ok(Self::builtin_call(fn_names::ELEMENTS, vec![arg], span));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported list value function: {}",
            list_fn.get_text()
        ))
    }

    /// Builds the numeric value functions (length, cardinality, ABS, MOD, trigonometric and
    /// logarithmic functions, POWER, SQRT, FLOOR, CEIL). Numeric arguments are built
    /// structurally from their numericValueExpression subtrees.
    pub(super) fn build_numeric_value_function_from_context(
        numeric_fn: &NumericValueFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(numeric_fn);

        if let Some(cardinality_expr) = numeric_fn.cardinalityExpression() {
            let (name, inner) = if cardinality_expr.SIZE().is_some() {
                (
                    fn_names::SIZE,
                    cardinality_expr
                        .listValueExpression()
                        .and_then(|e| e.valueExpression()),
                )
            } else {
                (
                    fn_names::CARDINALITY,
                    cardinality_expr
                        .cardinalityExpressionArgument()
                        .and_then(|a| a.valueExpression()),
                )
            };
            let inner = inner.ok_or_else(|| {
                Self::parse_err(format!("CST not valid: argument missing in {name}"))
            })?;
            let arg = Self::build_expression_from_context(&inner)?;
            return Ok(Self::builtin_call(name, vec![arg], span));
        }

        if let Some(length_expr) = numeric_fn.lengthExpression() {
            let (name, inner) = if let Some(char_len) = length_expr.charLengthExpression() {
                let name = if char_len.CHAR_LENGTH().is_some() {
                    fn_names::CHAR_LENGTH
                } else {
                    fn_names::CHARACTER_LENGTH
                };
                (
                    name,
                    char_len
                        .characterStringValueExpression()
                        .and_then(|e| e.valueExpression()),
                )
            } else if let Some(byte_len) = length_expr.byteLengthExpression() {
                let name = if byte_len.BYTE_LENGTH().is_some() {
                    fn_names::BYTE_LENGTH
                } else {
                    fn_names::OCTET_LENGTH
                };
                (
                    name,
                    byte_len
                        .byteStringValueExpression()
                        .and_then(|e| e.valueExpression()),
                )
            } else if let Some(path_len) = length_expr.pathLengthExpression() {
                (
                    fn_names::PATH_LENGTH,
                    path_len.pathValueExpression().and_then(|e| e.valueExpression()),
                )
            } else {
                return Self::parse_error(format!(
                    "Numeric function variant not supported: {}",
                    numeric_fn.get_text()
                ));
            };

            let inner = inner.ok_or_else(|| {
                Self::parse_err(format!("CST not valid: valueExpression missing in {name}"))
            })?;
            let arg = Self::build_expression_from_context(&inner)?;
            return Ok(Self::builtin_call(name, vec![arg], span));
        }

        if let Some(abs_fn) = numeric_fn.absoluteValueExpression() {
            let value_expr = abs_fn.valueExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: valueExpression missing in ABS".into())
            })?;
            let arg = Self::build_expression_from_context(&value_expr)?;
            return Ok(Self::builtin_call(fn_names::ABS, vec![arg], span));
        }

        if let Some(floor_fn) = numeric_fn.floorFunction() {
            let arg = Self::required_numeric_argument(floor_fn.numericValueExpression(), "FLOOR")?;
            return Ok(Self::builtin_call(fn_names::FLOOR, vec![arg], span));
        }

        if let Some(ceil_fn) = numeric_fn.ceilingFunction() {
            let arg = Self::required_numeric_argument(ceil_fn.numericValueExpression(), "CEIL")?;
            return Ok(Self::builtin_call(fn_names::CEIL, vec![arg], span));
        }

        if let Some(sqrt_fn) = numeric_fn.squareRoot() {
            let arg = Self::required_numeric_argument(sqrt_fn.numericValueExpression(), "SQRT")?;
            return Ok(Self::builtin_call(fn_names::SQRT, vec![arg], span));
        }

        if let Some(power_fn) = numeric_fn.powerFunction() {
            let base = Self::required_numeric_argument(
                power_fn
                    .numericValueExpressionBase()
                    .and_then(|b| b.numericValueExpression()),
                "POWER base",
            )?;
            let exponent = Self::required_numeric_argument(
                power_fn
                    .numericValueExpressionExponent()
                    .and_then(|e| e.numericValueExpression()),
                "POWER exponent",
            )?;
            return Ok(Self::builtin_call(fn_names::POWER, vec![base, exponent], span));
        }

        if let Some(mod_fn) = numeric_fn.modulusExpression() {
            let dividend = Self::required_numeric_argument(
                mod_fn
                    .numericValueExpressionDividend()
                    .and_then(|d| d.numericValueExpression()),
                "MOD dividend",
            )?;
            let divisor = Self::required_numeric_argument(
                mod_fn
                    .numericValueExpressionDivisor()
                    .and_then(|d| d.numericValueExpression()),
                "MOD divisor",
            )?;
            return Ok(Self::builtin_call(fn_names::MOD, vec![dividend, divisor], span));
        }

        if let Some(trig_fn) = numeric_fn.trigonometricFunction() {
            let name = trig_fn
                .trigonometricFunctionName()
                .map(|n| n.get_text().to_ascii_uppercase())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: trigonometricFunctionName missing in trigonometric function"
                            .into(),
                    )
                })?;
            let arg = Self::required_numeric_argument(
                trig_fn.numericValueExpression(),
                "trigonometric function",
            )?;
            return Ok(Self::builtin_call(&name, vec![arg], span));
        }

        if let Some(log_fn) = numeric_fn.generalLogarithmFunction() {
            let base = Self::required_numeric_argument(
                log_fn
                    .generalLogarithmBase()
                    .and_then(|b| b.numericValueExpression()),
                "LOG base",
            )?;
            let arg = Self::required_numeric_argument(
                log_fn
                    .generalLogarithmArgument()
                    .and_then(|a| a.numericValueExpression()),
                "LOG argument",
            )?;
            return Ok(Self::builtin_call(fn_names::LOG, vec![base, arg], span));
        }

        if let Some(log10_fn) = numeric_fn.commonLogarithm() {
            let arg = Self::required_numeric_argument(log10_fn.numericValueExpression(), "LOG10")?;
            return Ok(Self::builtin_call(fn_names::LOG10, vec![arg], span));
        }

        if let Some(ln_fn) = numeric_fn.naturalLogarithm() {
            let arg = Self::required_numeric_argument(ln_fn.numericValueExpression(), "LN")?;
            return Ok(Self::builtin_call(fn_names::LN, vec![arg], span));
        }

        if let Some(exp_fn) = numeric_fn.exponentialFunction() {
            let arg = Self::required_numeric_argument(exp_fn.numericValueExpression(), "EXP")?;
            return Ok(Self::builtin_call(fn_names::EXP, vec![arg], span));
        }

        Self::parse_error(format!(
            "Numeric function variant not supported: {}",
            numeric_fn.get_text()
        ))
    }

    /// Builds DURATION_BETWEEN(left, right) with its optional temporal duration qualifier.
    fn build_datetime_subtraction_from_context(
        dt_sub: &DatetimeSubtractionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(dt_sub);

        let params = dt_sub.datetimeSubtractionParameters().ok_or_else(|| {
            Self::parse_err("CST not valid: datetimeSubtractionParameters missing".into())
        })?;

        let left_ctx = params
            .datetimeValueExpression1()
            .and_then(|x| x.datetimeValueExpression())
            .and_then(|x| x.valueExpression())
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: datetimeValueExpression1 missing in DURATION_BETWEEN".into(),
                )
            })?;

        let right_ctx = params
            .datetimeValueExpression2()
            .and_then(|x| x.datetimeValueExpression())
            .and_then(|x| x.valueExpression())
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: datetimeValueExpression2 missing in DURATION_BETWEEN".into(),
                )
            })?;

        let mut arguments = vec![
            Self::build_expression_from_context(&left_ctx)?,
            Self::build_expression_from_context(&right_ctx)?,
        ];

        if let Some(qualifier) = dt_sub.temporalDurationQualifier() {
            // The qualifier is a keyword modifier; it is preserved in its canonical spelling,
            // matching `TypeSpec::Duration::qualifier`.
            let qualifier = if qualifier.YEAR().is_some() {
                "YEAR TO MONTH"
            } else {
                "DAY TO SECOND"
            };
            arguments.push(Expression::Literal(Literal::String(qualifier.to_string())));
        }

        Ok(Self::builtin_call(fn_names::DURATION_BETWEEN, arguments, span))
    }

    /// Builds DURATION(...) and ABS(duration).
    fn build_duration_value_function_from_context(
        duration_fn: &DurationValueFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(duration_fn);

        if let Some(duration_ctor) = duration_fn.durationFunction() {
            let params = duration_ctor.durationFunctionParameters().ok_or_else(|| {
                Self::parse_err("CST not valid: durationFunctionParameters missing in DURATION".into())
            })?;
            let argument = Self::build_temporal_function_argument(
                params.durationString().and_then(|s| s.characterStringLiteral()),
                params.recordConstructor(),
                "DURATION",
            )?;

            return Ok(Self::builtin_call(fn_names::DURATION, vec![argument], span));
        }

        if let Some(abs_duration) = duration_fn.absoluteValueExpression() {
            let value_expr = abs_duration.valueExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: valueExpression missing in duration ABS".into())
            })?;
            let arg = Self::build_expression_from_context(&value_expr)?;
            return Ok(Self::builtin_call(fn_names::ABS, vec![arg], span));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported duration value function: {}",
            duration_fn.get_text()
        ))
    }

    /// Builds the argument of a temporal constructor function: either a character string
    /// literal (decoded as a string literal) or a record constructor (built as a map).
    fn build_temporal_function_argument(
        string_literal: Option<Rc<CharacterStringLiteralContextAll>>,
        record: Option<Rc<RecordConstructorContextAll>>,
        function: &str,
    ) -> GqlResult<Expression> {
        if let Some(string_literal) = string_literal {
            return Ok(Expression::Literal(Literal::String(
                Self::decode_character_string_literal(&string_literal)?,
            )));
        }

        if let Some(record) = record {
            return Self::build_map_expression_from_record_constructor(&record);
        }

        Self::parse_error(format!(
            "CST not valid: {function} parameters must be a string or a record constructor"
        ))
    }

    /// Builds the date, time and datetime constructor functions.
    fn build_datetime_value_function_from_context(
        dt_fn: &DatetimeValueFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(dt_fn);

        if let Some(date_fn) = dt_fn.dateFunction() {
            if date_fn.CURRENT_DATE().is_some() {
                return Ok(Self::builtin_call(fn_names::CURRENT_DATE, vec![], span));
            }
            let arguments = date_fn
                .dateFunctionParameters()
                .map(|params| {
                    Self::build_temporal_function_argument(
                        params.dateString().and_then(|s| s.characterStringLiteral()),
                        params.recordConstructor(),
                        fn_names::DATE,
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            return Ok(Self::builtin_call(fn_names::DATE, arguments, span));
        }

        if let Some(time_fn) = dt_fn.timeFunction() {
            if time_fn.CURRENT_TIME().is_some() {
                return Ok(Self::builtin_call(fn_names::CURRENT_TIME, vec![], span));
            }
            let arguments = time_fn
                .timeFunctionParameters()
                .map(|params| {
                    Self::build_temporal_function_argument(
                        params.timeString().and_then(|s| s.characterStringLiteral()),
                        params.recordConstructor(),
                        fn_names::ZONED_TIME,
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            return Ok(Self::builtin_call(fn_names::ZONED_TIME, arguments, span));
        }

        if let Some(local_time_fn) = dt_fn.localtimeFunction() {
            let arguments = local_time_fn
                .timeFunctionParameters()
                .map(|params| {
                    Self::build_temporal_function_argument(
                        params.timeString().and_then(|s| s.characterStringLiteral()),
                        params.recordConstructor(),
                        fn_names::LOCAL_TIME,
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            return Ok(Self::builtin_call(fn_names::LOCAL_TIME, arguments, span));
        }

        if let Some(datetime_fn) = dt_fn.datetimeFunction() {
            if datetime_fn.CURRENT_TIMESTAMP().is_some() {
                return Ok(Self::builtin_call(fn_names::CURRENT_TIMESTAMP, vec![], span));
            }
            let arguments = datetime_fn
                .datetimeFunctionParameters()
                .map(|params| {
                    Self::build_temporal_function_argument(
                        params
                            .datetimeString()
                            .and_then(|s| s.characterStringLiteral()),
                        params.recordConstructor(),
                        fn_names::ZONED_DATETIME,
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            return Ok(Self::builtin_call(fn_names::ZONED_DATETIME, arguments, span));
        }

        if let Some(local_dt_fn) = dt_fn.localdatetimeFunction() {
            let name = if local_dt_fn.LOCAL_TIMESTAMP().is_some() {
                fn_names::LOCAL_TIMESTAMP
            } else if local_dt_fn.LOCAL_DATETIME().is_some() {
                fn_names::LOCAL_DATETIME
            } else {
                return Self::parse_error("LOCAL_TIMESTAMP/LOCAL_DATETIME parsing failed");
            };

            let arguments = local_dt_fn
                .datetimeFunctionParameters()
                .map(|params| {
                    Self::build_temporal_function_argument(
                        params
                            .datetimeString()
                            .and_then(|s| s.characterStringLiteral()),
                        params.recordConstructor(),
                        name,
                    )
                })
                .transpose()?
                .into_iter()
                .collect();
            return Ok(Self::builtin_call(name, arguments, span));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported datetime value function: {}",
            dt_fn.get_text()
        ))
    }

    /// Builds the expression of a `[PROPERTY] GRAPH <graph expression>` value. Object
    /// expressions and binding variables map to existing expression nodes and substituted
    /// parameters map to parameter references. Catalog graph references and the current/home
    /// graph have no structural representation yet and fail explicitly.
    fn build_graph_expression_from_context(
        graph_expression: &GraphExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(graph_expression);

        if let Some(object) = graph_expression.objectExpressionPrimary() {
            return Self::build_object_expression_primary_from_context(&object);
        }

        if let Some(name) = graph_expression.objectNameOrBindingVariable() {
            return Ok(Expression::Variable {
                name: name.get_text(),
                span,
            });
        }

        if let Some(reference) = graph_expression.graphReference() {
            if let Some(parameter) = reference.referenceParameterSpecification() {
                return Ok(Expression::Literal(Literal::Parameter(parameter.get_text())));
            }
            return Self::unsupported_construct(
                &format!("graph reference 'GRAPH {}'", reference.get_text()),
                "issue #15",
                span,
            );
        }

        if graph_expression.currentGraph().is_some() {
            return Self::unsupported_construct(
                &format!("current graph expression 'GRAPH {}'", graph_expression.get_text()),
                "issue #15",
                span,
            );
        }

        Self::parse_error(format!(
            "CST not valid: unsupported graph expression: {}",
            graph_expression.get_text()
        ))
    }

    /// Builds an Expression from an ObjectExpressionPrimaryContext (`VARIABLE <primary>`, a
    /// parenthesized expression, or a non-parenthesized primary special case).
    fn build_object_expression_primary_from_context(
        object: &ObjectExpressionPrimaryContextAll,
    ) -> GqlResult<Expression> {
        if let Some(primary) = object.valueExpressionPrimary() {
            return Self::build_primary_expression_from_context(&primary);
        }

        if let Some(inner) = object
            .parenthesizedValueExpression()
            .and_then(|paren| paren.valueExpression())
        {
            return Self::build_expression_from_context(&inner);
        }

        if let Some(special) = object.nonParenthesizedValueExpressionPrimarySpecialCase() {
            return Self::build_expression_from_non_parenthesized_special_case(&special);
        }

        Self::parse_error(format!(
            "CST not valid: unsupported object expression: {}",
            object.get_text()
        ))
    }

    /// Returns true when `text` is an unsigned integer literal whose magnitude is that of
    /// i64::MIN, which is only representable once negated.
    fn is_i64_min_magnitude(text: &str) -> bool {
        Self::decode_unsigned_integer_magnitude(text) == Some(1u128 << 63)
    }

    /// Decodes an unsigned integer literal (decimal, 0x, 0o or 0b, with optional underscores)
    /// into its magnitude. Returns `None` for any other text.
    fn decode_unsigned_integer_magnitude(text: &str) -> Option<u128> {
        let cleaned = text.trim().replace('_', "").to_ascii_lowercase();
        let (digits, radix) = if let Some(digits) = cleaned.strip_prefix("0x") {
            (digits, 16)
        } else if let Some(digits) = cleaned.strip_prefix("0o") {
            (digits, 8)
        } else if let Some(digits) = cleaned.strip_prefix("0b") {
            (digits, 2)
        } else {
            (cleaned.as_str(), 10)
        };

        if digits.is_empty() || !digits.chars().all(|c| c.is_digit(radix)) {
            return None;
        }
        u128::from_str_radix(digits, radix).ok()
    }

    /// Function to build a LEFT or RIGHT function call from a SubCharacterOrByteStringContext, extracting the source string and length arguments and building the corresponding FunctionCall expression.
    pub(super) fn build_sub_character_function_from_context(
        ctx: &SubCharacterOrByteStringContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(ctx);

        let name = if ctx.LEFT().is_some() {
            fn_names::LEFT
        } else {
            fn_names::RIGHT
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
        Self::required_numeric_argument(ctx.numericValueExpression(), "stringLength")
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
            return Self::build_expression_from_unsigned_value_specification(
                &unsigned_value_specification,
            );
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

        if let Some(paren) = value_expression_primary.parenthesizedValueExpression()
            && let Some(inner) = paren.valueExpression()
        {
            return Self::build_expression_from_context(&inner);
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
            if let Some(name) = pattern.nodeTypeName()
                && let Some(identifier) = name.identifier()
            {
                return Some(Self::identifier_text(&identifier.get_text()));
            }

            if let Some(alias) = pattern.localNodeTypeAlias()
                && let Some(identifier) = alias.regularIdentifier()
            {
                return Some(Self::identifier_text(&identifier.get_text()));
            }
        }

        if let Some(phrase) = cst.nodeTypePhrase() {
            if let Some(filler) = phrase.nodeTypePhraseFiller()
                && let Some(name) = filler.nodeTypeName()
                && let Some(identifier) = name.identifier()
            {
                return Some(Self::identifier_text(&identifier.get_text()));
            }

            if let Some(alias) = phrase.localNodeTypeAlias()
                && let Some(identifier) = alias.regularIdentifier()
            {
                return Some(Self::identifier_text(&identifier.get_text()));
            }
        }

        None
    }

    /// Function to extract the identifier of an edge type from an EdgeTypeSpecificationContext, checking both the pattern and phrase forms of edge type specification and returning the identifier if found.
    fn extract_edge_type_identifier(cst: &EdgeTypeSpecificationContextAll) -> Option<String> {
        if let Some(pattern) = cst.edgeTypePattern()
            && let Some(name) = pattern.edgeTypeName()
            && let Some(identifier) = name.identifier()
        {
            return Some(identifier.get_text());
        }

        if let Some(phrase) = cst.edgeTypePhrase()
            && let Some(filler) = phrase.edgeTypePhraseFiller()
            && let Some(name) = filler.edgeTypeName()
            && let Some(identifier) = name.identifier()
        {
            return Some(Self::identifier_text(&identifier.get_text()));
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
                fn_names::COALESCE
            } else {
                fn_names::NULLIF
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

        if let Some(object) = binding_table.objectExpressionPrimary() {
            return Self::build_object_expression_primary_from_context(&object);
        }

        if let Some(name) = binding_table.objectNameOrBindingVariable() {
            return Ok(Expression::Variable {
                name: name.get_text(),
                span,
            });
        }

        if let Some(reference) = binding_table.bindingTableReference() {
            if let Some(parameter) = reference.referenceParameterSpecification() {
                return Ok(Expression::Literal(Literal::Parameter(parameter.get_text())));
            }
            return Self::unsupported_construct(
                &format!("binding table reference 'TABLE {}'", reference.get_text()),
                "issue #15",
                span,
            );
        }

        Self::parse_error(format!(
            "CST not valid: unsupported binding table expression: {}",
            binding_table.get_text()
        ))
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
                    label_expression: label_part
                        .labelExpression()
                        .map(|le| Self::identifiers_text(&le.get_text())),
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

        Self::parse_error(format!(
            "CST not valid: unsupported WHEN operand: {}",
            operand.get_text()
        ))
    }

    /// Function to build an Expression from a NonParenthesizedValueExpressionPrimaryContext, which is either a binding variable reference or a non-parenthesized special case.
    pub(super) fn build_expression_from_non_parenthesized_primary(
        primary: &NonParenthesizedValueExpressionPrimaryContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(primary);

        if let Some(binding) = primary.bindingVariableReference() {
            return Ok(Expression::Variable {
                name: binding.get_text(),
                span,
            });
        }

        if let Some(special) = primary.nonParenthesizedValueExpressionPrimarySpecialCase() {
            return Self::build_expression_from_non_parenthesized_special_case(&special);
        }

        Self::parse_error(format!(
            "CST not valid: unsupported non-parenthesized value expression primary: {}",
            primary.get_text()
        ))
    }

    /// Function to build an Expression from a NonParenthesizedValueExpressionPrimarySpecialCaseContext, handling aggregate functions, unsigned value specifications, path constructors, property references, value queries, CASE expressions, CAST specifications, ELEMENT_ID function calls, and inline LET expressions.
    pub(super) fn build_expression_from_non_parenthesized_special_case(
        special: &NonParenthesizedValueExpressionPrimarySpecialCaseContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(special);

        if let Some(aggregate_fn) = special.aggregateFunction() {
            return Self::build_function_call_from_aggregate_context(&aggregate_fn);
        }

        if let Some(unsigned) = special.unsignedValueSpecification() {
            return Self::build_expression_from_unsigned_value_specification(&unsigned);
        }

        if let Some(path_constructor) = special.pathValueConstructor() {
            return Self::build_path_constructor_expression_from_context(&path_constructor);
        }

        if let Some(base_primary) = special.valueExpressionPrimary() {
            let property_name = special.propertyName().ok_or_else(|| {
                Self::parse_err("CST not valid: propertyName missing in property reference".into())
            })?;
            return Ok(Expression::PropertyAccess(PropertyAccess {
                object: Self::property_reference_object(&base_primary)?,
                property: Self::identifier_text(&property_name.get_text()),
                span,
            }));
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
                span,
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

            return Ok(Self::builtin_call(
                fn_names::ELEMENT_ID,
                vec![Expression::Variable {
                    name: element_ref.get_text(),
                    span: span.clone(),
                }],
                span,
            ));
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

            let evaluation_expression = Box::new(Self::build_expression_from_context(&in_expr)?);

            return Ok(Expression::InlineLet(InlineLetExpression {
                variable_definitions,
                evaluation_expression,
                span,
            }));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported value expression primary: {}",
            special.get_text()
        ))
    }

    /// Function to build a path constructor expression from a PathValueConstructorContext, extracting the sequence of path elements from the context, building the corresponding Expressions for each element, and returning the resulting Expression::PathConstructor with the list of element expressions.
    pub(super) fn build_path_constructor_expression_from_context(
        path_constructor: &PathValueConstructorContextAll,
    ) -> GqlResult<Expression> {
        let mut elements = Vec::new();

        if let Some(by_enum) = path_constructor.pathValueConstructorByEnumeration()
            && let Some(list) = by_enum.pathElementList()
        {
            if let Some(start) = list.pathElementListStart()
                && let Some(node_ref) = start.nodeReferenceValueExpression()
                && let Some(prim) = node_ref.valueExpressionPrimary()
            {
                elements.push(Self::build_primary_expression_from_context(&prim)?);
            }

            for step in list.pathElementListStep_all() {
                if let Some(edge_ref) = step.edgeReferenceValueExpression()
                    && let Some(prim) = edge_ref.valueExpressionPrimary()
                {
                    elements.push(Self::build_primary_expression_from_context(&prim)?);
                }
                if let Some(node_ref) = step.nodeReferenceValueExpression()
                    && let Some(prim) = node_ref.valueExpressionPrimary()
                {
                    elements.push(Self::build_primary_expression_from_context(&prim)?);
                }
            }
        }

        Ok(Expression::PathConstructor(elements))
    }

    /// Returns the DISTINCT/ALL qualifier carried by an optional set quantifier.
    fn distinct_qualifier_from_set_quantifier(
        set_quantifier: Option<Rc<SetQuantifierContextAll>>,
    ) -> DistinctQualifier {
        match set_quantifier {
            Some(set_quantifier) if set_quantifier.DISTINCT().is_some() => {
                DistinctQualifier::Distinct
            }
            Some(set_quantifier) if set_quantifier.ALL().is_some() => DistinctQualifier::All,
            _ => DistinctQualifier::None,
        }
    }

    /// Function to build a FunctionCall expression from an AggregateFunctionContext, handling COUNT(*), general set functions, and binary set functions. Function names are canonicalized to upper case, set quantifiers are preserved, and every argument is built structurally.
    pub(super) fn build_function_call_from_aggregate_context(
        aggregate_function: &AggregateFunctionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(aggregate_function);

        if aggregate_function.COUNT().is_some() && aggregate_function.ASTERISK().is_some() {
            // COUNT(*) keeps its established representation: a single "*" marker argument.
            return Ok(Self::builtin_call(
                fn_names::COUNT,
                vec![Expression::Literal(Literal::String("*".to_string()))],
                span,
            ));
        }

        if let Some(general_set_function) = aggregate_function.generalSetFunction() {
            let name = general_set_function
                .generalSetFunctionType()
                .map(|function_type| function_type.get_text().to_ascii_uppercase())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: generalSetFunctionType missing in aggregate function"
                            .into(),
                    )
                })?;

            let argument = general_set_function
                .valueExpression()
                .ok_or_else(|| {
                    Self::parse_err(format!("CST not valid: argument missing in {name}"))
                })
                .and_then(|expression| Self::build_expression_from_context(&expression))?;

            return Ok(Expression::FunctionCall(FunctionCall {
                name,
                distinct: Self::distinct_qualifier_from_set_quantifier(
                    general_set_function.setQuantifier(),
                ),
                arguments: vec![argument],
                span,
            }));
        }

        if let Some(binary_set_function) = aggregate_function.binarySetFunction() {
            let name = binary_set_function
                .binarySetFunctionType()
                .map(|function_type| function_type.get_text().to_ascii_uppercase())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: binarySetFunctionType missing in aggregate function".into(),
                    )
                })?;

            let dependent = binary_set_function.dependentValueExpression().ok_or_else(|| {
                Self::parse_err(format!(
                    "CST not valid: dependent value expression missing in {name}"
                ))
            })?;
            let independent = binary_set_function
                .independentValueExpression()
                .ok_or_else(|| {
                    Self::parse_err(format!(
                        "CST not valid: independent value expression missing in {name}"
                    ))
                })?;

            let arguments = vec![
                Self::required_numeric_argument(dependent.numericValueExpression(), &name)?,
                Self::required_numeric_argument(independent.numericValueExpression(), &name)?,
            ];

            return Ok(Expression::FunctionCall(FunctionCall {
                distinct: Self::distinct_qualifier_from_set_quantifier(dependent.setQuantifier()),
                name,
                arguments,
                span,
            }));
        }

        Self::parse_error("CST not valid: unrecognized aggregateFunction")
    }

    /// Returns the binding variable that a property reference is applied to. The public
    /// `PropertyAccess` node can only reference a binding variable by name, so property
    /// references on any other expression (nested properties, parameters, function results,
    /// literals) fail explicitly instead of storing the source text of the object.
    fn property_reference_object(base_primary: &ValueExpressionPrimaryContextAll) -> GqlResult<String> {
        if let Some(binding) = base_primary.bindingVariableReference() {
            return Ok(binding.get_text());
        }

        if let Some(inner) = base_primary
            .parenthesizedValueExpression()
            .and_then(|paren| paren.valueExpression())
            && let Expression::Variable { name, .. } = Self::build_expression_from_context(&inner)?
        {
            return Ok(name);
        }

        Self::unsupported_construct(
            &format!(
                "property reference on non-variable expression '{}'",
                base_primary.get_text()
            ),
            "the 0.1.0 expression redesign",
            Self::extract_span(base_primary),
        )
    }

    /// Function to build a PropertyAccess expression from a ValueExpressionPrimaryContext (`<primary>.<property>`).
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

        let base_primary = value_expression_primary
            .valueExpressionPrimary()
            .ok_or_else(|| {
                Self::parse_err("CST not valid: object missing in property access".into())
            })?;

        Ok(PropertyAccess {
            object: Self::property_reference_object(&base_primary)?,
            property,
            span,
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

    /// Function to build a MapExpression from a RecordLiteralContext.
    pub(super) fn build_map_expression_from_record_literal(
        record_literal: &RecordLiteralContextAll,
    ) -> GqlResult<Expression> {
        let constructor = record_literal
            .recordConstructor()
            .ok_or_else(|| Self::parse_err("CST not valid: recordConstructor missing".into()))?;

        Self::build_map_expression_from_record_constructor(&constructor)
    }

    /// Function to build a MapExpression from a RecordConstructorContext, building every field value structurally.
    pub(super) fn build_map_expression_from_record_constructor(
        constructor: &RecordConstructorContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(constructor);

        let properties = constructor
            .fieldsSpecification()
            .and_then(|spec| spec.fieldList())
            .map(|list| {
                list.field_all()
                    .into_iter()
                    .map(|field| {
                        let key = field
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

    /// Function to build an Expression from an UnsignedValueSpecificationContext: list and record literals become array and map expressions, `SESSION_USER` becomes a niladic built-in call, and every other form becomes a literal.
    pub(super) fn build_expression_from_unsigned_value_specification(
        unsigned_value_specification: &UnsignedValueSpecificationContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(unsigned_value_specification);

        if let Some(general_literal) = unsigned_value_specification
            .unsignedLiteral()
            .and_then(|literal| literal.generalLiteral())
        {
            if let Some(list_literal) = general_literal.listLiteral() {
                return Self::build_array_expression_from_list_literal(&list_literal);
            }
            if let Some(record_literal) = general_literal.recordLiteral() {
                return Self::build_map_expression_from_record_literal(&record_literal);
            }
        }

        if let Some(general_value_specification) =
            unsigned_value_specification.generalValueSpecification()
            && general_value_specification.SESSION_USER().is_some()
        {
            return Ok(Self::builtin_call(fn_names::SESSION_USER, vec![], span));
        }

        Ok(Expression::Literal(Self::build_literal_from_context(
            unsigned_value_specification,
        )?))
    }

    /// Function to build a Literal from an UnsignedValueSpecificationContext. Forms that are not literals (list and record constructors, `SESSION_USER`) are rejected; use `build_expression_from_unsigned_value_specification` for those.
    pub(super) fn build_literal_from_context(
        unsigned_value_specification: &UnsignedValueSpecificationContextAll,
    ) -> GqlResult<Literal> {
        if let Some(unsigned_literal) = unsigned_value_specification.unsignedLiteral() {
            return Self::build_literal_from_unsigned_literal(&unsigned_literal);
        }

        if let Some(general_value_specification) =
            unsigned_value_specification.generalValueSpecification()
        {
            if let Some(parameter) = general_value_specification.dynamicParameterSpecification() {
                return Ok(Literal::Parameter(parameter.get_text()));
            }
            return Self::parse_error(format!(
                "CST not valid: general value specification is not a literal: {}",
                general_value_specification.get_text()
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
            return Self::decode_unsigned_numeric_literal(&unsigned_numeric_literal.get_text());
        }

        if let Some(general_literal) = unsigned_literal.generalLiteral() {
            return Self::build_literal_from_general_literal(&general_literal);
        }

        Self::parse_error("CST not valid: unsignedLiteral without supported literal")
    }

    fn build_literal_from_general_literal(
        general_literal: &GeneralLiteralContextAll,
    ) -> GqlResult<Literal> {
        if let Some(boolean_literal) = general_literal.BOOLEAN_LITERAL() {
            let text = boolean_literal.get_text();
            // UNKNOWN is the null value of the boolean type.
            return Ok(if text.eq_ignore_ascii_case("true") {
                Literal::Boolean(true)
            } else if text.eq_ignore_ascii_case("false") {
                Literal::Boolean(false)
            } else {
                Literal::Null
            });
        }

        if let Some(character_string_literal) = general_literal.characterStringLiteral() {
            return Ok(Literal::String(Self::decode_character_string_literal(
                &character_string_literal,
            )?));
        }

        if general_literal.nullLiteral().is_some() {
            return Ok(Literal::Null);
        }

        if let Some(temporal_literal) = general_literal.temporalLiteral() {
            let string_literal = if let Some(date) = temporal_literal.dateLiteral() {
                date.dateString().and_then(|s| s.characterStringLiteral())
            } else if let Some(time) = temporal_literal.timeLiteral() {
                time.timeString().and_then(|s| s.characterStringLiteral())
            } else if let Some(datetime) = temporal_literal.datetimeLiteral() {
                datetime
                    .datetimeString()
                    .and_then(|s| s.characterStringLiteral())
            } else {
                None
            }
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: string missing in temporal literal".into())
                })?;

            return Ok(Literal::DateTime(Self::decode_character_string_literal(
                &string_literal,
            )?));
        }

        if let Some(duration_literal) = general_literal.durationLiteral() {
            let string_literal = duration_literal
                .durationString()
                .and_then(|s| s.characterStringLiteral())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: string missing in duration literal".into())
                })?;

            return Ok(Literal::Duration(Self::decode_character_string_literal(
                &string_literal,
            )?));
        }

        if general_literal.BYTE_STRING_LITERAL().is_some() {
            return Self::unsupported_construct(
                &format!("byte string literal {}", general_literal.get_text()),
                "issue #10",
                Self::extract_span(general_literal),
            );
        }

        Self::parse_error(format!(
            "CST not valid: general literal must be built as an expression: {}",
            general_literal.get_text()
        ))
    }

    /// Decodes the value of a character string literal token.
    pub(super) fn decode_character_string_literal(
        ctx: &CharacterStringLiteralContextAll,
    ) -> GqlResult<String> {
        let text = ctx.get_text();
        Self::string_literal_text(&text).ok_or_else(|| {
            Self::parse_err(format!("CST not valid: malformed character string literal: {text}"))
        })
    }

    /// Rejects integer syntax outside i64 without reinterpreting it as floating point.
    pub(super) fn validate_integer_literal_range(text: &str) -> GqlResult<()> {
        let digits = text
            .strip_prefix('-')
            .or_else(|| text.strip_prefix('+'))
            .unwrap_or(text);
        if !digits.is_empty()
            && digits.bytes().all(|byte| byte.is_ascii_digit())
            && text.parse::<i64>().is_err()
        {
            return Self::parse_error(format!(
                "Integer literal '{}' is outside the signed 64-bit range",
                text
            ));
        }
        Ok(())
    }

    /// Decodes an unsigned numeric literal token: decimal, hexadecimal (`0x`), octal (`0o`) and
    /// binary (`0b`) integers with optional underscores become integer literals; decimals,
    /// scientific notation and the approximate suffixes `F`/`D` become float literals. The
    /// exact suffix `M` keeps integers integral and maps other exact decimals to float
    /// literals, the representation used for unsuffixed decimals.
    pub(super) fn decode_unsigned_numeric_literal(text: &str) -> GqlResult<Literal> {
        let trimmed = text.trim();
        let cleaned = trimmed.replace('_', "");
        let lower = cleaned.to_ascii_lowercase();

        if lower.starts_with("0x") || lower.starts_with("0o") || lower.starts_with("0b") {
            let magnitude = Self::decode_unsigned_integer_magnitude(&lower).ok_or_else(|| {
                Self::parse_err(format!("Invalid integer literal '{trimmed}'"))
            })?;
            return i64::try_from(magnitude).map(Literal::Integer).map_err(|_| {
                Self::parse_err(format!(
                    "Integer literal '{trimmed}' is outside the signed 64-bit range"
                ))
            });
        }

        let (body, approximate) = match lower.as_bytes().last() {
            Some(b'm') => (&lower[..lower.len() - 1], false),
            Some(b'f') | Some(b'd') => (&lower[..lower.len() - 1], true),
            _ => (lower.as_str(), false),
        };

        if !approximate && !body.is_empty() && body.bytes().all(|byte| byte.is_ascii_digit()) {
            Self::validate_integer_literal_range(body)?;
            return body
                .parse::<i64>()
                .map(Literal::Integer)
                .map_err(|_| Self::parse_err(format!("Invalid integer literal '{trimmed}'")));
        }

        body.parse::<f64>()
            .map(Literal::Float)
            .map_err(|_| Self::parse_err(format!("Invalid numeric literal '{trimmed}'")))
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
