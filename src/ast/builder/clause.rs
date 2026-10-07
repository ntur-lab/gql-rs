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
    /// Function to build a ReturnItem from a ReturnItemContext, extracting the expression and optional alias, and handling the case where the expression is an aggregating value expression that may contain a nested value expression or be a literal.
    pub(super) fn build_return_item(item_ctx: &ReturnItemContextAll) -> GqlResult<ReturnItem> {
        let span = Self::extract_span(item_ctx);

        let aggregating_expression = item_ctx.aggregatingValueExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: aggregatingValueExpression missing".into())
        })?;

        let expression = aggregating_expression
            .valueExpression()
            .map(|expression| Self::build_expression_from_context(&expression))
            .unwrap_or_else(|| {
                Ok(Expression::Literal(Self::build_literal_from_leaf_text(
                    &aggregating_expression.get_text(),
                )))
            })?;

        let alias = item_ctx.returnItemAlias().and_then(|alias_ctx| {
            alias_ctx
                .identifier()
                .map(|identifier| Self::identifier_text(&identifier.get_text()))
        });

        Ok(ReturnItem { expression, alias, span })
    }

    /// Function to build a ReturnClause from a ReturnStatementContext
    pub(super) fn build_return_clause_from_context(
        return_statement: &ReturnStatementContextAll,
    ) -> GqlResult<ReturnClause> {
        let span = Self::extract_span(return_statement);

        let body = return_statement
            .returnStatementBody()
            .ok_or_else(|| Self::parse_err("CST not valid: returnStatementBody missing".into()))?;

        let distinct = match body.setQuantifier() {
            Some(set_quantifier) if set_quantifier.DISTINCT().is_some() => {
                DistinctQualifier::Distinct
            }
            Some(set_quantifier) if set_quantifier.ALL().is_some() => DistinctQualifier::All,
            _ => DistinctQualifier::None,
        };

        let items = body
            .returnItemList()
            .map(|item_list| {
                item_list
                    .returnItem_all()
                    .into_iter()
                    .map(|item_ctx| Self::build_return_item(&item_ctx))
                    .collect::<GqlResult<Vec<_>>>()
            })
            .transpose()?
            .unwrap_or_default();
        
        Ok(ReturnClause { distinct, items, span })
    }

    /// Function to build a GroupClause from a GroupByClauseContext
    pub(super) fn build_group_clause_from_context(
        group_by_clause: &GroupByClauseContextAll,
    ) -> GqlResult<GroupClause> {
        let span = Self::extract_span(group_by_clause);

        let list = group_by_clause.groupingElementList().ok_or_else(|| {
            Self::parse_err("CST not valid: groupingElementList missing in GROUP BY".into())
        })?;
        if list.emptyGroupingSet().is_some() {
            return Ok(GroupClause {
                keys: None,
                is_empty_grouping_set: true,
                span,
            });
        }

        let keys = list
            .groupingElement_all()
            .into_iter()
            .map(|el| Self::build_grouping_element_from_context(&el))
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(GroupClause {
            keys: Some(keys),
            is_empty_grouping_set: false,
            span,
        })
    }

    /// Function to build a GroupKey from a GroupingElementContext, handling only grouping by binding variable for now and extracting the relevant components from the CST.
    pub(super) fn build_grouping_element_from_context(
        grouping_element: &GroupingElementContextAll,
    ) -> GqlResult<GroupKey> {
        let span = Self::extract_span(grouping_element);
        
        let binding_ref = grouping_element.bindingVariableReference().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: bindingVariableReference missing in groupingElement".into(),
            )
        })?;

        Ok(GroupKey::BindingVariable{ name: binding_ref.get_text(), span })
    }

    /// Function to build an OrderClause from an OrderByClauseContext, extracting the list of sort specifications and building the corresponding OrderItems.
    pub(super) fn build_order_clause_from_context(
        order_by_clause: &OrderByClauseContextAll,
    ) -> GqlResult<OrderClause> {
        let span = Self::extract_span(order_by_clause);

        let sort_list = order_by_clause.sortSpecificationList().ok_or_else(|| {
            Self::parse_err("CST not valid: sortSpecificationList missing".into())
        })?;

        let items = sort_list
            .sortSpecification_all()
            .into_iter()
            .map(|sort_spec| Self::build_order_item_from_context(&sort_spec))
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(OrderClause { items, span })
    }

    /// Function to build an OrderItem from a SortSpecificationContext, extracting the sort key, ordering specification and null ordering, and building the corresponding expression and order item components.
    pub(super) fn build_order_item_from_context(
        sort_spec: &SortSpecificationContextAll,
    ) -> GqlResult<OrderItem> {
        let span = Self::extract_span(sort_spec);

        let sort_key = sort_spec
            .sortKey()
            .ok_or_else(|| Self::parse_err("CST not valid: sortKey missing".into()))?;

        let aggregating_expression = sort_key.aggregatingValueExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: aggregatingValueExpression missing in sortKey".into())
        })?;

        let expression =
            Self::build_expression_from_aggregating_value_expression(&aggregating_expression)?;

        let direction = match sort_spec.orderingSpecification() {
            Some(ordering) if ordering.DESC().is_some() || ordering.DESCENDING().is_some() => {
                OrderDirection::Descending
            }
            _ => OrderDirection::Ascending,
        };

        let nulls_ordering = sort_spec.nullOrdering().map(|null_ordering| {
            if null_ordering.FIRST().is_some() {
                NullsOrdering::First
            } else {
                NullsOrdering::Last
            }
        });

        Ok(OrderItem {
            expression,
            direction,
            nulls_ordering,
            span,
        })
    }

    /// Function to build an expression from an AggregatingValueExpressionContext, handling the case where it contains a ValueExpression and the case where it is just a literal (e.g. for ORDER BY 1)
    pub(super) fn build_expression_from_aggregating_value_expression(
        aggregating_expression: &AggregatingValueExpressionContextAll,
    ) -> GqlResult<Expression> {
        aggregating_expression
            .valueExpression()
            .map(|expression| Self::build_expression_from_context(&expression))
            .unwrap_or_else(|| {
                Ok(Expression::Literal(Self::build_literal_from_leaf_text(
                    &aggregating_expression.get_text(),
                )))
            })
    }

    /// Function to build a LimitClause from an OrderByAndPageStatementContext, extracting the limit and offset specifications and building the corresponding PageValues.
    pub(super) fn build_limit_clause_from_context(
        order_by_and_page: &OrderByAndPageStatementContextAll,
    ) -> GqlResult<Option<LimitClause>> {
        let span = Self::extract_span(order_by_and_page);

        let count = order_by_and_page
            .limitClause()
            .and_then(|ctx| ctx.nonNegativeIntegerSpecification())
            .map(|spec| Self::parse_non_negative_integer_spec(&spec))
            .transpose()?;

        let offset = order_by_and_page
            .offsetClause()
            .and_then(|ctx| ctx.nonNegativeIntegerSpecification())
            .map(|spec| Self::parse_non_negative_integer_spec(&spec))
            .transpose()?;

        if count.is_none() && offset.is_none() {
            return Ok(None);
        }

        Ok(Some(LimitClause { count, offset, span }))
    }

    /// Function to parse a nonNegativeIntegerSpecification, which can be either a literal unsigned integer or a dynamic parameter, and return the corresponding PageValue.
    pub(super) fn parse_non_negative_integer_spec(
        spec: &NonNegativeIntegerSpecificationContextAll,
    ) -> GqlResult<PageValue> {
        let span = Self::extract_span(spec);

        if let Some(unsigned) = spec.unsignedInteger() {
            let parsed = unsigned
                .get_text()
                .parse::<usize>()
                .map_err(|_| Self::parse_err("Invalid unsigned integer in LIMIT/OFFSET".into()))?;
            return Ok(PageValue::Number{ value: parsed, span });
        }

        if let Some(param) = spec.dynamicParameterSpecification() {
            let name = param
                .GENERAL_PARAMETER_REFERENCE()
                .map(|t| t.get_text())
                .unwrap_or_else(|| param.get_text());
            return Ok(PageValue::Parameter{ name, span });
        }

        Self::parse_error("CST not valid: nonNegativeIntegerSpecification without value")
    }
}
