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
    pub(crate) fn build_from_cst_root(cst: &GqlProgramContextAll) -> GqlResult<QueryAST> {
        let session_close = cst.sessionCloseCommand().map(|command| SessionCloseCommand {
            span: Self::extract_span(&*command),
        });

        let Some(activity) = cst.programActivity() else {
            let close = Self::required(session_close, "program activity or SESSION CLOSE")?;
            return Ok(QueryAST { span: close.span.clone(), body: ProgramBody::CloseOnly(close) });
        };

        let mut span = Self::extract_span(&*activity);
        if let Some(close) = &session_close {
            span.end_line = close.span.end_line;
            span.end_col = close.span.end_col;
        }

        let activity = if let Some(session) = activity.sessionActivity() {
            ProgramActivity::Session(Self::build_session_activity(&session))
        } else {
            let transaction = Self::required(activity.transactionActivity(), "transaction activity")?;
            ProgramActivity::Transaction(Self::build_transaction_activity(&transaction)?)
        };

        Ok(QueryAST { body: ProgramBody::Activity { activity, session_close }, span })
    }

    fn build_session_activity(cst: &SessionActivityContextAll) -> SessionActivity {
        // The grammar places all SET commands before any RESET commands.
        let sets = cst.sessionSetCommand_all().into_iter().map(|command| SessionCommand::Set {
            clause: Self::build_session_set_clause(&command),
            span: Self::extract_span(&*command),
        });
        let resets = cst.sessionResetCommand_all().into_iter().map(|command| SessionCommand::Reset {
            clause: Self::build_session_reset_clause(&command),
            span: Self::extract_span(&*command),
        });
        SessionActivity { commands: sets.chain(resets).collect(), span: Self::extract_span(cst) }
    }

    fn build_transaction_activity(cst: &TransactionActivityContextAll) -> GqlResult<TransactionActivity> {
        let start = cst.startTransactionCommand().map(|command| StartTransactionCommand {
            mode: Self::build_transaction_mode_from_start_command(&command),
            span: Self::extract_span(&*command),
        });
        let end = cst.endTransactionCommand().map(|command| {
            if let Some(commit) = command.commitCommand() {
                Ok(EndTransactionCommand::Commit { span: Self::extract_span(&*commit) })
            } else if let Some(rollback) = command.rollbackCommand() {
                Ok(EndTransactionCommand::Rollback { span: Self::extract_span(&*rollback) })
            } else {
                Self::parse_error("CST missing COMMIT or ROLLBACK")
            }
        }).transpose()?;

        if let Some(procedure) = cst.procedureSpecification() {
            let body = Self::required(procedure.procedureBody(), "procedure body")?;
            return Ok(TransactionActivity::WithBody {
                start,
                body: Self::build_procedure_body(&body)?,
                end,
                span: Self::extract_span(cst),
            });
        }

        match (start, end) {
            (Some(start), None) => Ok(TransactionActivity::StartOnly(start)),
            (None, Some(end)) => Ok(TransactionActivity::EndOnly(end)),
            _ => Self::parse_error("CST has invalid transaction boundaries"),
        }
    }

    pub(super) fn build_procedure_body(cst: &ProcedureBodyContextAll) -> GqlResult<ProcedureBody> {
        let at_schema = cst.atSchemaClause().map(|clause| {
            let schema = Self::required(clause.schemaReference(), "AT schema reference")?;
            Ok(AtSchemaClause {
                schema: Self::build_schema_reference(&schema)?,
                schema_span: Self::extract_span(&*schema),
                span: Self::extract_span(&*clause),
            })
        }).transpose()?;
        let definitions = cst.bindingVariableDefinitionBlock().map(|block| {
            block.bindingVariableDefinition_all().iter().map(|definition| {
                Self::build_local_definition(definition)
            }).collect::<GqlResult<Vec<_>>>()
        }).transpose()?.unwrap_or_default();
        let block = Self::required(cst.statementBlock(), "statement block")?;
        let first = Self::required(block.statement(), "first statement")?;
        let first = Self::build_statement_from_context(&first)?;
        let continuations = block.nextStatement_all().iter().map(|next| {
            let statement = Self::required(next.statement(), "statement after NEXT")?;
            Ok(NextContinuation {
                yield_clause: next.yieldClause().map(|clause| Self::build_yield_clause_from_context(&clause)).transpose()?,
                statement: Self::build_statement_from_context(&statement)?,
                span: Self::extract_span(&**next),
            })
        }).collect::<GqlResult<Vec<_>>>()?;

        Ok(ProcedureBody {
            at_schema,
            definitions,
            statements: StatementBlock {
                first,
                continuations,
                span: Self::extract_span(&*block),
            },
            span: Self::extract_span(cst),
        })
    }

    pub(super) fn build_nested_statement(
        nested: &NestedQuerySpecificationContextAll,
        graph_expression: Option<GraphExpressionClause>,
        span: Span,
    ) -> GqlResult<Statement> {
        let body = Self::required(nested.procedureBody(), "nested procedure body")?;
        let body = Box::new(Self::build_procedure_body(&body)?);
        if body.statements.iter().all(Statement::is_query) {
            return Ok(Statement::Query(Query::Nested(NestedQuery { body, graph_expression, span })));
        }

        let statement = Statement::Nested(body);
        Ok(match graph_expression {
            Some(graph_expression) => Statement::Focused(FocusedStatement {
                graph_expression, statement: Box::new(statement), span,
            }),
            None => statement,
        })
    }

    pub(super) fn try_build_nested_data_statement(cst: &LinearDataModifyingStatementContextAll) -> GqlResult<Option<Statement>> {
        if let Some(nested) = cst.ambientLinearDataModifyingStatement()
            .and_then(|ambient| ambient.nestedDataModifyingProcedureSpecification())
        {
            let body = Self::required(nested.procedureBody(), "nested data procedure body")?;
            return Ok(Some(Statement::Nested(Box::new(Self::build_procedure_body(&body)?))));
        }

        if let Some(focused) = cst.focusedLinearDataModifyingStatement()
            .and_then(|focused| focused.focusedNestedDataModifyingProcedureSpecification())
        {
            let body = Self::required(
                focused.nestedDataModifyingProcedureSpecification().and_then(|nested| nested.procedureBody()),
                "focused data procedure body",
            )?;
            let graph = Self::required(focused.useGraphClause().and_then(|clause| clause.graphExpression()), "focused data graph")?;
            return Ok(Some(Statement::Focused(FocusedStatement {
                graph_expression: Self::build_graph_expression_clause(&graph),
                statement: Box::new(Statement::Nested(Box::new(Self::build_procedure_body(&body)?))),
                span: Self::extract_span(&*focused),
            })));
        }

        Ok(None)
    }

    pub(super) fn build_query_from_nested_query_spec(nested: &NestedQuerySpecificationContextAll) -> GqlResult<Query> {
        match Self::build_nested_statement(nested, None, Self::extract_span(nested))? {
            Statement::Query(query) => Ok(query),
            _ => Self::parse_error("Nested query specification must contain query statements"),
        }
    }

    fn build_local_definition(cst: &BindingVariableDefinitionContextAll) -> GqlResult<BindingVariableDefinition> {
        let span = Self::extract_span(cst);
        if let Some(graph) = cst.graphVariableDefinition() {
            let name = Self::required(graph.bindingVariable(), "graph variable name")?;
            let typed = Self::required(graph.optTypedGraphInitializer(), "graph variable initializer")?;
            let initializer = Self::required(typed.graphInitializer().and_then(|i| i.graphExpression()), "graph expression")?;
            let declared_type = typed.graphReferenceValueType().map(|value_type| {
                Ok(DeclaredVariableType {
                    value_type: Self::build_graph_reference_type(&value_type)?,
                    explicitly_typed: typed.typed().is_some(),
                    span: Self::extract_span(&*value_type),
                })
            }).transpose()?;
            return Ok(BindingVariableDefinition::Graph(GraphVariableDefinition {
                name: Self::identifier_text(&name.get_text()),
                declared_type,
                initializer: Self::build_local_graph_expression(&initializer)?,
                property_keyword: graph.PROPERTY().is_some(),
                span,
            }));
        }

        if let Some(table) = cst.bindingTableVariableDefinition() {
            let name = Self::required(table.bindingVariable(), "table variable name")?;
            let typed = Self::required(table.optTypedBindingTableInitializer(), "table variable initializer")?;
            let initializer = Self::required(
                typed.bindingTableInitializer().and_then(|i| i.bindingTableExpression()), "table expression",
            )?;
            let declared_type = typed.bindingTableReferenceValueType().map(|value_type| {
                Ok(DeclaredVariableType {
                    value_type: Self::build_binding_table_reference_type(&value_type)?,
                    explicitly_typed: typed.typed().is_some(),
                    span: Self::extract_span(&*value_type),
                })
            }).transpose()?;
            return Ok(BindingVariableDefinition::BindingTable(BindingTableVariableDefinition {
                name: Self::identifier_text(&name.get_text()),
                declared_type,
                initializer: Self::build_local_binding_table_expression(&initializer)?,
                binding_keyword: table.BINDING().is_some(),
                span,
            }));
        }

        let value = Self::required(cst.valueVariableDefinition(), "value variable definition")?;
        let name = Self::required(value.bindingVariable(), "value variable name")?;
        let typed = Self::required(value.optTypedValueInitializer(), "value variable initializer")?;
        let initializer = Self::required(typed.valueInitializer().and_then(|i| i.valueExpression()), "initializer expression")?;
        let declared_type = typed.valueType().map(|value_type| {
            Ok(DeclaredVariableType {
                value_type: Self::build_value_type(&value_type)?,
                explicitly_typed: typed.typed().is_some(),
                span: Self::extract_span(&*value_type),
            })
        }).transpose()?;
        Ok(BindingVariableDefinition::Value(ValueVariableDefinition {
            name: Self::identifier_text(&name.get_text()),
            declared_type,
            initializer: Self::build_value_initializer(&initializer)?,
            initializer_span: Self::extract_span(&*initializer),
            span,
        }))
    }

    fn build_value_initializer(cst: &ValueExpressionContextAll) -> GqlResult<ValueInitializer> {
        match cst {
            ValueExpressionContextAll::PropertyGraphExprAltContext(value) => {
                let graph = Self::required(value.graphExpression(), "graph-valued initializer")?;
                Ok(ValueInitializer::Graph(Self::build_local_graph_expression(&graph)?))
            }
            ValueExpressionContextAll::BindingTableExprAltContext(value) => {
                let table = Self::required(value.bindingTableExpression(), "table-valued initializer")?;
                Ok(ValueInitializer::BindingTable(Self::build_local_binding_table_expression(&table)?))
            }
            _ => Ok(ValueInitializer::Expression(Self::build_expression_from_context(cst)?)),
        }
    }

    fn build_local_graph_expression(cst: &GraphExpressionContextAll) -> GqlResult<GraphExpression> {
        let value = if let Some(reference) = cst.graphReference() {
            if let Some(home) = reference.homeGraph() {
                GraphExpressionValue::Home { property: home.HOME_PROPERTY_GRAPH().is_some() }
            } else if let Some(parameter) = reference.referenceParameterSpecification() {
                GraphExpressionValue::Parameter(parameter.get_text())
            } else if let Some(name) = reference.delimitedGraphName() {
                GraphExpressionValue::DelimitedName(Self::identifier_text(&name.get_text()))
            } else {
                let parent = Self::required(reference.catalogObjectParentReference(), "catalog graph parent")?;
                let name = Self::required(reference.graphName(), "catalog graph name")?;
                GraphExpressionValue::Catalog(Self::build_catalog_object_reference(&parent, &name.get_text(), Self::extract_span(&*reference))?)
            }
        } else if let Some(current) = cst.currentGraph() {
            GraphExpressionValue::Current { property: current.CURRENT_PROPERTY_GRAPH().is_some() }
        } else if let Some(object) = cst.objectExpressionPrimary() {
            GraphExpressionValue::Object(Self::build_local_object_expression(&object)?)
        } else {
            let name = Self::required(cst.objectNameOrBindingVariable(), "graph name or binding variable")?;
            GraphExpressionValue::NameOrVariable(Self::identifier_text(&name.get_text()))
        };

        Ok(GraphExpression { value, span: Self::extract_span(cst) })
    }

    fn build_local_binding_table_expression(cst: &BindingTableExpressionContextAll) -> GqlResult<BindingTableExpression> {
        let value = if let Some(reference) = cst.bindingTableReference() {
            if let Some(parameter) = reference.referenceParameterSpecification() {
                BindingTableExpressionValue::Parameter(parameter.get_text())
            } else if let Some(name) = reference.delimitedBindingTableName() {
                BindingTableExpressionValue::DelimitedName(Self::identifier_text(&name.get_text()))
            } else {
                let parent = Self::required(reference.catalogObjectParentReference(), "catalog table parent")?;
                let name = Self::required(reference.bindingTableName(), "catalog table name")?;
                BindingTableExpressionValue::Catalog(Self::build_catalog_object_reference(&parent, &name.get_text(), Self::extract_span(&*reference))?)
            }
        } else if let Some(nested) = cst.nestedBindingTableQuerySpecification() {
            let nested = Self::required(nested.nestedQuerySpecification(), "nested table query")?;
            BindingTableExpressionValue::Query(Box::new(Self::build_query_from_nested_query_spec(&nested)?))
        } else if let Some(object) = cst.objectExpressionPrimary() {
            BindingTableExpressionValue::Object(Self::build_local_object_expression(&object)?)
        } else {
            let name = Self::required(cst.objectNameOrBindingVariable(), "table name or binding variable")?;
            BindingTableExpressionValue::NameOrVariable(Self::identifier_text(&name.get_text()))
        };

        Ok(BindingTableExpression { value, span: Self::extract_span(cst) })
    }

    fn build_local_object_expression(cst: &ObjectExpressionPrimaryContextAll) -> GqlResult<ObjectExpression> {
        let expression = Self::build_object_expression_primary_from_context(cst)?;
        Ok(if cst.VARIABLE().is_some() {
            ObjectExpression::Variable(expression)
        } else if cst.parenthesizedValueExpression().is_some() {
            ObjectExpression::Parenthesized(expression)
        } else {
            ObjectExpression::Primary(expression)
        })
    }

    fn build_catalog_object_reference(
        cst: &CatalogObjectParentReferenceContextAll, name: &str, span: Span,
    ) -> GqlResult<CatalogObjectReference> {
        Ok(CatalogObjectReference {
            schema: cst.schemaReference().map(|schema| Self::build_schema_reference(&schema)).transpose()?,
            parent: cst.objectName_all().iter().map(|name| Self::identifier_text(&name.get_text())).collect(),
            name: Self::identifier_text(name),
            span,
        })
    }

    fn build_schema_reference(cst: &SchemaReferenceContextAll) -> GqlResult<SchemaReference> {
        if let Some(absolute) = cst.absoluteCatalogSchemaReference() {
            let directory = absolute.absoluteDirectoryPath().and_then(|path| path.simpleDirectoryPath());
            let mut path = Self::build_schema_directory(directory.as_deref());
            if let Some(name) = absolute.schemaName() {
                path.push(Self::identifier_text(&name.get_text()));
            }
            return Ok(SchemaReference::Absolute { path });
        }

        if let Some(relative) = cst.relativeCatalogSchemaReference() {
            if let Some(predefined) = relative.predefinedSchemaReference() {
                return Ok(if predefined.HOME_SCHEMA().is_some() {
                    SchemaReference::Home
                } else if predefined.CURRENT_SCHEMA().is_some() {
                    SchemaReference::Current
                } else {
                    SchemaReference::CurrentDirectory
                });
            }
            let directory = Self::required(relative.relativeDirectoryPath(), "relative schema directory")?;
            let name = Self::required(relative.schemaName(), "relative schema name")?;
            let mut path = Self::build_schema_directory(directory.simpleDirectoryPath().as_deref());
            path.push(Self::identifier_text(&name.get_text()));
            let parent_levels = (0..).take_while(|&index| directory.DOUBLE_PERIOD(index).is_some()).count();
            return Ok(SchemaReference::Relative { parent_levels, path });
        }

        let parameter = Self::required(cst.referenceParameterSpecification(), "schema parameter")?;
        Ok(SchemaReference::Parameter(parameter.get_text()))
    }

    fn build_schema_directory(cst: Option<&SimpleDirectoryPathContextAll>) -> Vec<String> {
        cst.into_iter().flat_map(|path| path.directoryName_all()).map(|name| {
            Self::identifier_text(&name.get_text())
        }).collect()
    }

    pub(super) fn required<T>(value: Option<T>, component: &str) -> GqlResult<T> {
        value.ok_or_else(|| Self::parse_err(format!("CST missing {component}")))
    }
}
