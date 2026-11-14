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
    /// Function to build a DataStatement from the CST context of a linear data modifying statement, handling both ambient and focused variants, and extracting the relevant components to construct the appropriate AST nodes for the data modification operation. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for a data modifying statement. --- IGNORE ---
    pub(super) fn build_data_statement_from_context(
        cst: &LinearDataModifyingStatementContextAll,
    ) -> GqlResult<DataStatement> {
        let span = Self::extract_span(cst);

        let simple_linear = if let Some(ambient) = cst.ambientLinearDataModifyingStatement() {
            let body = ambient
                .ambientLinearDataModifyingStatementBody()
                .ok_or_else(|| Self::parse_err("Expected ambientLinearDataModifyingStatementBody".to_string()))?;
            body.simpleLinearDataAccessingStatement()
                .ok_or_else(|| Self::parse_err("Expected simpleLinearDataAccessingStatement".to_string()))?
        } else if let Some(focused) = cst.focusedLinearDataModifyingStatement() {
            let body = focused
                .focusedLinearDataModifyingStatementBody()
                .ok_or_else(|| Self::parse_err("Expected focusedLinearDataModifyingStatementBody".to_string()))?;
            body.simpleLinearDataAccessingStatement()
                .ok_or_else(|| Self::parse_err("Expected simpleLinearDataAccessingStatement".to_string()))?
        } else {
            return Err(Self::parse_err(
                "Expected ambient or focused linear data modifying statement".to_string(),
            ));
        };

        let stmts = simple_linear.simpleDataAccessingStatement_all();

        if stmts.is_empty() {
            return Err(Self::parse_err("No statements in data modifying statement".to_string()));
        }

        if let Some(data_stmt) = stmts[0].simpleDataModifyingStatement() {
            if stmts.len() != 1 {
                return Err(Self::parse_err(
                    "Standalone INSERT/SET/REMOVE/DELETE should have only one statement".to_string(),
                ));
            }
            if let Some(prim) = data_stmt.primitiveDataModifyingStatement() {
                if let Some(set_stmt) = prim.setStatement() {
                    let items = Self::build_set_items_from_context(&set_stmt)?;
                    Ok(DataStatement::Set(SetStatement {
                        items,
                        span: span.clone(),
                    }))
                } else if let Some(remove_stmt) = prim.removeStatement() {
                    let items = Self::build_remove_items_from_context(&remove_stmt)?;
                    Ok(DataStatement::Remove(RemoveStatement {
                        remove_clause: items,
                        span: span.clone(),
                    }))
                } else if let Some(delete_stmt) = prim.deleteStatement() {
                    let detach = delete_stmt.DETACH().is_some();
                    let delete_item_list = delete_stmt
                        .deleteItemList()
                        .ok_or_else(|| Self::parse_err("Expected deleteItemList".to_string()))?;
                    let expressions = Self::build_expressions_from_delete_item_list(&delete_item_list)?;
                    Ok(DataStatement::Delete(DeleteStatement {
                        delete_clause: DeleteClause {
                            expressions,
                            detach,
                            span: span.clone(),
                        },
                        span: span.clone(),
                    }))
                } else if let Some(insert_stmt) = prim.insertStatement() {
                    Self::build_insert_statement_from_context(&insert_stmt)
                } else {
                    Err(Self::parse_err(
                        "Expected setStatement, removeStatement, deleteStatement, or insertStatement".to_string(),
                    ))
                }
            } else if let Some(call_data_stmt) = data_stmt.callDataModifyingProcedureStatement() {
                let call_proc = call_data_stmt
                    .callProcedureStatement()
                    .ok_or_else(|| Self::parse_err("CST not valid: missing callProcedureStatement in data CALL".into()))?;
                let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                Ok(DataStatement::Call(CallStatement {
                    call_clause,
                    span: span.clone(),
                }))
            } else {
                Err(Self::parse_err(
                    "Expected primitiveDataModifyingStatement or callDataModifyingProcedureStatement".to_string(),
                ))
            }
        } else if let Some(query_stmt) = stmts[0].simpleQueryStatement() {
            if stmts.len() != 2 {
                return Err(Self::parse_err(
                    "MATCH INSERT/SET/REMOVE/DELETE should have exactly two statements".to_string(),
                ));
            }
            let primitive = query_stmt
                .primitiveQueryStatement()
                .ok_or_else(|| Self::parse_err("Expected primitiveQueryStatement".to_string()))?;
            let match_stmt = primitive
                .matchStatement()
                .ok_or_else(|| Self::parse_err("Expected matchStatement".to_string()))?;
            let (graph_pattern, optional) = Self::extract_graph_pattern(&match_stmt)?;
            let match_clause = Self::build_match_clause(&graph_pattern, optional)?;
            let mut where_clause: Option<WhereClause> = None;
            if let Some(where_ctx) = graph_pattern.graphPatternWhereClause() {
                where_clause = Some(Self::build_where_clause_from_context(&where_ctx)?);
            }
            let match_clauses = vec![match_clause];

            let data_stmt = stmts[1]
                .simpleDataModifyingStatement()
                .ok_or_else(|| Self::parse_err("Expected simpleDataModifyingStatement".to_string()))?;
            let prim = data_stmt
                .primitiveDataModifyingStatement()
                .ok_or_else(|| Self::parse_err("Expected primitiveDataModifyingStatement".to_string()))?;

            if let Some(set_stmt) = prim.setStatement() {
                let items = Self::build_set_items_from_context(&set_stmt)?;
                Ok(DataStatement::MatchSet(MatchSetStatement {
                    match_clauses,
                    where_clause,
                    items,
                    span: span.clone(),
                }))
            } else if let Some(remove_stmt) = prim.removeStatement() {
                let items = Self::build_remove_items_from_context(&remove_stmt)?;
                Ok(DataStatement::MatchRemove(MatchRemoveStatement {
                    match_clause: match_clauses[0].clone(),
                    where_clause,
                    remove_clause: items,
                    span: span.clone(),
                }))
            } else if let Some(delete_stmt) = prim.deleteStatement() {
                let detach = delete_stmt.DETACH().is_some();
                let delete_item_list = delete_stmt
                    .deleteItemList()
                    .ok_or_else(|| Self::parse_err("Expected deleteItemList".to_string()))?;
                let expressions = Self::build_expressions_from_delete_item_list(&delete_item_list)?;
                Ok(DataStatement::MatchDelete(MatchDeleteStatement {
                    match_clause: match_clauses[0].clone(),
                    where_clause,
                    delete_clause: DeleteClause {
                        expressions,
                        detach,
                        span: span.clone(),
                    },
                    span: span.clone(),
                }))
            } else {
                Err(Self::parse_err(
                    "Expected insertStatement, setStatement, removeStatement, or deleteStatement".to_string(),
                ))
            }
        } else {
            Err(Self::parse_err("Unsupported data statement type".to_string()))
        }
    }

    /// Function to attempt building a query-like statement from a linear data modifying statement, checking if the structure of the CST corresponds to a query that can be represented as a sequence of query steps without data modification. If the structure matches, it constructs the appropriate AST nodes for the query; otherwise, it returns None to indicate that the statement is not purely query-like. This function is useful for distinguishing between statements that are intended to modify data and those that are intended to perform queries, allowing for more accurate AST construction and validation. --- IGNORE ---
    pub(super) fn try_build_query_like_statement_from_linear_data(
        linear_data: &LinearDataModifyingStatementContextAll,
    ) -> GqlResult<Option<Statement>> {
        let span = Self::extract_span(linear_data);

        let mut pipeline = Vec::new();
        let mut has_data_modify = false;
        let mut has_input_query = false;
        let mut input_insert = None;
        let mut terminal_result = None;

        if let Some(focused) = linear_data.focusedLinearDataModifyingStatement()
            && let Some(body) = focused.focusedLinearDataModifyingStatementBody()
        {
            terminal_result = body.primitiveResultStatement();
            if let Some(use_clause) = body.useGraphClause()
                && let Some(graph_expr) = use_clause.graphExpression()
            {
                pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(&graph_expr)));
            }
            if let Some(simple_linear) = body.simpleLinearDataAccessingStatement() {
                let statements = simple_linear.simpleDataAccessingStatement_all();
                for (index, statement) in statements.iter().enumerate() {
                    if let Some(query_stmt) = statement.simpleQueryStatement() {
                        has_input_query = true;
                        if let Some(primitive) = query_stmt.primitiveQueryStatement() {
                            if let Some(m) = primitive.matchStatement() {
                                let (graph_pattern, optional) = Self::extract_graph_pattern(&m)?;
                                let match_clause = Self::build_match_clause(&graph_pattern, optional)?;
                                pipeline.push(QueryStep::Match(match_clause));

                                if let Some(where_ctx) = graph_pattern.graphPatternWhereClause() {
                                    let where_clause = Self::build_where_clause_from_context(&where_ctx)?;
                                    pipeline.push(QueryStep::Filter(FilterClause {
                                        condition: where_clause.condition,
                                        span: span.clone(),
                                    }));
                                }
                            } else if let Some(l) = primitive.letStatement() {
                                let let_clause = Self::build_let_statement_from_context(&l)?;
                                pipeline.push(QueryStep::Let(let_clause));
                            } else if let Some(f) = primitive.filterStatement() {
                                let filter_clause = Self::build_filter_clause_from_context(&f)?;
                                pipeline.push(QueryStep::Filter(filter_clause));
                            } else if let Some(f) = primitive.forStatement() {
                                let for_clause = Self::build_for_clause_from_context(&f)?;
                                pipeline.push(QueryStep::For(for_clause));
                            } else if let Some(page) = primitive.orderByAndPageStatement() {
                                let order_clause = page.orderByClause()
                                    .map(|ctx| Self::build_order_clause_from_context(&ctx)).transpose()?;
                                let limit_clause = Self::build_limit_clause_from_context(&page)?;
                                pipeline.push(QueryStep::Page(OrderByAndPageStatement {
                                    order_clause,
                                    limit_clause,
                                    span: Self::extract_span(&*page),
                                }));
                            } else {
                                return Self::parse_error("Unsupported input query clause before data modification");
                            }
                        } else if let Some(call_query) = query_stmt.callQueryStatement() {
                            let call_proc = call_query.callProcedureStatement().ok_or_else(|| {
                                Self::parse_err("CST not valid: missing callProcedureStatement in query CALL".into())
                            })?;
                            let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                            pipeline.push(QueryStep::Call(call_clause));
                        }
                    } else if let Some(data_stmt) = statement.simpleDataModifyingStatement() {
                        if let Some(call_data_stmt) = data_stmt.callDataModifyingProcedureStatement() {
                            let call_proc = call_data_stmt.callProcedureStatement().ok_or_else(|| {
                                Self::parse_err("CST not valid: missing callProcedureStatement in data CALL".into())
                            })?;
                            let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                            pipeline.push(QueryStep::Call(call_clause));
                        } else if let Some(insert_stmt) = data_stmt
                            .primitiveDataModifyingStatement()
                            .and_then(|primitive| primitive.insertStatement())
                        {
                            if index + 1 != statements.len() {
                                return Self::parse_error("Operations after INSERT are not supported by the current AST builder");
                            }
                            has_data_modify = true;
                            input_insert = Some(Self::build_insert_statement_from_context(&insert_stmt)?);
                            break;
                        } else {
                            has_data_modify = true;
                            break;
                        }
                    }
                }
            }
        }

        if let Some(ambient) = linear_data.ambientLinearDataModifyingStatement()
            && let Some(body) = ambient.ambientLinearDataModifyingStatementBody()
            && let Some(simple_linear) = body.simpleLinearDataAccessingStatement()
        {
            terminal_result = body.primitiveResultStatement();
            let statements = simple_linear.simpleDataAccessingStatement_all();
            for (index, statement) in statements.iter().enumerate() {
                if let Some(query_stmt) = statement.simpleQueryStatement() {
                    has_input_query = true;
                    if let Some(primitive) = query_stmt.primitiveQueryStatement() {
                        if let Some(m) = primitive.matchStatement() {
                            let (graph_pattern, optional) = Self::extract_graph_pattern(&m)?;
                            let match_clause = Self::build_match_clause(&graph_pattern, optional)?;
                            pipeline.push(QueryStep::Match(match_clause));

                            if let Some(where_ctx) = graph_pattern.graphPatternWhereClause() {
                                let where_clause = Self::build_where_clause_from_context(&where_ctx)?;
                                pipeline.push(QueryStep::Filter(FilterClause {
                                    condition: where_clause.condition,
                                    span: span.clone(),
                                }));
                            }
                        } else if let Some(l) = primitive.letStatement() {
                            let let_clause = Self::build_let_statement_from_context(&l)?;
                            pipeline.push(QueryStep::Let(let_clause));
                        } else if let Some(f) = primitive.filterStatement() {
                            let filter_clause = Self::build_filter_clause_from_context(&f)?;
                            pipeline.push(QueryStep::Filter(filter_clause));
                        } else if let Some(f) = primitive.forStatement() {
                            let for_clause = Self::build_for_clause_from_context(&f)?;
                            pipeline.push(QueryStep::For(for_clause));
                        } else if let Some(page) = primitive.orderByAndPageStatement() {
                            let order_clause = page.orderByClause()
                                .map(|ctx| Self::build_order_clause_from_context(&ctx)).transpose()?;
                            let limit_clause = Self::build_limit_clause_from_context(&page)?;
                            pipeline.push(QueryStep::Page(OrderByAndPageStatement {
                                order_clause,
                                limit_clause,
                                span: Self::extract_span(&*page),
                            }));
                        } else {
                            return Self::parse_error("Unsupported input query clause before data modification");
                        }
                    } else if let Some(call_query) = query_stmt.callQueryStatement() {
                        let call_proc = call_query.callProcedureStatement().ok_or_else(|| {
                            Self::parse_err("CST not valid: missing callProcedureStatement in query CALL".into())
                        })?;
                        let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                        pipeline.push(QueryStep::Call(call_clause));
                    }
                } else if let Some(data_stmt) = statement.simpleDataModifyingStatement() {
                    if let Some(call_data_stmt) = data_stmt.callDataModifyingProcedureStatement() {
                        let call_proc = call_data_stmt.callProcedureStatement().ok_or_else(|| {
                            Self::parse_err("CST not valid: missing callProcedureStatement in data CALL".into())
                        })?;
                        let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                        pipeline.push(QueryStep::Call(call_clause));
                    } else if let Some(insert_stmt) = data_stmt
                        .primitiveDataModifyingStatement()
                        .and_then(|primitive| primitive.insertStatement())
                    {
                        if index + 1 != statements.len() {
                            return Self::parse_error("Operations after INSERT are not supported by the current AST builder");
                        }
                        has_data_modify = true;
                        input_insert = Some(Self::build_insert_statement_from_context(&insert_stmt)?);
                        break;
                    } else {
                        has_data_modify = true;
                        break;
                    }
                }
            }
        }

        if has_input_query
            && let Some(insert) = input_insert
        {
            // Graph focus must cover the mutation and result, not just its input.
            if matches!(pipeline.first(), Some(QueryStep::UseGraph(_))) {
                pipeline.remove(0);
            }
            let query = Query::Basic(BasicQuery {
                pipeline,
                result_statement: PrimitiveResultStatement::Finish { span: span.clone() },
                span: span.clone(),
            });
            let mut target = Statement::Data(insert);
            if let Some(result) = terminal_result {
                let result_statement = Self::build_result_statement_from_primitive_result_statement(&result)?;
                let result_span = Self::extract_span(&*result);
                target = Statement::Next(NextStatement {
                    source_statement: Box::new(target),
                    target_statement: Box::new(Statement::Query(Query::Basic(BasicQuery {
                        pipeline: Vec::new(),
                        result_statement,
                        span: result_span,
                    }))),
                    yield_clause: None,
                    next: None,
                    span: span.clone(),
                });
            }
            let statement = Statement::Next(NextStatement {
                source_statement: Box::new(Statement::Query(query)),
                target_statement: Box::new(target),
                yield_clause: None,
                next: None,
                span,
            });
            return Self::wrap_focused_linear_data_statement(linear_data, statement).map(Some);
        }

        if !has_data_modify && !pipeline.is_empty() {
            let query = Query::Basic(BasicQuery {
                pipeline,
                result_statement: PrimitiveResultStatement::Finish { span: span.clone() },
                span: span.clone(),
            });
            return Ok(Some(Statement::Query(query)));
        }

        Ok(None)
    }

    /// Function to build an InsertStatement from an InsertStatementContext, extracting the graph pattern and constructing the corresponding AST nodes for the INSERT operation. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for an INSERT statement, such as the graph pattern and the list of path patterns to be inserted. --- IGNORE ---
    pub(super) fn build_insert_statement_from_context(cst: &InsertStatementContextAll) -> GqlResult<DataStatement> {
        let span = Self::extract_span(cst);

        let graph = cst
            .insertGraphPattern()
            .ok_or_else(|| Self::parse_err("Expected insertGraphPattern".to_string()))?;

        let list = graph
            .insertPathPatternList()
            .ok_or_else(|| Self::parse_err("Expected insertPathPatternList".to_string()))?;

        let patterns = list
            .insertPathPattern_all()
            .into_iter()
            .map(|p| Self::build_insert_path_pattern_from_context(&p))
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(DataStatement::Insert(InsertStatement {
            patterns,
            span: span.clone(),
        }))
    }

    /// Function to build a PathPattern from an InsertPathPatternContext, extracting the sequence of node and edge patterns that define the path to be inserted, and constructing the corresponding AST nodes for the path pattern. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for an INSERT path pattern, such as at least one node and a consistent sequence of nodes and edges. --- IGNORE ---
    pub(super) fn build_insert_path_pattern_from_context(cst: &InsertPathPatternContextAll) -> GqlResult<PathPattern> {
        let span = Self::extract_span(cst);

        let nodes = cst.insertNodePattern_all();
        if nodes.is_empty() {
            return Self::parse_error("INSERT path must contain at least one node");
        }

        let edges = cst.insertEdgePattern_all();
        if edges.len() + 1 != nodes.len() {
            return Self::parse_error("INSERT path has inconsistent node/edge sequence");
        }

        let mut elements = Vec::with_capacity(nodes.len() + edges.len());

        elements.push(PatternElement::Node(Self::build_insert_node_from_context(&nodes[0])?));

        for i in 0..edges.len() {
            elements.push(PatternElement::Edge(Self::build_insert_edge_from_context(&edges[i])?));
            elements.push(PatternElement::Node(Self::build_insert_node_from_context(
                &nodes[i + 1],
            )?));
        }

        Ok(PathPattern {
            variable: None,
            path_mode: None,
            path_search: None,
            simplified_pattern: None,
            alternation: None,
            elements,
            span: span.clone(),
        })
    }

    /// Function to build a Node from an InsertNodePatternContext, extracting the variable name, labels, and properties for the node to be inserted, and constructing the corresponding AST node for the node pattern. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for an INSERT node pattern, such as a valid variable declaration and properly specified labels and properties. --- IGNORE ---
    pub(super) fn build_insert_node_from_context(cst: &InsertNodePatternContextAll) -> GqlResult<Node> {
        let span = Self::extract_span(cst);

        let filler = cst.insertElementPatternFiller();

        let id = filler
            .as_ref()
            .and_then(|f| f.elementVariableDeclaration())
            .map(|v| v.get_text());

        let labels = filler
            .as_ref()
            .and_then(|f| f.labelAndPropertySetSpecification())
            .and_then(|s| s.labelSetSpecification())
            .map(|ls| Self::build_insert_labels_expression(&ls))
            .transpose()?;

        let properties = filler
            .as_ref()
            .and_then(|f| f.labelAndPropertySetSpecification())
            .and_then(|s| s.elementPropertySpecification())
            .map(|p| Self::build_property_map_from_context(&p))
            .transpose()?;

        Ok(Node {
            id,
            labels,
            properties,
            where_clause: None,
            span: span.clone(),
        })
    }

    /// Function to build an Edge from an InsertEdgePatternContext, extracting the variable name, labels, properties, and direction for the edge to be inserted, and constructing the corresponding AST node for the edge pattern. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for an INSERT edge pattern, such as a valid variable declaration and properly specified labels and properties. --- IGNORE ---
    pub(super) fn build_insert_edge_from_context(cst: &InsertEdgePatternContextAll) -> GqlResult<Edge> {
        let span = Self::extract_span(cst);

        let direction = if cst.insertEdgePointingLeft().is_some() {
            EdgeDirection::Incoming
        } else if cst.insertEdgePointingRight().is_some() {
            EdgeDirection::Outgoing
        } else {
            EdgeDirection::Undirected
        };

        let filler = if let Some(x) = cst.insertEdgePointingLeft() {
            x.insertElementPatternFiller()
        } else if let Some(x) = cst.insertEdgePointingRight() {
            x.insertElementPatternFiller()
        } else if let Some(x) = cst.insertEdgeUndirected() {
            x.insertElementPatternFiller()
        } else {
            None
        };

        let id = filler
            .as_ref()
            .and_then(|f| f.elementVariableDeclaration())
            .map(|v| v.get_text());

        let labels = filler
            .as_ref()
            .and_then(|f| f.labelAndPropertySetSpecification())
            .and_then(|s| s.labelSetSpecification())
            .map(|ls| Self::build_insert_labels_expression(&ls))
            .transpose()?;

        let properties = filler
            .as_ref()
            .and_then(|f| f.labelAndPropertySetSpecification())
            .and_then(|s| s.elementPropertySpecification())
            .map(|p| Self::build_property_map_from_context(&p))
            .transpose()?;

        Ok(Edge {
            id,
            labels,
            properties,
            where_clause: None,
            direction,
            quantifier: None,
            span: span.clone(),
        })
    }

    /// Function to build an Expression representing a set of labels from a LabelSetSpecificationContext, extracting the label names and constructing a logical expression that represents the presence of those labels. This function also includes error handling to ensure that the CST structure is valid and contains at least one label, as well as constructing the appropriate AST nodes for the label expression. --- IGNORE ---
    pub(super) fn build_insert_labels_expression(cst: &LabelSetSpecificationContextAll) -> GqlResult<Expression> {
        let span = Self::extract_span(cst);

        let labels = cst.labelName_all();
        if labels.is_empty() {
            return Self::parse_error("Empty labelSetSpecification in INSERT");
        }

        let mut expr = Expression::Literal(Literal::String(Self::identifier_text(&labels[0].get_text())));
        for label in labels.into_iter().skip(1) {
            expr = Expression::Binary(BinaryExpression {
                left: Box::new(expr),
                operator: Operator::And,
                right: Box::new(Expression::Literal(Literal::String(Self::identifier_text(
                    &label.get_text(),
                )))),
                span: span.clone(),
            });
        }

        Ok(expr)
    }

    /// Function to build a list of SetItem from a SetStatementContext, handling different types of SET items (property setting, label setting, variable setting) and extracting the relevant components from the CST to construct the corresponding AST nodes for each type of SET item. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for each type of SET item. --- IGNORE ---
    pub(super) fn build_set_items_from_context(cst: &SetStatementContextAll) -> GqlResult<Vec<SetItem>> {
        let set_item_list = cst
            .setItemList()
            .ok_or_else(|| Self::parse_err("Expected setItemList".to_string()))?;
        let items = set_item_list.setItem_all();
        items
            .iter()
            .map(|item| Self::build_set_item_from_context(item))
            .collect()
    }

    /// Function to build a SetItem from a SetItemContext, handling property setting, label setting, and variable setting, and extracting the relevant components from the CST. --- IGNORE ---
    pub(super) fn build_set_item_from_context(cst: &SetItemContextAll) -> GqlResult<SetItem> {
        let span = Self::extract_span(cst);

        if let Some(prop) = cst.setPropertyItem() {
            let binding = prop
                .bindingVariableReference()
                .ok_or_else(|| Self::parse_err("Expected bindingVariableReference".to_string()))?
                .get_text();
            let prop_name = prop
                .propertyName()
                .ok_or_else(|| Self::parse_err("Expected propertyName".to_string()))?
                .get_text();
            let prop_name = Self::identifier_text(&prop_name);
            let value_expr = prop
                .valueExpression()
                .ok_or_else(|| Self::parse_err("Expected valueExpression".to_string()))?;
            let value = Self::build_expression_from_context(&value_expr)?;
            Ok(SetItem::Property {
                property: PropertyAccess {
                    object: binding,
                    property: prop_name,
                    span: span.clone(),
                },
                value,
                span: span.clone(),
            })
        } else if let Some(label_item) = cst.setLabelItem() {
            let variable = label_item
                .bindingVariableReference()
                .ok_or_else(|| Self::parse_err("Expected bindingVariableReference".to_string()))?
                .get_text();
            let label_name = label_item
                .labelName()
                .ok_or_else(|| Self::parse_err("Expected labelName".to_string()))?
                .get_text();
            let label_name = Self::identifier_text(&label_name);
            let labels = Expression::Literal(Literal::String(label_name));
            Ok(SetItem::Label {
                variable,
                labels,
                span: span.clone(),
            })
        } else if let Some(all_prop) = cst.setAllPropertiesItem() {
            let variable = all_prop
                .bindingVariableReference()
                .ok_or_else(|| Self::parse_err("Expected bindingVariableReference".to_string()))?
                .get_text();
            let property_list = all_prop.propertyKeyValuePairList();
            let properties = if let Some(list) = property_list {
                Self::build_property_key_value_pair_list(&list)?
            } else {
                Vec::new()
            };
            let value = Expression::Map(properties);
            Ok(SetItem::Variable {
                variable,
                value,
                span: span.clone(),
            })
        } else {
            let raw = cst.get_text();
            let variable = raw
                .split(|ch: char| ch == '.' || ch == ':' || ch.is_whitespace())
                .find(|token| !token.is_empty())
                .unwrap_or("__set_target__")
                .to_string();

            Ok(SetItem::Variable {
                variable,
                value: Expression::Literal(Literal::String(raw)),
                span: span.clone(),
            })
        }
    }

    /// Function to build a list of RemoveItem from a RemoveStatementContext, handling different types of REMOVE items (property removal, label removal, variable removal) and extracting the relevant components from the CST to construct the corresponding AST nodes for each type of REMOVE item. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for each type of REMOVE item. --- IGNORE ---
    pub(super) fn build_remove_items_from_context(cst: &RemoveStatementContextAll) -> GqlResult<Vec<RemoveItem>> {
        let remove_item_list = cst
            .removeItemList()
            .ok_or_else(|| Self::parse_err("Expected removeItemList".to_string()))?;
        let items = remove_item_list.removeItem_all();
        items
            .iter()
            .map(|item| Self::build_remove_item_from_context(item))
            .collect()
    }

    /// Function to build a RemoveItem from a RemoveItemContext, handling property removal, label removal, and variable removal, and extracting the relevant components from the CST. --- IGNORE ---
    pub(super) fn build_remove_item_from_context(cst: &RemoveItemContextAll) -> GqlResult<RemoveItem> {
        let span = Self::extract_span(cst);

        if let Some(prop) = cst.removePropertyItem() {
            let binding = prop
                .bindingVariableReference()
                .ok_or_else(|| Self::parse_err("Expected bindingVariableReference".to_string()))?
                .get_text();
            let prop_name = prop
                .propertyName()
                .ok_or_else(|| Self::parse_err("Expected propertyName".to_string()))?
                .get_text();
            let prop_name = Self::identifier_text(&prop_name);
            Ok(RemoveItem::Property {
                property: PropertyAccess {
                    object: binding,
                    property: prop_name,
                    span: span.clone(),
                },
                span: span.clone(),
            })
        } else if let Some(label_item) = cst.removeLabelItem() {
            let variable = label_item
                .bindingVariableReference()
                .ok_or_else(|| Self::parse_err("Expected bindingVariableReference".to_string()))?
                .get_text();
            let label_name = label_item
                .labelName()
                .ok_or_else(|| Self::parse_err("Expected labelName".to_string()))?
                .get_text();
            let label_name = Self::identifier_text(&label_name);
            let labels = Expression::Literal(Literal::String(label_name));
            Ok(RemoveItem::Label {
                variable,
                labels,
                span: span.clone(),
            })
        } else {
            Ok(RemoveItem::Variable {
                name: cst.get_text(),
                span: span.clone(),
            })
        }
    }

    /// Function to build a list of Expressions from a DeleteItemListContext, extracting the value expressions for each delete item in the CST and constructing the corresponding AST nodes for the delete clause. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for each delete item, such as a value expression. --- IGNORE ---
    pub(super) fn build_expressions_from_delete_item_list(cst: &DeleteItemListContextAll) -> GqlResult<Vec<Expression>> {
        let delete_items = cst.deleteItem_all();
        delete_items
            .iter()
            .map(|item| {
                let value_expr = item
                    .valueExpression()
                    .ok_or_else(|| Self::parse_err("Expected valueExpression in deleteItem".to_string()))?;
                Self::build_expression_from_context(&value_expr)
            })
            .collect()
    }

    /// Function to build a list of PropertyItem from a PropertyKeyValuePairListContext, extracting the key-value pairs for properties and constructing the corresponding AST nodes for the property map. This function also includes error handling to ensure that the CST structure is valid and contains the expected elements for each property key-value pair, such as a property name and a value expression. --- IGNORE ---
    pub(super) fn build_property_key_value_pair_list(
        cst: &PropertyKeyValuePairListContextAll,
    ) -> GqlResult<Vec<PropertyItem>> {
        let span = Self::extract_span(cst);

        let pairs = cst.propertyKeyValuePair_all();
        pairs
            .iter()
            .map(|pair| {
                let key = pair
                    .propertyName()
                    .ok_or_else(|| Self::parse_err("Expected propertyName".to_string()))?
                    .get_text();
                let key = Self::identifier_text(&key);
                let value_expr = pair
                    .valueExpression()
                    .ok_or_else(|| Self::parse_err("Expected valueExpression".to_string()))?;
                let value = Self::build_expression_from_context(&value_expr)?;
                Ok(PropertyItem {
                    key,
                    value,
                    span: span.clone(),
                })
            })
            .collect()
    }
}
