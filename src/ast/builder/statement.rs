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
    /// Builds a Statement AST node from a StatementContext, determining the specific type of statement (query, data modification, catalog modification) based on the presence of specific CST patterns and delegating to specialized builder functions for each statement type. It performs validation to ensure that the expected structure of the CST is present for each statement type and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_statement_from_context(
        statement: &StatementContextAll,
    ) -> GqlResult<Statement> {
        let span = Self::extract_span(statement);

        if let Some(composite_query) = statement.compositeQueryStatement() {
            let composite_expr = composite_query.compositeQueryExpression().ok_or_else(|| {
                Self::parse_err("Query not valid: missing compositeQueryExpression".into())
            })?;
            let query = Self::build_query_from_composite_expression(&composite_expr)?;
            Ok(Statement::Query(query))
        } else if let Some(linear_data) = statement.linearDataModifyingStatement() {
            if let Some(query_like_statement) =
                Self::try_build_query_like_statement_from_linear_data(&linear_data)?
            {
                return Ok(query_like_statement);
            }

            let data_statement = Self::build_data_statement_from_context(&linear_data)?;

            let mut opt_primitive_result = None;
            if let Some(ambient) = linear_data.ambientLinearDataModifyingStatement() {
                if let Some(body) = ambient.ambientLinearDataModifyingStatementBody() {
                    opt_primitive_result = body.primitiveResultStatement();
                }
            } else if let Some(focused) = linear_data.focusedLinearDataModifyingStatement() {
                if let Some(body) = focused.focusedLinearDataModifyingStatementBody() {
                    opt_primitive_result = body.primitiveResultStatement();
                }
            }

            if let Some(prim_res_ctx) = opt_primitive_result {
                let result_statement =
                    Self::build_result_statement_from_primitive_result_statement(&prim_res_ctx)?;
                let prim_span = Self::extract_span(&*prim_res_ctx);

                let query = Query::Basic(BasicQuery {
                    pipeline: Vec::new(),
                    result_statement,
                    span: prim_span,
                });

                return Ok(Statement::Next(NextStatement {
                    source_statement: Box::new(Statement::Data(data_statement)),
                    target_statement: Box::new(Statement::Query(query)),
                    yield_clause: None,
                    next: None,
                    span: span.clone(),
                }));
            }

            Ok(Statement::Data(data_statement))
        } else if let Some(linear_catalog) = statement.linearCatalogModifyingStatement() {
            let stmts = linear_catalog.simpleCatalogModifyingStatement_all();
            if !stmts.is_empty() {
                let mut catalog_statements = Vec::new();

                for stmt in stmts {
                    if let Some(call_stmt) = stmt.callCatalogModifyingProcedureStatement() {
                        let call_proc = call_stmt.callProcedureStatement().ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: missing callProcedureStatement in catalog CALL"
                                    .into(),
                            )
                        })?;
                        let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                        catalog_statements.push(CatalogStatement::Call(CallStatement {
                            call_clause,
                            span: span.clone(),
                        }));
                        continue;
                    }

                    if let Some(primitive) = stmt.primitiveCatalogModifyingStatement() {
                        match Self::build_ddl_statement_from_primitive_catalog(&primitive) {
                            Ok(ddl) => catalog_statements.push(ddl),
                            Err(err) => catalog_statements.push(CatalogStatement::Raw(
                                RawCatalogStatement {
                                    text: primitive.get_text(),
                                    parse_error: Some(format!("{}", err)),
                                    span: span.clone(),
                                },
                            )),
                        }
                    }
                }

                if !catalog_statements.is_empty() {
                    return Ok(Statement::Catalog(CatalogStatementBlock {
                        statements: catalog_statements,
                    }));
                }
            }
            Self::parse_error("Catalog statement block cannot be empty")
        } else {
            Self::parse_error("Unsupported statement variant for AST builder")
        }
    }

    /// Builds a SessionStatement from a SessionActivityContext and preserves typed SET/RESET payloads.
    pub(super) fn build_session_command_from_context(
        cst: &SessionActivityContextAll,
    ) -> GqlResult<SessionStatement> {
        let set_clauses = cst
            .sessionSetCommand_all()
            .iter()
            .map(|cmd| Self::build_session_set_clause(cmd))
            .collect::<Vec<_>>();
        let reset_clauses = cst
            .sessionResetCommand_all()
            .iter()
            .filter_map(|cmd| Self::build_session_reset_clause(cmd))
            .collect::<Vec<_>>();

        let kind = match (!set_clauses.is_empty(), !reset_clauses.is_empty()) {
            (true, false) => SessionCommandKind::Set,
            (false, true) => SessionCommandKind::Reset,
            (true, true) => SessionCommandKind::Activity,
            (false, false) => {
                return Self::parse_error(
                    "CST not valid: sessionActivity without SET/RESET commands",
                );
            }
        };

        Ok(SessionStatement {
            kind,
            set_clauses,
            reset_clauses,
        })
    }

    /// Builds a TransactionStatement from a TransactionActivityContext and preserves the transaction mode when present.
    pub(super) fn build_transaction_or_catalog_statement_from_context(
        cst: &TransactionActivityContextAll,
    ) -> GqlResult<Statement> {
        let mut kind = TransactionCommandKind::Activity;
        let mut mode = None;

        if let Some(start) = cst.startTransactionCommand() {
            kind = TransactionCommandKind::Start;
            mode = Self::build_transaction_mode_from_start_command(&start);
        }

        let end_command = if let Some(end) = cst.endTransactionCommand() {
            if end.commitCommand().is_some() {
                Some(TransactionCommandKind::Commit)
            } else if end.rollbackCommand().is_some() {
                Some(TransactionCommandKind::Rollback)
            } else {
                None
            }
        } else {
            None
        };

        let body = if let Some(procedure) = cst.procedureSpecification() {
            let procedure_body = procedure.procedureBody().ok_or_else(|| {
                Self::parse_err("CST not valid: missing procedureBody".into())
            })?;
            let statement_block = procedure_body.statementBlock().ok_or_else(|| {
                Self::parse_err("CST not valid: missing statementBlock".into())
            })?;

            Some(Box::new(Self::build_statement_block_from_context(&statement_block)?))
        } else {
            None
        };

        if cst.startTransactionCommand().is_none() && body.is_none() {
            if let Some(end_kind) = end_command {
                return Ok(Statement::Transaction(TransactionStatement {
                    kind: end_kind,
                    mode: None,
                    body: None,
                    end_command: None,
                }));
            }
        }

        if kind == TransactionCommandKind::Activity && body.is_none() && end_command.is_none() {
            return Self::parse_error(format!(
                "CST not valid: unsupported transactionActivity variant: {}",
                cst.get_text()
            ));
        }

        Ok(Statement::Transaction(TransactionStatement {
            kind,
            mode,
            body,
            end_command,
        }))
    }

    /// Builds a START TRANSACTION statement from its CST node.
    pub(super) fn build_start_transaction_statement(
        start: &StartTransactionCommandContextAll,
    ) -> Statement {
        Statement::Transaction(TransactionStatement {
            kind: TransactionCommandKind::Start,
            mode: Self::build_transaction_mode_from_start_command(start),
            body: None,
            end_command: None,
        })
    }

    /// Builds a COMMIT/ROLLBACK statement from its CST node.
    pub(super) fn build_end_transaction_statement(
        end: &EndTransactionCommandContextAll,
    ) -> GqlResult<Statement> {
        if end.commitCommand().is_some() {
            return Ok(Statement::Transaction(TransactionStatement {
                kind: TransactionCommandKind::Commit,
                mode: None,
                body: None,
                end_command: None,
            }));
        }
        if end.rollbackCommand().is_some() {
            return Ok(Statement::Transaction(TransactionStatement {
                kind: TransactionCommandKind::Rollback,
                mode: None,
                body: None,
                end_command: None,
            }));
        }

        Self::parse_error("CST not valid: endTransactionCommand without COMMIT/ROLLBACK")
    }

    /// Builds a typed SESSION SET clause from the CST branch.
    fn build_session_set_clause(cst: &SessionSetCommandContextAll) -> SessionSetClause {
        let span = Self::extract_span(cst);

        if let Some(schema) = cst.sessionSetSchemaClause() {
            let schema_reference = schema
                .schemaReference()
                .map(|x| x.get_text())
                .unwrap_or_default();
            return SessionSetClause::Schema { schema_reference };
        }

        if let Some(graph) = cst.sessionSetGraphClause() {
            let graph_expression = graph
                .graphExpression()
                .map(|x| Self::build_graph_expression_clause(&x))
                .unwrap_or(GraphExpressionClause {
                    kind: GraphExpressionKind::CurrentGraph,
                    text: String::new(),
                    graph_reference: None,
                    object_name_or_binding_variable: None,
                    span: span.clone(),
                });
            return SessionSetClause::Graph {
                property_graph_keyword: graph.PROPERTY().is_some(),
                graph_expression,
            };
        }

        if let Some(time_zone) = cst.sessionSetTimeZoneClause() {
            let value = time_zone
                .setTimeZoneValue()
                .map(|x| x.get_text())
                .unwrap_or_default();
            return SessionSetClause::TimeZone {
                value: SessionTimeZoneValue::Literal(value),
            };
        }

        if let Some(parameter) = cst.sessionSetParameterClause() {
            return SessionSetClause::Parameter(Self::build_session_set_parameter_clause(
                &parameter,
            ));
        }

        SessionSetClause::Schema {
            schema_reference: cst.get_text(),
        }
    }

    /// Builds a SessionSetParameterClause from a SessionSetParameterClauseContext, determining the specific kind of parameter being set (graph, binding table, value) based on the presence of specific CST patterns and extracting the relevant components to construct the corresponding AST nodes. It performs validation to ensure that the expected structure of the CST is present for each parameter type and returns a default clause if no specific parameter type is identified.
    fn build_session_set_parameter_clause(
        cst: &SessionSetParameterClauseContextAll,
    ) -> SessionSetParameterClause {
        let span = Self::extract_span(cst);

        if let Some(graph_param) = cst.sessionSetGraphParameterClause() {
            return SessionSetParameterClause {
                kind: SessionSetParameterKind::Graph {
                    property_graph_keyword: graph_param.PROPERTY().is_some(),
                },
                parameter_name: graph_param
                    .sessionSetParameterName()
                    .map(|name| Self::build_session_set_parameter_name(&name))
                    .unwrap_or(SessionSetParameterName {
                        if_not_exists: false,
                        parameter_specification: String::new(),
                        span: span.clone(),
                    }),
                initializer: graph_param
                    .optTypedGraphInitializer()
                    .map(|init| init.get_text()),
                graph_initializer: None,
                value_initializer: None,
                binding_table_initializer: None,
            };
        }

        if let Some(table_param) = cst.sessionSetBindingTableParameterClause() {
            return SessionSetParameterClause {
                kind: SessionSetParameterKind::BindingTable {
                    binding_keyword: table_param.BINDING().is_some(),
                },
                parameter_name: table_param
                    .sessionSetParameterName()
                    .map(|name| Self::build_session_set_parameter_name(&name))
                    .unwrap_or(SessionSetParameterName {
                        if_not_exists: false,
                        parameter_specification: String::new(),
                        span: span.clone(),
                    }),
                initializer: table_param
                    .optTypedBindingTableInitializer()
                    .map(|init| init.get_text()),
                graph_initializer: None,
                value_initializer: None,
                binding_table_initializer: None,
            };
        }

        if let Some(value_param) = cst.sessionSetValueParameterClause() {
            return SessionSetParameterClause {
                kind: SessionSetParameterKind::Value,
                parameter_name: value_param
                    .sessionSetParameterName()
                    .map(|name| Self::build_session_set_parameter_name(&name))
                    .unwrap_or(SessionSetParameterName {
                        if_not_exists: false,
                        parameter_specification: String::new(),
                        span: span.clone(),
                    }),
                initializer: value_param
                    .optTypedValueInitializer()
                    .map(|init| init.get_text()),
                graph_initializer: None,
                value_initializer: None,
                binding_table_initializer: None,
            };
        }

        SessionSetParameterClause {
            kind: SessionSetParameterKind::Value,
            parameter_name: SessionSetParameterName {
                if_not_exists: false,
                parameter_specification: String::new(),
                span: span.clone(),
            },
            initializer: None,
            graph_initializer: None,
            value_initializer: None,
            binding_table_initializer: None,
        }
    }

    /// Builds a SessionSetParameterName from a SessionSetParameterNameContext, extracting the parameter specification and IF NOT EXISTS flag from the CST to construct the corresponding AST node. It performs validation to ensure that the expected structure of the CST is present and returns a default parameter name if no specific information is identified.
    fn build_session_set_parameter_name(
        cst: &SessionSetParameterNameContextAll,
    ) -> SessionSetParameterName {
        let span = Self::extract_span(cst);

        SessionSetParameterName {
            if_not_exists: cst.IF().is_some() && cst.NOT().is_some() && cst.EXISTS().is_some(),
            parameter_specification: cst
                .sessionParameterSpecification()
                .map(|spec| spec.get_text())
                .unwrap_or_default(),
            span: span.clone(),
        }
    }

    fn build_session_reset_clause(
        cst: &SessionResetCommandContextAll,
    ) -> Option<SessionResetClause> {
        let args = cst.sessionResetArguments()?;

        let target = if args.PARAMETERS().is_some() || args.CHARACTERISTICS().is_some() {
            SessionResetTarget::ParametersOrCharacteristics {
                all: args.ALL().is_some(),
                target: if args.PARAMETERS().is_some() {
                    SessionResetCollectionTarget::Parameters
                } else {
                    SessionResetCollectionTarget::Characteristics
                },
            }
        } else if args.SCHEMA().is_some() {
            SessionResetTarget::Schema
        } else if args.GRAPH().is_some() {
            SessionResetTarget::Graph {
                property_graph_keyword: args.PROPERTY().is_some(),
            }
        } else if args.TIME().is_some() && args.ZONE().is_some() {
            SessionResetTarget::TimeZone
        } else {
            SessionResetTarget::Parameter {
                has_parameter_keyword: args.PARAMETER().is_some(),
                parameter_specification: args
                    .sessionParameterSpecification()
                    .map(|spec| spec.get_text())
                    .unwrap_or_default(),
            }
        };

        Some(SessionResetClause { target })
    }

    /// Builds a TransactionMode from a StartTransactionCommandContext by examining the transaction characteristics specified in the CST and determining if a transaction mode (read-only, read-write) is explicitly defined. It checks for the presence of specific CST patterns that correspond to each transaction mode and returns the appropriate TransactionMode variant if a match is found, or None if no explicit transaction mode is defined.
    fn build_transaction_mode_from_start_command(
        cst: &StartTransactionCommandContextAll,
    ) -> Option<TransactionMode> {
        let characteristics = cst.transactionCharacteristics()?;

        for mode in characteristics.transactionMode_all() {
            if let Some(access_mode) = mode.transactionAccessMode() {
                if access_mode.READ().is_some() && access_mode.ONLY().is_some() {
                    return Some(TransactionMode::ReadOnly);
                }
                if access_mode.READ().is_some() && access_mode.WRITE().is_some() {
                    return Some(TransactionMode::ReadWrite);
                }
            }
        }

        None
    }

    /// Build a Query-like Statement from a LinearDataModifyingStatementContext, handling specific cases where certain data modifying statements are represented as queries in the AST for compatibility reasons. It checks for the presence of specific CST patterns that correspond to these special cases and constructs the corresponding Query AST nodes if a match is found, returning None if the LinearDataModifyingStatement does not match any of the special cases.
    pub(super) fn build_ddl_statement_from_primitive_catalog(
        primitive: &PrimitiveCatalogModifyingStatementContextAll,
    ) -> GqlResult<CatalogStatement> {
        if let Some(ctx) = primitive.createSchemaStatement() {
            let target_ctx = ctx.catalogSchemaParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogSchemaParentAndName".into())
            })?;
            return Ok(CatalogStatement::CreateSchema(CreateSchema {
                if_not_exists: ctx.IF().is_some() && ctx.NOT().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_schema_name(&target_ctx)?,
            }));
        }
        if let Some(ctx) = primitive.dropSchemaStatement() {
            let target_ctx = ctx.catalogSchemaParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogSchemaParentAndName".into())
            })?;
            return Ok(CatalogStatement::DropSchema(DropSchema {
                if_exists: ctx.IF().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_schema_name(&target_ctx)?,
            }));
        }
        if let Some(ctx) = primitive.createGraphStatement() {
            let target_ctx = ctx.catalogGraphParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogGraphParentAndName".into())
            })?;

            let graph_type_clause = if let Some(open_ctx) = ctx.openGraphType() {
                CreateGraphTypeClause::Open(OpenGraphTypeClause {
                    typed: open_ctx.typed().is_some(),
                    property_graph_keyword: open_ctx.PROPERTY().is_some(),
                    graph_keyword: open_ctx.GRAPH().is_some(),
                })
            } else if let Some(of_ctx) = ctx.ofGraphType() {
                Self::build_of_graph_type_clause(&of_ctx)?
            } else {
                return Self::parse_error(
                    "CST not valid: missing graph type clause in CREATE GRAPH",
                );
            };

            let graph_source = if let Some(src_ctx) = ctx.graphSource() {
                let graph_expr = src_ctx.graphExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: missing graphExpression in graphSource".into())
                })?;
                Some(GraphSourceClause {
                    graph_expression: Self::build_graph_expression_clause(&graph_expr),
                })
            } else {
                None
            };

            return Ok(CatalogStatement::CreateGraph(CreateGraph {
                or_replace: ctx.OR().is_some() && ctx.REPLACE().is_some(),
                property_graph_keyword: ctx.PROPERTY().is_some(),
                if_not_exists: ctx.IF().is_some() && ctx.NOT().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_object_name_from_graph_parent(&target_ctx)?,
                graph_type_clause,
                graph_source,
            }));
        }
        if let Some(ctx) = primitive.dropGraphStatement() {
            let target_ctx = ctx.catalogGraphParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogGraphParentAndName".into())
            })?;
            return Ok(CatalogStatement::DropGraph(DropGraph {
                property_graph_keyword: ctx.PROPERTY().is_some(),
                if_exists: ctx.IF().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_object_name_from_graph_parent(&target_ctx)?,
            }));
        }
        if let Some(ctx) = primitive.createGraphTypeStatement() {
            let target_ctx = ctx.catalogGraphTypeParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogGraphTypeParentAndName".into())
            })?;
            let source_ctx = ctx
                .graphTypeSource()
                .ok_or_else(|| Self::parse_err("CST not valid: missing graphTypeSource".into()))?;
            return Ok(CatalogStatement::CreateGraphType(CreateGraphType {
                or_replace: ctx.OR().is_some() && ctx.REPLACE().is_some(),
                property_graph_keyword: ctx.PROPERTY().is_some(),
                if_not_exists: ctx.IF().is_some() && ctx.NOT().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_object_name_from_graph_type_parent(&target_ctx)?,
                source: Self::build_graph_type_source_clause(&source_ctx)?,
            }));
        }
        if let Some(ctx) = primitive.dropGraphTypeStatement() {
            let target_ctx = ctx.catalogGraphTypeParentAndName().ok_or_else(|| {
                Self::parse_err("CST not valid: missing catalogGraphTypeParentAndName".into())
            })?;
            return Ok(CatalogStatement::DropGraphType(DropGraphType {
                property_graph_keyword: ctx.PROPERTY().is_some(),
                if_exists: ctx.IF().is_some() && ctx.EXISTS().is_some(),
                target: Self::build_catalog_object_name_from_graph_type_parent(&target_ctx)?,
            }));
        }

        Self::parse_error(format!(
            "CST not valid: unsupported primitiveCatalogModifyingStatement variant: {}",
            primitive.get_text()
        ))
    }

    /// Builds a CatalogSchemaName from a CatalogSchemaParentAndNameContext, extracting the absolute directory path and schema name from the CST and performing validation to ensure that the required components are present. It returns an error if any required component is missing or if the structure of the CST is not as expected.
    pub(super) fn build_catalog_schema_name(
        cst: &CatalogSchemaParentAndNameContextAll,
    ) -> GqlResult<CatalogSchemaName> {
        let span = Self::extract_span(cst);

        let absolute_directory_path = cst
            .absoluteDirectoryPath()
            .ok_or_else(|| Self::parse_err("CST not valid: missing absoluteDirectoryPath".into()))?
            .get_text();
        let schema_name = cst
            .schemaName()
            .ok_or_else(|| Self::parse_err("CST not valid: missing schemaName".into()))?
            .get_text();

        Ok(CatalogSchemaName {
            absolute_directory_path,
            schema_name,
            span: span.clone(),
        })
    }

    /// Builds a CatalogObjectName from a CatalogGraphParentAndNameContext, extracting the graph name and optional parent reference from the CST and performing validation to ensure that the required components are present. It returns an error if any required component is missing or if the structure of the CST is not as expected.
    pub(super) fn build_catalog_object_name_from_graph_parent(
        cst: &CatalogGraphParentAndNameContextAll,
    ) -> GqlResult<CatalogObjectName> {
        let span = Self::extract_span(cst);

        let name = cst
            .graphName()
            .ok_or_else(|| Self::parse_err("CST not valid: missing graphName".into()))?
            .get_text();
        let parent = cst.catalogObjectParentReference().map(|p| p.get_text());

        Ok(CatalogObjectName {
            parent,
            name,
            span: span.clone(),
        })
    }

    /// Builds a CatalogObjectName from a CatalogGraphTypeParentAndNameContext, extracting the graph type name and optional parent reference from the CST and performing validation to ensure that the required components are present. It returns an error if any required component is missing or if the structure of the CST is not as expected.
    pub(super) fn build_catalog_object_name_from_graph_type_parent(
        cst: &CatalogGraphTypeParentAndNameContextAll,
    ) -> GqlResult<CatalogObjectName> {
        let span = Self::extract_span(cst);

        let name = cst
            .graphTypeName()
            .ok_or_else(|| Self::parse_err("CST not valid: missing graphTypeName".into()))?
            .get_text();
        let parent = cst.catalogObjectParentReference().map(|p| p.get_text());

        Ok(CatalogObjectName {
            parent,
            name,
            span: span.clone(),
        })
    }

    /// Builds an OfGraphTypeClause from an OfGraphTypeContext, handling the different variants of the ofGraphType clause (LIKE, TYPED reference, nested specification) and extracting the relevant components from the CST to construct the corresponding AST nodes. It performs validation to ensure that the expected structure of the CST is present for each variant and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_of_graph_type_clause(
        cst: &OfGraphTypeContextAll,
    ) -> GqlResult<CreateGraphTypeClause> {
        if let Some(like_ctx) = cst.graphTypeLikeGraph() {
            let graph_expr = like_ctx.graphExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: missing graphExpression in LIKE".into())
            })?;
            return Ok(CreateGraphTypeClause::Of(OfGraphTypeClause::LikeGraph {
                graph_expression: Self::build_graph_expression_clause(&graph_expr),
            }));
        }

        if let Some(reference_ctx) = cst.graphTypeReference() {
            return Ok(CreateGraphTypeClause::Of(
                OfGraphTypeClause::TypedReference {
                    typed: cst.typed().is_some(),
                    graph_type_reference: reference_ctx.get_text(),
                },
            ));
        }

        if let Some(nested_ctx) = cst.nestedGraphTypeSpecification() {
            return Ok(CreateGraphTypeClause::Of(
                OfGraphTypeClause::TypedNestedSpecification {
                    typed: cst.typed().is_some(),
                    property_graph_keyword: cst.PROPERTY().is_some(),
                    graph_keyword: cst.GRAPH().is_some(),
                    nested_graph_type_specification: Self::build_nested_graph_type_specification(
                        &nested_ctx,
                    ),
                },
            ));
        }

        Self::parse_error("CST not valid: unsupported ofGraphType variant")
    }

    /// Builds a GraphTypeSourceClause from a GraphTypeSourceContext, handling the different variants of the graphTypeSource clause (COPY OF, LIKE, nested specification) and extracting the relevant components from the CST to construct the corresponding AST nodes. It performs validation to ensure that the expected structure of the CST is present for each variant and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_graph_type_source_clause(
        cst: &GraphTypeSourceContextAll,
    ) -> GqlResult<GraphTypeSourceClause> {
        if let Some(copy_ctx) = cst.copyOfGraphType() {
            let graph_type_reference = copy_ctx
                .graphTypeReference()
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: missing graphTypeReference in copyOfGraphType".into(),
                    )
                })?
                .get_text();

            return Ok(GraphTypeSourceClause::CopyOf {
                with_as_keyword: cst.AS().is_some(),
                graph_type_reference,
            });
        }

        if let Some(like_ctx) = cst.graphTypeLikeGraph() {
            let graph_expr = like_ctx.graphExpression().ok_or_else(|| {
                Self::parse_err("CST not valid: missing graphExpression in LIKE".into())
            })?;
            return Ok(GraphTypeSourceClause::LikeGraph {
                graph_expression: Self::build_graph_expression_clause(&graph_expr),
            });
        }

        if let Some(nested_ctx) = cst.nestedGraphTypeSpecification() {
            return Ok(GraphTypeSourceClause::NestedSpecification {
                with_as_keyword: cst.AS().is_some(),
                nested_graph_type_specification: Self::build_nested_graph_type_specification(
                    &nested_ctx,
                ),
            });
        }

        Self::parse_error("CST not valid: unsupported graphTypeSource variant")
    }

    /// Builds a NestedGraphTypeSpecification from a NestedGraphTypeSpecificationContext, extracting the list of element type specifications (nodes and edges) from the CST and constructing the corresponding AST nodes for each element type. It performs validation to ensure that the expected structure of the CST is present and returns an empty specification if no element type specifications are found.
    pub(super) fn build_nested_graph_type_specification(
        cst: &NestedGraphTypeSpecificationContextAll,
    ) -> NestedGraphTypeSpecification {
        let element_type_specifications = cst
            .graphTypeSpecificationBody()
            .and_then(|body| body.elementTypeList())
            .map(|list| {
                list.elementTypeSpecification_all()
                    .iter()
                    .filter_map(|element| {
                        if let Some(node_spec) = element.nodeTypeSpecification() {
                            return Some(ElementTypeSpecification::Node(NodeTypeSpecification {
                                text: node_spec.get_text(),
                                local_alias: Self::extract_local_node_type_alias(&node_spec),
                                type_name: Self::extract_node_type_name(&node_spec),
                                property_types: Self::extract_property_types_from_node_spec(
                                    &node_spec,
                                ),
                            }));
                        }

                        element.edgeTypeSpecification().map(|edge_spec| {
                            ElementTypeSpecification::Edge(EdgeTypeSpecification {
                                text: edge_spec.get_text(),
                                type_name: Self::extract_edge_type_name(&edge_spec),
                                source_alias: Self::extract_source_node_alias(&edge_spec),
                                destination_alias: Self::extract_destination_node_alias(&edge_spec),
                                directed: Self::extract_edge_directed_flag(&edge_spec),
                                property_types: Self::extract_property_types_from_edge_spec(
                                    &edge_spec,
                                ),
                            })
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        NestedGraphTypeSpecification {
            element_type_specifications,
        }
    }

    /// Helper function to extract property type specifications from a NodeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to property type specifications in both node type patterns and phrases, and constructing the corresponding PropertyTypesSpecification AST node if a match is found. It returns None if no property type specification is identified in the CST.
    pub(super) fn extract_property_types_from_node_spec(
        cst: &NodeTypeSpecificationContextAll,
    ) -> Option<PropertyTypesSpecification> {
        let from_pattern = cst
            .nodeTypePattern()
            .and_then(|pattern| pattern.nodeTypeFiller())
            .and_then(|filler| filler.nodeTypeImpliedContent())
            .and_then(|content| content.nodeTypePropertyTypes())
            .and_then(|prop| prop.propertyTypesSpecification())
            .map(|spec| Self::build_property_types_specification(&spec));

        if from_pattern.is_some() {
            return from_pattern;
        }

        cst.nodeTypePhrase()
            .and_then(|phrase| phrase.nodeTypePhraseFiller())
            .and_then(|filler| filler.nodeTypeFiller())
            .and_then(|filler| filler.nodeTypeImpliedContent())
            .and_then(|content| content.nodeTypePropertyTypes())
            .and_then(|prop| prop.propertyTypesSpecification())
            .map(|spec| Self::build_property_types_specification(&spec))
    }

    /// Helper function to extract property type specifications from an EdgeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to property type specifications in both edge type patterns and phrases, and constructing the corresponding PropertyTypesSpecification AST node if a match is found. It returns None if no property type specification is identified in the CST.
    pub(super) fn extract_property_types_from_edge_spec(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> Option<PropertyTypesSpecification> {
        let from_phrase = cst
            .edgeTypePhrase()
            .and_then(|phrase| phrase.edgeTypePhraseFiller())
            .and_then(|filler| filler.edgeTypeFiller())
            .and_then(|filler| filler.edgeTypeImpliedContent())
            .and_then(|content| content.edgeTypePropertyTypes())
            .and_then(|prop| prop.propertyTypesSpecification())
            .map(|spec| Self::build_property_types_specification(&spec));

        if from_phrase.is_some() {
            return from_phrase;
        }

        Self::extract_property_types_specification_from_text(&cst.get_text())
    }

    /// Helper function to extract a local node type alias from a NodeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to local node type aliases in both node type patterns and phrases, and returning the alias text if a match is found. It returns None if no local node type alias is identified in the CST.
    pub(super) fn extract_local_node_type_alias(
        cst: &NodeTypeSpecificationContextAll,
    ) -> Option<String> {
        cst.nodeTypePattern()
            .and_then(|pattern| pattern.localNodeTypeAlias())
            .map(|alias| alias.get_text())
            .or_else(|| {
                cst.nodeTypePhrase()
                    .and_then(|phrase| phrase.localNodeTypeAlias())
                    .map(|alias| alias.get_text())
            })
    }

    /// Helper function to extract a node type name from a NodeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to node type names in both node type patterns and phrases, and returning the name text if a match is found. It returns None if no node type name is identified in the CST.
    pub(super) fn extract_node_type_name(cst: &NodeTypeSpecificationContextAll) -> Option<String> {
        cst.nodeTypePattern()
            .and_then(|pattern| pattern.nodeTypeName())
            .map(|name| name.get_text())
            .or_else(|| {
                cst.nodeTypePhrase()
                    .and_then(|phrase| phrase.nodeTypePhraseFiller())
                    .and_then(|filler| filler.nodeTypeName())
                    .map(|name| name.get_text())
            })
    }

    /// Helper function to extract an edge type name from an EdgeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to edge type names in both edge type patterns and phrases, and returning the name text if a match is found. It returns None if no edge type name is identified in the CST.
    pub(super) fn extract_edge_type_name(cst: &EdgeTypeSpecificationContextAll) -> Option<String> {
        cst.edgeTypePattern()
            .and_then(|pattern| pattern.edgeTypeName())
            .map(|name| name.get_text())
            .or_else(|| {
                cst.edgeTypePhrase()
                    .and_then(|phrase| phrase.edgeTypePhraseFiller())
                    .and_then(|filler| filler.edgeTypeName())
                    .map(|name| name.get_text())
            })
    }

    /// Helper function to extract a source node alias from an EdgeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to source node aliases in both edge type patterns and phrases, and returning the alias text if a match is found. It returns None if no source node alias is identified in the CST.
    pub(super) fn extract_source_node_alias(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> Option<String> {
        cst.edgeTypePattern()
            .and_then(|pattern| {
                if let Some(directed) = pattern.edgeTypePatternDirected() {
                    if let Some(right) = directed.edgeTypePatternPointingRight() {
                        return right
                            .sourceNodeTypeReference()
                            .and_then(|src| src.sourceNodeTypeAlias())
                            .map(|alias| alias.get_text());
                    }

                    if let Some(left) = directed.edgeTypePatternPointingLeft() {
                        return left
                            .sourceNodeTypeReference()
                            .and_then(|src| src.sourceNodeTypeAlias())
                            .map(|alias| alias.get_text());
                    }
                }

                pattern
                    .edgeTypePatternUndirected()
                    .and_then(|undirected| undirected.sourceNodeTypeReference())
                    .and_then(|src| src.sourceNodeTypeAlias())
                    .map(|alias| alias.get_text())
            })
            .or_else(|| {
                cst.edgeTypePhrase()
                    .and_then(|phrase| phrase.endpointPairPhrase())
                    .and_then(|pair| pair.endpointPair())
                    .and_then(|pair| pair.endpointPairDirected())
                    .and_then(|directed| directed.endpointPairPointingRight())
                    .and_then(|p| p.sourceNodeTypeAlias())
                    .map(|alias| alias.get_text())
            })
    }

    /// Helper function to extract a destination node alias from an EdgeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to destination node aliases in both edge type patterns and phrases, and returning the alias text if a match is found. It returns None if no destination node alias is identified in the CST.
    pub(super) fn extract_destination_node_alias(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> Option<String> {
        cst.edgeTypePattern()
            .and_then(|pattern| {
                if let Some(directed) = pattern.edgeTypePatternDirected() {
                    if let Some(right) = directed.edgeTypePatternPointingRight() {
                        return right
                            .destinationNodeTypeReference()
                            .and_then(|dst| dst.destinationNodeTypeAlias())
                            .map(|alias| alias.get_text());
                    }

                    if let Some(left) = directed.edgeTypePatternPointingLeft() {
                        return left
                            .destinationNodeTypeReference()
                            .and_then(|dst| dst.destinationNodeTypeAlias())
                            .map(|alias| alias.get_text());
                    }
                }

                pattern
                    .edgeTypePatternUndirected()
                    .and_then(|undirected| undirected.destinationNodeTypeReference())
                    .and_then(|dst| dst.destinationNodeTypeAlias())
                    .map(|alias| alias.get_text())
            })
            .or_else(|| {
                cst.edgeTypePhrase()
                    .and_then(|phrase| phrase.endpointPairPhrase())
                    .and_then(|pair| pair.endpointPair())
                    .and_then(|pair| pair.endpointPairDirected())
                    .and_then(|directed| directed.endpointPairPointingRight())
                    .and_then(|p| p.destinationNodeTypeAlias())
                    .map(|alias| alias.get_text())
            })
    }

    /// Helper function to extract a directed flag from an EdgeTypeSpecificationContext, checking for the presence of specific CST patterns that correspond to directed or undirected edge type specifications in both edge type patterns and phrases, and returning true for directed edges, false for undirected edges, or None if no explicit directedness is identified in the CST.
    pub(super) fn extract_edge_directed_flag(
        cst: &EdgeTypeSpecificationContextAll,
    ) -> Option<bool> {
        if cst
            .edgeTypePattern()
            .and_then(|pattern| pattern.edgeTypePatternDirected())
            .is_some()
        {
            return Some(true);
        }

        if cst
            .edgeTypePattern()
            .and_then(|pattern| pattern.edgeTypePatternUndirected())
            .is_some()
        {
            return Some(false);
        }

        if let Some(kind) = cst.edgeTypePhrase().and_then(|phrase| phrase.edgeKind()) {
            if kind.DIRECTED().is_some() {
                return Some(true);
            }
            if kind.UNDIRECTED().is_some() {
                return Some(false);
            }
        }

        None
    }

    /// Helper function to extract property type specifications from a text string, looking for a pattern that matches a property types specification (enclosed in curly braces) and parsing the individual property type entries to construct a PropertyTypesSpecification AST node. It returns None if no valid property types specification is found in the text.
    pub(super) fn extract_property_types_specification_from_text(
        text: &str,
    ) -> Option<PropertyTypesSpecification> {
        let open = text.find('{')?;
        let close = text.rfind('}')?;
        if close <= open {
            return None;
        }

        let body = text[open + 1..close].trim();
        if body.is_empty() {
            return Some(PropertyTypesSpecification { properties: vec![] });
        }

        let properties = body
            .split(',')
            .filter_map(|entry| {
                let token = entry.trim();
                if token.is_empty() {
                    return None;
                }

                let mut parts = token.split_whitespace();
                let name = parts.next()?.to_string();
                let mut typed = false;
                let mut value_parts = Vec::new();

                for part in parts {
                    if part.eq_ignore_ascii_case("TYPED") {
                        typed = true;
                    } else {
                        value_parts.push(part);
                    }
                }

                let value_type = value_parts.join(" ");
                Some(PropertyTypeSpecification {
                    name,
                    typed,
                    value_type,
                    parsed_value_type: None,
                    span: Span::default(),
                })
            })
            .collect::<Vec<_>>();

        Some(PropertyTypesSpecification { properties })
    }

    /// Builds a PropertyTypesSpecification from a PropertyTypesSpecificationContext, extracting the list of property type specifications from the CST and constructing the corresponding AST nodes for each property. It performs validation to ensure that the expected structure of the CST is present and returns an empty specification if no property type specifications are found.
    pub(super) fn build_property_types_specification(
        cst: &PropertyTypesSpecificationContextAll,
    ) -> PropertyTypesSpecification {
        let span = Self::extract_span(cst);

        let properties = cst
            .propertyTypeList()
            .map(|list| {
                list.propertyType_all()
                    .iter()
                    .map(|property| {
                        let parsed_value_type = property
                            .propertyValueType()
                            .and_then(|value| value.valueType())
                            .and_then(|value_type| {
                                Self::build_type_spec_from_value_type(&value_type).ok()
                            });

                        PropertyTypeSpecification {
                            name: property
                                .propertyName()
                                .map(|name| name.get_text())
                                .unwrap_or_default(),
                            typed: property.typed().is_some(),
                            value_type: property
                                .propertyValueType()
                                .map(|value| value.get_text())
                                .unwrap_or_default(),
                            parsed_value_type,
                            span: span.clone(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        PropertyTypesSpecification { properties }
    }

    /// Builds a GraphExpressionClause from a GraphExpressionContext, determining the specific kind of graph expression based on the presence of certain components in the CST (e.g., graph reference, object expression primary, object name or binding variable) and extracting the relevant text to construct the corresponding AST node. It performs validation to ensure that the expected structure of the CST is present for each kind of graph expression and returns an error if the structure is not as expected.
    pub(super) fn build_graph_expression_clause(
        cst: &GraphExpressionContextAll,
    ) -> GraphExpressionClause {
        let span = Self::extract_span(cst);

        let kind = if cst.graphReference().is_some() {
            GraphExpressionKind::GraphReference
        } else if cst.objectExpressionPrimary().is_some() {
            GraphExpressionKind::ObjectExpressionPrimary
        } else if cst.objectNameOrBindingVariable().is_some() {
            GraphExpressionKind::ObjectNameOrBindingVariable
        } else {
            GraphExpressionKind::CurrentGraph
        };

        GraphExpressionClause {
            kind,
            text: cst.get_text(),
            graph_reference: cst.graphReference().map(|x| x.get_text()),
            object_name_or_binding_variable: cst
                .objectNameOrBindingVariable()
                .map(|x| x.get_text()),
            span: span.clone(),
        }
    }

    /// Builds a Query from a CompositeQueryExpressionContext, handling both simple queries and set operations, and extracting the relevant components from the CST.
    pub(super) fn build_query_from_composite_expression(
        expr: &CompositeQueryExpressionContextAll,
    ) -> GqlResult<Query> {
        let span = Self::extract_span(expr);

        if let Some(left_expr) = expr.compositeQueryExpression() {
            let left_query = Self::build_query_from_composite_expression(&left_expr)?;

            let conjunction = expr.queryConjunction().ok_or_else(|| {
                Self::parse_err("Query not valid: missing queryConjunction".into())
            })?;

            let operation = if conjunction.OTHERWISE().is_some() {
                SetOperationType::Otherwise
            } else {
                let set_operator = conjunction.setOperator().ok_or_else(|| {
                    Self::parse_err("Query not valid: missing setOperator".into())
                })?;
                Self::build_set_operation_type(&set_operator)?
            };

            let right_primary = expr.compositeQueryPrimary().ok_or_else(|| {
                Self::parse_err("Query not valid: missing compositeQueryPrimary".into())
            })?;
            let right_query = Self::build_query_from_composite_primary(&right_primary)?;

            return Ok(Query::SetOperation(SetOperation {
                left: Box::new(left_query),
                operation,
                right: Box::new(right_query),
                limit_clause: None,
                order_clause: None,
                span: span.clone(),
            }));
        }
        let primary = expr.compositeQueryPrimary().ok_or_else(|| {
            Self::parse_err("Query not valid: missing compositeQueryPrimary".into())
        })?;

        Self::build_query_from_composite_primary(&primary)
    }

    /// Builds a Statement from a StatementBlockContext, handling both single statements and chains of statements connected by NEXT, and extracting the relevant components from the CST.
    pub(super) fn build_statement_block_from_context(
        block: &StatementBlockContextAll,
    ) -> GqlResult<Statement> {
        let first_ctx = block
            .statement()
            .ok_or_else(|| Self::parse_err("Query not valid: missing statement".into()))?;
        let first_statement = Self::build_statement_from_context(&first_ctx)?;

        let next_list = block.nextStatement_all();
        if next_list.is_empty() {
            return Ok(first_statement);
        }

        let chain = Self::build_next_chain(first_statement, &next_list, 0)?;
        Ok(Statement::Next(chain))
    }

    /// Builds a NextStatement from a chain of NextStatementContext, recursively processing the list of NEXT statements and extracting the relevant components from the CST for each step in the chain.
    pub(super) fn build_next_chain(
        source_statement: Statement,
        next_list: &[Rc<NextStatementContextAll>],
        index: usize,
    ) -> GqlResult<NextStatement> {
        let span = if let Some(next_ctx) = next_list.get(index) {
            Self::extract_span(&**next_ctx)
        } else {
            Span::default()
        };

        let next_ctx = next_list
            .get(index)
            .ok_or_else(|| Self::parse_err("NEXT chain index out of bounds".into()))?;

        let target_ctx = next_ctx.statement().ok_or_else(|| {
            Self::parse_err("Query not valid: missing statement after NEXT".into())
        })?;
        let target_statement = Self::build_statement_from_context(&target_ctx)?;

        let yield_clause = next_ctx
            .yieldClause()
            .map(|y| Self::build_yield_clause_from_context(&y))
            .transpose()?;

        let next = if index + 1 < next_list.len() {
            Some(Box::new(Self::build_next_chain(
                target_statement.clone(),
                next_list,
                index + 1,
            )?))
        } else {
            None
        };

        Ok(NextStatement {
            source_statement: Box::new(source_statement),
            target_statement: Box::new(target_statement),
            yield_clause,
            next,
            span: span.clone(),
        })
    }

    /// Builds a Query from a CompositeQueryPrimaryContext, handling both simple queries and set operations, and extracting the relevant components from the CST.
    pub(super) fn build_query_from_composite_primary(
        primary: &CompositeQueryPrimaryContextAll,
    ) -> GqlResult<Query> {
        let linear = primary.linearQueryStatement().ok_or_else(|| {
            Self::parse_err("Query not valid: missing linearQueryStatement".into())
        })?;

        if let Some(ambient) = linear.ambientLinearQueryStatement() {
            return Self::build_query_from_ambient_linear_query_statement(&ambient);
        }

        if let Some(focused) = linear.focusedLinearQueryStatement() {
            return Self::build_query_from_focused_linear_query_statement(&focused);
        }

        Self::parse_error("Query not valid: unsupported linearQueryStatement variant")
    }

    /// Builds a Query from an AmbientLinearQueryStatementContext, handling both nested query specifications and simple linear query statements, and extracting the relevant components from the CST.
    pub(super) fn build_query_from_ambient_linear_query_statement(
        ambient: &AmbientLinearQueryStatementContextAll,
    ) -> GqlResult<Query> {
        let span = Self::extract_span(ambient);
        if let Some(nested) = ambient.nestedQuerySpecification() {
            return Self::build_query_from_nested_query_spec(&nested);
        }

        let mut pipeline: Vec<QueryStep> = Vec::new();
        if let Some(simple_linear_query) = ambient.simpleLinearQueryStatement() {
            pipeline.extend(Self::build_pipeline_from_simple_linear_query_statement(
                &simple_linear_query,
            )?);
        }

        let primitive_result_statement = ambient.primitiveResultStatement().ok_or_else(|| {
            Self::parse_err("Query not valid: missing primitiveResultStatement".into())
        })?;

        let result_statement = Self::build_result_statement_from_primitive_result_statement(
            &primitive_result_statement,
        )?;

        Ok(Query::Basic(BasicQuery {
            pipeline,
            result_statement,
            span: span.clone(),
        }))
    }

    /// Builds a Query from a FocusedLinearQueryStatementContext, handling nested query specifications, simple linear query statements, and primitive result statements, and extracting the relevant components from the CST to construct the corresponding AST nodes.
    pub(super) fn build_query_from_focused_linear_query_statement(
        focused: &FocusedLinearQueryStatementContextAll,
    ) -> GqlResult<Query> {
        let span = Self::extract_span(focused);

        if let Some(select_statement) = focused.selectStatement() {
            return Self::build_query_from_select_statement(&select_statement);
        }

        if let Some(focused_nested) = focused.focusedNestedQuerySpecification() {
            let nested = focused_nested.nestedQuerySpecification().ok_or_else(|| {
                Self::parse_err(
                    "Query not valid: missing nestedQuerySpecification in focused nested query"
                        .into(),
                )
            })?;
            let mut query = Self::build_query_from_nested_query_spec(&nested)?;

            if let Some(use_clause) = focused_nested.useGraphClause() {
                if let Some(graph_expr) = use_clause.graphExpression() {
                    let graph_clause = Self::build_graph_expression_clause(&graph_expr);
                    Self::inject_use_graph_into_query(&mut query, &graph_clause);
                }
            }

            return Ok(query);
        }

        if let Some(focused_primitive) = focused.focusedPrimitiveResultStatement() {
            let mut pipeline: Vec<QueryStep> = Vec::new();

            if let Some(use_clause) = focused_primitive.useGraphClause() {
                if let Some(graph_expr) = use_clause.graphExpression() {
                    pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(
                        &graph_expr,
                    )));
                }
            }

            let primitive_result_statement = focused_primitive
                .primitiveResultStatement()
                .ok_or_else(|| {
                    Self::parse_err(
                "Query not valid: missing primitiveResultStatement in focused primitive result"
                    .into(),
            )
                })?;
            let result_statement = Self::build_result_statement_from_primitive_result_statement(
                &primitive_result_statement,
            )?;

            return Ok(Query::Basic(BasicQuery {
                pipeline,
                result_statement,
                span: span.clone(),
            }));
        }

        let mut pipeline: Vec<QueryStep> = Vec::new();
        for part in focused.focusedLinearQueryStatementPart_all() {
            if let Some(use_clause) = part.useGraphClause() {
                if let Some(graph_expr) = use_clause.graphExpression() {
                    pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(
                        &graph_expr,
                    )));
                }
            }

            if let Some(simple_linear) = part.simpleLinearQueryStatement() {
                pipeline.extend(Self::build_pipeline_from_simple_linear_query_statement(
                    &simple_linear,
                )?);
            }
        }

        let final_part = focused
            .focusedLinearQueryAndPrimitiveResultStatementPart()
            .ok_or_else(|| {
                Self::parse_err("Query not valid: missing final focused linear query part".into())
            })?;

        if let Some(use_clause) = final_part.useGraphClause() {
            if let Some(graph_expr) = use_clause.graphExpression() {
                pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(
                    &graph_expr,
                )));
            }
        }

        if let Some(simple_linear) = final_part.simpleLinearQueryStatement() {
            pipeline.extend(Self::build_pipeline_from_simple_linear_query_statement(
                &simple_linear,
            )?);
        }

        let primitive_result_statement =
            final_part.primitiveResultStatement().ok_or_else(|| {
                Self::parse_err(
                    "Query not valid: missing primitiveResultStatement in focused final part"
                        .into(),
                )
            })?;

        let result_statement = Self::build_result_statement_from_primitive_result_statement(
            &primitive_result_statement,
        )?;

        Ok(Query::Basic(BasicQuery {
            pipeline,
            result_statement,
            span: span.clone(),
        }))
    }

    /// Builds a Query from a NestedQuerySpecificationContext, extracting the procedure body and statement block from the CST, and constructing the corresponding AST nodes for the nested query. It performs validation to ensure that the expected structure of the CST is present and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_query_from_nested_query_spec(
        nested: &NestedQuerySpecificationContextAll,
    ) -> GqlResult<Query> {
        let procedure_body = nested.procedureBody().ok_or_else(|| {
            Self::parse_err("Query not valid: missing procedureBody in nested query".into())
        })?;

        let statement_block = procedure_body.statementBlock().ok_or_else(|| {
            Self::parse_err("Query not valid: missing statementBlock in nested query".into())
        })?;

        let statement = Self::build_statement_block_from_context(&statement_block)?;
        match statement {
            Statement::Query(query) => Ok(query),
            _ => Self::parse_error("Nested query specification must contain a query statement"),
        }
    }

    /// Helper function to inject a UseGraph clause into a Query, recursively traversing the structure of the query to find the appropriate location to insert the UseGraph step at the beginning of the pipeline for basic queries, or to propagate it down to both sides of a set operation for composite queries. It modifies the query in place and does not return a value.
    pub(super) fn inject_use_graph_into_query(
        query: &mut Query,
        graph_clause: &GraphExpressionClause,
    ) {
        match query {
            Query::Basic(basic) => {
                basic
                    .pipeline
                    .insert(0, QueryStep::UseGraph(graph_clause.clone()));
            }
            Query::SetOperation(set_op) => {
                Self::inject_use_graph_into_query(&mut set_op.left, graph_clause);
                Self::inject_use_graph_into_query(&mut set_op.right, graph_clause);
            }
        }
    }

    /// Builds a pipeline of QuerySteps from a SimpleLinearQueryStatementContext, handling different types of simple query statements (e.g., MATCH, OPTIONAL MATCH, LET, FILTER, FOR, ORDER BY) and extracting the relevant components from the CST to construct the corresponding AST nodes for each step in the pipeline. It performs validation to ensure that the expected structure of the CST is present and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_pipeline_from_simple_linear_query_statement(
        simple_linear_query: &SimpleLinearQueryStatementContextAll,
    ) -> GqlResult<Vec<QueryStep>> {
        let span = Self::extract_span(simple_linear_query);
        let mut pipeline: Vec<QueryStep> = Vec::new();

        for simple_query_statement in simple_linear_query.simpleQueryStatement_all() {
            if let Some(primitive) = simple_query_statement.primitiveQueryStatement() {
                if let Some(m) = primitive.matchStatement() {
                    for (graph_pattern, optional) in Self::extract_graph_patterns(&m)? {
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
                } else if let Some(l) = primitive.letStatement() {
                    let let_clause = Self::build_let_statement_from_context(&l)?;
                    pipeline.push(QueryStep::Let(let_clause));
                } else if let Some(f) = primitive.filterStatement() {
                    let filter_clause = Self::build_filter_clause_from_context(&f)?;
                    pipeline.push(QueryStep::Filter(filter_clause));
                } else if let Some(f) = primitive.forStatement() {
                    let for_clause = Self::build_for_clause_from_context(&f)?;
                    pipeline.push(QueryStep::For(for_clause));
                } else if let Some(p) = primitive.orderByAndPageStatement() {
                    let order_clause = p
                        .orderByClause()
                        .map(|ctx| Self::build_order_clause_from_context(&ctx))
                        .transpose()?;
                    let limit_clause = Self::build_limit_clause_from_context(&p)?;

                    pipeline.push(QueryStep::Page(OrderByAndPageStatement {
                        order_clause,
                        limit_clause,
                        span: span.clone(),
                    }));
                }

                if primitive.matchStatement().is_none()
                    && primitive.letStatement().is_none()
                    && primitive.filterStatement().is_none()
                    && primitive.forStatement().is_none()
                    && primitive.orderByAndPageStatement().is_none()
                {
                    return Self::parse_error(format!(
                        "Unsupported primitive query statement: {}",
                        primitive.get_text()
                    ));
                }
            } else if let Some(call_query) = simple_query_statement.callQueryStatement() {
                let call_proc = call_query.callProcedureStatement().ok_or_else(|| {
                    Self::parse_err("CST not valid: missing callProcedureStatement".into())
                })?;
                let call_clause = Self::build_call_clause_from_context(&call_proc)?;
                pipeline.push(QueryStep::Call(call_clause));
            } else {
                return Self::parse_error(format!(
                    "Unknown query statement variant encountered: {}",
                    simple_query_statement.get_text()
                ));
            }
        }
        Ok(pipeline)
    }

    /// Builds a PrimitiveResultStatement from a PrimitiveResultStatementContext, handling both RETURN and FINISH statements, and extracting the relevant components from the CST to construct the corresponding AST node. It performs validation to ensure that the expected structure of the CST is present and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_result_statement_from_primitive_result_statement(
        primitive_result_statement: &PrimitiveResultStatementContextAll,
    ) -> GqlResult<PrimitiveResultStatement> {
        let span = Self::extract_span(primitive_result_statement);
        if primitive_result_statement.FINISH().is_some() {
            return Ok(PrimitiveResultStatement::Finish { span: span.clone() });
        }

        if let Some(return_stmt) = primitive_result_statement.returnStatement() {
            let return_clause = Self::build_return_clause_from_context(&return_stmt)?;

            let group_clause = return_stmt
                .returnStatementBody()
                .and_then(|body| body.groupByClause())
                .map(|ctx| Self::build_group_clause_from_context(&ctx))
                .transpose()?;

            let page_clause =
                if let Some(order_page) = primitive_result_statement.orderByAndPageStatement() {
                    let order_clause = order_page
                        .orderByClause()
                        .map(|ctx| Self::build_order_clause_from_context(&ctx))
                        .transpose()?;
                    let limit_clause = Self::build_limit_clause_from_context(&order_page)?;

                    Some(OrderByAndPageStatement {
                        order_clause,
                        limit_clause,
                        span: span.clone(),
                    })
                } else {
                    None
                };

            return Ok(PrimitiveResultStatement::Return {
                return_clause,
                group_clause,
                page_clause,
                span: span.clone(),
            });
        }

        Self::parse_error("Query not valid: primitiveResultStatement without RETURN or FINISH")
    }

    /// Builds a Query from a SelectStatementContext, handling the various components of a SELECT statement (e.g., MATCH, WHERE, RETURN, GROUP BY, HAVING, ORDER BY, LIMIT) and extracting the relevant components from the CST to construct the corresponding AST nodes for each part of the SELECT statement. It performs validation to ensure that the expected structure of the CST is present and returns an error if any required component is missing or if the structure is not as expected.
    pub(super) fn build_query_from_select_statement(
        select_stmt: &SelectStatementContextAll,
    ) -> GqlResult<Query> {
        let span = Self::extract_span(select_stmt);
        let mut pipeline: Vec<QueryStep> = Vec::new();

        if let Some(body) = select_stmt.selectStatementBody() {
            if let Some(graph_matches) = body.selectGraphMatchList() {
                for select_graph_match in graph_matches.selectGraphMatch_all() {
                    if let Some(graph_expression) = select_graph_match.graphExpression() {
                        pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(
                            &graph_expression,
                        )));
                    }

                    let match_statement = select_graph_match.matchStatement().ok_or_else(|| {
                        Self::parse_err(
                            "CST not valid: missing matchStatement in SELECT FROM".into(),
                        )
                    })?;

                    for (graph_pattern, optional) in Self::extract_graph_patterns(&match_statement)?
                    {
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
            } else if let Some(query_spec) = body.selectQuerySpecification() {
                if let Some(graph_expression) = query_spec.graphExpression() {
                    pipeline.push(QueryStep::UseGraph(Self::build_graph_expression_clause(
                        &graph_expression,
                    )));
                }

                if let Some(nested_query_spec) = query_spec.nestedQuerySpecification() {
                    let from_query = Self::build_query_from_nested_query_spec(&nested_query_spec)?;
                    pipeline.push(QueryStep::FromSubquery(Box::new(from_query)));
                }
            }
        }

        if let Some(where_clause) = select_stmt.whereClause() {
            let condition = where_clause
                .searchCondition()
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: searchCondition missing in SELECT WHERE".into())
                })?
                .booleanValueExpression()
                .and_then(|expr| expr.valueExpression())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: booleanValueExpression missing in SELECT WHERE".into(),
                    )
                })
                .and_then(|expr| Self::build_expression_from_context(&expr))?;

            pipeline.push(QueryStep::Filter(FilterClause {
                condition,
                span: span.clone(),
            }));
        }

        let distinct = match select_stmt.setQuantifier() {
            Some(set_quantifier) if set_quantifier.DISTINCT().is_some() => {
                DistinctQualifier::Distinct
            }
            Some(set_quantifier) if set_quantifier.ALL().is_some() => DistinctQualifier::All,
            _ => DistinctQualifier::None,
        };

        let items = if let Some(select_item_list) = select_stmt.selectItemList() {
            select_item_list
                .selectItem_all()
                .into_iter()
                .map(|item| {
                    let expression = item
                        .aggregatingValueExpression()
                        .ok_or_else(|| {
                            Self::parse_err(
                                "CST not valid: aggregatingValueExpression missing in SELECT item"
                                    .into(),
                            )
                        })
                        .and_then(|agg| {
                            Self::build_expression_from_aggregating_value_expression(&agg)
                        })?;

                    let alias = item
                        .selectItemAlias()
                        .and_then(|alias_ctx| alias_ctx.identifier())
                        .map(|id| id.get_text());

                    Ok(ReturnItem {
                        expression,
                        alias,
                        span: span.clone(),
                    })
                })
                .collect::<GqlResult<Vec<_>>>()?
        } else {
            vec![ReturnItem {
                expression: Expression::Literal(Literal::String("*".to_string())),
                alias: None,
                span: span.clone(),
            }]
        };

        let return_clause = ReturnClause {
            distinct,
            items,
            span: span.clone(),
        };

        let group_clause = select_stmt
            .groupByClause()
            .map(|ctx| Self::build_group_clause_from_context(&ctx))
            .transpose()?;

        if let Some(having_clause) = select_stmt.havingClause() {
            let condition = having_clause
                .searchCondition()
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: searchCondition missing in HAVING".into())
                })?
                .booleanValueExpression()
                .and_then(|expr| expr.valueExpression())
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: booleanValueExpression missing in HAVING".into(),
                    )
                })
                .and_then(|expr| Self::build_expression_from_context(&expr))?;

            pipeline.push(QueryStep::Having(FilterClause {
                condition,
                span: span.clone(),
            }));
        }

        let order_clause = select_stmt
            .orderByClause()
            .map(|ctx| Self::build_order_clause_from_context(&ctx))
            .transpose()?;

        let limit_clause = Self::build_limit_clause_from_optional(
            select_stmt.offsetClause(),
            select_stmt.limitClause(),
        )?;

        let page_clause = if order_clause.is_some() || limit_clause.is_some() {
            Some(OrderByAndPageStatement {
                order_clause,
                limit_clause,
                span: span.clone(),
            })
        } else {
            None
        };

        Ok(Query::Basic(BasicQuery {
            pipeline,
            result_statement: PrimitiveResultStatement::Return {
                return_clause,
                group_clause,
                page_clause,
                span: span.clone(),
            },
            span: span.clone(),
        }))
    }

    /// Builds a LimitClause from optional OffsetClauseContext and LimitClauseContext, extracting the count and offset values from the CST if present, and constructing the corresponding AST node. It performs validation to ensure that the count and offset values are valid non-negative integers, and returns an error if the values are not valid or if the structure of the CST is not as expected.
    pub(super) fn build_limit_clause_from_optional(
        offset_clause: Option<Rc<OffsetClauseContextAll>>,
        limit_clause: Option<Rc<LimitClauseContextAll>>,
    ) -> GqlResult<Option<LimitClause>> {
        let span = if let Some(limit) = &limit_clause {
            Self::extract_span(&**limit)
        } else if let Some(offset) = &offset_clause {
            Self::extract_span(&**offset)
        } else {
            Span::default()
        };

        let count = limit_clause
            .and_then(|ctx| ctx.nonNegativeIntegerSpecification())
            .map(|spec| Self::parse_non_negative_integer_spec(&spec))
            .transpose()?;

        let offset = offset_clause
            .and_then(|ctx| ctx.nonNegativeIntegerSpecification())
            .map(|spec| Self::parse_non_negative_integer_spec(&spec))
            .transpose()?;

        if count.is_none() && offset.is_none() {
            return Ok(None);
        }

        Ok(Some(LimitClause {
            count,
            offset,
            span: span.clone(),
        }))
    }

    /// Builds a SetOperationType from a SetOperatorContext, handling UNION, INTERSECT, EXCEPT, and their ALL and DISTINCT variants, and extracting the relevant components from the CST.
    pub(super) fn build_set_operation_type(
        op: &SetOperatorContextAll,
    ) -> GqlResult<SetOperationType> {
        if op.UNION().is_some() {
            return Ok(match op.setQuantifier() {
                Some(q) if q.ALL().is_some() => SetOperationType::UnionAll,
                Some(q) if q.DISTINCT().is_some() => SetOperationType::UnionDistinct,
                _ => SetOperationType::Union,
            });
        }

        if op.INTERSECT().is_some() {
            return Ok(match op.setQuantifier() {
                Some(q) if q.ALL().is_some() => SetOperationType::IntersectAll,
                Some(q) if q.DISTINCT().is_some() => SetOperationType::IntersectDistinct,
                _ => SetOperationType::Intersect,
            });
        }

        if op.EXCEPT().is_some() {
            return Ok(match op.setQuantifier() {
                Some(q) if q.ALL().is_some() => SetOperationType::ExceptAll,
                Some(q) if q.DISTINCT().is_some() => SetOperationType::ExceptDistinct,
                _ => SetOperationType::Except,
            });
        }

        Self::parse_error("Unsupported set operator")
    }

    /// Builds a LetStatement from a LetStatementContext, extracting the list of variable definitions from the CST and constructing the corresponding AST node, handling both binding variable definitions and value variable definitions.
    pub(super) fn build_let_statement_from_context(
        cst: &LetStatementContextAll,
    ) -> GqlResult<LetStatement> {
        let span = Self::extract_span(cst);

        let list = cst
            .letVariableDefinitionList()
            .ok_or_else(|| Self::parse_err("Expected letVariableDefinitionList".to_string()))?;

        let variable_definitions = list
            .letVariableDefinition_all()
            .into_iter()
            .map(|def| Self::build_let_variable_definition_from_context(&def))
            .collect::<GqlResult<Vec<_>>>()?;

        if variable_definitions.is_empty() {
            return Err(Self::parse_err(
                "LET must contain at least one variable definition".to_string(),
            ));
        }

        Ok(LetStatement {
            variable_definitions,
            span: span.clone(),
        })
    }

    /// Builds a CallClause from a CallProcedureStatementContext, extracting the procedure name, arguments, and yield items.
    pub(super) fn build_call_clause_from_context(
        cst: &CallProcedureStatementContextAll,
    ) -> GqlResult<CallClause> {
        let span = Self::extract_span(cst);

        let is_optional = cst.OPTIONAL().is_some();
        let proc_call = cst.procedureCall().ok_or_else(|| {
            Self::parse_err("CST not valid: missing procedureCall in CALL statement".into())
        })?;

        if let Some(named_call) = proc_call.namedProcedureCall() {
            let procedure_name = named_call
                .procedureReference()
                .map(|pr| pr.get_text())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: missing procedureReference".into())
                })?;

            let mut arguments = Vec::new();
            if let Some(arg_list) = named_call.procedureArgumentList() {
                for arg in arg_list.procedureArgument_all() {
                    let expr_ctx = arg.valueExpression().ok_or_else(|| {
                        Self::parse_err("CST not valid: missing valueExpression in argument".into())
                    })?;
                    let expression = match Self::build_expression_from_context(&expr_ctx) {
                        Ok(expr) => expr,
                        Err(_) => Self::build_expression_from_leaf_text(
                            &expr_ctx.get_text(),
                            span.clone(),
                        )
                        .unwrap_or_else(|_| {
                            Expression::Literal(Literal::String(expr_ctx.get_text()))
                        }),
                    };
                    arguments.push(expression);
                }
            }

            let yield_items = if let Some(yield_ctx) = named_call.yieldClause() {
                Some(Self::build_yield_clause_from_context(&yield_ctx)?)
            } else {
                None
            };

            Ok(CallClause {
                procedure_name,
                arguments,
                yield_items,
                is_optional,
                inline_procedure_body: None,
                inline_procedure_statement: None,
                span: span.clone(),
            })
        } else if let Some(inline_call) = proc_call.inlineProcedureCall() {
            let inline_spec_ctx = inline_call.nestedProcedureSpecification().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: missing nestedProcedureSpecification in inline CALL".into(),
                )
            })?;
            let procedure_spec = inline_spec_ctx.procedureSpecification().ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: missing procedureSpecification in inline CALL".into(),
                )
            })?;

            let procedure_body = procedure_spec.procedureBody().ok_or_else(|| {
                Self::parse_err("CST not valid: missing procedureBody in inline CALL".into())
            })?;

            let statement_block = procedure_body.statementBlock().ok_or_else(|| {
                Self::parse_err("CST not valid: missing statementBlock in inline CALL".into())
            })?;

            let inline_statement = Self::build_statement_block_from_context(&statement_block)?;

            let mut arguments = Vec::new();

            if let Some(scope_clause) = inline_call.variableScopeClause() {
                if let Some(ref_list) = scope_clause.bindingVariableReferenceList() {
                    for scoped_var in ref_list.bindingVariableReference_all() {
                        arguments.push(Expression::Variable {
                            name: scoped_var.get_text(),
                            span: span.clone(),
                        });
                    }
                }
            }

            Ok(CallClause {
                procedure_name: "__INLINE_CALL__".to_string(),
                arguments,
                yield_items: None,
                is_optional,
                inline_procedure_body: None,
                inline_procedure_statement: Some(Box::new(inline_statement)),
                span: span.clone(),
            })
        } else {
            Self::parse_error(format!(
                "CST not valid: unsupported procedureCall variant: {}",
                proc_call.get_text()
            ))
        }
    }

    /// Builds a VariableDefinition from a LetVariableDefinitionContext, handling both binding variable definitions and value variable definitions, and extracting the relevant components from the CST.
    pub(super) fn build_let_variable_definition_from_context(
        cst: &LetVariableDefinitionContextAll,
    ) -> GqlResult<VariableDefinition> {
        let span = Self::extract_span(cst);

        if let Some(binding) = cst.bindingVariable() {
            let variable_name = binding.get_text();
            let value_expr = cst
                .valueExpression()
                .ok_or_else(|| Self::parse_err("Expected valueExpression in LET".to_string()))?;
            let expression = Self::build_expression_from_context(&value_expr)?;

            return Ok(VariableDefinition {
                variable_name,
                expression,
                span: span.clone(),
            });
        }

        if let Some(value_def) = cst.valueVariableDefinition() {
            let variable_name = value_def
                .bindingVariable()
                .ok_or_else(|| {
                    Self::parse_err("Expected bindingVariable in VALUE definition".to_string())
                })?
                .get_text();

            let value_expr = value_def
                .optTypedValueInitializer()
                .and_then(|init| init.valueInitializer())
                .and_then(|init| init.valueExpression())
                .ok_or_else(|| {
                    Self::parse_err("Expected value initializer in VALUE definition".to_string())
                })?;

            let expression = Self::build_expression_from_context(&value_expr)?;

            return Ok(VariableDefinition {
                variable_name,
                expression,
                span: span.clone(),
            });
        }

        let raw = cst.get_text();
        if let Some(eq_idx) = raw.find('=') {
            let left = raw[..eq_idx].trim();
            let right = raw[eq_idx + 1..].trim();

            let variable_name = left
                .split_whitespace()
                .last()
                .filter(|v| !v.is_empty())
                .unwrap_or("__unnamed_let_var__")
                .to_string();

            let expression = Self::build_expression_from_leaf_text(right, span.clone())
                .unwrap_or_else(|_| Expression::Literal(Literal::String(right.to_string())));

            return Ok(VariableDefinition {
                variable_name,
                expression,
                span: span.clone(),
            });
        }

        Ok(VariableDefinition {
            variable_name: "__unnamed_let_var__".to_string(),
            expression: Expression::Literal(Literal::String(raw)),
            span: span.clone(),
        })
    }

    /// Builds a YieldClause from a YieldClauseContext, extracting the list of yield items and their aliases from the CST, and constructing the corresponding AST nodes.
    pub(super) fn build_yield_clause_from_context(
        cst: &YieldClauseContextAll,
    ) -> GqlResult<YieldClause> {
        let span = Self::extract_span(cst);

        let list = cst
            .yieldItemList()
            .ok_or_else(|| Self::parse_err("CST not valid: yieldItemList missing".into()))?;

        let items = list
            .yieldItem_all()
            .into_iter()
            .map(|it| {
                let column_name = it
                    .yieldItemName()
                    .and_then(|n| n.fieldName())
                    .map(|f| f.get_text().trim().to_string())
                    .ok_or_else(|| {
                        Self::parse_err("CST not valid: yieldItemName missing".into())
                    })?;

                let alias = it
                    .yieldItemAlias()
                    .and_then(|a| a.bindingVariable())
                    .map(|b| b.get_text().trim().to_string());

                Ok(YieldItem {
                    column_name,
                    alias,
                    span: span.clone(),
                })
            })
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(YieldClause {
            items,
            span: span.clone(),
        })
    }
}
