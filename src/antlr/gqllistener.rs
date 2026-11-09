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

#![allow(nonstandard_style)]
// Generated from GQL.g4 by ANTLR 4.8
use super::gqlparser::*;
use antlr_rust::tree::ParseTreeListener;

pub trait GQLListener<'input>: ParseTreeListener<'input, GQLParserContextType> {
    /**
     * Enter a parse tree produced by {@link GQLParser#gqlProgram}.
     * @param ctx the parse tree
     */
    fn enter_gqlProgram(&mut self, _ctx: &GqlProgramContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#gqlProgram}.
     * @param ctx the parse tree
     */
    fn exit_gqlProgram(&mut self, _ctx: &GqlProgramContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#programActivity}.
     * @param ctx the parse tree
     */
    fn enter_programActivity(&mut self, _ctx: &ProgramActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#programActivity}.
     * @param ctx the parse tree
     */
    fn exit_programActivity(&mut self, _ctx: &ProgramActivityContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionActivity}.
     * @param ctx the parse tree
     */
    fn enter_sessionActivity(&mut self, _ctx: &SessionActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionActivity}.
     * @param ctx the parse tree
     */
    fn exit_sessionActivity(&mut self, _ctx: &SessionActivityContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#transactionActivity}.
     * @param ctx the parse tree
     */
    fn enter_transactionActivity(&mut self, _ctx: &TransactionActivityContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#transactionActivity}.
     * @param ctx the parse tree
     */
    fn exit_transactionActivity(&mut self, _ctx: &TransactionActivityContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endTransactionCommand}.
     * @param ctx the parse tree
     */
    fn enter_endTransactionCommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endTransactionCommand}.
     * @param ctx the parse tree
     */
    fn exit_endTransactionCommand(&mut self, _ctx: &EndTransactionCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetCommand}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetCommand(&mut self, _ctx: &SessionSetCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetCommand}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetCommand(&mut self, _ctx: &SessionSetCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetSchemaClause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetSchemaClause(&mut self, _ctx: &SessionSetSchemaClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetGraphClause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetGraphClause(&mut self, _ctx: &SessionSetGraphClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetTimeZoneClause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetTimeZoneClause(&mut self, _ctx: &SessionSetTimeZoneClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setTimeZoneValue}.
     * @param ctx the parse tree
     */
    fn enter_setTimeZoneValue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setTimeZoneValue}.
     * @param ctx the parse tree
     */
    fn exit_setTimeZoneValue(&mut self, _ctx: &SetTimeZoneValueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetParameterClause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetParameterClause(&mut self, _ctx: &SessionSetParameterClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetGraphParameterClause(
        &mut self,
        _ctx: &SessionSetGraphParameterClauseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetGraphParameterClause(
        &mut self,
        _ctx: &SessionSetGraphParameterClauseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetBindingTableParameterClause(
        &mut self,
        _ctx: &SessionSetBindingTableParameterClauseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetBindingTableParameterClause(
        &mut self,
        _ctx: &SessionSetBindingTableParameterClauseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetValueParameterClause(
        &mut self,
        _ctx: &SessionSetValueParameterClauseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetValueParameterClause(
        &mut self,
        _ctx: &SessionSetValueParameterClauseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionSetParameterName}.
     * @param ctx the parse tree
     */
    fn enter_sessionSetParameterName(&mut self, _ctx: &SessionSetParameterNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionSetParameterName}.
     * @param ctx the parse tree
     */
    fn exit_sessionSetParameterName(&mut self, _ctx: &SessionSetParameterNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionResetCommand}.
     * @param ctx the parse tree
     */
    fn enter_sessionResetCommand(&mut self, _ctx: &SessionResetCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionResetCommand}.
     * @param ctx the parse tree
     */
    fn exit_sessionResetCommand(&mut self, _ctx: &SessionResetCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionResetArguments}.
     * @param ctx the parse tree
     */
    fn enter_sessionResetArguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionResetArguments}.
     * @param ctx the parse tree
     */
    fn exit_sessionResetArguments(&mut self, _ctx: &SessionResetArgumentsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionCloseCommand}.
     * @param ctx the parse tree
     */
    fn enter_sessionCloseCommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionCloseCommand}.
     * @param ctx the parse tree
     */
    fn exit_sessionCloseCommand(&mut self, _ctx: &SessionCloseCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sessionParameterSpecification}.
     * @param ctx the parse tree
     */
    fn enter_sessionParameterSpecification(
        &mut self,
        _ctx: &SessionParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sessionParameterSpecification}.
     * @param ctx the parse tree
     */
    fn exit_sessionParameterSpecification(
        &mut self,
        _ctx: &SessionParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#startTransactionCommand}.
     * @param ctx the parse tree
     */
    fn enter_startTransactionCommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#startTransactionCommand}.
     * @param ctx the parse tree
     */
    fn exit_startTransactionCommand(&mut self, _ctx: &StartTransactionCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#transactionCharacteristics}.
     * @param ctx the parse tree
     */
    fn enter_transactionCharacteristics(
        &mut self,
        _ctx: &TransactionCharacteristicsContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#transactionCharacteristics}.
     * @param ctx the parse tree
     */
    fn exit_transactionCharacteristics(
        &mut self,
        _ctx: &TransactionCharacteristicsContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#transactionMode}.
     * @param ctx the parse tree
     */
    fn enter_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#transactionMode}.
     * @param ctx the parse tree
     */
    fn exit_transactionMode(&mut self, _ctx: &TransactionModeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#transactionAccessMode}.
     * @param ctx the parse tree
     */
    fn enter_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#transactionAccessMode}.
     * @param ctx the parse tree
     */
    fn exit_transactionAccessMode(&mut self, _ctx: &TransactionAccessModeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#rollbackCommand}.
     * @param ctx the parse tree
     */
    fn enter_rollbackCommand(&mut self, _ctx: &RollbackCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#rollbackCommand}.
     * @param ctx the parse tree
     */
    fn exit_rollbackCommand(&mut self, _ctx: &RollbackCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#commitCommand}.
     * @param ctx the parse tree
     */
    fn enter_commitCommand(&mut self, _ctx: &CommitCommandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#commitCommand}.
     * @param ctx the parse tree
     */
    fn exit_commitCommand(&mut self, _ctx: &CommitCommandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn enter_nestedProcedureSpecification(
        &mut self,
        _ctx: &NestedProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn exit_nestedProcedureSpecification(
        &mut self,
        _ctx: &NestedProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureSpecification}.
     * @param ctx the parse tree
     */
    fn enter_procedureSpecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureSpecification}.
     * @param ctx the parse tree
     */
    fn exit_procedureSpecification(&mut self, _ctx: &ProcedureSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn enter_nestedDataModifyingProcedureSpecification(
        &mut self,
        _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn exit_nestedDataModifyingProcedureSpecification(
        &mut self,
        _ctx: &NestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn enter_nestedQuerySpecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn exit_nestedQuerySpecification(&mut self, _ctx: &NestedQuerySpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureBody}.
     * @param ctx the parse tree
     */
    fn enter_procedureBody(&mut self, _ctx: &ProcedureBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureBody}.
     * @param ctx the parse tree
     */
    fn exit_procedureBody(&mut self, _ctx: &ProcedureBodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
     * @param ctx the parse tree
     */
    fn enter_bindingVariableDefinitionBlock(
        &mut self,
        _ctx: &BindingVariableDefinitionBlockContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
     * @param ctx the parse tree
     */
    fn exit_bindingVariableDefinitionBlock(
        &mut self,
        _ctx: &BindingVariableDefinitionBlockContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
     * @param ctx the parse tree
     */
    fn enter_bindingVariableDefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
     * @param ctx the parse tree
     */
    fn exit_bindingVariableDefinition(&mut self, _ctx: &BindingVariableDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#statementBlock}.
     * @param ctx the parse tree
     */
    fn enter_statementBlock(&mut self, _ctx: &StatementBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#statementBlock}.
     * @param ctx the parse tree
     */
    fn exit_statementBlock(&mut self, _ctx: &StatementBlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#statement}.
     * @param ctx the parse tree
     */
    fn enter_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#statement}.
     * @param ctx the parse tree
     */
    fn exit_statement(&mut self, _ctx: &StatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nextStatement}.
     * @param ctx the parse tree
     */
    fn enter_nextStatement(&mut self, _ctx: &NextStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nextStatement}.
     * @param ctx the parse tree
     */
    fn exit_nextStatement(&mut self, _ctx: &NextStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphVariableDefinition}.
     * @param ctx the parse tree
     */
    fn enter_graphVariableDefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphVariableDefinition}.
     * @param ctx the parse tree
     */
    fn exit_graphVariableDefinition(&mut self, _ctx: &GraphVariableDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
     * @param ctx the parse tree
     */
    fn enter_optTypedGraphInitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
     * @param ctx the parse tree
     */
    fn exit_optTypedGraphInitializer(&mut self, _ctx: &OptTypedGraphInitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphInitializer}.
     * @param ctx the parse tree
     */
    fn enter_graphInitializer(&mut self, _ctx: &GraphInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphInitializer}.
     * @param ctx the parse tree
     */
    fn exit_graphInitializer(&mut self, _ctx: &GraphInitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableVariableDefinition(
        &mut self,
        _ctx: &BindingTableVariableDefinitionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableVariableDefinition(
        &mut self,
        _ctx: &BindingTableVariableDefinitionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn enter_optTypedBindingTableInitializer(
        &mut self,
        _ctx: &OptTypedBindingTableInitializerContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn exit_optTypedBindingTableInitializer(
        &mut self,
        _ctx: &OptTypedBindingTableInitializerContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableInitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableInitializer(&mut self, _ctx: &BindingTableInitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueVariableDefinition}.
     * @param ctx the parse tree
     */
    fn enter_valueVariableDefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueVariableDefinition}.
     * @param ctx the parse tree
     */
    fn exit_valueVariableDefinition(&mut self, _ctx: &ValueVariableDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
     * @param ctx the parse tree
     */
    fn enter_optTypedValueInitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
     * @param ctx the parse tree
     */
    fn exit_optTypedValueInitializer(&mut self, _ctx: &OptTypedValueInitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueInitializer}.
     * @param ctx the parse tree
     */
    fn enter_valueInitializer(&mut self, _ctx: &ValueInitializerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueInitializer}.
     * @param ctx the parse tree
     */
    fn exit_valueInitializer(&mut self, _ctx: &ValueInitializerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphExpression}.
     * @param ctx the parse tree
     */
    fn enter_graphExpression(&mut self, _ctx: &GraphExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphExpression}.
     * @param ctx the parse tree
     */
    fn exit_graphExpression(&mut self, _ctx: &GraphExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#currentGraph}.
     * @param ctx the parse tree
     */
    fn enter_currentGraph(&mut self, _ctx: &CurrentGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#currentGraph}.
     * @param ctx the parse tree
     */
    fn exit_currentGraph(&mut self, _ctx: &CurrentGraphContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableExpression}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableExpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableExpression}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableExpression(&mut self, _ctx: &BindingTableExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
     * @param ctx the parse tree
     */
    fn enter_nestedBindingTableQuerySpecification(
        &mut self,
        _ctx: &NestedBindingTableQuerySpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
     * @param ctx the parse tree
     */
    fn exit_nestedBindingTableQuerySpecification(
        &mut self,
        _ctx: &NestedBindingTableQuerySpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn enter_objectExpressionPrimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn exit_objectExpressionPrimary(&mut self, _ctx: &ObjectExpressionPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_linearCatalogModifyingStatement(
        &mut self,
        _ctx: &LinearCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_linearCatalogModifyingStatement(
        &mut self,
        _ctx: &LinearCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleCatalogModifyingStatement(
        &mut self,
        _ctx: &SimpleCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleCatalogModifyingStatement(
        &mut self,
        _ctx: &SimpleCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_primitiveCatalogModifyingStatement(
        &mut self,
        _ctx: &PrimitiveCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_primitiveCatalogModifyingStatement(
        &mut self,
        _ctx: &PrimitiveCatalogModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#createSchemaStatement}.
     * @param ctx the parse tree
     */
    fn enter_createSchemaStatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#createSchemaStatement}.
     * @param ctx the parse tree
     */
    fn exit_createSchemaStatement(&mut self, _ctx: &CreateSchemaStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dropSchemaStatement}.
     * @param ctx the parse tree
     */
    fn enter_dropSchemaStatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dropSchemaStatement}.
     * @param ctx the parse tree
     */
    fn exit_dropSchemaStatement(&mut self, _ctx: &DropSchemaStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#createGraphStatement}.
     * @param ctx the parse tree
     */
    fn enter_createGraphStatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#createGraphStatement}.
     * @param ctx the parse tree
     */
    fn exit_createGraphStatement(&mut self, _ctx: &CreateGraphStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#openGraphType}.
     * @param ctx the parse tree
     */
    fn enter_openGraphType(&mut self, _ctx: &OpenGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#openGraphType}.
     * @param ctx the parse tree
     */
    fn exit_openGraphType(&mut self, _ctx: &OpenGraphTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#ofGraphType}.
     * @param ctx the parse tree
     */
    fn enter_ofGraphType(&mut self, _ctx: &OfGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#ofGraphType}.
     * @param ctx the parse tree
     */
    fn exit_ofGraphType(&mut self, _ctx: &OfGraphTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
     * @param ctx the parse tree
     */
    fn enter_graphTypeLikeGraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
     * @param ctx the parse tree
     */
    fn exit_graphTypeLikeGraph(&mut self, _ctx: &GraphTypeLikeGraphContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphSource}.
     * @param ctx the parse tree
     */
    fn enter_graphSource(&mut self, _ctx: &GraphSourceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphSource}.
     * @param ctx the parse tree
     */
    fn exit_graphSource(&mut self, _ctx: &GraphSourceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dropGraphStatement}.
     * @param ctx the parse tree
     */
    fn enter_dropGraphStatement(&mut self, _ctx: &DropGraphStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dropGraphStatement}.
     * @param ctx the parse tree
     */
    fn exit_dropGraphStatement(&mut self, _ctx: &DropGraphStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn enter_createGraphTypeStatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn exit_createGraphTypeStatement(&mut self, _ctx: &CreateGraphTypeStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphTypeSource}.
     * @param ctx the parse tree
     */
    fn enter_graphTypeSource(&mut self, _ctx: &GraphTypeSourceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphTypeSource}.
     * @param ctx the parse tree
     */
    fn exit_graphTypeSource(&mut self, _ctx: &GraphTypeSourceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#copyOfGraphType}.
     * @param ctx the parse tree
     */
    fn enter_copyOfGraphType(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#copyOfGraphType}.
     * @param ctx the parse tree
     */
    fn exit_copyOfGraphType(&mut self, _ctx: &CopyOfGraphTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn enter_dropGraphTypeStatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn exit_dropGraphTypeStatement(&mut self, _ctx: &DropGraphTypeStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn enter_callCatalogModifyingProcedureStatement(
        &mut self,
        _ctx: &CallCatalogModifyingProcedureStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn exit_callCatalogModifyingProcedureStatement(
        &mut self,
        _ctx: &CallCatalogModifyingProcedureStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_linearDataModifyingStatement(
        &mut self,
        _ctx: &LinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_linearDataModifyingStatement(
        &mut self,
        _ctx: &LinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_focusedLinearDataModifyingStatement(
        &mut self,
        _ctx: &FocusedLinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_focusedLinearDataModifyingStatement(
        &mut self,
        _ctx: &FocusedLinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn enter_focusedLinearDataModifyingStatementBody(
        &mut self,
        _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn exit_focusedLinearDataModifyingStatementBody(
        &mut self,
        _ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn enter_focusedNestedDataModifyingProcedureSpecification(
        &mut self,
        _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn exit_focusedNestedDataModifyingProcedureSpecification(
        &mut self,
        _ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_ambientLinearDataModifyingStatement(
        &mut self,
        _ctx: &AmbientLinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_ambientLinearDataModifyingStatement(
        &mut self,
        _ctx: &AmbientLinearDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn enter_ambientLinearDataModifyingStatementBody(
        &mut self,
        _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn exit_ambientLinearDataModifyingStatementBody(
        &mut self,
        _ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleLinearDataAccessingStatement(
        &mut self,
        _ctx: &SimpleLinearDataAccessingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleLinearDataAccessingStatement(
        &mut self,
        _ctx: &SimpleLinearDataAccessingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleDataAccessingStatement(
        &mut self,
        _ctx: &SimpleDataAccessingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleDataAccessingStatement(
        &mut self,
        _ctx: &SimpleDataAccessingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleDataModifyingStatement(
        &mut self,
        _ctx: &SimpleDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleDataModifyingStatement(
        &mut self,
        _ctx: &SimpleDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn enter_primitiveDataModifyingStatement(
        &mut self,
        _ctx: &PrimitiveDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn exit_primitiveDataModifyingStatement(
        &mut self,
        _ctx: &PrimitiveDataModifyingStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#insertStatement}.
     * @param ctx the parse tree
     */
    fn enter_insertStatement(&mut self, _ctx: &InsertStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertStatement}.
     * @param ctx the parse tree
     */
    fn exit_insertStatement(&mut self, _ctx: &InsertStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setStatement}.
     * @param ctx the parse tree
     */
    fn enter_setStatement(&mut self, _ctx: &SetStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setStatement}.
     * @param ctx the parse tree
     */
    fn exit_setStatement(&mut self, _ctx: &SetStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setItemList}.
     * @param ctx the parse tree
     */
    fn enter_setItemList(&mut self, _ctx: &SetItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setItemList}.
     * @param ctx the parse tree
     */
    fn exit_setItemList(&mut self, _ctx: &SetItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setItem}.
     * @param ctx the parse tree
     */
    fn enter_setItem(&mut self, _ctx: &SetItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setItem}.
     * @param ctx the parse tree
     */
    fn exit_setItem(&mut self, _ctx: &SetItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setPropertyItem}.
     * @param ctx the parse tree
     */
    fn enter_setPropertyItem(&mut self, _ctx: &SetPropertyItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setPropertyItem}.
     * @param ctx the parse tree
     */
    fn exit_setPropertyItem(&mut self, _ctx: &SetPropertyItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
     * @param ctx the parse tree
     */
    fn enter_setAllPropertiesItem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
     * @param ctx the parse tree
     */
    fn exit_setAllPropertiesItem(&mut self, _ctx: &SetAllPropertiesItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setLabelItem}.
     * @param ctx the parse tree
     */
    fn enter_setLabelItem(&mut self, _ctx: &SetLabelItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setLabelItem}.
     * @param ctx the parse tree
     */
    fn exit_setLabelItem(&mut self, _ctx: &SetLabelItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#removeStatement}.
     * @param ctx the parse tree
     */
    fn enter_removeStatement(&mut self, _ctx: &RemoveStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#removeStatement}.
     * @param ctx the parse tree
     */
    fn exit_removeStatement(&mut self, _ctx: &RemoveStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#removeItemList}.
     * @param ctx the parse tree
     */
    fn enter_removeItemList(&mut self, _ctx: &RemoveItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#removeItemList}.
     * @param ctx the parse tree
     */
    fn exit_removeItemList(&mut self, _ctx: &RemoveItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#removeItem}.
     * @param ctx the parse tree
     */
    fn enter_removeItem(&mut self, _ctx: &RemoveItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#removeItem}.
     * @param ctx the parse tree
     */
    fn exit_removeItem(&mut self, _ctx: &RemoveItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#removePropertyItem}.
     * @param ctx the parse tree
     */
    fn enter_removePropertyItem(&mut self, _ctx: &RemovePropertyItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#removePropertyItem}.
     * @param ctx the parse tree
     */
    fn exit_removePropertyItem(&mut self, _ctx: &RemovePropertyItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#removeLabelItem}.
     * @param ctx the parse tree
     */
    fn enter_removeLabelItem(&mut self, _ctx: &RemoveLabelItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#removeLabelItem}.
     * @param ctx the parse tree
     */
    fn exit_removeLabelItem(&mut self, _ctx: &RemoveLabelItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#deleteStatement}.
     * @param ctx the parse tree
     */
    fn enter_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#deleteStatement}.
     * @param ctx the parse tree
     */
    fn exit_deleteStatement(&mut self, _ctx: &DeleteStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#deleteItemList}.
     * @param ctx the parse tree
     */
    fn enter_deleteItemList(&mut self, _ctx: &DeleteItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#deleteItemList}.
     * @param ctx the parse tree
     */
    fn exit_deleteItemList(&mut self, _ctx: &DeleteItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#deleteItem}.
     * @param ctx the parse tree
     */
    fn enter_deleteItem(&mut self, _ctx: &DeleteItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#deleteItem}.
     * @param ctx the parse tree
     */
    fn exit_deleteItem(&mut self, _ctx: &DeleteItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn enter_callDataModifyingProcedureStatement(
        &mut self,
        _ctx: &CallDataModifyingProcedureStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn exit_callDataModifyingProcedureStatement(
        &mut self,
        _ctx: &CallDataModifyingProcedureStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#compositeQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_compositeQueryStatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#compositeQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_compositeQueryStatement(&mut self, _ctx: &CompositeQueryStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#compositeQueryExpression}.
     * @param ctx the parse tree
     */
    fn enter_compositeQueryExpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#compositeQueryExpression}.
     * @param ctx the parse tree
     */
    fn exit_compositeQueryExpression(&mut self, _ctx: &CompositeQueryExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#queryConjunction}.
     * @param ctx the parse tree
     */
    fn enter_queryConjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#queryConjunction}.
     * @param ctx the parse tree
     */
    fn exit_queryConjunction(&mut self, _ctx: &QueryConjunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setOperator}.
     * @param ctx the parse tree
     */
    fn enter_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setOperator}.
     * @param ctx the parse tree
     */
    fn exit_setOperator(&mut self, _ctx: &SetOperatorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
     * @param ctx the parse tree
     */
    fn enter_compositeQueryPrimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
     * @param ctx the parse tree
     */
    fn exit_compositeQueryPrimary(&mut self, _ctx: &CompositeQueryPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#linearQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_linearQueryStatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#linearQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_linearQueryStatement(&mut self, _ctx: &LinearQueryStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_focusedLinearQueryStatement(
        &mut self,
        _ctx: &FocusedLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_focusedLinearQueryStatement(
        &mut self,
        _ctx: &FocusedLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
     * @param ctx the parse tree
     */
    fn enter_focusedLinearQueryStatementPart(
        &mut self,
        _ctx: &FocusedLinearQueryStatementPartContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
     * @param ctx the parse tree
     */
    fn exit_focusedLinearQueryStatementPart(
        &mut self,
        _ctx: &FocusedLinearQueryStatementPartContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
     * @param ctx the parse tree
     */
    fn enter_focusedLinearQueryAndPrimitiveResultStatementPart(
        &mut self,
        _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
     * @param ctx the parse tree
     */
    fn exit_focusedLinearQueryAndPrimitiveResultStatementPart(
        &mut self,
        _ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn enter_focusedPrimitiveResultStatement(
        &mut self,
        _ctx: &FocusedPrimitiveResultStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn exit_focusedPrimitiveResultStatement(
        &mut self,
        _ctx: &FocusedPrimitiveResultStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn enter_focusedNestedQuerySpecification(
        &mut self,
        _ctx: &FocusedNestedQuerySpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn exit_focusedNestedQuerySpecification(
        &mut self,
        _ctx: &FocusedNestedQuerySpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_ambientLinearQueryStatement(
        &mut self,
        _ctx: &AmbientLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_ambientLinearQueryStatement(
        &mut self,
        _ctx: &AmbientLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleLinearQueryStatement(
        &mut self,
        _ctx: &SimpleLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleLinearQueryStatement(
        &mut self,
        _ctx: &SimpleLinearQueryStatementContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleQueryStatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleQueryStatement(&mut self, _ctx: &SimpleQueryStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_primitiveQueryStatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_primitiveQueryStatement(&mut self, _ctx: &PrimitiveQueryStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#matchStatement}.
     * @param ctx the parse tree
     */
    fn enter_matchStatement(&mut self, _ctx: &MatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#matchStatement}.
     * @param ctx the parse tree
     */
    fn exit_matchStatement(&mut self, _ctx: &MatchStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleMatchStatement}.
     * @param ctx the parse tree
     */
    fn enter_simpleMatchStatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleMatchStatement}.
     * @param ctx the parse tree
     */
    fn exit_simpleMatchStatement(&mut self, _ctx: &SimpleMatchStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#optionalMatchStatement}.
     * @param ctx the parse tree
     */
    fn enter_optionalMatchStatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#optionalMatchStatement}.
     * @param ctx the parse tree
     */
    fn exit_optionalMatchStatement(&mut self, _ctx: &OptionalMatchStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#optionalOperand}.
     * @param ctx the parse tree
     */
    fn enter_optionalOperand(&mut self, _ctx: &OptionalOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#optionalOperand}.
     * @param ctx the parse tree
     */
    fn exit_optionalOperand(&mut self, _ctx: &OptionalOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#matchStatementBlock}.
     * @param ctx the parse tree
     */
    fn enter_matchStatementBlock(&mut self, _ctx: &MatchStatementBlockContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#matchStatementBlock}.
     * @param ctx the parse tree
     */
    fn exit_matchStatementBlock(&mut self, _ctx: &MatchStatementBlockContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#callQueryStatement}.
     * @param ctx the parse tree
     */
    fn enter_callQueryStatement(&mut self, _ctx: &CallQueryStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#callQueryStatement}.
     * @param ctx the parse tree
     */
    fn exit_callQueryStatement(&mut self, _ctx: &CallQueryStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#filterStatement}.
     * @param ctx the parse tree
     */
    fn enter_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#filterStatement}.
     * @param ctx the parse tree
     */
    fn exit_filterStatement(&mut self, _ctx: &FilterStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#letStatement}.
     * @param ctx the parse tree
     */
    fn enter_letStatement(&mut self, _ctx: &LetStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#letStatement}.
     * @param ctx the parse tree
     */
    fn exit_letStatement(&mut self, _ctx: &LetStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
     * @param ctx the parse tree
     */
    fn enter_letVariableDefinitionList(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
     * @param ctx the parse tree
     */
    fn exit_letVariableDefinitionList(&mut self, _ctx: &LetVariableDefinitionListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#letVariableDefinition}.
     * @param ctx the parse tree
     */
    fn enter_letVariableDefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#letVariableDefinition}.
     * @param ctx the parse tree
     */
    fn exit_letVariableDefinition(&mut self, _ctx: &LetVariableDefinitionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#forStatement}.
     * @param ctx the parse tree
     */
    fn enter_forStatement(&mut self, _ctx: &ForStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#forStatement}.
     * @param ctx the parse tree
     */
    fn exit_forStatement(&mut self, _ctx: &ForStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#forItem}.
     * @param ctx the parse tree
     */
    fn enter_forItem(&mut self, _ctx: &ForItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#forItem}.
     * @param ctx the parse tree
     */
    fn exit_forItem(&mut self, _ctx: &ForItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#forItemAlias}.
     * @param ctx the parse tree
     */
    fn enter_forItemAlias(&mut self, _ctx: &ForItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#forItemAlias}.
     * @param ctx the parse tree
     */
    fn exit_forItemAlias(&mut self, _ctx: &ForItemAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#forItemSource}.
     * @param ctx the parse tree
     */
    fn enter_forItemSource(&mut self, _ctx: &ForItemSourceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#forItemSource}.
     * @param ctx the parse tree
     */
    fn exit_forItemSource(&mut self, _ctx: &ForItemSourceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
     * @param ctx the parse tree
     */
    fn enter_forOrdinalityOrOffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
     * @param ctx the parse tree
     */
    fn exit_forOrdinalityOrOffset(&mut self, _ctx: &ForOrdinalityOrOffsetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
     * @param ctx the parse tree
     */
    fn enter_orderByAndPageStatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
     * @param ctx the parse tree
     */
    fn exit_orderByAndPageStatement(&mut self, _ctx: &OrderByAndPageStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#primitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn enter_primitiveResultStatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#primitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn exit_primitiveResultStatement(&mut self, _ctx: &PrimitiveResultStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#returnStatement}.
     * @param ctx the parse tree
     */
    fn enter_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#returnStatement}.
     * @param ctx the parse tree
     */
    fn exit_returnStatement(&mut self, _ctx: &ReturnStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#returnStatementBody}.
     * @param ctx the parse tree
     */
    fn enter_returnStatementBody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#returnStatementBody}.
     * @param ctx the parse tree
     */
    fn exit_returnStatementBody(&mut self, _ctx: &ReturnStatementBodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#returnItemList}.
     * @param ctx the parse tree
     */
    fn enter_returnItemList(&mut self, _ctx: &ReturnItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#returnItemList}.
     * @param ctx the parse tree
     */
    fn exit_returnItemList(&mut self, _ctx: &ReturnItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#returnItem}.
     * @param ctx the parse tree
     */
    fn enter_returnItem(&mut self, _ctx: &ReturnItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#returnItem}.
     * @param ctx the parse tree
     */
    fn exit_returnItem(&mut self, _ctx: &ReturnItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#returnItemAlias}.
     * @param ctx the parse tree
     */
    fn enter_returnItemAlias(&mut self, _ctx: &ReturnItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#returnItemAlias}.
     * @param ctx the parse tree
     */
    fn exit_returnItemAlias(&mut self, _ctx: &ReturnItemAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectStatement}.
     * @param ctx the parse tree
     */
    fn enter_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectStatement}.
     * @param ctx the parse tree
     */
    fn exit_selectStatement(&mut self, _ctx: &SelectStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectItemList}.
     * @param ctx the parse tree
     */
    fn enter_selectItemList(&mut self, _ctx: &SelectItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectItemList}.
     * @param ctx the parse tree
     */
    fn exit_selectItemList(&mut self, _ctx: &SelectItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectItem}.
     * @param ctx the parse tree
     */
    fn enter_selectItem(&mut self, _ctx: &SelectItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectItem}.
     * @param ctx the parse tree
     */
    fn exit_selectItem(&mut self, _ctx: &SelectItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectItemAlias}.
     * @param ctx the parse tree
     */
    fn enter_selectItemAlias(&mut self, _ctx: &SelectItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectItemAlias}.
     * @param ctx the parse tree
     */
    fn exit_selectItemAlias(&mut self, _ctx: &SelectItemAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#havingClause}.
     * @param ctx the parse tree
     */
    fn enter_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#havingClause}.
     * @param ctx the parse tree
     */
    fn exit_havingClause(&mut self, _ctx: &HavingClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectStatementBody}.
     * @param ctx the parse tree
     */
    fn enter_selectStatementBody(&mut self, _ctx: &SelectStatementBodyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectStatementBody}.
     * @param ctx the parse tree
     */
    fn exit_selectStatementBody(&mut self, _ctx: &SelectStatementBodyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectGraphMatchList}.
     * @param ctx the parse tree
     */
    fn enter_selectGraphMatchList(&mut self, _ctx: &SelectGraphMatchListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectGraphMatchList}.
     * @param ctx the parse tree
     */
    fn exit_selectGraphMatchList(&mut self, _ctx: &SelectGraphMatchListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectGraphMatch}.
     * @param ctx the parse tree
     */
    fn enter_selectGraphMatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectGraphMatch}.
     * @param ctx the parse tree
     */
    fn exit_selectGraphMatch(&mut self, _ctx: &SelectGraphMatchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#selectQuerySpecification}.
     * @param ctx the parse tree
     */
    fn enter_selectQuerySpecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#selectQuerySpecification}.
     * @param ctx the parse tree
     */
    fn exit_selectQuerySpecification(&mut self, _ctx: &SelectQuerySpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#callProcedureStatement}.
     * @param ctx the parse tree
     */
    fn enter_callProcedureStatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#callProcedureStatement}.
     * @param ctx the parse tree
     */
    fn exit_callProcedureStatement(&mut self, _ctx: &CallProcedureStatementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureCall}.
     * @param ctx the parse tree
     */
    fn enter_procedureCall(&mut self, _ctx: &ProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureCall}.
     * @param ctx the parse tree
     */
    fn exit_procedureCall(&mut self, _ctx: &ProcedureCallContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#inlineProcedureCall}.
     * @param ctx the parse tree
     */
    fn enter_inlineProcedureCall(&mut self, _ctx: &InlineProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#inlineProcedureCall}.
     * @param ctx the parse tree
     */
    fn exit_inlineProcedureCall(&mut self, _ctx: &InlineProcedureCallContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#variableScopeClause}.
     * @param ctx the parse tree
     */
    fn enter_variableScopeClause(&mut self, _ctx: &VariableScopeClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#variableScopeClause}.
     * @param ctx the parse tree
     */
    fn exit_variableScopeClause(&mut self, _ctx: &VariableScopeClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
     * @param ctx the parse tree
     */
    fn enter_bindingVariableReferenceList(
        &mut self,
        _ctx: &BindingVariableReferenceListContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
     * @param ctx the parse tree
     */
    fn exit_bindingVariableReferenceList(
        &mut self,
        _ctx: &BindingVariableReferenceListContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#namedProcedureCall}.
     * @param ctx the parse tree
     */
    fn enter_namedProcedureCall(&mut self, _ctx: &NamedProcedureCallContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#namedProcedureCall}.
     * @param ctx the parse tree
     */
    fn exit_namedProcedureCall(&mut self, _ctx: &NamedProcedureCallContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureArgumentList}.
     * @param ctx the parse tree
     */
    fn enter_procedureArgumentList(&mut self, _ctx: &ProcedureArgumentListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureArgumentList}.
     * @param ctx the parse tree
     */
    fn exit_procedureArgumentList(&mut self, _ctx: &ProcedureArgumentListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureArgument}.
     * @param ctx the parse tree
     */
    fn enter_procedureArgument(&mut self, _ctx: &ProcedureArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureArgument}.
     * @param ctx the parse tree
     */
    fn exit_procedureArgument(&mut self, _ctx: &ProcedureArgumentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#atSchemaClause}.
     * @param ctx the parse tree
     */
    fn enter_atSchemaClause(&mut self, _ctx: &AtSchemaClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#atSchemaClause}.
     * @param ctx the parse tree
     */
    fn exit_atSchemaClause(&mut self, _ctx: &AtSchemaClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#useGraphClause}.
     * @param ctx the parse tree
     */
    fn enter_useGraphClause(&mut self, _ctx: &UseGraphClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#useGraphClause}.
     * @param ctx the parse tree
     */
    fn exit_useGraphClause(&mut self, _ctx: &UseGraphClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternBindingTable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternBindingTable(&mut self, _ctx: &GraphPatternBindingTableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternYieldClause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternYieldClause(&mut self, _ctx: &GraphPatternYieldClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternYieldItemList(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternYieldItemList(&mut self, _ctx: &GraphPatternYieldItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternYieldItem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternYieldItem(&mut self, _ctx: &GraphPatternYieldItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPattern}.
     * @param ctx the parse tree
     */
    fn enter_graphPattern(&mut self, _ctx: &GraphPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPattern}.
     * @param ctx the parse tree
     */
    fn exit_graphPattern(&mut self, _ctx: &GraphPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#matchMode}.
     * @param ctx the parse tree
     */
    fn enter_matchMode(&mut self, _ctx: &MatchModeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#matchMode}.
     * @param ctx the parse tree
     */
    fn exit_matchMode(&mut self, _ctx: &MatchModeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
     * @param ctx the parse tree
     */
    fn enter_repeatableElementsMatchMode(
        &mut self,
        _ctx: &RepeatableElementsMatchModeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
     * @param ctx the parse tree
     */
    fn exit_repeatableElementsMatchMode(
        &mut self,
        _ctx: &RepeatableElementsMatchModeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
     * @param ctx the parse tree
     */
    fn enter_differentEdgesMatchMode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
     * @param ctx the parse tree
     */
    fn exit_differentEdgesMatchMode(&mut self, _ctx: &DifferentEdgesMatchModeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
     * @param ctx the parse tree
     */
    fn enter_elementBindingsOrElements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
     * @param ctx the parse tree
     */
    fn exit_elementBindingsOrElements(&mut self, _ctx: &ElementBindingsOrElementsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
     * @param ctx the parse tree
     */
    fn enter_edgeBindingsOrEdges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
     * @param ctx the parse tree
     */
    fn exit_edgeBindingsOrEdges(&mut self, _ctx: &EdgeBindingsOrEdgesContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathPatternList}.
     * @param ctx the parse tree
     */
    fn enter_pathPatternList(&mut self, _ctx: &PathPatternListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathPatternList}.
     * @param ctx the parse tree
     */
    fn exit_pathPatternList(&mut self, _ctx: &PathPatternListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathPattern}.
     * @param ctx the parse tree
     */
    fn enter_pathPattern(&mut self, _ctx: &PathPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathPattern}.
     * @param ctx the parse tree
     */
    fn exit_pathPattern(&mut self, _ctx: &PathPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_pathVariableDeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_pathVariableDeclaration(&mut self, _ctx: &PathVariableDeclarationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#keepClause}.
     * @param ctx the parse tree
     */
    fn enter_keepClause(&mut self, _ctx: &KeepClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#keepClause}.
     * @param ctx the parse tree
     */
    fn exit_keepClause(&mut self, _ctx: &KeepClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternWhereClause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternWhereClause(&mut self, _ctx: &GraphPatternWhereClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertGraphPattern}.
     * @param ctx the parse tree
     */
    fn enter_insertGraphPattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertGraphPattern}.
     * @param ctx the parse tree
     */
    fn exit_insertGraphPattern(&mut self, _ctx: &InsertGraphPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertPathPatternList}.
     * @param ctx the parse tree
     */
    fn enter_insertPathPatternList(&mut self, _ctx: &InsertPathPatternListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertPathPatternList}.
     * @param ctx the parse tree
     */
    fn exit_insertPathPatternList(&mut self, _ctx: &InsertPathPatternListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertPathPattern}.
     * @param ctx the parse tree
     */
    fn enter_insertPathPattern(&mut self, _ctx: &InsertPathPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertPathPattern}.
     * @param ctx the parse tree
     */
    fn exit_insertPathPattern(&mut self, _ctx: &InsertPathPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertNodePattern}.
     * @param ctx the parse tree
     */
    fn enter_insertNodePattern(&mut self, _ctx: &InsertNodePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertNodePattern}.
     * @param ctx the parse tree
     */
    fn exit_insertNodePattern(&mut self, _ctx: &InsertNodePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertEdgePattern}.
     * @param ctx the parse tree
     */
    fn enter_insertEdgePattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertEdgePattern}.
     * @param ctx the parse tree
     */
    fn exit_insertEdgePattern(&mut self, _ctx: &InsertEdgePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn enter_insertEdgePointingLeft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn exit_insertEdgePointingLeft(&mut self, _ctx: &InsertEdgePointingLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn enter_insertEdgePointingRight(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn exit_insertEdgePointingRight(&mut self, _ctx: &InsertEdgePointingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn enter_insertEdgeUndirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn exit_insertEdgeUndirected(&mut self, _ctx: &InsertEdgeUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
     * @param ctx the parse tree
     */
    fn enter_insertElementPatternFiller(
        &mut self,
        _ctx: &InsertElementPatternFillerContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
     * @param ctx the parse tree
     */
    fn exit_insertElementPatternFiller(
        &mut self,
        _ctx: &InsertElementPatternFillerContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
     * @param ctx the parse tree
     */
    fn enter_labelAndPropertySetSpecification(
        &mut self,
        _ctx: &LabelAndPropertySetSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
     * @param ctx the parse tree
     */
    fn exit_labelAndPropertySetSpecification(
        &mut self,
        _ctx: &LabelAndPropertySetSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#pathPatternPrefix}.
     * @param ctx the parse tree
     */
    fn enter_pathPatternPrefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathPatternPrefix}.
     * @param ctx the parse tree
     */
    fn exit_pathPatternPrefix(&mut self, _ctx: &PathPatternPrefixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathModePrefix}.
     * @param ctx the parse tree
     */
    fn enter_pathModePrefix(&mut self, _ctx: &PathModePrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathModePrefix}.
     * @param ctx the parse tree
     */
    fn exit_pathModePrefix(&mut self, _ctx: &PathModePrefixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathMode}.
     * @param ctx the parse tree
     */
    fn enter_pathMode(&mut self, _ctx: &PathModeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathMode}.
     * @param ctx the parse tree
     */
    fn exit_pathMode(&mut self, _ctx: &PathModeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathSearchPrefix}.
     * @param ctx the parse tree
     */
    fn enter_pathSearchPrefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathSearchPrefix}.
     * @param ctx the parse tree
     */
    fn exit_pathSearchPrefix(&mut self, _ctx: &PathSearchPrefixContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#allPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_allPathSearch(&mut self, _ctx: &AllPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#allPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_allPathSearch(&mut self, _ctx: &AllPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathOrPaths}.
     * @param ctx the parse tree
     */
    fn enter_pathOrPaths(&mut self, _ctx: &PathOrPathsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathOrPaths}.
     * @param ctx the parse tree
     */
    fn exit_pathOrPaths(&mut self, _ctx: &PathOrPathsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#anyPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_anyPathSearch(&mut self, _ctx: &AnyPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#anyPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_anyPathSearch(&mut self, _ctx: &AnyPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numberOfPaths}.
     * @param ctx the parse tree
     */
    fn enter_numberOfPaths(&mut self, _ctx: &NumberOfPathsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#numberOfPaths}.
     * @param ctx the parse tree
     */
    fn exit_numberOfPaths(&mut self, _ctx: &NumberOfPathsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#shortestPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_shortestPathSearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#shortestPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_shortestPathSearch(&mut self, _ctx: &ShortestPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#allShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_allShortestPathSearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#allShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_allShortestPathSearch(&mut self, _ctx: &AllShortestPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_anyShortestPathSearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_anyShortestPathSearch(&mut self, _ctx: &AnyShortestPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn enter_countedShortestPathSearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn exit_countedShortestPathSearch(&mut self, _ctx: &CountedShortestPathSearchContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
     * @param ctx the parse tree
     */
    fn enter_countedShortestGroupSearch(
        &mut self,
        _ctx: &CountedShortestGroupSearchContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
     * @param ctx the parse tree
     */
    fn exit_countedShortestGroupSearch(
        &mut self,
        _ctx: &CountedShortestGroupSearchContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#numberOfGroups}.
     * @param ctx the parse tree
     */
    fn enter_numberOfGroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#numberOfGroups}.
     * @param ctx the parse tree
     */
    fn exit_numberOfGroups(&mut self, _ctx: &NumberOfGroupsContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ppePathTerm}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn enter_ppePathTerm(&mut self, _ctx: &PpePathTermContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ppePathTerm}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn exit_ppePathTerm(&mut self, _ctx: &PpePathTermContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ppeMultisetAlternation}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn enter_ppeMultisetAlternation(&mut self, _ctx: &PpeMultisetAlternationContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ppeMultisetAlternation}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn exit_ppeMultisetAlternation(&mut self, _ctx: &PpeMultisetAlternationContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ppePatternUnion}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn enter_ppePatternUnion(&mut self, _ctx: &PpePatternUnionContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ppePatternUnion}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn exit_ppePatternUnion(&mut self, _ctx: &PpePatternUnionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathTerm}.
     * @param ctx the parse tree
     */
    fn enter_pathTerm(&mut self, _ctx: &PathTermContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathTerm}.
     * @param ctx the parse tree
     */
    fn exit_pathTerm(&mut self, _ctx: &PathTermContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code pfPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn enter_pfPathPrimary(&mut self, _ctx: &PfPathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code pfPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn exit_pfPathPrimary(&mut self, _ctx: &PfPathPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code pfQuantifiedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn enter_pfQuantifiedPathPrimary(&mut self, _ctx: &PfQuantifiedPathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code pfQuantifiedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn exit_pfQuantifiedPathPrimary(&mut self, _ctx: &PfQuantifiedPathPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code pfQuestionedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn enter_pfQuestionedPathPrimary(&mut self, _ctx: &PfQuestionedPathPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code pfQuestionedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn exit_pfQuestionedPathPrimary(&mut self, _ctx: &PfQuestionedPathPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ppElementPattern}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn enter_ppElementPattern(&mut self, _ctx: &PpElementPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code ppElementPattern}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn exit_ppElementPattern(&mut self, _ctx: &PpElementPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code ppParenthesizedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn enter_ppParenthesizedPathPatternExpression(
        &mut self,
        _ctx: &PpParenthesizedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code ppParenthesizedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn exit_ppParenthesizedPathPatternExpression(
        &mut self,
        _ctx: &PpParenthesizedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code ppSimplifiedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn enter_ppSimplifiedPathPatternExpression(
        &mut self,
        _ctx: &PpSimplifiedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code ppSimplifiedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn exit_ppSimplifiedPathPatternExpression(
        &mut self,
        _ctx: &PpSimplifiedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#elementPattern}.
     * @param ctx the parse tree
     */
    fn enter_elementPattern(&mut self, _ctx: &ElementPatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementPattern}.
     * @param ctx the parse tree
     */
    fn exit_elementPattern(&mut self, _ctx: &ElementPatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodePattern}.
     * @param ctx the parse tree
     */
    fn enter_nodePattern(&mut self, _ctx: &NodePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodePattern}.
     * @param ctx the parse tree
     */
    fn exit_nodePattern(&mut self, _ctx: &NodePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementPatternFiller}.
     * @param ctx the parse tree
     */
    fn enter_elementPatternFiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementPatternFiller}.
     * @param ctx the parse tree
     */
    fn exit_elementPatternFiller(&mut self, _ctx: &ElementPatternFillerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_elementVariableDeclaration(
        &mut self,
        _ctx: &ElementVariableDeclarationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_elementVariableDeclaration(
        &mut self,
        _ctx: &ElementVariableDeclarationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#isLabelExpression}.
     * @param ctx the parse tree
     */
    fn enter_isLabelExpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#isLabelExpression}.
     * @param ctx the parse tree
     */
    fn exit_isLabelExpression(&mut self, _ctx: &IsLabelExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#isOrColon}.
     * @param ctx the parse tree
     */
    fn enter_isOrColon(&mut self, _ctx: &IsOrColonContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#isOrColon}.
     * @param ctx the parse tree
     */
    fn exit_isOrColon(&mut self, _ctx: &IsOrColonContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementPatternPredicate}.
     * @param ctx the parse tree
     */
    fn enter_elementPatternPredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementPatternPredicate}.
     * @param ctx the parse tree
     */
    fn exit_elementPatternPredicate(&mut self, _ctx: &ElementPatternPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn enter_elementPatternWhereClause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn exit_elementPatternWhereClause(&mut self, _ctx: &ElementPatternWhereClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementPropertySpecification}.
     * @param ctx the parse tree
     */
    fn enter_elementPropertySpecification(
        &mut self,
        _ctx: &ElementPropertySpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#elementPropertySpecification}.
     * @param ctx the parse tree
     */
    fn exit_elementPropertySpecification(
        &mut self,
        _ctx: &ElementPropertySpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
     * @param ctx the parse tree
     */
    fn enter_propertyKeyValuePairList(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
     * @param ctx the parse tree
     */
    fn exit_propertyKeyValuePairList(&mut self, _ctx: &PropertyKeyValuePairListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
     * @param ctx the parse tree
     */
    fn enter_propertyKeyValuePair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
     * @param ctx the parse tree
     */
    fn exit_propertyKeyValuePair(&mut self, _ctx: &PropertyKeyValuePairContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgePattern}.
     * @param ctx the parse tree
     */
    fn enter_edgePattern(&mut self, _ctx: &EdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgePattern}.
     * @param ctx the parse tree
     */
    fn exit_edgePattern(&mut self, _ctx: &EdgePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgePattern}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgePattern(&mut self, _ctx: &FullEdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgePattern}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgePattern(&mut self, _ctx: &FullEdgePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgePointingLeft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgePointingLeft(&mut self, _ctx: &FullEdgePointingLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgeUndirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgeUndirected(&mut self, _ctx: &FullEdgeUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgePointingRight(&mut self, _ctx: &FullEdgePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgePointingRight(&mut self, _ctx: &FullEdgePointingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgeLeftOrUndirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgeLeftOrUndirected(&mut self, _ctx: &FullEdgeLeftOrUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgeUndirectedOrRight(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgeUndirectedOrRight(&mut self, _ctx: &FullEdgeUndirectedOrRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgeLeftOrRight(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgeLeftOrRight(&mut self, _ctx: &FullEdgeLeftOrRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
     * @param ctx the parse tree
     */
    fn enter_fullEdgeAnyDirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
     * @param ctx the parse tree
     */
    fn exit_fullEdgeAnyDirection(&mut self, _ctx: &FullEdgeAnyDirectionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#abbreviatedEdgePattern}.
     * @param ctx the parse tree
     */
    fn enter_abbreviatedEdgePattern(&mut self, _ctx: &AbbreviatedEdgePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#abbreviatedEdgePattern}.
     * @param ctx the parse tree
     */
    fn exit_abbreviatedEdgePattern(&mut self, _ctx: &AbbreviatedEdgePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedPathPatternExpression(
        &mut self,
        _ctx: &ParenthesizedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedPathPatternExpression(
        &mut self,
        _ctx: &ParenthesizedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn enter_subpathVariableDeclaration(
        &mut self,
        _ctx: &SubpathVariableDeclarationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn exit_subpathVariableDeclaration(
        &mut self,
        _ctx: &SubpathVariableDeclarationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedPathPatternWhereClause(
        &mut self,
        _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedPathPatternWhereClause(
        &mut self,
        _ctx: &ParenthesizedPathPatternWhereClauseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code labelExpressionNegation}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionNegation(&mut self, _ctx: &LabelExpressionNegationContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code labelExpressionNegation}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionNegation(&mut self, _ctx: &LabelExpressionNegationContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code labelExpressionDisjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionDisjunction(
        &mut self,
        _ctx: &LabelExpressionDisjunctionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code labelExpressionDisjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionDisjunction(
        &mut self,
        _ctx: &LabelExpressionDisjunctionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code labelExpressionParenthesized}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionParenthesized(
        &mut self,
        _ctx: &LabelExpressionParenthesizedContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code labelExpressionParenthesized}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionParenthesized(
        &mut self,
        _ctx: &LabelExpressionParenthesizedContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code labelExpressionWildcard}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionWildcard(&mut self, _ctx: &LabelExpressionWildcardContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code labelExpressionWildcard}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionWildcard(&mut self, _ctx: &LabelExpressionWildcardContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code labelExpressionConjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionConjunction(
        &mut self,
        _ctx: &LabelExpressionConjunctionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code labelExpressionConjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionConjunction(
        &mut self,
        _ctx: &LabelExpressionConjunctionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code labelExpressionName}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn enter_labelExpressionName(&mut self, _ctx: &LabelExpressionNameContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code labelExpressionName}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn exit_labelExpressionName(&mut self, _ctx: &LabelExpressionNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathVariableReference}.
     * @param ctx the parse tree
     */
    fn enter_pathVariableReference(&mut self, _ctx: &PathVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathVariableReference}.
     * @param ctx the parse tree
     */
    fn exit_pathVariableReference(&mut self, _ctx: &PathVariableReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementVariableReference}.
     * @param ctx the parse tree
     */
    fn enter_elementVariableReference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementVariableReference}.
     * @param ctx the parse tree
     */
    fn exit_elementVariableReference(&mut self, _ctx: &ElementVariableReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
     * @param ctx the parse tree
     */
    fn enter_graphPatternQuantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
     * @param ctx the parse tree
     */
    fn exit_graphPatternQuantifier(&mut self, _ctx: &GraphPatternQuantifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fixedQuantifier}.
     * @param ctx the parse tree
     */
    fn enter_fixedQuantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fixedQuantifier}.
     * @param ctx the parse tree
     */
    fn exit_fixedQuantifier(&mut self, _ctx: &FixedQuantifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalQuantifier}.
     * @param ctx the parse tree
     */
    fn enter_generalQuantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalQuantifier}.
     * @param ctx the parse tree
     */
    fn exit_generalQuantifier(&mut self, _ctx: &GeneralQuantifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#lowerBound}.
     * @param ctx the parse tree
     */
    fn enter_lowerBound(&mut self, _ctx: &LowerBoundContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#lowerBound}.
     * @param ctx the parse tree
     */
    fn exit_lowerBound(&mut self, _ctx: &LowerBoundContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#upperBound}.
     * @param ctx the parse tree
     */
    fn enter_upperBound(&mut self, _ctx: &UpperBoundContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#upperBound}.
     * @param ctx the parse tree
     */
    fn exit_upperBound(&mut self, _ctx: &UpperBoundContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedPathPatternExpression(
        &mut self,
        _ctx: &SimplifiedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedPathPatternExpression(
        &mut self,
        _ctx: &SimplifiedPathPatternExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingLeft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingLeft(&mut self, _ctx: &SimplifiedDefaultingLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingUndirected(
        &mut self,
        _ctx: &SimplifiedDefaultingUndirectedContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingUndirected(
        &mut self,
        _ctx: &SimplifiedDefaultingUndirectedContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingRight(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingRight(&mut self, _ctx: &SimplifiedDefaultingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingLeftOrUndirected(
        &mut self,
        _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingLeftOrUndirected(
        &mut self,
        _ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingUndirectedOrRight(
        &mut self,
        _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingUndirectedOrRight(
        &mut self,
        _ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingLeftOrRight(
        &mut self,
        _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingLeftOrRight(
        &mut self,
        _ctx: &SimplifiedDefaultingLeftOrRightContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDefaultingAnyDirection(
        &mut self,
        _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDefaultingAnyDirection(
        &mut self,
        _ctx: &SimplifiedDefaultingAnyDirectionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedContents}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedContents(&mut self, _ctx: &SimplifiedContentsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedContents}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedContents(&mut self, _ctx: &SimplifiedContentsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedPathUnion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedPathUnion(&mut self, _ctx: &SimplifiedPathUnionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedMultisetAlternation(
        &mut self,
        _ctx: &SimplifiedMultisetAlternationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedMultisetAlternation(
        &mut self,
        _ctx: &SimplifiedMultisetAlternationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code simplifiedFactorLowLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedFactorLowLabel(&mut self, _ctx: &SimplifiedFactorLowLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code simplifiedFactorLowLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedFactorLowLabel(&mut self, _ctx: &SimplifiedFactorLowLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code simplifiedConcatenationLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedConcatenationLabel(
        &mut self,
        _ctx: &SimplifiedConcatenationLabelContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code simplifiedConcatenationLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedConcatenationLabel(
        &mut self,
        _ctx: &SimplifiedConcatenationLabelContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code simplifiedConjunctionLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedConjunctionLabel(
        &mut self,
        _ctx: &SimplifiedConjunctionLabelContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code simplifiedConjunctionLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedConjunctionLabel(
        &mut self,
        _ctx: &SimplifiedConjunctionLabelContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code simplifiedFactorHighLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedFactorHighLabel(&mut self, _ctx: &SimplifiedFactorHighLabelContext<'input>) {
    }
    /**
     * Exit a parse tree produced by the {@code simplifiedFactorHighLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedFactorHighLabel(&mut self, _ctx: &SimplifiedFactorHighLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedFactorHigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedFactorHigh(&mut self, _ctx: &SimplifiedFactorHighContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedQuantified}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedQuantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedQuantified}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedQuantified(&mut self, _ctx: &SimplifiedQuantifiedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedQuestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedQuestioned(&mut self, _ctx: &SimplifiedQuestionedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedTertiary}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedTertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedTertiary}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedTertiary(&mut self, _ctx: &SimplifiedTertiaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedDirectionOverride(
        &mut self,
        _ctx: &SimplifiedDirectionOverrideContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedDirectionOverride(
        &mut self,
        _ctx: &SimplifiedDirectionOverrideContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideLeft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideLeft(&mut self, _ctx: &SimplifiedOverrideLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideUndirected(
        &mut self,
        _ctx: &SimplifiedOverrideUndirectedContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideUndirected(
        &mut self,
        _ctx: &SimplifiedOverrideUndirectedContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideRight(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideRight(&mut self, _ctx: &SimplifiedOverrideRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideLeftOrUndirected(
        &mut self,
        _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideLeftOrUndirected(
        &mut self,
        _ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideUndirectedOrRight(
        &mut self,
        _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideUndirectedOrRight(
        &mut self,
        _ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideLeftOrRight(
        &mut self,
        _ctx: &SimplifiedOverrideLeftOrRightContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideLeftOrRight(
        &mut self,
        _ctx: &SimplifiedOverrideLeftOrRightContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedOverrideAnyDirection(
        &mut self,
        _ctx: &SimplifiedOverrideAnyDirectionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedOverrideAnyDirection(
        &mut self,
        _ctx: &SimplifiedOverrideAnyDirectionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedSecondary}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedSecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedSecondary}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedSecondary(&mut self, _ctx: &SimplifiedSecondaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedNegation}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedNegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedNegation}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedNegation(&mut self, _ctx: &SimplifiedNegationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simplifiedPrimary}.
     * @param ctx the parse tree
     */
    fn enter_simplifiedPrimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simplifiedPrimary}.
     * @param ctx the parse tree
     */
    fn exit_simplifiedPrimary(&mut self, _ctx: &SimplifiedPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#whereClause}.
     * @param ctx the parse tree
     */
    fn enter_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#whereClause}.
     * @param ctx the parse tree
     */
    fn exit_whereClause(&mut self, _ctx: &WhereClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#yieldClause}.
     * @param ctx the parse tree
     */
    fn enter_yieldClause(&mut self, _ctx: &YieldClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#yieldClause}.
     * @param ctx the parse tree
     */
    fn exit_yieldClause(&mut self, _ctx: &YieldClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#yieldItemList}.
     * @param ctx the parse tree
     */
    fn enter_yieldItemList(&mut self, _ctx: &YieldItemListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#yieldItemList}.
     * @param ctx the parse tree
     */
    fn exit_yieldItemList(&mut self, _ctx: &YieldItemListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#yieldItem}.
     * @param ctx the parse tree
     */
    fn enter_yieldItem(&mut self, _ctx: &YieldItemContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#yieldItem}.
     * @param ctx the parse tree
     */
    fn exit_yieldItem(&mut self, _ctx: &YieldItemContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#yieldItemName}.
     * @param ctx the parse tree
     */
    fn enter_yieldItemName(&mut self, _ctx: &YieldItemNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#yieldItemName}.
     * @param ctx the parse tree
     */
    fn exit_yieldItemName(&mut self, _ctx: &YieldItemNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#yieldItemAlias}.
     * @param ctx the parse tree
     */
    fn enter_yieldItemAlias(&mut self, _ctx: &YieldItemAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#yieldItemAlias}.
     * @param ctx the parse tree
     */
    fn exit_yieldItemAlias(&mut self, _ctx: &YieldItemAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#groupByClause}.
     * @param ctx the parse tree
     */
    fn enter_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#groupByClause}.
     * @param ctx the parse tree
     */
    fn exit_groupByClause(&mut self, _ctx: &GroupByClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#groupingElementList}.
     * @param ctx the parse tree
     */
    fn enter_groupingElementList(&mut self, _ctx: &GroupingElementListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#groupingElementList}.
     * @param ctx the parse tree
     */
    fn exit_groupingElementList(&mut self, _ctx: &GroupingElementListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#groupingElement}.
     * @param ctx the parse tree
     */
    fn enter_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#groupingElement}.
     * @param ctx the parse tree
     */
    fn exit_groupingElement(&mut self, _ctx: &GroupingElementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#emptyGroupingSet}.
     * @param ctx the parse tree
     */
    fn enter_emptyGroupingSet(&mut self, _ctx: &EmptyGroupingSetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#emptyGroupingSet}.
     * @param ctx the parse tree
     */
    fn exit_emptyGroupingSet(&mut self, _ctx: &EmptyGroupingSetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#orderByClause}.
     * @param ctx the parse tree
     */
    fn enter_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#orderByClause}.
     * @param ctx the parse tree
     */
    fn exit_orderByClause(&mut self, _ctx: &OrderByClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sortSpecificationList}.
     * @param ctx the parse tree
     */
    fn enter_sortSpecificationList(&mut self, _ctx: &SortSpecificationListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sortSpecificationList}.
     * @param ctx the parse tree
     */
    fn exit_sortSpecificationList(&mut self, _ctx: &SortSpecificationListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sortSpecification}.
     * @param ctx the parse tree
     */
    fn enter_sortSpecification(&mut self, _ctx: &SortSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sortSpecification}.
     * @param ctx the parse tree
     */
    fn exit_sortSpecification(&mut self, _ctx: &SortSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sortKey}.
     * @param ctx the parse tree
     */
    fn enter_sortKey(&mut self, _ctx: &SortKeyContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sortKey}.
     * @param ctx the parse tree
     */
    fn exit_sortKey(&mut self, _ctx: &SortKeyContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#orderingSpecification}.
     * @param ctx the parse tree
     */
    fn enter_orderingSpecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#orderingSpecification}.
     * @param ctx the parse tree
     */
    fn exit_orderingSpecification(&mut self, _ctx: &OrderingSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nullOrdering}.
     * @param ctx the parse tree
     */
    fn enter_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nullOrdering}.
     * @param ctx the parse tree
     */
    fn exit_nullOrdering(&mut self, _ctx: &NullOrderingContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#limitClause}.
     * @param ctx the parse tree
     */
    fn enter_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#limitClause}.
     * @param ctx the parse tree
     */
    fn exit_limitClause(&mut self, _ctx: &LimitClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#offsetClause}.
     * @param ctx the parse tree
     */
    fn enter_offsetClause(&mut self, _ctx: &OffsetClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#offsetClause}.
     * @param ctx the parse tree
     */
    fn exit_offsetClause(&mut self, _ctx: &OffsetClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#offsetSynonym}.
     * @param ctx the parse tree
     */
    fn enter_offsetSynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#offsetSynonym}.
     * @param ctx the parse tree
     */
    fn exit_offsetSynonym(&mut self, _ctx: &OffsetSynonymContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#schemaReference}.
     * @param ctx the parse tree
     */
    fn enter_schemaReference(&mut self, _ctx: &SchemaReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#schemaReference}.
     * @param ctx the parse tree
     */
    fn exit_schemaReference(&mut self, _ctx: &SchemaReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn enter_absoluteCatalogSchemaReference(
        &mut self,
        _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn exit_absoluteCatalogSchemaReference(
        &mut self,
        _ctx: &AbsoluteCatalogSchemaReferenceContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
     * @param ctx the parse tree
     */
    fn enter_catalogSchemaParentAndName(
        &mut self,
        _ctx: &CatalogSchemaParentAndNameContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
     * @param ctx the parse tree
     */
    fn exit_catalogSchemaParentAndName(
        &mut self,
        _ctx: &CatalogSchemaParentAndNameContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn enter_relativeCatalogSchemaReference(
        &mut self,
        _ctx: &RelativeCatalogSchemaReferenceContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn exit_relativeCatalogSchemaReference(
        &mut self,
        _ctx: &RelativeCatalogSchemaReferenceContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
     * @param ctx the parse tree
     */
    fn enter_predefinedSchemaReference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
     * @param ctx the parse tree
     */
    fn exit_predefinedSchemaReference(&mut self, _ctx: &PredefinedSchemaReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
     * @param ctx the parse tree
     */
    fn enter_absoluteDirectoryPath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
     * @param ctx the parse tree
     */
    fn exit_absoluteDirectoryPath(&mut self, _ctx: &AbsoluteDirectoryPathContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
     * @param ctx the parse tree
     */
    fn enter_relativeDirectoryPath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
     * @param ctx the parse tree
     */
    fn exit_relativeDirectoryPath(&mut self, _ctx: &RelativeDirectoryPathContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
     * @param ctx the parse tree
     */
    fn enter_simpleDirectoryPath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
     * @param ctx the parse tree
     */
    fn exit_simpleDirectoryPath(&mut self, _ctx: &SimpleDirectoryPathContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphReference}.
     * @param ctx the parse tree
     */
    fn enter_graphReference(&mut self, _ctx: &GraphReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphReference}.
     * @param ctx the parse tree
     */
    fn exit_graphReference(&mut self, _ctx: &GraphReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
     * @param ctx the parse tree
     */
    fn enter_catalogGraphParentAndName(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
     * @param ctx the parse tree
     */
    fn exit_catalogGraphParentAndName(&mut self, _ctx: &CatalogGraphParentAndNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#homeGraph}.
     * @param ctx the parse tree
     */
    fn enter_homeGraph(&mut self, _ctx: &HomeGraphContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#homeGraph}.
     * @param ctx the parse tree
     */
    fn exit_homeGraph(&mut self, _ctx: &HomeGraphContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphTypeReference}.
     * @param ctx the parse tree
     */
    fn enter_graphTypeReference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphTypeReference}.
     * @param ctx the parse tree
     */
    fn exit_graphTypeReference(&mut self, _ctx: &GraphTypeReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
     * @param ctx the parse tree
     */
    fn enter_catalogGraphTypeParentAndName(
        &mut self,
        _ctx: &CatalogGraphTypeParentAndNameContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
     * @param ctx the parse tree
     */
    fn exit_catalogGraphTypeParentAndName(
        &mut self,
        _ctx: &CatalogGraphTypeParentAndNameContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableReference}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableReference(&mut self, _ctx: &BindingTableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableReference}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableReference(&mut self, _ctx: &BindingTableReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureReference}.
     * @param ctx the parse tree
     */
    fn enter_procedureReference(&mut self, _ctx: &ProcedureReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureReference}.
     * @param ctx the parse tree
     */
    fn exit_procedureReference(&mut self, _ctx: &ProcedureReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
     * @param ctx the parse tree
     */
    fn enter_catalogProcedureParentAndName(
        &mut self,
        _ctx: &CatalogProcedureParentAndNameContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
     * @param ctx the parse tree
     */
    fn exit_catalogProcedureParentAndName(
        &mut self,
        _ctx: &CatalogProcedureParentAndNameContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
     * @param ctx the parse tree
     */
    fn enter_catalogObjectParentReference(
        &mut self,
        _ctx: &CatalogObjectParentReferenceContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
     * @param ctx the parse tree
     */
    fn exit_catalogObjectParentReference(
        &mut self,
        _ctx: &CatalogObjectParentReferenceContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#referenceParameterSpecification}.
     * @param ctx the parse tree
     */
    fn enter_referenceParameterSpecification(
        &mut self,
        _ctx: &ReferenceParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#referenceParameterSpecification}.
     * @param ctx the parse tree
     */
    fn exit_referenceParameterSpecification(
        &mut self,
        _ctx: &ReferenceParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
     * @param ctx the parse tree
     */
    fn enter_nestedGraphTypeSpecification(
        &mut self,
        _ctx: &NestedGraphTypeSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
     * @param ctx the parse tree
     */
    fn exit_nestedGraphTypeSpecification(
        &mut self,
        _ctx: &NestedGraphTypeSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
     * @param ctx the parse tree
     */
    fn enter_graphTypeSpecificationBody(
        &mut self,
        _ctx: &GraphTypeSpecificationBodyContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
     * @param ctx the parse tree
     */
    fn exit_graphTypeSpecificationBody(
        &mut self,
        _ctx: &GraphTypeSpecificationBodyContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#elementTypeList}.
     * @param ctx the parse tree
     */
    fn enter_elementTypeList(&mut self, _ctx: &ElementTypeListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementTypeList}.
     * @param ctx the parse tree
     */
    fn exit_elementTypeList(&mut self, _ctx: &ElementTypeListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementTypeSpecification}.
     * @param ctx the parse tree
     */
    fn enter_elementTypeSpecification(&mut self, _ctx: &ElementTypeSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementTypeSpecification}.
     * @param ctx the parse tree
     */
    fn exit_elementTypeSpecification(&mut self, _ctx: &ElementTypeSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeSpecification(&mut self, _ctx: &NodeTypeSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeSpecification(&mut self, _ctx: &NodeTypeSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypePattern}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypePattern(&mut self, _ctx: &NodeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypePattern}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypePattern(&mut self, _ctx: &NodeTypePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypePhrase}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypePhrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypePhrase}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypePhrase(&mut self, _ctx: &NodeTypePhraseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypePhraseFiller(&mut self, _ctx: &NodeTypePhraseFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypePhraseFiller(&mut self, _ctx: &NodeTypePhraseFillerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeFiller}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeFiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeFiller}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeFiller(&mut self, _ctx: &NodeTypeFillerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#localNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn enter_localNodeTypeAlias(&mut self, _ctx: &LocalNodeTypeAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#localNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn exit_localNodeTypeAlias(&mut self, _ctx: &LocalNodeTypeAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeImpliedContent(&mut self, _ctx: &NodeTypeImpliedContentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeImpliedContent(&mut self, _ctx: &NodeTypeImpliedContentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeKeyLabelSet(&mut self, _ctx: &NodeTypeKeyLabelSetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeKeyLabelSet(&mut self, _ctx: &NodeTypeKeyLabelSetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeLabelSet(&mut self, _ctx: &NodeTypeLabelSetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeLabelSet(&mut self, _ctx: &NodeTypeLabelSetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypePropertyTypes(&mut self, _ctx: &NodeTypePropertyTypesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypePropertyTypes(&mut self, _ctx: &NodeTypePropertyTypesContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeSpecification(&mut self, _ctx: &EdgeTypeSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeSpecification(&mut self, _ctx: &EdgeTypeSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePattern}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePattern}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePattern(&mut self, _ctx: &EdgeTypePatternContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePhrase}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePhrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePhrase}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePhrase(&mut self, _ctx: &EdgeTypePhraseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePhraseFiller(&mut self, _ctx: &EdgeTypePhraseFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePhraseFiller(&mut self, _ctx: &EdgeTypePhraseFillerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeFiller}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeFiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeFiller}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeFiller(&mut self, _ctx: &EdgeTypeFillerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeImpliedContent(&mut self, _ctx: &EdgeTypeImpliedContentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeImpliedContent(&mut self, _ctx: &EdgeTypeImpliedContentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeKeyLabelSet(&mut self, _ctx: &EdgeTypeKeyLabelSetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeKeyLabelSet(&mut self, _ctx: &EdgeTypeKeyLabelSetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeLabelSet(&mut self, _ctx: &EdgeTypeLabelSetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeLabelSet(&mut self, _ctx: &EdgeTypeLabelSetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePropertyTypes(&mut self, _ctx: &EdgeTypePropertyTypesContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePropertyTypes(&mut self, _ctx: &EdgeTypePropertyTypesContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePatternDirected}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePatternDirected(&mut self, _ctx: &EdgeTypePatternDirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePatternDirected}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePatternDirected(&mut self, _ctx: &EdgeTypePatternDirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePatternPointingRight}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePatternPointingRight(
        &mut self,
        _ctx: &EdgeTypePatternPointingRightContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePatternPointingRight}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePatternPointingRight(
        &mut self,
        _ctx: &EdgeTypePatternPointingRightContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePatternPointingLeft}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePatternPointingLeft(
        &mut self,
        _ctx: &EdgeTypePatternPointingLeftContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePatternPointingLeft}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePatternPointingLeft(
        &mut self,
        _ctx: &EdgeTypePatternPointingLeftContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypePatternUndirected}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypePatternUndirected(&mut self, _ctx: &EdgeTypePatternUndirectedContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypePatternUndirected}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypePatternUndirected(&mut self, _ctx: &EdgeTypePatternUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#arcTypePointingRight}.
     * @param ctx the parse tree
     */
    fn enter_arcTypePointingRight(&mut self, _ctx: &ArcTypePointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#arcTypePointingRight}.
     * @param ctx the parse tree
     */
    fn exit_arcTypePointingRight(&mut self, _ctx: &ArcTypePointingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
     * @param ctx the parse tree
     */
    fn enter_arcTypePointingLeft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
     * @param ctx the parse tree
     */
    fn exit_arcTypePointingLeft(&mut self, _ctx: &ArcTypePointingLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#arcTypeUndirected}.
     * @param ctx the parse tree
     */
    fn enter_arcTypeUndirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#arcTypeUndirected}.
     * @param ctx the parse tree
     */
    fn exit_arcTypeUndirected(&mut self, _ctx: &ArcTypeUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn enter_sourceNodeTypeReference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn exit_sourceNodeTypeReference(&mut self, _ctx: &SourceNodeTypeReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn enter_destinationNodeTypeReference(
        &mut self,
        _ctx: &DestinationNodeTypeReferenceContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn exit_destinationNodeTypeReference(
        &mut self,
        _ctx: &DestinationNodeTypeReferenceContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeKind}.
     * @param ctx the parse tree
     */
    fn enter_edgeKind(&mut self, _ctx: &EdgeKindContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeKind}.
     * @param ctx the parse tree
     */
    fn exit_edgeKind(&mut self, _ctx: &EdgeKindContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPairPhrase}.
     * @param ctx the parse tree
     */
    fn enter_endpointPairPhrase(&mut self, _ctx: &EndpointPairPhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPairPhrase}.
     * @param ctx the parse tree
     */
    fn exit_endpointPairPhrase(&mut self, _ctx: &EndpointPairPhraseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPair}.
     * @param ctx the parse tree
     */
    fn enter_endpointPair(&mut self, _ctx: &EndpointPairContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPair}.
     * @param ctx the parse tree
     */
    fn exit_endpointPair(&mut self, _ctx: &EndpointPairContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPairDirected}.
     * @param ctx the parse tree
     */
    fn enter_endpointPairDirected(&mut self, _ctx: &EndpointPairDirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPairDirected}.
     * @param ctx the parse tree
     */
    fn exit_endpointPairDirected(&mut self, _ctx: &EndpointPairDirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPairPointingRight}.
     * @param ctx the parse tree
     */
    fn enter_endpointPairPointingRight(&mut self, _ctx: &EndpointPairPointingRightContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPairPointingRight}.
     * @param ctx the parse tree
     */
    fn exit_endpointPairPointingRight(&mut self, _ctx: &EndpointPairPointingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPairPointingLeft}.
     * @param ctx the parse tree
     */
    fn enter_endpointPairPointingLeft(&mut self, _ctx: &EndpointPairPointingLeftContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPairPointingLeft}.
     * @param ctx the parse tree
     */
    fn exit_endpointPairPointingLeft(&mut self, _ctx: &EndpointPairPointingLeftContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#endpointPairUndirected}.
     * @param ctx the parse tree
     */
    fn enter_endpointPairUndirected(&mut self, _ctx: &EndpointPairUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#endpointPairUndirected}.
     * @param ctx the parse tree
     */
    fn exit_endpointPairUndirected(&mut self, _ctx: &EndpointPairUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#connectorPointingRight}.
     * @param ctx the parse tree
     */
    fn enter_connectorPointingRight(&mut self, _ctx: &ConnectorPointingRightContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#connectorPointingRight}.
     * @param ctx the parse tree
     */
    fn exit_connectorPointingRight(&mut self, _ctx: &ConnectorPointingRightContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#connectorUndirected}.
     * @param ctx the parse tree
     */
    fn enter_connectorUndirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#connectorUndirected}.
     * @param ctx the parse tree
     */
    fn exit_connectorUndirected(&mut self, _ctx: &ConnectorUndirectedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sourceNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn enter_sourceNodeTypeAlias(&mut self, _ctx: &SourceNodeTypeAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sourceNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn exit_sourceNodeTypeAlias(&mut self, _ctx: &SourceNodeTypeAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#destinationNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn enter_destinationNodeTypeAlias(&mut self, _ctx: &DestinationNodeTypeAliasContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#destinationNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn exit_destinationNodeTypeAlias(&mut self, _ctx: &DestinationNodeTypeAliasContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#labelSetPhrase}.
     * @param ctx the parse tree
     */
    fn enter_labelSetPhrase(&mut self, _ctx: &LabelSetPhraseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#labelSetPhrase}.
     * @param ctx the parse tree
     */
    fn exit_labelSetPhrase(&mut self, _ctx: &LabelSetPhraseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#labelSetSpecification}.
     * @param ctx the parse tree
     */
    fn enter_labelSetSpecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#labelSetSpecification}.
     * @param ctx the parse tree
     */
    fn exit_labelSetSpecification(&mut self, _ctx: &LabelSetSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyTypesSpecification}.
     * @param ctx the parse tree
     */
    fn enter_propertyTypesSpecification(
        &mut self,
        _ctx: &PropertyTypesSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyTypesSpecification}.
     * @param ctx the parse tree
     */
    fn exit_propertyTypesSpecification(
        &mut self,
        _ctx: &PropertyTypesSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyTypeList}.
     * @param ctx the parse tree
     */
    fn enter_propertyTypeList(&mut self, _ctx: &PropertyTypeListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyTypeList}.
     * @param ctx the parse tree
     */
    fn exit_propertyTypeList(&mut self, _ctx: &PropertyTypeListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyType}.
     * @param ctx the parse tree
     */
    fn enter_propertyType(&mut self, _ctx: &PropertyTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyType}.
     * @param ctx the parse tree
     */
    fn exit_propertyType(&mut self, _ctx: &PropertyTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyValueType}.
     * @param ctx the parse tree
     */
    fn enter_propertyValueType(&mut self, _ctx: &PropertyValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyValueType}.
     * @param ctx the parse tree
     */
    fn exit_propertyValueType(&mut self, _ctx: &PropertyValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableType}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableType(&mut self, _ctx: &BindingTableTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableType}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableType(&mut self, _ctx: &BindingTableTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code dynamicPropertyValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_dynamicPropertyValueTypeLabel(
        &mut self,
        _ctx: &DynamicPropertyValueTypeLabelContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code dynamicPropertyValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_dynamicPropertyValueTypeLabel(
        &mut self,
        _ctx: &DynamicPropertyValueTypeLabelContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code closedDynamicUnionTypeAtl1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_closedDynamicUnionTypeAtl1(
        &mut self,
        _ctx: &ClosedDynamicUnionTypeAtl1Context<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code closedDynamicUnionTypeAtl1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_closedDynamicUnionTypeAtl1(
        &mut self,
        _ctx: &ClosedDynamicUnionTypeAtl1Context<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code closedDynamicUnionTypeAtl2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_closedDynamicUnionTypeAtl2(
        &mut self,
        _ctx: &ClosedDynamicUnionTypeAtl2Context<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code closedDynamicUnionTypeAtl2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_closedDynamicUnionTypeAtl2(
        &mut self,
        _ctx: &ClosedDynamicUnionTypeAtl2Context<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code pathValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_pathValueTypeLabel(&mut self, _ctx: &PathValueTypeLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code pathValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_pathValueTypeLabel(&mut self, _ctx: &PathValueTypeLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code listValueTypeAlt3}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_listValueTypeAlt3(&mut self, _ctx: &ListValueTypeAlt3Context<'input>) {}
    /**
     * Exit a parse tree produced by the {@code listValueTypeAlt3}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_listValueTypeAlt3(&mut self, _ctx: &ListValueTypeAlt3Context<'input>) {}
    /**
     * Enter a parse tree produced by the {@code listValueTypeAlt2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_listValueTypeAlt2(&mut self, _ctx: &ListValueTypeAlt2Context<'input>) {}
    /**
     * Exit a parse tree produced by the {@code listValueTypeAlt2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_listValueTypeAlt2(&mut self, _ctx: &ListValueTypeAlt2Context<'input>) {}
    /**
     * Enter a parse tree produced by the {@code listValueTypeAlt1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_listValueTypeAlt1(&mut self, _ctx: &ListValueTypeAlt1Context<'input>) {}
    /**
     * Exit a parse tree produced by the {@code listValueTypeAlt1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_listValueTypeAlt1(&mut self, _ctx: &ListValueTypeAlt1Context<'input>) {}
    /**
     * Enter a parse tree produced by the {@code predefinedTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_predefinedTypeLabel(&mut self, _ctx: &PredefinedTypeLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code predefinedTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_predefinedTypeLabel(&mut self, _ctx: &PredefinedTypeLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code recordTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_recordTypeLabel(&mut self, _ctx: &RecordTypeLabelContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code recordTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_recordTypeLabel(&mut self, _ctx: &RecordTypeLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code openDynamicUnionTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn enter_openDynamicUnionTypeLabel(&mut self, _ctx: &OpenDynamicUnionTypeLabelContext<'input>) {
    }
    /**
     * Exit a parse tree produced by the {@code openDynamicUnionTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn exit_openDynamicUnionTypeLabel(&mut self, _ctx: &OpenDynamicUnionTypeLabelContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#typed}.
     * @param ctx the parse tree
     */
    fn enter_typed(&mut self, _ctx: &TypedContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#typed}.
     * @param ctx the parse tree
     */
    fn exit_typed(&mut self, _ctx: &TypedContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#predefinedType}.
     * @param ctx the parse tree
     */
    fn enter_predefinedType(&mut self, _ctx: &PredefinedTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#predefinedType}.
     * @param ctx the parse tree
     */
    fn exit_predefinedType(&mut self, _ctx: &PredefinedTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#booleanType}.
     * @param ctx the parse tree
     */
    fn enter_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#booleanType}.
     * @param ctx the parse tree
     */
    fn exit_booleanType(&mut self, _ctx: &BooleanTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#characterStringType}.
     * @param ctx the parse tree
     */
    fn enter_characterStringType(&mut self, _ctx: &CharacterStringTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#characterStringType}.
     * @param ctx the parse tree
     */
    fn exit_characterStringType(&mut self, _ctx: &CharacterStringTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#byteStringType}.
     * @param ctx the parse tree
     */
    fn enter_byteStringType(&mut self, _ctx: &ByteStringTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#byteStringType}.
     * @param ctx the parse tree
     */
    fn exit_byteStringType(&mut self, _ctx: &ByteStringTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#minLength}.
     * @param ctx the parse tree
     */
    fn enter_minLength(&mut self, _ctx: &MinLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#minLength}.
     * @param ctx the parse tree
     */
    fn exit_minLength(&mut self, _ctx: &MinLengthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#maxLength}.
     * @param ctx the parse tree
     */
    fn enter_maxLength(&mut self, _ctx: &MaxLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#maxLength}.
     * @param ctx the parse tree
     */
    fn exit_maxLength(&mut self, _ctx: &MaxLengthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fixedLength}.
     * @param ctx the parse tree
     */
    fn enter_fixedLength(&mut self, _ctx: &FixedLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fixedLength}.
     * @param ctx the parse tree
     */
    fn exit_fixedLength(&mut self, _ctx: &FixedLengthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numericType}.
     * @param ctx the parse tree
     */
    fn enter_numericType(&mut self, _ctx: &NumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#numericType}.
     * @param ctx the parse tree
     */
    fn exit_numericType(&mut self, _ctx: &NumericTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#exactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_exactNumericType(&mut self, _ctx: &ExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#exactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_exactNumericType(&mut self, _ctx: &ExactNumericTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#binaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_binaryExactNumericType(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#binaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_binaryExactNumericType(&mut self, _ctx: &BinaryExactNumericTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_signedBinaryExactNumericType(
        &mut self,
        _ctx: &SignedBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_signedBinaryExactNumericType(
        &mut self,
        _ctx: &SignedBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_unsignedBinaryExactNumericType(
        &mut self,
        _ctx: &UnsignedBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_unsignedBinaryExactNumericType(
        &mut self,
        _ctx: &UnsignedBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_verboseBinaryExactNumericType(
        &mut self,
        _ctx: &VerboseBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_verboseBinaryExactNumericType(
        &mut self,
        _ctx: &VerboseBinaryExactNumericTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#decimalExactNumericType}.
     * @param ctx the parse tree
     */
    fn enter_decimalExactNumericType(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#decimalExactNumericType}.
     * @param ctx the parse tree
     */
    fn exit_decimalExactNumericType(&mut self, _ctx: &DecimalExactNumericTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#precision}.
     * @param ctx the parse tree
     */
    fn enter_precision(&mut self, _ctx: &PrecisionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#precision}.
     * @param ctx the parse tree
     */
    fn exit_precision(&mut self, _ctx: &PrecisionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#scale}.
     * @param ctx the parse tree
     */
    fn enter_scale(&mut self, _ctx: &ScaleContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#scale}.
     * @param ctx the parse tree
     */
    fn exit_scale(&mut self, _ctx: &ScaleContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#approximateNumericType}.
     * @param ctx the parse tree
     */
    fn enter_approximateNumericType(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#approximateNumericType}.
     * @param ctx the parse tree
     */
    fn exit_approximateNumericType(&mut self, _ctx: &ApproximateNumericTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#temporalType}.
     * @param ctx the parse tree
     */
    fn enter_temporalType(&mut self, _ctx: &TemporalTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#temporalType}.
     * @param ctx the parse tree
     */
    fn exit_temporalType(&mut self, _ctx: &TemporalTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#temporalInstantType}.
     * @param ctx the parse tree
     */
    fn enter_temporalInstantType(&mut self, _ctx: &TemporalInstantTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#temporalInstantType}.
     * @param ctx the parse tree
     */
    fn exit_temporalInstantType(&mut self, _ctx: &TemporalInstantTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeType}.
     * @param ctx the parse tree
     */
    fn enter_datetimeType(&mut self, _ctx: &DatetimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeType}.
     * @param ctx the parse tree
     */
    fn exit_datetimeType(&mut self, _ctx: &DatetimeTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#localdatetimeType}.
     * @param ctx the parse tree
     */
    fn enter_localdatetimeType(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#localdatetimeType}.
     * @param ctx the parse tree
     */
    fn exit_localdatetimeType(&mut self, _ctx: &LocaldatetimeTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dateType}.
     * @param ctx the parse tree
     */
    fn enter_dateType(&mut self, _ctx: &DateTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dateType}.
     * @param ctx the parse tree
     */
    fn exit_dateType(&mut self, _ctx: &DateTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeType}.
     * @param ctx the parse tree
     */
    fn enter_timeType(&mut self, _ctx: &TimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeType}.
     * @param ctx the parse tree
     */
    fn exit_timeType(&mut self, _ctx: &TimeTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#localtimeType}.
     * @param ctx the parse tree
     */
    fn enter_localtimeType(&mut self, _ctx: &LocaltimeTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#localtimeType}.
     * @param ctx the parse tree
     */
    fn exit_localtimeType(&mut self, _ctx: &LocaltimeTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#temporalDurationType}.
     * @param ctx the parse tree
     */
    fn enter_temporalDurationType(&mut self, _ctx: &TemporalDurationTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#temporalDurationType}.
     * @param ctx the parse tree
     */
    fn exit_temporalDurationType(&mut self, _ctx: &TemporalDurationTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#temporalDurationQualifier}.
     * @param ctx the parse tree
     */
    fn enter_temporalDurationQualifier(&mut self, _ctx: &TemporalDurationQualifierContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#temporalDurationQualifier}.
     * @param ctx the parse tree
     */
    fn exit_temporalDurationQualifier(&mut self, _ctx: &TemporalDurationQualifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#referenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_referenceValueType(&mut self, _ctx: &ReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#referenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_referenceValueType(&mut self, _ctx: &ReferenceValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#immaterialValueType}.
     * @param ctx the parse tree
     */
    fn enter_immaterialValueType(&mut self, _ctx: &ImmaterialValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#immaterialValueType}.
     * @param ctx the parse tree
     */
    fn exit_immaterialValueType(&mut self, _ctx: &ImmaterialValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nullType}.
     * @param ctx the parse tree
     */
    fn enter_nullType(&mut self, _ctx: &NullTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nullType}.
     * @param ctx the parse tree
     */
    fn exit_nullType(&mut self, _ctx: &NullTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#emptyType}.
     * @param ctx the parse tree
     */
    fn enter_emptyType(&mut self, _ctx: &EmptyTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#emptyType}.
     * @param ctx the parse tree
     */
    fn exit_emptyType(&mut self, _ctx: &EmptyTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_graphReferenceValueType(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_graphReferenceValueType(&mut self, _ctx: &GraphReferenceValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_closedGraphReferenceValueType(
        &mut self,
        _ctx: &ClosedGraphReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_closedGraphReferenceValueType(
        &mut self,
        _ctx: &ClosedGraphReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_openGraphReferenceValueType(
        &mut self,
        _ctx: &OpenGraphReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_openGraphReferenceValueType(
        &mut self,
        _ctx: &OpenGraphReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableReferenceValueType(
        &mut self,
        _ctx: &BindingTableReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableReferenceValueType(
        &mut self,
        _ctx: &BindingTableReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_nodeReferenceValueType(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_nodeReferenceValueType(&mut self, _ctx: &NodeReferenceValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_closedNodeReferenceValueType(
        &mut self,
        _ctx: &ClosedNodeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_closedNodeReferenceValueType(
        &mut self,
        _ctx: &ClosedNodeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_openNodeReferenceValueType(
        &mut self,
        _ctx: &OpenNodeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_openNodeReferenceValueType(
        &mut self,
        _ctx: &OpenNodeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_edgeReferenceValueType(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_edgeReferenceValueType(&mut self, _ctx: &EdgeReferenceValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_closedEdgeReferenceValueType(
        &mut self,
        _ctx: &ClosedEdgeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_closedEdgeReferenceValueType(
        &mut self,
        _ctx: &ClosedEdgeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn enter_openEdgeReferenceValueType(
        &mut self,
        _ctx: &OpenEdgeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn exit_openEdgeReferenceValueType(
        &mut self,
        _ctx: &OpenEdgeReferenceValueTypeContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#pathValueType}.
     * @param ctx the parse tree
     */
    fn enter_pathValueType(&mut self, _ctx: &PathValueTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathValueType}.
     * @param ctx the parse tree
     */
    fn exit_pathValueType(&mut self, _ctx: &PathValueTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueTypeName}.
     * @param ctx the parse tree
     */
    fn enter_listValueTypeName(&mut self, _ctx: &ListValueTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueTypeName}.
     * @param ctx the parse tree
     */
    fn exit_listValueTypeName(&mut self, _ctx: &ListValueTypeNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
     * @param ctx the parse tree
     */
    fn enter_listValueTypeNameSynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
     * @param ctx the parse tree
     */
    fn exit_listValueTypeNameSynonym(&mut self, _ctx: &ListValueTypeNameSynonymContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#recordType}.
     * @param ctx the parse tree
     */
    fn enter_recordType(&mut self, _ctx: &RecordTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#recordType}.
     * @param ctx the parse tree
     */
    fn exit_recordType(&mut self, _ctx: &RecordTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
     * @param ctx the parse tree
     */
    fn enter_fieldTypesSpecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
     * @param ctx the parse tree
     */
    fn exit_fieldTypesSpecification(&mut self, _ctx: &FieldTypesSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldTypeList}.
     * @param ctx the parse tree
     */
    fn enter_fieldTypeList(&mut self, _ctx: &FieldTypeListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldTypeList}.
     * @param ctx the parse tree
     */
    fn exit_fieldTypeList(&mut self, _ctx: &FieldTypeListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#notNull}.
     * @param ctx the parse tree
     */
    fn enter_notNull(&mut self, _ctx: &NotNullContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#notNull}.
     * @param ctx the parse tree
     */
    fn exit_notNull(&mut self, _ctx: &NotNullContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldType}.
     * @param ctx the parse tree
     */
    fn enter_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldType}.
     * @param ctx the parse tree
     */
    fn exit_fieldType(&mut self, _ctx: &FieldTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#searchCondition}.
     * @param ctx the parse tree
     */
    fn enter_searchCondition(&mut self, _ctx: &SearchConditionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#searchCondition}.
     * @param ctx the parse tree
     */
    fn exit_searchCondition(&mut self, _ctx: &SearchConditionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#predicate}.
     * @param ctx the parse tree
     */
    fn enter_predicate(&mut self, _ctx: &PredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#predicate}.
     * @param ctx the parse tree
     */
    fn exit_predicate(&mut self, _ctx: &PredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#compOp}.
     * @param ctx the parse tree
     */
    fn enter_compOp(&mut self, _ctx: &CompOpContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#compOp}.
     * @param ctx the parse tree
     */
    fn exit_compOp(&mut self, _ctx: &CompOpContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#existsPredicate}.
     * @param ctx the parse tree
     */
    fn enter_existsPredicate(&mut self, _ctx: &ExistsPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#existsPredicate}.
     * @param ctx the parse tree
     */
    fn exit_existsPredicate(&mut self, _ctx: &ExistsPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nullPredicate}.
     * @param ctx the parse tree
     */
    fn enter_nullPredicate(&mut self, _ctx: &NullPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nullPredicate}.
     * @param ctx the parse tree
     */
    fn exit_nullPredicate(&mut self, _ctx: &NullPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nullPredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_nullPredicatePart2(&mut self, _ctx: &NullPredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nullPredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_nullPredicatePart2(&mut self, _ctx: &NullPredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueTypePredicate}.
     * @param ctx the parse tree
     */
    fn enter_valueTypePredicate(&mut self, _ctx: &ValueTypePredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueTypePredicate}.
     * @param ctx the parse tree
     */
    fn exit_valueTypePredicate(&mut self, _ctx: &ValueTypePredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueTypePredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_valueTypePredicatePart2(&mut self, _ctx: &ValueTypePredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueTypePredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_valueTypePredicatePart2(&mut self, _ctx: &ValueTypePredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#normalizedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_normalizedPredicatePart2(&mut self, _ctx: &NormalizedPredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#normalizedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_normalizedPredicatePart2(&mut self, _ctx: &NormalizedPredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#directedPredicate}.
     * @param ctx the parse tree
     */
    fn enter_directedPredicate(&mut self, _ctx: &DirectedPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#directedPredicate}.
     * @param ctx the parse tree
     */
    fn exit_directedPredicate(&mut self, _ctx: &DirectedPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#directedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_directedPredicatePart2(&mut self, _ctx: &DirectedPredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#directedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_directedPredicatePart2(&mut self, _ctx: &DirectedPredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#labeledPredicate}.
     * @param ctx the parse tree
     */
    fn enter_labeledPredicate(&mut self, _ctx: &LabeledPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#labeledPredicate}.
     * @param ctx the parse tree
     */
    fn exit_labeledPredicate(&mut self, _ctx: &LabeledPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#labeledPredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_labeledPredicatePart2(&mut self, _ctx: &LabeledPredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#labeledPredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_labeledPredicatePart2(&mut self, _ctx: &LabeledPredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#isLabeledOrColon}.
     * @param ctx the parse tree
     */
    fn enter_isLabeledOrColon(&mut self, _ctx: &IsLabeledOrColonContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#isLabeledOrColon}.
     * @param ctx the parse tree
     */
    fn exit_isLabeledOrColon(&mut self, _ctx: &IsLabeledOrColonContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sourceDestinationPredicate}.
     * @param ctx the parse tree
     */
    fn enter_sourceDestinationPredicate(
        &mut self,
        _ctx: &SourceDestinationPredicateContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#sourceDestinationPredicate}.
     * @param ctx the parse tree
     */
    fn exit_sourceDestinationPredicate(
        &mut self,
        _ctx: &SourceDestinationPredicateContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeReference}.
     * @param ctx the parse tree
     */
    fn enter_nodeReference(&mut self, _ctx: &NodeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeReference}.
     * @param ctx the parse tree
     */
    fn exit_nodeReference(&mut self, _ctx: &NodeReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#sourcePredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_sourcePredicatePart2(&mut self, _ctx: &SourcePredicatePart2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#sourcePredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_sourcePredicatePart2(&mut self, _ctx: &SourcePredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#destinationPredicatePart2}.
     * @param ctx the parse tree
     */
    fn enter_destinationPredicatePart2(&mut self, _ctx: &DestinationPredicatePart2Context<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#destinationPredicatePart2}.
     * @param ctx the parse tree
     */
    fn exit_destinationPredicatePart2(&mut self, _ctx: &DestinationPredicatePart2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeReference}.
     * @param ctx the parse tree
     */
    fn enter_edgeReference(&mut self, _ctx: &EdgeReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeReference}.
     * @param ctx the parse tree
     */
    fn exit_edgeReference(&mut self, _ctx: &EdgeReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#all_differentPredicate}.
     * @param ctx the parse tree
     */
    fn enter_all_differentPredicate(&mut self, _ctx: &All_differentPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#all_differentPredicate}.
     * @param ctx the parse tree
     */
    fn exit_all_differentPredicate(&mut self, _ctx: &All_differentPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#samePredicate}.
     * @param ctx the parse tree
     */
    fn enter_samePredicate(&mut self, _ctx: &SamePredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#samePredicate}.
     * @param ctx the parse tree
     */
    fn exit_samePredicate(&mut self, _ctx: &SamePredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#property_existsPredicate}.
     * @param ctx the parse tree
     */
    fn enter_property_existsPredicate(&mut self, _ctx: &Property_existsPredicateContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#property_existsPredicate}.
     * @param ctx the parse tree
     */
    fn exit_property_existsPredicate(&mut self, _ctx: &Property_existsPredicateContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code conjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_conjunctiveExprAlt(&mut self, _ctx: &ConjunctiveExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code conjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_conjunctiveExprAlt(&mut self, _ctx: &ConjunctiveExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code propertyGraphExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_propertyGraphExprAlt(&mut self, _ctx: &PropertyGraphExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code propertyGraphExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_propertyGraphExprAlt(&mut self, _ctx: &PropertyGraphExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code multDivExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_multDivExprAlt(&mut self, _ctx: &MultDivExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code multDivExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_multDivExprAlt(&mut self, _ctx: &MultDivExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code bindingTableExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableExprAlt(&mut self, _ctx: &BindingTableExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code bindingTableExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableExprAlt(&mut self, _ctx: &BindingTableExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code signedExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_signedExprAlt(&mut self, _ctx: &SignedExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code signedExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_signedExprAlt(&mut self, _ctx: &SignedExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code isNotExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_isNotExprAlt(&mut self, _ctx: &IsNotExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code isNotExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_isNotExprAlt(&mut self, _ctx: &IsNotExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code normalizedPredicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_normalizedPredicateExprAlt(
        &mut self,
        _ctx: &NormalizedPredicateExprAltContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by the {@code normalizedPredicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_normalizedPredicateExprAlt(
        &mut self,
        _ctx: &NormalizedPredicateExprAltContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by the {@code notExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_notExprAlt(&mut self, _ctx: &NotExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code notExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_notExprAlt(&mut self, _ctx: &NotExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code valueFunctionExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_valueFunctionExprAlt(&mut self, _ctx: &ValueFunctionExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code valueFunctionExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_valueFunctionExprAlt(&mut self, _ctx: &ValueFunctionExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code concatenationExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_concatenationExprAlt(&mut self, _ctx: &ConcatenationExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code concatenationExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_concatenationExprAlt(&mut self, _ctx: &ConcatenationExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code disjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_disjunctiveExprAlt(&mut self, _ctx: &DisjunctiveExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code disjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_disjunctiveExprAlt(&mut self, _ctx: &DisjunctiveExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code comparisonExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_comparisonExprAlt(&mut self, _ctx: &ComparisonExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code comparisonExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_comparisonExprAlt(&mut self, _ctx: &ComparisonExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code primaryExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_primaryExprAlt(&mut self, _ctx: &PrimaryExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code primaryExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_primaryExprAlt(&mut self, _ctx: &PrimaryExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code addSubtractExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_addSubtractExprAlt(&mut self, _ctx: &AddSubtractExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code addSubtractExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_addSubtractExprAlt(&mut self, _ctx: &AddSubtractExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by the {@code predicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn enter_predicateExprAlt(&mut self, _ctx: &PredicateExprAltContext<'input>) {}
    /**
     * Exit a parse tree produced by the {@code predicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn exit_predicateExprAlt(&mut self, _ctx: &PredicateExprAltContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueFunction}.
     * @param ctx the parse tree
     */
    fn enter_valueFunction(&mut self, _ctx: &ValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueFunction}.
     * @param ctx the parse tree
     */
    fn exit_valueFunction(&mut self, _ctx: &ValueFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#booleanValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_booleanValueExpression(&mut self, _ctx: &BooleanValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#booleanValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_booleanValueExpression(&mut self, _ctx: &BooleanValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#characterOrByteStringFunction}.
     * @param ctx the parse tree
     */
    fn enter_characterOrByteStringFunction(
        &mut self,
        _ctx: &CharacterOrByteStringFunctionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#characterOrByteStringFunction}.
     * @param ctx the parse tree
     */
    fn exit_characterOrByteStringFunction(
        &mut self,
        _ctx: &CharacterOrByteStringFunctionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#subCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn enter_subCharacterOrByteString(&mut self, _ctx: &SubCharacterOrByteStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#subCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn exit_subCharacterOrByteString(&mut self, _ctx: &SubCharacterOrByteStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimSingleCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn enter_trimSingleCharacterOrByteString(
        &mut self,
        _ctx: &TrimSingleCharacterOrByteStringContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#trimSingleCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn exit_trimSingleCharacterOrByteString(
        &mut self,
        _ctx: &TrimSingleCharacterOrByteStringContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#foldCharacterString}.
     * @param ctx the parse tree
     */
    fn enter_foldCharacterString(&mut self, _ctx: &FoldCharacterStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#foldCharacterString}.
     * @param ctx the parse tree
     */
    fn exit_foldCharacterString(&mut self, _ctx: &FoldCharacterStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimMultiCharacterCharacterString}.
     * @param ctx the parse tree
     */
    fn enter_trimMultiCharacterCharacterString(
        &mut self,
        _ctx: &TrimMultiCharacterCharacterStringContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#trimMultiCharacterCharacterString}.
     * @param ctx the parse tree
     */
    fn exit_trimMultiCharacterCharacterString(
        &mut self,
        _ctx: &TrimMultiCharacterCharacterStringContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#normalizeCharacterString}.
     * @param ctx the parse tree
     */
    fn enter_normalizeCharacterString(&mut self, _ctx: &NormalizeCharacterStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#normalizeCharacterString}.
     * @param ctx the parse tree
     */
    fn exit_normalizeCharacterString(&mut self, _ctx: &NormalizeCharacterStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_nodeReferenceValueExpression(
        &mut self,
        _ctx: &NodeReferenceValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_nodeReferenceValueExpression(
        &mut self,
        _ctx: &NodeReferenceValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_edgeReferenceValueExpression(
        &mut self,
        _ctx: &EdgeReferenceValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_edgeReferenceValueExpression(
        &mut self,
        _ctx: &EdgeReferenceValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#aggregatingValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_aggregatingValueExpression(
        &mut self,
        _ctx: &AggregatingValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#aggregatingValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_aggregatingValueExpression(
        &mut self,
        _ctx: &AggregatingValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#valueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn enter_valueExpressionPrimary(&mut self, _ctx: &ValueExpressionPrimaryContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn exit_valueExpressionPrimary(&mut self, _ctx: &ValueExpressionPrimaryContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#parenthesizedValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_parenthesizedValueExpression(
        &mut self,
        _ctx: &ParenthesizedValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#parenthesizedValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_parenthesizedValueExpression(
        &mut self,
        _ctx: &ParenthesizedValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn enter_nonParenthesizedValueExpressionPrimary(
        &mut self,
        _ctx: &NonParenthesizedValueExpressionPrimaryContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn exit_nonParenthesizedValueExpressionPrimary(
        &mut self,
        _ctx: &NonParenthesizedValueExpressionPrimaryContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimarySpecialCase}.
     * @param ctx the parse tree
     */
    fn enter_nonParenthesizedValueExpressionPrimarySpecialCase(
        &mut self,
        _ctx: &NonParenthesizedValueExpressionPrimarySpecialCaseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimarySpecialCase}.
     * @param ctx the parse tree
     */
    fn exit_nonParenthesizedValueExpressionPrimarySpecialCase(
        &mut self,
        _ctx: &NonParenthesizedValueExpressionPrimarySpecialCaseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
     * @param ctx the parse tree
     */
    fn enter_unsignedValueSpecification(
        &mut self,
        _ctx: &UnsignedValueSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
     * @param ctx the parse tree
     */
    fn exit_unsignedValueSpecification(
        &mut self,
        _ctx: &UnsignedValueSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#nonNegativeIntegerSpecification}.
     * @param ctx the parse tree
     */
    fn enter_nonNegativeIntegerSpecification(
        &mut self,
        _ctx: &NonNegativeIntegerSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#nonNegativeIntegerSpecification}.
     * @param ctx the parse tree
     */
    fn exit_nonNegativeIntegerSpecification(
        &mut self,
        _ctx: &NonNegativeIntegerSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#generalValueSpecification}.
     * @param ctx the parse tree
     */
    fn enter_generalValueSpecification(&mut self, _ctx: &GeneralValueSpecificationContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#generalValueSpecification}.
     * @param ctx the parse tree
     */
    fn exit_generalValueSpecification(&mut self, _ctx: &GeneralValueSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dynamicParameterSpecification}.
     * @param ctx the parse tree
     */
    fn enter_dynamicParameterSpecification(
        &mut self,
        _ctx: &DynamicParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#dynamicParameterSpecification}.
     * @param ctx the parse tree
     */
    fn exit_dynamicParameterSpecification(
        &mut self,
        _ctx: &DynamicParameterSpecificationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#letValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_letValueExpression(&mut self, _ctx: &LetValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#letValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_letValueExpression(&mut self, _ctx: &LetValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#valueQueryExpression}.
     * @param ctx the parse tree
     */
    fn enter_valueQueryExpression(&mut self, _ctx: &ValueQueryExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#valueQueryExpression}.
     * @param ctx the parse tree
     */
    fn exit_valueQueryExpression(&mut self, _ctx: &ValueQueryExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#caseExpression}.
     * @param ctx the parse tree
     */
    fn enter_caseExpression(&mut self, _ctx: &CaseExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#caseExpression}.
     * @param ctx the parse tree
     */
    fn exit_caseExpression(&mut self, _ctx: &CaseExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#caseAbbreviation}.
     * @param ctx the parse tree
     */
    fn enter_caseAbbreviation(&mut self, _ctx: &CaseAbbreviationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#caseAbbreviation}.
     * @param ctx the parse tree
     */
    fn exit_caseAbbreviation(&mut self, _ctx: &CaseAbbreviationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#caseSpecification}.
     * @param ctx the parse tree
     */
    fn enter_caseSpecification(&mut self, _ctx: &CaseSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#caseSpecification}.
     * @param ctx the parse tree
     */
    fn exit_caseSpecification(&mut self, _ctx: &CaseSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleCase}.
     * @param ctx the parse tree
     */
    fn enter_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleCase}.
     * @param ctx the parse tree
     */
    fn exit_simpleCase(&mut self, _ctx: &SimpleCaseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#searchedCase}.
     * @param ctx the parse tree
     */
    fn enter_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#searchedCase}.
     * @param ctx the parse tree
     */
    fn exit_searchedCase(&mut self, _ctx: &SearchedCaseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#simpleWhenClause}.
     * @param ctx the parse tree
     */
    fn enter_simpleWhenClause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#simpleWhenClause}.
     * @param ctx the parse tree
     */
    fn exit_simpleWhenClause(&mut self, _ctx: &SimpleWhenClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#searchedWhenClause}.
     * @param ctx the parse tree
     */
    fn enter_searchedWhenClause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#searchedWhenClause}.
     * @param ctx the parse tree
     */
    fn exit_searchedWhenClause(&mut self, _ctx: &SearchedWhenClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elseClause}.
     * @param ctx the parse tree
     */
    fn enter_elseClause(&mut self, _ctx: &ElseClauseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elseClause}.
     * @param ctx the parse tree
     */
    fn exit_elseClause(&mut self, _ctx: &ElseClauseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#caseOperand}.
     * @param ctx the parse tree
     */
    fn enter_caseOperand(&mut self, _ctx: &CaseOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#caseOperand}.
     * @param ctx the parse tree
     */
    fn exit_caseOperand(&mut self, _ctx: &CaseOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#whenOperandList}.
     * @param ctx the parse tree
     */
    fn enter_whenOperandList(&mut self, _ctx: &WhenOperandListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#whenOperandList}.
     * @param ctx the parse tree
     */
    fn exit_whenOperandList(&mut self, _ctx: &WhenOperandListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#whenOperand}.
     * @param ctx the parse tree
     */
    fn enter_whenOperand(&mut self, _ctx: &WhenOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#whenOperand}.
     * @param ctx the parse tree
     */
    fn exit_whenOperand(&mut self, _ctx: &WhenOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#result}.
     * @param ctx the parse tree
     */
    fn enter_result(&mut self, _ctx: &ResultContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#result}.
     * @param ctx the parse tree
     */
    fn exit_result(&mut self, _ctx: &ResultContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#resultExpression}.
     * @param ctx the parse tree
     */
    fn enter_resultExpression(&mut self, _ctx: &ResultExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#resultExpression}.
     * @param ctx the parse tree
     */
    fn exit_resultExpression(&mut self, _ctx: &ResultExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#castSpecification}.
     * @param ctx the parse tree
     */
    fn enter_castSpecification(&mut self, _ctx: &CastSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#castSpecification}.
     * @param ctx the parse tree
     */
    fn exit_castSpecification(&mut self, _ctx: &CastSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#castOperand}.
     * @param ctx the parse tree
     */
    fn enter_castOperand(&mut self, _ctx: &CastOperandContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#castOperand}.
     * @param ctx the parse tree
     */
    fn exit_castOperand(&mut self, _ctx: &CastOperandContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#castTarget}.
     * @param ctx the parse tree
     */
    fn enter_castTarget(&mut self, _ctx: &CastTargetContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#castTarget}.
     * @param ctx the parse tree
     */
    fn exit_castTarget(&mut self, _ctx: &CastTargetContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#aggregateFunction}.
     * @param ctx the parse tree
     */
    fn enter_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#aggregateFunction}.
     * @param ctx the parse tree
     */
    fn exit_aggregateFunction(&mut self, _ctx: &AggregateFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalSetFunction}.
     * @param ctx the parse tree
     */
    fn enter_generalSetFunction(&mut self, _ctx: &GeneralSetFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalSetFunction}.
     * @param ctx the parse tree
     */
    fn exit_generalSetFunction(&mut self, _ctx: &GeneralSetFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#binarySetFunction}.
     * @param ctx the parse tree
     */
    fn enter_binarySetFunction(&mut self, _ctx: &BinarySetFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#binarySetFunction}.
     * @param ctx the parse tree
     */
    fn exit_binarySetFunction(&mut self, _ctx: &BinarySetFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalSetFunctionType}.
     * @param ctx the parse tree
     */
    fn enter_generalSetFunctionType(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalSetFunctionType}.
     * @param ctx the parse tree
     */
    fn exit_generalSetFunctionType(&mut self, _ctx: &GeneralSetFunctionTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#setQuantifier}.
     * @param ctx the parse tree
     */
    fn enter_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#setQuantifier}.
     * @param ctx the parse tree
     */
    fn exit_setQuantifier(&mut self, _ctx: &SetQuantifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#binarySetFunctionType}.
     * @param ctx the parse tree
     */
    fn enter_binarySetFunctionType(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#binarySetFunctionType}.
     * @param ctx the parse tree
     */
    fn exit_binarySetFunctionType(&mut self, _ctx: &BinarySetFunctionTypeContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dependentValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_dependentValueExpression(&mut self, _ctx: &DependentValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dependentValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_dependentValueExpression(&mut self, _ctx: &DependentValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#independentValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_independentValueExpression(
        &mut self,
        _ctx: &IndependentValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#independentValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_independentValueExpression(
        &mut self,
        _ctx: &IndependentValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#element_idFunction}.
     * @param ctx the parse tree
     */
    fn enter_element_idFunction(&mut self, _ctx: &Element_idFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#element_idFunction}.
     * @param ctx the parse tree
     */
    fn exit_element_idFunction(&mut self, _ctx: &Element_idFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingVariableReference}.
     * @param ctx the parse tree
     */
    fn enter_bindingVariableReference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingVariableReference}.
     * @param ctx the parse tree
     */
    fn exit_bindingVariableReference(&mut self, _ctx: &BindingVariableReferenceContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_pathValueExpression(&mut self, _ctx: &PathValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_pathValueExpression(&mut self, _ctx: &PathValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathValueConstructor}.
     * @param ctx the parse tree
     */
    fn enter_pathValueConstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathValueConstructor}.
     * @param ctx the parse tree
     */
    fn exit_pathValueConstructor(&mut self, _ctx: &PathValueConstructorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn enter_pathValueConstructorByEnumeration(
        &mut self,
        _ctx: &PathValueConstructorByEnumerationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#pathValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn exit_pathValueConstructorByEnumeration(
        &mut self,
        _ctx: &PathValueConstructorByEnumerationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#pathElementList}.
     * @param ctx the parse tree
     */
    fn enter_pathElementList(&mut self, _ctx: &PathElementListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathElementList}.
     * @param ctx the parse tree
     */
    fn exit_pathElementList(&mut self, _ctx: &PathElementListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathElementListStart}.
     * @param ctx the parse tree
     */
    fn enter_pathElementListStart(&mut self, _ctx: &PathElementListStartContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathElementListStart}.
     * @param ctx the parse tree
     */
    fn exit_pathElementListStart(&mut self, _ctx: &PathElementListStartContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathElementListStep}.
     * @param ctx the parse tree
     */
    fn enter_pathElementListStep(&mut self, _ctx: &PathElementListStepContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathElementListStep}.
     * @param ctx the parse tree
     */
    fn exit_pathElementListStep(&mut self, _ctx: &PathElementListStepContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_listValueExpression(&mut self, _ctx: &ListValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_listValueExpression(&mut self, _ctx: &ListValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueFunction}.
     * @param ctx the parse tree
     */
    fn enter_listValueFunction(&mut self, _ctx: &ListValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueFunction}.
     * @param ctx the parse tree
     */
    fn exit_listValueFunction(&mut self, _ctx: &ListValueFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimListFunction}.
     * @param ctx the parse tree
     */
    fn enter_trimListFunction(&mut self, _ctx: &TrimListFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#trimListFunction}.
     * @param ctx the parse tree
     */
    fn exit_trimListFunction(&mut self, _ctx: &TrimListFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementsFunction}.
     * @param ctx the parse tree
     */
    fn enter_elementsFunction(&mut self, _ctx: &ElementsFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementsFunction}.
     * @param ctx the parse tree
     */
    fn exit_elementsFunction(&mut self, _ctx: &ElementsFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueConstructor}.
     * @param ctx the parse tree
     */
    fn enter_listValueConstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueConstructor}.
     * @param ctx the parse tree
     */
    fn exit_listValueConstructor(&mut self, _ctx: &ListValueConstructorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn enter_listValueConstructorByEnumeration(
        &mut self,
        _ctx: &ListValueConstructorByEnumerationContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#listValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn exit_listValueConstructorByEnumeration(
        &mut self,
        _ctx: &ListValueConstructorByEnumerationContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#listElementList}.
     * @param ctx the parse tree
     */
    fn enter_listElementList(&mut self, _ctx: &ListElementListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listElementList}.
     * @param ctx the parse tree
     */
    fn exit_listElementList(&mut self, _ctx: &ListElementListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listElement}.
     * @param ctx the parse tree
     */
    fn enter_listElement(&mut self, _ctx: &ListElementContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listElement}.
     * @param ctx the parse tree
     */
    fn exit_listElement(&mut self, _ctx: &ListElementContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#recordConstructor}.
     * @param ctx the parse tree
     */
    fn enter_recordConstructor(&mut self, _ctx: &RecordConstructorContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#recordConstructor}.
     * @param ctx the parse tree
     */
    fn exit_recordConstructor(&mut self, _ctx: &RecordConstructorContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldsSpecification}.
     * @param ctx the parse tree
     */
    fn enter_fieldsSpecification(&mut self, _ctx: &FieldsSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldsSpecification}.
     * @param ctx the parse tree
     */
    fn exit_fieldsSpecification(&mut self, _ctx: &FieldsSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldList}.
     * @param ctx the parse tree
     */
    fn enter_fieldList(&mut self, _ctx: &FieldListContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldList}.
     * @param ctx the parse tree
     */
    fn exit_fieldList(&mut self, _ctx: &FieldListContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#field}.
     * @param ctx the parse tree
     */
    fn enter_field(&mut self, _ctx: &FieldContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#field}.
     * @param ctx the parse tree
     */
    fn exit_field(&mut self, _ctx: &FieldContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#truthValue}.
     * @param ctx the parse tree
     */
    fn enter_truthValue(&mut self, _ctx: &TruthValueContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#truthValue}.
     * @param ctx the parse tree
     */
    fn exit_truthValue(&mut self, _ctx: &TruthValueContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_numericValueExpression(&mut self, _ctx: &NumericValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_numericValueExpression(&mut self, _ctx: &NumericValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueFunction}.
     * @param ctx the parse tree
     */
    fn enter_numericValueFunction(&mut self, _ctx: &NumericValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueFunction}.
     * @param ctx the parse tree
     */
    fn exit_numericValueFunction(&mut self, _ctx: &NumericValueFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#lengthExpression}.
     * @param ctx the parse tree
     */
    fn enter_lengthExpression(&mut self, _ctx: &LengthExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#lengthExpression}.
     * @param ctx the parse tree
     */
    fn exit_lengthExpression(&mut self, _ctx: &LengthExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#cardinalityExpression}.
     * @param ctx the parse tree
     */
    fn enter_cardinalityExpression(&mut self, _ctx: &CardinalityExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#cardinalityExpression}.
     * @param ctx the parse tree
     */
    fn exit_cardinalityExpression(&mut self, _ctx: &CardinalityExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#cardinalityExpressionArgument}.
     * @param ctx the parse tree
     */
    fn enter_cardinalityExpressionArgument(
        &mut self,
        _ctx: &CardinalityExpressionArgumentContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#cardinalityExpressionArgument}.
     * @param ctx the parse tree
     */
    fn exit_cardinalityExpressionArgument(
        &mut self,
        _ctx: &CardinalityExpressionArgumentContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#charLengthExpression}.
     * @param ctx the parse tree
     */
    fn enter_charLengthExpression(&mut self, _ctx: &CharLengthExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#charLengthExpression}.
     * @param ctx the parse tree
     */
    fn exit_charLengthExpression(&mut self, _ctx: &CharLengthExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#byteLengthExpression}.
     * @param ctx the parse tree
     */
    fn enter_byteLengthExpression(&mut self, _ctx: &ByteLengthExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#byteLengthExpression}.
     * @param ctx the parse tree
     */
    fn exit_byteLengthExpression(&mut self, _ctx: &ByteLengthExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathLengthExpression}.
     * @param ctx the parse tree
     */
    fn enter_pathLengthExpression(&mut self, _ctx: &PathLengthExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathLengthExpression}.
     * @param ctx the parse tree
     */
    fn exit_pathLengthExpression(&mut self, _ctx: &PathLengthExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#absoluteValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_absoluteValueExpression(&mut self, _ctx: &AbsoluteValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#absoluteValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_absoluteValueExpression(&mut self, _ctx: &AbsoluteValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#modulusExpression}.
     * @param ctx the parse tree
     */
    fn enter_modulusExpression(&mut self, _ctx: &ModulusExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#modulusExpression}.
     * @param ctx the parse tree
     */
    fn exit_modulusExpression(&mut self, _ctx: &ModulusExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueExpressionDividend}.
     * @param ctx the parse tree
     */
    fn enter_numericValueExpressionDividend(
        &mut self,
        _ctx: &NumericValueExpressionDividendContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueExpressionDividend}.
     * @param ctx the parse tree
     */
    fn exit_numericValueExpressionDividend(
        &mut self,
        _ctx: &NumericValueExpressionDividendContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueExpressionDivisor}.
     * @param ctx the parse tree
     */
    fn enter_numericValueExpressionDivisor(
        &mut self,
        _ctx: &NumericValueExpressionDivisorContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueExpressionDivisor}.
     * @param ctx the parse tree
     */
    fn exit_numericValueExpressionDivisor(
        &mut self,
        _ctx: &NumericValueExpressionDivisorContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#trigonometricFunction}.
     * @param ctx the parse tree
     */
    fn enter_trigonometricFunction(&mut self, _ctx: &TrigonometricFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#trigonometricFunction}.
     * @param ctx the parse tree
     */
    fn exit_trigonometricFunction(&mut self, _ctx: &TrigonometricFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trigonometricFunctionName}.
     * @param ctx the parse tree
     */
    fn enter_trigonometricFunctionName(&mut self, _ctx: &TrigonometricFunctionNameContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#trigonometricFunctionName}.
     * @param ctx the parse tree
     */
    fn exit_trigonometricFunctionName(&mut self, _ctx: &TrigonometricFunctionNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalLogarithmFunction}.
     * @param ctx the parse tree
     */
    fn enter_generalLogarithmFunction(&mut self, _ctx: &GeneralLogarithmFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalLogarithmFunction}.
     * @param ctx the parse tree
     */
    fn exit_generalLogarithmFunction(&mut self, _ctx: &GeneralLogarithmFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalLogarithmBase}.
     * @param ctx the parse tree
     */
    fn enter_generalLogarithmBase(&mut self, _ctx: &GeneralLogarithmBaseContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalLogarithmBase}.
     * @param ctx the parse tree
     */
    fn exit_generalLogarithmBase(&mut self, _ctx: &GeneralLogarithmBaseContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalLogarithmArgument}.
     * @param ctx the parse tree
     */
    fn enter_generalLogarithmArgument(&mut self, _ctx: &GeneralLogarithmArgumentContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalLogarithmArgument}.
     * @param ctx the parse tree
     */
    fn exit_generalLogarithmArgument(&mut self, _ctx: &GeneralLogarithmArgumentContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#commonLogarithm}.
     * @param ctx the parse tree
     */
    fn enter_commonLogarithm(&mut self, _ctx: &CommonLogarithmContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#commonLogarithm}.
     * @param ctx the parse tree
     */
    fn exit_commonLogarithm(&mut self, _ctx: &CommonLogarithmContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#naturalLogarithm}.
     * @param ctx the parse tree
     */
    fn enter_naturalLogarithm(&mut self, _ctx: &NaturalLogarithmContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#naturalLogarithm}.
     * @param ctx the parse tree
     */
    fn exit_naturalLogarithm(&mut self, _ctx: &NaturalLogarithmContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#exponentialFunction}.
     * @param ctx the parse tree
     */
    fn enter_exponentialFunction(&mut self, _ctx: &ExponentialFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#exponentialFunction}.
     * @param ctx the parse tree
     */
    fn exit_exponentialFunction(&mut self, _ctx: &ExponentialFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#powerFunction}.
     * @param ctx the parse tree
     */
    fn enter_powerFunction(&mut self, _ctx: &PowerFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#powerFunction}.
     * @param ctx the parse tree
     */
    fn exit_powerFunction(&mut self, _ctx: &PowerFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueExpressionBase}.
     * @param ctx the parse tree
     */
    fn enter_numericValueExpressionBase(
        &mut self,
        _ctx: &NumericValueExpressionBaseContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueExpressionBase}.
     * @param ctx the parse tree
     */
    fn exit_numericValueExpressionBase(
        &mut self,
        _ctx: &NumericValueExpressionBaseContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#numericValueExpressionExponent}.
     * @param ctx the parse tree
     */
    fn enter_numericValueExpressionExponent(
        &mut self,
        _ctx: &NumericValueExpressionExponentContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#numericValueExpressionExponent}.
     * @param ctx the parse tree
     */
    fn exit_numericValueExpressionExponent(
        &mut self,
        _ctx: &NumericValueExpressionExponentContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#squareRoot}.
     * @param ctx the parse tree
     */
    fn enter_squareRoot(&mut self, _ctx: &SquareRootContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#squareRoot}.
     * @param ctx the parse tree
     */
    fn exit_squareRoot(&mut self, _ctx: &SquareRootContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#floorFunction}.
     * @param ctx the parse tree
     */
    fn enter_floorFunction(&mut self, _ctx: &FloorFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#floorFunction}.
     * @param ctx the parse tree
     */
    fn exit_floorFunction(&mut self, _ctx: &FloorFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#ceilingFunction}.
     * @param ctx the parse tree
     */
    fn enter_ceilingFunction(&mut self, _ctx: &CeilingFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#ceilingFunction}.
     * @param ctx the parse tree
     */
    fn exit_ceilingFunction(&mut self, _ctx: &CeilingFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#characterStringValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_characterStringValueExpression(
        &mut self,
        _ctx: &CharacterStringValueExpressionContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#characterStringValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_characterStringValueExpression(
        &mut self,
        _ctx: &CharacterStringValueExpressionContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#byteStringValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_byteStringValueExpression(&mut self, _ctx: &ByteStringValueExpressionContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#byteStringValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_byteStringValueExpression(&mut self, _ctx: &ByteStringValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimOperands}.
     * @param ctx the parse tree
     */
    fn enter_trimOperands(&mut self, _ctx: &TrimOperandsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#trimOperands}.
     * @param ctx the parse tree
     */
    fn exit_trimOperands(&mut self, _ctx: &TrimOperandsContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimCharacterOrByteStringSource}.
     * @param ctx the parse tree
     */
    fn enter_trimCharacterOrByteStringSource(
        &mut self,
        _ctx: &TrimCharacterOrByteStringSourceContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#trimCharacterOrByteStringSource}.
     * @param ctx the parse tree
     */
    fn exit_trimCharacterOrByteStringSource(
        &mut self,
        _ctx: &TrimCharacterOrByteStringSourceContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#trimSpecification}.
     * @param ctx the parse tree
     */
    fn enter_trimSpecification(&mut self, _ctx: &TrimSpecificationContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#trimSpecification}.
     * @param ctx the parse tree
     */
    fn exit_trimSpecification(&mut self, _ctx: &TrimSpecificationContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#trimCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn enter_trimCharacterOrByteString(&mut self, _ctx: &TrimCharacterOrByteStringContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#trimCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn exit_trimCharacterOrByteString(&mut self, _ctx: &TrimCharacterOrByteStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#normalForm}.
     * @param ctx the parse tree
     */
    fn enter_normalForm(&mut self, _ctx: &NormalFormContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#normalForm}.
     * @param ctx the parse tree
     */
    fn exit_normalForm(&mut self, _ctx: &NormalFormContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#stringLength}.
     * @param ctx the parse tree
     */
    fn enter_stringLength(&mut self, _ctx: &StringLengthContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#stringLength}.
     * @param ctx the parse tree
     */
    fn exit_stringLength(&mut self, _ctx: &StringLengthContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_datetimeValueExpression(&mut self, _ctx: &DatetimeValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_datetimeValueExpression(&mut self, _ctx: &DatetimeValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeValueFunction}.
     * @param ctx the parse tree
     */
    fn enter_datetimeValueFunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeValueFunction}.
     * @param ctx the parse tree
     */
    fn exit_datetimeValueFunction(&mut self, _ctx: &DatetimeValueFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dateFunction}.
     * @param ctx the parse tree
     */
    fn enter_dateFunction(&mut self, _ctx: &DateFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dateFunction}.
     * @param ctx the parse tree
     */
    fn exit_dateFunction(&mut self, _ctx: &DateFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeFunction}.
     * @param ctx the parse tree
     */
    fn enter_timeFunction(&mut self, _ctx: &TimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeFunction}.
     * @param ctx the parse tree
     */
    fn exit_timeFunction(&mut self, _ctx: &TimeFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#localtimeFunction}.
     * @param ctx the parse tree
     */
    fn enter_localtimeFunction(&mut self, _ctx: &LocaltimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#localtimeFunction}.
     * @param ctx the parse tree
     */
    fn exit_localtimeFunction(&mut self, _ctx: &LocaltimeFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeFunction}.
     * @param ctx the parse tree
     */
    fn enter_datetimeFunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeFunction}.
     * @param ctx the parse tree
     */
    fn exit_datetimeFunction(&mut self, _ctx: &DatetimeFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#localdatetimeFunction}.
     * @param ctx the parse tree
     */
    fn enter_localdatetimeFunction(&mut self, _ctx: &LocaldatetimeFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#localdatetimeFunction}.
     * @param ctx the parse tree
     */
    fn exit_localdatetimeFunction(&mut self, _ctx: &LocaldatetimeFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dateFunctionParameters}.
     * @param ctx the parse tree
     */
    fn enter_dateFunctionParameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dateFunctionParameters}.
     * @param ctx the parse tree
     */
    fn exit_dateFunctionParameters(&mut self, _ctx: &DateFunctionParametersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn enter_timeFunctionParameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn exit_timeFunctionParameters(&mut self, _ctx: &TimeFunctionParametersContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn enter_datetimeFunctionParameters(
        &mut self,
        _ctx: &DatetimeFunctionParametersContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn exit_datetimeFunctionParameters(
        &mut self,
        _ctx: &DatetimeFunctionParametersContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#durationValueExpression}.
     * @param ctx the parse tree
     */
    fn enter_durationValueExpression(&mut self, _ctx: &DurationValueExpressionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#durationValueExpression}.
     * @param ctx the parse tree
     */
    fn exit_durationValueExpression(&mut self, _ctx: &DurationValueExpressionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeSubtraction}.
     * @param ctx the parse tree
     */
    fn enter_datetimeSubtraction(&mut self, _ctx: &DatetimeSubtractionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeSubtraction}.
     * @param ctx the parse tree
     */
    fn exit_datetimeSubtraction(&mut self, _ctx: &DatetimeSubtractionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeSubtractionParameters}.
     * @param ctx the parse tree
     */
    fn enter_datetimeSubtractionParameters(
        &mut self,
        _ctx: &DatetimeSubtractionParametersContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeSubtractionParameters}.
     * @param ctx the parse tree
     */
    fn exit_datetimeSubtractionParameters(
        &mut self,
        _ctx: &DatetimeSubtractionParametersContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeValueExpression1}.
     * @param ctx the parse tree
     */
    fn enter_datetimeValueExpression1(&mut self, _ctx: &DatetimeValueExpression1Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeValueExpression1}.
     * @param ctx the parse tree
     */
    fn exit_datetimeValueExpression1(&mut self, _ctx: &DatetimeValueExpression1Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeValueExpression2}.
     * @param ctx the parse tree
     */
    fn enter_datetimeValueExpression2(&mut self, _ctx: &DatetimeValueExpression2Context<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeValueExpression2}.
     * @param ctx the parse tree
     */
    fn exit_datetimeValueExpression2(&mut self, _ctx: &DatetimeValueExpression2Context<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#durationValueFunction}.
     * @param ctx the parse tree
     */
    fn enter_durationValueFunction(&mut self, _ctx: &DurationValueFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#durationValueFunction}.
     * @param ctx the parse tree
     */
    fn exit_durationValueFunction(&mut self, _ctx: &DurationValueFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#durationFunction}.
     * @param ctx the parse tree
     */
    fn enter_durationFunction(&mut self, _ctx: &DurationFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#durationFunction}.
     * @param ctx the parse tree
     */
    fn exit_durationFunction(&mut self, _ctx: &DurationFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#durationFunctionParameters}.
     * @param ctx the parse tree
     */
    fn enter_durationFunctionParameters(
        &mut self,
        _ctx: &DurationFunctionParametersContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#durationFunctionParameters}.
     * @param ctx the parse tree
     */
    fn exit_durationFunctionParameters(
        &mut self,
        _ctx: &DurationFunctionParametersContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#objectName}.
     * @param ctx the parse tree
     */
    fn enter_objectName(&mut self, _ctx: &ObjectNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#objectName}.
     * @param ctx the parse tree
     */
    fn exit_objectName(&mut self, _ctx: &ObjectNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
     * @param ctx the parse tree
     */
    fn enter_objectNameOrBindingVariable(
        &mut self,
        _ctx: &ObjectNameOrBindingVariableContext<'input>,
    ) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
     * @param ctx the parse tree
     */
    fn exit_objectNameOrBindingVariable(
        &mut self,
        _ctx: &ObjectNameOrBindingVariableContext<'input>,
    ) {
    }
    /**
     * Enter a parse tree produced by {@link GQLParser#directoryName}.
     * @param ctx the parse tree
     */
    fn enter_directoryName(&mut self, _ctx: &DirectoryNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#directoryName}.
     * @param ctx the parse tree
     */
    fn exit_directoryName(&mut self, _ctx: &DirectoryNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#schemaName}.
     * @param ctx the parse tree
     */
    fn enter_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#schemaName}.
     * @param ctx the parse tree
     */
    fn exit_schemaName(&mut self, _ctx: &SchemaNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphName}.
     * @param ctx the parse tree
     */
    fn enter_graphName(&mut self, _ctx: &GraphNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphName}.
     * @param ctx the parse tree
     */
    fn exit_graphName(&mut self, _ctx: &GraphNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#delimitedGraphName}.
     * @param ctx the parse tree
     */
    fn enter_delimitedGraphName(&mut self, _ctx: &DelimitedGraphNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#delimitedGraphName}.
     * @param ctx the parse tree
     */
    fn exit_delimitedGraphName(&mut self, _ctx: &DelimitedGraphNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#graphTypeName}.
     * @param ctx the parse tree
     */
    fn enter_graphTypeName(&mut self, _ctx: &GraphTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#graphTypeName}.
     * @param ctx the parse tree
     */
    fn exit_graphTypeName(&mut self, _ctx: &GraphTypeNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeTypeName}.
     * @param ctx the parse tree
     */
    fn enter_nodeTypeName(&mut self, _ctx: &NodeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeTypeName}.
     * @param ctx the parse tree
     */
    fn exit_nodeTypeName(&mut self, _ctx: &NodeTypeNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeTypeName}.
     * @param ctx the parse tree
     */
    fn enter_edgeTypeName(&mut self, _ctx: &EdgeTypeNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeTypeName}.
     * @param ctx the parse tree
     */
    fn exit_edgeTypeName(&mut self, _ctx: &EdgeTypeNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingTableName}.
     * @param ctx the parse tree
     */
    fn enter_bindingTableName(&mut self, _ctx: &BindingTableNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingTableName}.
     * @param ctx the parse tree
     */
    fn exit_bindingTableName(&mut self, _ctx: &BindingTableNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
     * @param ctx the parse tree
     */
    fn enter_delimitedBindingTableName(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
     * @param ctx the parse tree
     */
    fn exit_delimitedBindingTableName(&mut self, _ctx: &DelimitedBindingTableNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#procedureName}.
     * @param ctx the parse tree
     */
    fn enter_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#procedureName}.
     * @param ctx the parse tree
     */
    fn exit_procedureName(&mut self, _ctx: &ProcedureNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#labelName}.
     * @param ctx the parse tree
     */
    fn enter_labelName(&mut self, _ctx: &LabelNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#labelName}.
     * @param ctx the parse tree
     */
    fn exit_labelName(&mut self, _ctx: &LabelNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#propertyName}.
     * @param ctx the parse tree
     */
    fn enter_propertyName(&mut self, _ctx: &PropertyNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#propertyName}.
     * @param ctx the parse tree
     */
    fn exit_propertyName(&mut self, _ctx: &PropertyNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#fieldName}.
     * @param ctx the parse tree
     */
    fn enter_fieldName(&mut self, _ctx: &FieldNameContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#fieldName}.
     * @param ctx the parse tree
     */
    fn exit_fieldName(&mut self, _ctx: &FieldNameContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#elementVariable}.
     * @param ctx the parse tree
     */
    fn enter_elementVariable(&mut self, _ctx: &ElementVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#elementVariable}.
     * @param ctx the parse tree
     */
    fn exit_elementVariable(&mut self, _ctx: &ElementVariableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#pathVariable}.
     * @param ctx the parse tree
     */
    fn enter_pathVariable(&mut self, _ctx: &PathVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#pathVariable}.
     * @param ctx the parse tree
     */
    fn exit_pathVariable(&mut self, _ctx: &PathVariableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#subpathVariable}.
     * @param ctx the parse tree
     */
    fn enter_subpathVariable(&mut self, _ctx: &SubpathVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#subpathVariable}.
     * @param ctx the parse tree
     */
    fn exit_subpathVariable(&mut self, _ctx: &SubpathVariableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#bindingVariable}.
     * @param ctx the parse tree
     */
    fn enter_bindingVariable(&mut self, _ctx: &BindingVariableContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#bindingVariable}.
     * @param ctx the parse tree
     */
    fn exit_bindingVariable(&mut self, _ctx: &BindingVariableContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedLiteral}.
     * @param ctx the parse tree
     */
    fn enter_unsignedLiteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedLiteral}.
     * @param ctx the parse tree
     */
    fn exit_unsignedLiteral(&mut self, _ctx: &UnsignedLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#generalLiteral}.
     * @param ctx the parse tree
     */
    fn enter_generalLiteral(&mut self, _ctx: &GeneralLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#generalLiteral}.
     * @param ctx the parse tree
     */
    fn exit_generalLiteral(&mut self, _ctx: &GeneralLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#temporalLiteral}.
     * @param ctx the parse tree
     */
    fn enter_temporalLiteral(&mut self, _ctx: &TemporalLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#temporalLiteral}.
     * @param ctx the parse tree
     */
    fn exit_temporalLiteral(&mut self, _ctx: &TemporalLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dateLiteral}.
     * @param ctx the parse tree
     */
    fn enter_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dateLiteral}.
     * @param ctx the parse tree
     */
    fn exit_dateLiteral(&mut self, _ctx: &DateLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeLiteral}.
     * @param ctx the parse tree
     */
    fn enter_timeLiteral(&mut self, _ctx: &TimeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeLiteral}.
     * @param ctx the parse tree
     */
    fn exit_timeLiteral(&mut self, _ctx: &TimeLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeLiteral}.
     * @param ctx the parse tree
     */
    fn enter_datetimeLiteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeLiteral}.
     * @param ctx the parse tree
     */
    fn exit_datetimeLiteral(&mut self, _ctx: &DatetimeLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#listLiteral}.
     * @param ctx the parse tree
     */
    fn enter_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#listLiteral}.
     * @param ctx the parse tree
     */
    fn exit_listLiteral(&mut self, _ctx: &ListLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#recordLiteral}.
     * @param ctx the parse tree
     */
    fn enter_recordLiteral(&mut self, _ctx: &RecordLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#recordLiteral}.
     * @param ctx the parse tree
     */
    fn exit_recordLiteral(&mut self, _ctx: &RecordLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#identifier}.
     * @param ctx the parse tree
     */
    fn enter_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#identifier}.
     * @param ctx the parse tree
     */
    fn exit_identifier(&mut self, _ctx: &IdentifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#regularIdentifier}.
     * @param ctx the parse tree
     */
    fn enter_regularIdentifier(&mut self, _ctx: &RegularIdentifierContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#regularIdentifier}.
     * @param ctx the parse tree
     */
    fn exit_regularIdentifier(&mut self, _ctx: &RegularIdentifierContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeZoneString}.
     * @param ctx the parse tree
     */
    fn enter_timeZoneString(&mut self, _ctx: &TimeZoneStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeZoneString}.
     * @param ctx the parse tree
     */
    fn exit_timeZoneString(&mut self, _ctx: &TimeZoneStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#characterStringLiteral}.
     * @param ctx the parse tree
     */
    fn enter_characterStringLiteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#characterStringLiteral}.
     * @param ctx the parse tree
     */
    fn exit_characterStringLiteral(&mut self, _ctx: &CharacterStringLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
     * @param ctx the parse tree
     */
    fn enter_unsignedNumericLiteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
     * @param ctx the parse tree
     */
    fn exit_unsignedNumericLiteral(&mut self, _ctx: &UnsignedNumericLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#exactNumericLiteral}.
     * @param ctx the parse tree
     */
    fn enter_exactNumericLiteral(&mut self, _ctx: &ExactNumericLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#exactNumericLiteral}.
     * @param ctx the parse tree
     */
    fn exit_exactNumericLiteral(&mut self, _ctx: &ExactNumericLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#approximateNumericLiteral}.
     * @param ctx the parse tree
     */
    fn enter_approximateNumericLiteral(&mut self, _ctx: &ApproximateNumericLiteralContext<'input>) {
    }
    /**
     * Exit a parse tree produced by {@link GQLParser#approximateNumericLiteral}.
     * @param ctx the parse tree
     */
    fn exit_approximateNumericLiteral(&mut self, _ctx: &ApproximateNumericLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedInteger}.
     * @param ctx the parse tree
     */
    fn enter_unsignedInteger(&mut self, _ctx: &UnsignedIntegerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedInteger}.
     * @param ctx the parse tree
     */
    fn exit_unsignedInteger(&mut self, _ctx: &UnsignedIntegerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#unsignedDecimalInteger}.
     * @param ctx the parse tree
     */
    fn enter_unsignedDecimalInteger(&mut self, _ctx: &UnsignedDecimalIntegerContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#unsignedDecimalInteger}.
     * @param ctx the parse tree
     */
    fn exit_unsignedDecimalInteger(&mut self, _ctx: &UnsignedDecimalIntegerContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nullLiteral}.
     * @param ctx the parse tree
     */
    fn enter_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nullLiteral}.
     * @param ctx the parse tree
     */
    fn exit_nullLiteral(&mut self, _ctx: &NullLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#dateString}.
     * @param ctx the parse tree
     */
    fn enter_dateString(&mut self, _ctx: &DateStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#dateString}.
     * @param ctx the parse tree
     */
    fn exit_dateString(&mut self, _ctx: &DateStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#timeString}.
     * @param ctx the parse tree
     */
    fn enter_timeString(&mut self, _ctx: &TimeStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#timeString}.
     * @param ctx the parse tree
     */
    fn exit_timeString(&mut self, _ctx: &TimeStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#datetimeString}.
     * @param ctx the parse tree
     */
    fn enter_datetimeString(&mut self, _ctx: &DatetimeStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#datetimeString}.
     * @param ctx the parse tree
     */
    fn exit_datetimeString(&mut self, _ctx: &DatetimeStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#durationLiteral}.
     * @param ctx the parse tree
     */
    fn enter_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#durationLiteral}.
     * @param ctx the parse tree
     */
    fn exit_durationLiteral(&mut self, _ctx: &DurationLiteralContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#durationString}.
     * @param ctx the parse tree
     */
    fn enter_durationString(&mut self, _ctx: &DurationStringContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#durationString}.
     * @param ctx the parse tree
     */
    fn exit_durationString(&mut self, _ctx: &DurationStringContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nodeSynonym}.
     * @param ctx the parse tree
     */
    fn enter_nodeSynonym(&mut self, _ctx: &NodeSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nodeSynonym}.
     * @param ctx the parse tree
     */
    fn exit_nodeSynonym(&mut self, _ctx: &NodeSynonymContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgesSynonym}.
     * @param ctx the parse tree
     */
    fn enter_edgesSynonym(&mut self, _ctx: &EdgesSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgesSynonym}.
     * @param ctx the parse tree
     */
    fn exit_edgesSynonym(&mut self, _ctx: &EdgesSynonymContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#edgeSynonym}.
     * @param ctx the parse tree
     */
    fn enter_edgeSynonym(&mut self, _ctx: &EdgeSynonymContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#edgeSynonym}.
     * @param ctx the parse tree
     */
    fn exit_edgeSynonym(&mut self, _ctx: &EdgeSynonymContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#userDefinedFunction}.
     * @param ctx the parse tree
     */
    fn enter_userDefinedFunction(&mut self, _ctx: &UserDefinedFunctionContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#userDefinedFunction}.
     * @param ctx the parse tree
     */
    fn exit_userDefinedFunction(&mut self, _ctx: &UserDefinedFunctionContext<'input>) {}
    /**
     * Enter a parse tree produced by {@link GQLParser#nonReservedWords}.
     * @param ctx the parse tree
     */
    fn enter_nonReservedWords(&mut self, _ctx: &NonReservedWordsContext<'input>) {}
    /**
     * Exit a parse tree produced by {@link GQLParser#nonReservedWords}.
     * @param ctx the parse tree
     */
    fn exit_nonReservedWords(&mut self, _ctx: &NonReservedWordsContext<'input>) {}
}

antlr_rust::coerce_from! { 'input : GQLListener<'input> }
