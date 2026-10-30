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

#![allow(nonstandard_style)]
// Generated from GQL.g4 by ANTLR 4.8
use super::gqlparser::*;
use antlr_rust::tree::{ParseTreeVisitor, ParseTreeVisitorCompat};

/**
 * This interface defines a complete generic visitor for a parse tree produced
 * by {@link GQLParser}.
 */
pub trait GQLVisitor<'input>: ParseTreeVisitor<'input, GQLParserContextType> {
    /**
     * Visit a parse tree produced by {@link GQLParser#gqlProgram}.
     * @param ctx the parse tree
     */
    fn visit_gqlProgram(&mut self, ctx: &GqlProgramContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#programActivity}.
     * @param ctx the parse tree
     */
    fn visit_programActivity(&mut self, ctx: &ProgramActivityContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionActivity}.
     * @param ctx the parse tree
     */
    fn visit_sessionActivity(&mut self, ctx: &SessionActivityContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionActivity}.
     * @param ctx the parse tree
     */
    fn visit_transactionActivity(&mut self, ctx: &TransactionActivityContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endTransactionCommand}.
     * @param ctx the parse tree
     */
    fn visit_endTransactionCommand(&mut self, ctx: &EndTransactionCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetCommand(&mut self, ctx: &SessionSetCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetSchemaClause(&mut self, ctx: &SessionSetSchemaClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetGraphClause(&mut self, ctx: &SessionSetGraphClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetTimeZoneClause(&mut self, ctx: &SessionSetTimeZoneClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setTimeZoneValue}.
     * @param ctx the parse tree
     */
    fn visit_setTimeZoneValue(&mut self, ctx: &SetTimeZoneValueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetParameterClause(&mut self, ctx: &SessionSetParameterClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetGraphParameterClause(
        &mut self,
        ctx: &SessionSetGraphParameterClauseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetBindingTableParameterClause(
        &mut self,
        ctx: &SessionSetBindingTableParameterClauseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetValueParameterClause(
        &mut self,
        ctx: &SessionSetValueParameterClauseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetParameterName}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetParameterName(&mut self, ctx: &SessionSetParameterNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionResetCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionResetCommand(&mut self, ctx: &SessionResetCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionResetArguments}.
     * @param ctx the parse tree
     */
    fn visit_sessionResetArguments(&mut self, ctx: &SessionResetArgumentsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionCloseCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionCloseCommand(&mut self, ctx: &SessionCloseCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_sessionParameterSpecification(
        &mut self,
        ctx: &SessionParameterSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#startTransactionCommand}.
     * @param ctx the parse tree
     */
    fn visit_startTransactionCommand(&mut self, ctx: &StartTransactionCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionCharacteristics}.
     * @param ctx the parse tree
     */
    fn visit_transactionCharacteristics(
        &mut self,
        ctx: &TransactionCharacteristicsContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionMode}.
     * @param ctx the parse tree
     */
    fn visit_transactionMode(&mut self, ctx: &TransactionModeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionAccessMode}.
     * @param ctx the parse tree
     */
    fn visit_transactionAccessMode(&mut self, ctx: &TransactionAccessModeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#rollbackCommand}.
     * @param ctx the parse tree
     */
    fn visit_rollbackCommand(&mut self, ctx: &RollbackCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#commitCommand}.
     * @param ctx the parse tree
     */
    fn visit_commitCommand(&mut self, ctx: &CommitCommandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedProcedureSpecification(
        &mut self,
        ctx: &NestedProcedureSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_procedureSpecification(&mut self, ctx: &ProcedureSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &NestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedQuerySpecification(&mut self, ctx: &NestedQuerySpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureBody}.
     * @param ctx the parse tree
     */
    fn visit_procedureBody(&mut self, ctx: &ProcedureBodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableDefinitionBlock(
        &mut self,
        ctx: &BindingVariableDefinitionBlockContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableDefinition(&mut self, ctx: &BindingVariableDefinitionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#statementBlock}.
     * @param ctx the parse tree
     */
    fn visit_statementBlock(&mut self, ctx: &StatementBlockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nextStatement}.
     * @param ctx the parse tree
     */
    fn visit_nextStatement(&mut self, ctx: &NextStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_graphVariableDefinition(&mut self, ctx: &GraphVariableDefinitionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedGraphInitializer(&mut self, ctx: &OptTypedGraphInitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphInitializer}.
     * @param ctx the parse tree
     */
    fn visit_graphInitializer(&mut self, ctx: &GraphInitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableVariableDefinition(
        &mut self,
        ctx: &BindingTableVariableDefinitionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedBindingTableInitializer(
        &mut self,
        ctx: &OptTypedBindingTableInitializerContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableInitializer(&mut self, ctx: &BindingTableInitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_valueVariableDefinition(&mut self, ctx: &ValueVariableDefinitionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedValueInitializer(&mut self, ctx: &OptTypedValueInitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueInitializer}.
     * @param ctx the parse tree
     */
    fn visit_valueInitializer(&mut self, ctx: &ValueInitializerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphExpression}.
     * @param ctx the parse tree
     */
    fn visit_graphExpression(&mut self, ctx: &GraphExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#currentGraph}.
     * @param ctx the parse tree
     */
    fn visit_currentGraph(&mut self, ctx: &CurrentGraphContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableExpression}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableExpression(&mut self, ctx: &BindingTableExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedBindingTableQuerySpecification(
        &mut self,
        ctx: &NestedBindingTableQuerySpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_objectExpressionPrimary(&mut self, ctx: &ObjectExpressionPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearCatalogModifyingStatement(
        &mut self,
        ctx: &LinearCatalogModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleCatalogModifyingStatement(
        &mut self,
        ctx: &SimpleCatalogModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveCatalogModifyingStatement(
        &mut self,
        ctx: &PrimitiveCatalogModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createSchemaStatement}.
     * @param ctx the parse tree
     */
    fn visit_createSchemaStatement(&mut self, ctx: &CreateSchemaStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropSchemaStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropSchemaStatement(&mut self, ctx: &DropSchemaStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createGraphStatement}.
     * @param ctx the parse tree
     */
    fn visit_createGraphStatement(&mut self, ctx: &CreateGraphStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openGraphType}.
     * @param ctx the parse tree
     */
    fn visit_openGraphType(&mut self, ctx: &OpenGraphTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ofGraphType}.
     * @param ctx the parse tree
     */
    fn visit_ofGraphType(&mut self, ctx: &OfGraphTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeLikeGraph(&mut self, ctx: &GraphTypeLikeGraphContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphSource}.
     * @param ctx the parse tree
     */
    fn visit_graphSource(&mut self, ctx: &GraphSourceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropGraphStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropGraphStatement(&mut self, ctx: &DropGraphStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn visit_createGraphTypeStatement(&mut self, ctx: &CreateGraphTypeStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeSource}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeSource(&mut self, ctx: &GraphTypeSourceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#copyOfGraphType}.
     * @param ctx the parse tree
     */
    fn visit_copyOfGraphType(&mut self, ctx: &CopyOfGraphTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropGraphTypeStatement(&mut self, ctx: &DropGraphTypeStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callCatalogModifyingProcedureStatement(
        &mut self,
        ctx: &CallCatalogModifyingProcedureStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearDataModifyingStatement(
        &mut self,
        ctx: &LinearDataModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearDataModifyingStatement(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearDataModifyingStatementBody(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_focusedNestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearDataModifyingStatement(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearDataModifyingStatementBody(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleLinearDataAccessingStatement(
        &mut self,
        ctx: &SimpleLinearDataAccessingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleDataAccessingStatement(
        &mut self,
        ctx: &SimpleDataAccessingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleDataModifyingStatement(
        &mut self,
        ctx: &SimpleDataModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveDataModifyingStatement(
        &mut self,
        ctx: &PrimitiveDataModifyingStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertStatement}.
     * @param ctx the parse tree
     */
    fn visit_insertStatement(&mut self, ctx: &InsertStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setStatement}.
     * @param ctx the parse tree
     */
    fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setItemList}.
     * @param ctx the parse tree
     */
    fn visit_setItemList(&mut self, ctx: &SetItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setItem}.
     * @param ctx the parse tree
     */
    fn visit_setItem(&mut self, ctx: &SetItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setPropertyItem}.
     * @param ctx the parse tree
     */
    fn visit_setPropertyItem(&mut self, ctx: &SetPropertyItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
     * @param ctx the parse tree
     */
    fn visit_setAllPropertiesItem(&mut self, ctx: &SetAllPropertiesItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setLabelItem}.
     * @param ctx the parse tree
     */
    fn visit_setLabelItem(&mut self, ctx: &SetLabelItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeStatement}.
     * @param ctx the parse tree
     */
    fn visit_removeStatement(&mut self, ctx: &RemoveStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeItemList}.
     * @param ctx the parse tree
     */
    fn visit_removeItemList(&mut self, ctx: &RemoveItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeItem}.
     * @param ctx the parse tree
     */
    fn visit_removeItem(&mut self, ctx: &RemoveItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removePropertyItem}.
     * @param ctx the parse tree
     */
    fn visit_removePropertyItem(&mut self, ctx: &RemovePropertyItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeLabelItem}.
     * @param ctx the parse tree
     */
    fn visit_removeLabelItem(&mut self, ctx: &RemoveLabelItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteStatement}.
     * @param ctx the parse tree
     */
    fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteItemList}.
     * @param ctx the parse tree
     */
    fn visit_deleteItemList(&mut self, ctx: &DeleteItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteItem}.
     * @param ctx the parse tree
     */
    fn visit_deleteItem(&mut self, ctx: &DeleteItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callDataModifyingProcedureStatement(
        &mut self,
        ctx: &CallDataModifyingProcedureStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryStatement(&mut self, ctx: &CompositeQueryStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryExpression}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryExpression(&mut self, ctx: &CompositeQueryExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#queryConjunction}.
     * @param ctx the parse tree
     */
    fn visit_queryConjunction(&mut self, ctx: &QueryConjunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setOperator}.
     * @param ctx the parse tree
     */
    fn visit_setOperator(&mut self, ctx: &SetOperatorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryPrimary(&mut self, ctx: &CompositeQueryPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearQueryStatement(&mut self, ctx: &LinearQueryStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryStatement(
        &mut self,
        ctx: &FocusedLinearQueryStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryStatementPartContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryAndPrimitiveResultStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedPrimitiveResultStatement(
        &mut self,
        ctx: &FocusedPrimitiveResultStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_focusedNestedQuerySpecification(
        &mut self,
        ctx: &FocusedNestedQuerySpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearQueryStatement(
        &mut self,
        ctx: &AmbientLinearQueryStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleLinearQueryStatement(
        &mut self,
        ctx: &SimpleLinearQueryStatementContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleQueryStatement(&mut self, ctx: &SimpleQueryStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveQueryStatement(&mut self, ctx: &PrimitiveQueryStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchStatement}.
     * @param ctx the parse tree
     */
    fn visit_matchStatement(&mut self, ctx: &MatchStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleMatchStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleMatchStatement(&mut self, ctx: &SimpleMatchStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optionalMatchStatement}.
     * @param ctx the parse tree
     */
    fn visit_optionalMatchStatement(&mut self, ctx: &OptionalMatchStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optionalOperand}.
     * @param ctx the parse tree
     */
    fn visit_optionalOperand(&mut self, ctx: &OptionalOperandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchStatementBlock}.
     * @param ctx the parse tree
     */
    fn visit_matchStatementBlock(&mut self, ctx: &MatchStatementBlockContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_callQueryStatement(&mut self, ctx: &CallQueryStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#filterStatement}.
     * @param ctx the parse tree
     */
    fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letStatement}.
     * @param ctx the parse tree
     */
    fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
     * @param ctx the parse tree
     */
    fn visit_letVariableDefinitionList(&mut self, ctx: &LetVariableDefinitionListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_letVariableDefinition(&mut self, ctx: &LetVariableDefinitionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forStatement}.
     * @param ctx the parse tree
     */
    fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItem}.
     * @param ctx the parse tree
     */
    fn visit_forItem(&mut self, ctx: &ForItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_forItemAlias(&mut self, ctx: &ForItemAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItemSource}.
     * @param ctx the parse tree
     */
    fn visit_forItemSource(&mut self, ctx: &ForItemSourceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
     * @param ctx the parse tree
     */
    fn visit_forOrdinalityOrOffset(&mut self, ctx: &ForOrdinalityOrOffsetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
     * @param ctx the parse tree
     */
    fn visit_orderByAndPageStatement(&mut self, ctx: &OrderByAndPageStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveResultStatement(&mut self, ctx: &PrimitiveResultStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnStatement}.
     * @param ctx the parse tree
     */
    fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_returnStatementBody(&mut self, ctx: &ReturnStatementBodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItemList}.
     * @param ctx the parse tree
     */
    fn visit_returnItemList(&mut self, ctx: &ReturnItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItem}.
     * @param ctx the parse tree
     */
    fn visit_returnItem(&mut self, ctx: &ReturnItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_returnItemAlias(&mut self, ctx: &ReturnItemAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectStatement}.
     * @param ctx the parse tree
     */
    fn visit_selectStatement(&mut self, ctx: &SelectStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItemList}.
     * @param ctx the parse tree
     */
    fn visit_selectItemList(&mut self, ctx: &SelectItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItem}.
     * @param ctx the parse tree
     */
    fn visit_selectItem(&mut self, ctx: &SelectItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_selectItemAlias(&mut self, ctx: &SelectItemAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#havingClause}.
     * @param ctx the parse tree
     */
    fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_selectStatementBody(&mut self, ctx: &SelectStatementBodyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectGraphMatchList}.
     * @param ctx the parse tree
     */
    fn visit_selectGraphMatchList(&mut self, ctx: &SelectGraphMatchListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectGraphMatch}.
     * @param ctx the parse tree
     */
    fn visit_selectGraphMatch(&mut self, ctx: &SelectGraphMatchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_selectQuerySpecification(&mut self, ctx: &SelectQuerySpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callProcedureStatement(&mut self, ctx: &CallProcedureStatementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureCall}.
     * @param ctx the parse tree
     */
    fn visit_procedureCall(&mut self, ctx: &ProcedureCallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#inlineProcedureCall}.
     * @param ctx the parse tree
     */
    fn visit_inlineProcedureCall(&mut self, ctx: &InlineProcedureCallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#variableScopeClause}.
     * @param ctx the parse tree
     */
    fn visit_variableScopeClause(&mut self, ctx: &VariableScopeClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableReferenceList(
        &mut self,
        ctx: &BindingVariableReferenceListContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#namedProcedureCall}.
     * @param ctx the parse tree
     */
    fn visit_namedProcedureCall(&mut self, ctx: &NamedProcedureCallContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureArgumentList}.
     * @param ctx the parse tree
     */
    fn visit_procedureArgumentList(&mut self, ctx: &ProcedureArgumentListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureArgument}.
     * @param ctx the parse tree
     */
    fn visit_procedureArgument(&mut self, ctx: &ProcedureArgumentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#atSchemaClause}.
     * @param ctx the parse tree
     */
    fn visit_atSchemaClause(&mut self, ctx: &AtSchemaClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#useGraphClause}.
     * @param ctx the parse tree
     */
    fn visit_useGraphClause(&mut self, ctx: &UseGraphClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternBindingTable(&mut self, ctx: &GraphPatternBindingTableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldClause(&mut self, ctx: &GraphPatternYieldClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldItemList(&mut self, ctx: &GraphPatternYieldItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldItem(&mut self, ctx: &GraphPatternYieldItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPattern}.
     * @param ctx the parse tree
     */
    fn visit_graphPattern(&mut self, ctx: &GraphPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchMode}.
     * @param ctx the parse tree
     */
    fn visit_matchMode(&mut self, ctx: &MatchModeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
     * @param ctx the parse tree
     */
    fn visit_repeatableElementsMatchMode(
        &mut self,
        ctx: &RepeatableElementsMatchModeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
     * @param ctx the parse tree
     */
    fn visit_differentEdgesMatchMode(&mut self, ctx: &DifferentEdgesMatchModeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
     * @param ctx the parse tree
     */
    fn visit_elementBindingsOrElements(&mut self, ctx: &ElementBindingsOrElementsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
     * @param ctx the parse tree
     */
    fn visit_edgeBindingsOrEdges(&mut self, ctx: &EdgeBindingsOrEdgesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPatternList}.
     * @param ctx the parse tree
     */
    fn visit_pathPatternList(&mut self, ctx: &PathPatternListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPattern}.
     * @param ctx the parse tree
     */
    fn visit_pathPattern(&mut self, ctx: &PathPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_pathVariableDeclaration(&mut self, ctx: &PathVariableDeclarationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#keepClause}.
     * @param ctx the parse tree
     */
    fn visit_keepClause(&mut self, ctx: &KeepClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternWhereClause(&mut self, ctx: &GraphPatternWhereClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertGraphPattern}.
     * @param ctx the parse tree
     */
    fn visit_insertGraphPattern(&mut self, ctx: &InsertGraphPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertPathPatternList}.
     * @param ctx the parse tree
     */
    fn visit_insertPathPatternList(&mut self, ctx: &InsertPathPatternListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertPathPattern}.
     * @param ctx the parse tree
     */
    fn visit_insertPathPattern(&mut self, ctx: &InsertPathPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertNodePattern}.
     * @param ctx the parse tree
     */
    fn visit_insertNodePattern(&mut self, ctx: &InsertNodePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePattern(&mut self, ctx: &InsertEdgePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePointingLeft(&mut self, ctx: &InsertEdgePointingLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePointingRight(&mut self, ctx: &InsertEdgePointingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgeUndirected(&mut self, ctx: &InsertEdgeUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
     * @param ctx the parse tree
     */
    fn visit_insertElementPatternFiller(
        &mut self,
        ctx: &InsertElementPatternFillerContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
     * @param ctx the parse tree
     */
    fn visit_labelAndPropertySetSpecification(
        &mut self,
        ctx: &LabelAndPropertySetSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPatternPrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathPatternPrefix(&mut self, ctx: &PathPatternPrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathModePrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathModePrefix(&mut self, ctx: &PathModePrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathMode}.
     * @param ctx the parse tree
     */
    fn visit_pathMode(&mut self, ctx: &PathModeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathSearchPrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathSearchPrefix(&mut self, ctx: &PathSearchPrefixContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#allPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_allPathSearch(&mut self, ctx: &AllPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathOrPaths}.
     * @param ctx the parse tree
     */
    fn visit_pathOrPaths(&mut self, ctx: &PathOrPathsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#anyPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_anyPathSearch(&mut self, ctx: &AnyPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numberOfPaths}.
     * @param ctx the parse tree
     */
    fn visit_numberOfPaths(&mut self, ctx: &NumberOfPathsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#shortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_shortestPathSearch(&mut self, ctx: &ShortestPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#allShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_allShortestPathSearch(&mut self, ctx: &AllShortestPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_anyShortestPathSearch(&mut self, ctx: &AnyShortestPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_countedShortestPathSearch(&mut self, ctx: &CountedShortestPathSearchContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
     * @param ctx the parse tree
     */
    fn visit_countedShortestGroupSearch(
        &mut self,
        ctx: &CountedShortestGroupSearchContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numberOfGroups}.
     * @param ctx the parse tree
     */
    fn visit_numberOfGroups(&mut self, ctx: &NumberOfGroupsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppePathTerm}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppePathTerm(&mut self, ctx: &PpePathTermContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppeMultisetAlternation}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppeMultisetAlternation(&mut self, ctx: &PpeMultisetAlternationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppePatternUnion}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppePatternUnion(&mut self, ctx: &PpePatternUnionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathTerm}.
     * @param ctx the parse tree
     */
    fn visit_pathTerm(&mut self, ctx: &PathTermContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfPathPrimary(&mut self, ctx: &PfPathPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfQuantifiedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfQuantifiedPathPrimary(&mut self, ctx: &PfQuantifiedPathPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfQuestionedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfQuestionedPathPrimary(&mut self, ctx: &PfQuestionedPathPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppElementPattern}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppElementPattern(&mut self, ctx: &PpElementPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppParenthesizedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppParenthesizedPathPatternExpression(
        &mut self,
        ctx: &PpParenthesizedPathPatternExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppSimplifiedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppSimplifiedPathPatternExpression(
        &mut self,
        ctx: &PpSimplifiedPathPatternExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPattern}.
     * @param ctx the parse tree
     */
    fn visit_elementPattern(&mut self, ctx: &ElementPatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodePattern}.
     * @param ctx the parse tree
     */
    fn visit_nodePattern(&mut self, ctx: &NodePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternFiller}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternFiller(&mut self, ctx: &ElementPatternFillerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_elementVariableDeclaration(
        &mut self,
        ctx: &ElementVariableDeclarationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isLabelExpression}.
     * @param ctx the parse tree
     */
    fn visit_isLabelExpression(&mut self, ctx: &IsLabelExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isOrColon}.
     * @param ctx the parse tree
     */
    fn visit_isOrColon(&mut self, ctx: &IsOrColonContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternPredicate}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternPredicate(&mut self, ctx: &ElementPatternPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternWhereClause(&mut self, ctx: &ElementPatternWhereClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPropertySpecification}.
     * @param ctx the parse tree
     */
    fn visit_elementPropertySpecification(
        &mut self,
        ctx: &ElementPropertySpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
     * @param ctx the parse tree
     */
    fn visit_propertyKeyValuePairList(&mut self, ctx: &PropertyKeyValuePairListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
     * @param ctx the parse tree
     */
    fn visit_propertyKeyValuePair(&mut self, ctx: &PropertyKeyValuePairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgePattern}.
     * @param ctx the parse tree
     */
    fn visit_edgePattern(&mut self, ctx: &EdgePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePattern(&mut self, ctx: &FullEdgePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePointingLeft(&mut self, ctx: &FullEdgePointingLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeUndirected(&mut self, ctx: &FullEdgeUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePointingRight(&mut self, ctx: &FullEdgePointingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeLeftOrUndirected(&mut self, ctx: &FullEdgeLeftOrUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeUndirectedOrRight(&mut self, ctx: &FullEdgeUndirectedOrRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeLeftOrRight(&mut self, ctx: &FullEdgeLeftOrRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeAnyDirection(&mut self, ctx: &FullEdgeAnyDirectionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#abbreviatedEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_abbreviatedEdgePattern(&mut self, ctx: &AbbreviatedEdgePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedPathPatternExpression(
        &mut self,
        ctx: &ParenthesizedPathPatternExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_subpathVariableDeclaration(
        &mut self,
        ctx: &SubpathVariableDeclarationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedPathPatternWhereClause(
        &mut self,
        ctx: &ParenthesizedPathPatternWhereClauseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionNegation}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionNegation(&mut self, ctx: &LabelExpressionNegationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionDisjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionDisjunction(
        &mut self,
        ctx: &LabelExpressionDisjunctionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionParenthesized}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionParenthesized(
        &mut self,
        ctx: &LabelExpressionParenthesizedContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionWildcard}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionWildcard(&mut self, ctx: &LabelExpressionWildcardContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionConjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionConjunction(
        &mut self,
        ctx: &LabelExpressionConjunctionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionName}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionName(&mut self, ctx: &LabelExpressionNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_pathVariableReference(&mut self, ctx: &PathVariableReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_elementVariableReference(&mut self, ctx: &ElementVariableReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternQuantifier(&mut self, ctx: &GraphPatternQuantifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fixedQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_fixedQuantifier(&mut self, ctx: &FixedQuantifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_generalQuantifier(&mut self, ctx: &GeneralQuantifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#lowerBound}.
     * @param ctx the parse tree
     */
    fn visit_lowerBound(&mut self, ctx: &LowerBoundContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#upperBound}.
     * @param ctx the parse tree
     */
    fn visit_upperBound(&mut self, ctx: &UpperBoundContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPathPatternExpression(
        &mut self,
        ctx: &SimplifiedPathPatternExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeft(&mut self, ctx: &SimplifiedDefaultingLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingRight(&mut self, ctx: &SimplifiedDefaultingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeftOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrRightContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingAnyDirection(
        &mut self,
        ctx: &SimplifiedDefaultingAnyDirectionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedContents}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedContents(&mut self, ctx: &SimplifiedContentsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPathUnion(&mut self, ctx: &SimplifiedPathUnionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedMultisetAlternation(
        &mut self,
        ctx: &SimplifiedMultisetAlternationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedFactorLowLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorLowLabel(&mut self, ctx: &SimplifiedFactorLowLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedConcatenationLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedConcatenationLabel(
        &mut self,
        ctx: &SimplifiedConcatenationLabelContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedConjunctionLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedConjunctionLabel(
        &mut self,
        ctx: &SimplifiedConjunctionLabelContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedFactorHighLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorHighLabel(&mut self, ctx: &SimplifiedFactorHighLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorHigh(&mut self, ctx: &SimplifiedFactorHighContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedQuantified}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedQuantified(&mut self, ctx: &SimplifiedQuantifiedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedQuestioned(&mut self, ctx: &SimplifiedQuestionedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedTertiary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedTertiary(&mut self, ctx: &SimplifiedTertiaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDirectionOverride(
        &mut self,
        ctx: &SimplifiedDirectionOverrideContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeft(&mut self, ctx: &SimplifiedOverrideLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideUndirected(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideRight(&mut self, ctx: &SimplifiedOverrideRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeftOrRight(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrRightContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideAnyDirection(
        &mut self,
        ctx: &SimplifiedOverrideAnyDirectionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedSecondary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedSecondary(&mut self, ctx: &SimplifiedSecondaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedNegation}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedNegation(&mut self, ctx: &SimplifiedNegationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPrimary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPrimary(&mut self, ctx: &SimplifiedPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whereClause}.
     * @param ctx the parse tree
     */
    fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldClause}.
     * @param ctx the parse tree
     */
    fn visit_yieldClause(&mut self, ctx: &YieldClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemList}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemList(&mut self, ctx: &YieldItemListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItem}.
     * @param ctx the parse tree
     */
    fn visit_yieldItem(&mut self, ctx: &YieldItemContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemName}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemName(&mut self, ctx: &YieldItemNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemAlias(&mut self, ctx: &YieldItemAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupByClause}.
     * @param ctx the parse tree
     */
    fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupingElementList}.
     * @param ctx the parse tree
     */
    fn visit_groupingElementList(&mut self, ctx: &GroupingElementListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupingElement}.
     * @param ctx the parse tree
     */
    fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#emptyGroupingSet}.
     * @param ctx the parse tree
     */
    fn visit_emptyGroupingSet(&mut self, ctx: &EmptyGroupingSetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderByClause}.
     * @param ctx the parse tree
     */
    fn visit_orderByClause(&mut self, ctx: &OrderByClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortSpecificationList}.
     * @param ctx the parse tree
     */
    fn visit_sortSpecificationList(&mut self, ctx: &SortSpecificationListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortSpecification}.
     * @param ctx the parse tree
     */
    fn visit_sortSpecification(&mut self, ctx: &SortSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortKey}.
     * @param ctx the parse tree
     */
    fn visit_sortKey(&mut self, ctx: &SortKeyContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderingSpecification}.
     * @param ctx the parse tree
     */
    fn visit_orderingSpecification(&mut self, ctx: &OrderingSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullOrdering}.
     * @param ctx the parse tree
     */
    fn visit_nullOrdering(&mut self, ctx: &NullOrderingContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#limitClause}.
     * @param ctx the parse tree
     */
    fn visit_limitClause(&mut self, ctx: &LimitClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#offsetClause}.
     * @param ctx the parse tree
     */
    fn visit_offsetClause(&mut self, ctx: &OffsetClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#offsetSynonym}.
     * @param ctx the parse tree
     */
    fn visit_offsetSynonym(&mut self, ctx: &OffsetSynonymContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#schemaReference}.
     * @param ctx the parse tree
     */
    fn visit_schemaReference(&mut self, ctx: &SchemaReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_absoluteCatalogSchemaReference(
        &mut self,
        ctx: &AbsoluteCatalogSchemaReferenceContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogSchemaParentAndName(
        &mut self,
        ctx: &CatalogSchemaParentAndNameContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_relativeCatalogSchemaReference(
        &mut self,
        ctx: &RelativeCatalogSchemaReferenceContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_predefinedSchemaReference(&mut self, ctx: &PredefinedSchemaReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_absoluteDirectoryPath(&mut self, ctx: &AbsoluteDirectoryPathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_relativeDirectoryPath(&mut self, ctx: &RelativeDirectoryPathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_simpleDirectoryPath(&mut self, ctx: &SimpleDirectoryPathContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphReference}.
     * @param ctx the parse tree
     */
    fn visit_graphReference(&mut self, ctx: &GraphReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogGraphParentAndName(&mut self, ctx: &CatalogGraphParentAndNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#homeGraph}.
     * @param ctx the parse tree
     */
    fn visit_homeGraph(&mut self, ctx: &HomeGraphContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeReference(&mut self, ctx: &GraphTypeReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogGraphTypeParentAndName(
        &mut self,
        ctx: &CatalogGraphTypeParentAndNameContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableReference}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableReference(&mut self, ctx: &BindingTableReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureReference}.
     * @param ctx the parse tree
     */
    fn visit_procedureReference(&mut self, ctx: &ProcedureReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogProcedureParentAndName(
        &mut self,
        ctx: &CatalogProcedureParentAndNameContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
     * @param ctx the parse tree
     */
    fn visit_catalogObjectParentReference(
        &mut self,
        ctx: &CatalogObjectParentReferenceContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#referenceParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_referenceParameterSpecification(
        &mut self,
        ctx: &ReferenceParameterSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedGraphTypeSpecification(
        &mut self,
        ctx: &NestedGraphTypeSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeSpecificationBody(
        &mut self,
        ctx: &GraphTypeSpecificationBodyContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementTypeList}.
     * @param ctx the parse tree
     */
    fn visit_elementTypeList(&mut self, ctx: &ElementTypeListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_elementTypeSpecification(&mut self, ctx: &ElementTypeSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeSpecification(&mut self, ctx: &NodeTypeSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePattern}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePattern(&mut self, ctx: &NodeTypePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePhrase}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePhrase(&mut self, ctx: &NodeTypePhraseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePhraseFiller(&mut self, ctx: &NodeTypePhraseFillerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeFiller}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeFiller(&mut self, ctx: &NodeTypeFillerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_localNodeTypeAlias(&mut self, ctx: &LocalNodeTypeAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeImpliedContent(&mut self, ctx: &NodeTypeImpliedContentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeKeyLabelSet(&mut self, ctx: &NodeTypeKeyLabelSetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeLabelSet(&mut self, ctx: &NodeTypeLabelSetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePropertyTypes(&mut self, ctx: &NodeTypePropertyTypesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeSpecification(&mut self, ctx: &EdgeTypeSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePattern}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePattern(&mut self, ctx: &EdgeTypePatternContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePhrase}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePhrase(&mut self, ctx: &EdgeTypePhraseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePhraseFiller(&mut self, ctx: &EdgeTypePhraseFillerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeFiller}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeFiller(&mut self, ctx: &EdgeTypeFillerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeImpliedContent(&mut self, ctx: &EdgeTypeImpliedContentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeKeyLabelSet(&mut self, ctx: &EdgeTypeKeyLabelSetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeLabelSet(&mut self, ctx: &EdgeTypeLabelSetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePropertyTypes(&mut self, ctx: &EdgeTypePropertyTypesContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternDirected}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternDirected(&mut self, ctx: &EdgeTypePatternDirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternPointingRight(
        &mut self,
        ctx: &EdgeTypePatternPointingRightContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternPointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternPointingLeft(
        &mut self,
        ctx: &EdgeTypePatternPointingLeftContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternUndirected}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternUndirected(&mut self, ctx: &EdgeTypePatternUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_arcTypePointingRight(&mut self, ctx: &ArcTypePointingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_arcTypePointingLeft(&mut self, ctx: &ArcTypePointingLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_arcTypeUndirected(&mut self, ctx: &ArcTypeUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_sourceNodeTypeReference(&mut self, ctx: &SourceNodeTypeReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_destinationNodeTypeReference(
        &mut self,
        ctx: &DestinationNodeTypeReferenceContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeKind}.
     * @param ctx the parse tree
     */
    fn visit_edgeKind(&mut self, ctx: &EdgeKindContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPhrase}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPhrase(&mut self, ctx: &EndpointPairPhraseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPair}.
     * @param ctx the parse tree
     */
    fn visit_endpointPair(&mut self, ctx: &EndpointPairContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairDirected}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairDirected(&mut self, ctx: &EndpointPairDirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPointingRight(&mut self, ctx: &EndpointPairPointingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPointingLeft(&mut self, ctx: &EndpointPairPointingLeftContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairUndirected}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairUndirected(&mut self, ctx: &EndpointPairUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#connectorPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_connectorPointingRight(&mut self, ctx: &ConnectorPointingRightContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#connectorUndirected}.
     * @param ctx the parse tree
     */
    fn visit_connectorUndirected(&mut self, ctx: &ConnectorUndirectedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_sourceNodeTypeAlias(&mut self, ctx: &SourceNodeTypeAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_destinationNodeTypeAlias(&mut self, ctx: &DestinationNodeTypeAliasContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelSetPhrase}.
     * @param ctx the parse tree
     */
    fn visit_labelSetPhrase(&mut self, ctx: &LabelSetPhraseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelSetSpecification}.
     * @param ctx the parse tree
     */
    fn visit_labelSetSpecification(&mut self, ctx: &LabelSetSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyTypesSpecification}.
     * @param ctx the parse tree
     */
    fn visit_propertyTypesSpecification(
        &mut self,
        ctx: &PropertyTypesSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyTypeList}.
     * @param ctx the parse tree
     */
    fn visit_propertyTypeList(&mut self, ctx: &PropertyTypeListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyType}.
     * @param ctx the parse tree
     */
    fn visit_propertyType(&mut self, ctx: &PropertyTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyValueType}.
     * @param ctx the parse tree
     */
    fn visit_propertyValueType(&mut self, ctx: &PropertyValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableType}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableType(&mut self, ctx: &BindingTableTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dynamicPropertyValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_dynamicPropertyValueTypeLabel(
        &mut self,
        ctx: &DynamicPropertyValueTypeLabelContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code closedDynamicUnionTypeAtl1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_closedDynamicUnionTypeAtl1(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl1Context<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code closedDynamicUnionTypeAtl2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_closedDynamicUnionTypeAtl2(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl2Context<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pathValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_pathValueTypeLabel(&mut self, ctx: &PathValueTypeLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt3}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt3(&mut self, ctx: &ListValueTypeAlt3Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt2(&mut self, ctx: &ListValueTypeAlt2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt1(&mut self, ctx: &ListValueTypeAlt1Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code predefinedTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_predefinedTypeLabel(&mut self, ctx: &PredefinedTypeLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code recordTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_recordTypeLabel(&mut self, ctx: &RecordTypeLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code openDynamicUnionTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_openDynamicUnionTypeLabel(&mut self, ctx: &OpenDynamicUnionTypeLabelContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#typed}.
     * @param ctx the parse tree
     */
    fn visit_typed(&mut self, ctx: &TypedContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predefinedType}.
     * @param ctx the parse tree
     */
    fn visit_predefinedType(&mut self, ctx: &PredefinedTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#booleanType}.
     * @param ctx the parse tree
     */
    fn visit_booleanType(&mut self, ctx: &BooleanTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringType}.
     * @param ctx the parse tree
     */
    fn visit_characterStringType(&mut self, ctx: &CharacterStringTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteStringType}.
     * @param ctx the parse tree
     */
    fn visit_byteStringType(&mut self, ctx: &ByteStringTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#minLength}.
     * @param ctx the parse tree
     */
    fn visit_minLength(&mut self, ctx: &MinLengthContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#maxLength}.
     * @param ctx the parse tree
     */
    fn visit_maxLength(&mut self, ctx: &MaxLengthContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fixedLength}.
     * @param ctx the parse tree
     */
    fn visit_fixedLength(&mut self, ctx: &FixedLengthContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericType}.
     * @param ctx the parse tree
     */
    fn visit_numericType(&mut self, ctx: &NumericTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_exactNumericType(&mut self, ctx: &ExactNumericTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_binaryExactNumericType(&mut self, ctx: &BinaryExactNumericTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_signedBinaryExactNumericType(
        &mut self,
        ctx: &SignedBinaryExactNumericTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_unsignedBinaryExactNumericType(
        &mut self,
        ctx: &UnsignedBinaryExactNumericTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_verboseBinaryExactNumericType(
        &mut self,
        ctx: &VerboseBinaryExactNumericTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#decimalExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_decimalExactNumericType(&mut self, ctx: &DecimalExactNumericTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#precision}.
     * @param ctx the parse tree
     */
    fn visit_precision(&mut self, ctx: &PrecisionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#scale}.
     * @param ctx the parse tree
     */
    fn visit_scale(&mut self, ctx: &ScaleContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#approximateNumericType}.
     * @param ctx the parse tree
     */
    fn visit_approximateNumericType(&mut self, ctx: &ApproximateNumericTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalType}.
     * @param ctx the parse tree
     */
    fn visit_temporalType(&mut self, ctx: &TemporalTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalInstantType}.
     * @param ctx the parse tree
     */
    fn visit_temporalInstantType(&mut self, ctx: &TemporalInstantTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeType}.
     * @param ctx the parse tree
     */
    fn visit_datetimeType(&mut self, ctx: &DatetimeTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localdatetimeType}.
     * @param ctx the parse tree
     */
    fn visit_localdatetimeType(&mut self, ctx: &LocaldatetimeTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateType}.
     * @param ctx the parse tree
     */
    fn visit_dateType(&mut self, ctx: &DateTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeType}.
     * @param ctx the parse tree
     */
    fn visit_timeType(&mut self, ctx: &TimeTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localtimeType}.
     * @param ctx the parse tree
     */
    fn visit_localtimeType(&mut self, ctx: &LocaltimeTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalDurationType}.
     * @param ctx the parse tree
     */
    fn visit_temporalDurationType(&mut self, ctx: &TemporalDurationTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalDurationQualifier}.
     * @param ctx the parse tree
     */
    fn visit_temporalDurationQualifier(&mut self, ctx: &TemporalDurationQualifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#referenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_referenceValueType(&mut self, ctx: &ReferenceValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#immaterialValueType}.
     * @param ctx the parse tree
     */
    fn visit_immaterialValueType(&mut self, ctx: &ImmaterialValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullType}.
     * @param ctx the parse tree
     */
    fn visit_nullType(&mut self, ctx: &NullTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#emptyType}.
     * @param ctx the parse tree
     */
    fn visit_emptyType(&mut self, ctx: &EmptyTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_graphReferenceValueType(&mut self, ctx: &GraphReferenceValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedGraphReferenceValueType(
        &mut self,
        ctx: &ClosedGraphReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openGraphReferenceValueType(
        &mut self,
        ctx: &OpenGraphReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableReferenceValueType(
        &mut self,
        ctx: &BindingTableReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_nodeReferenceValueType(&mut self, ctx: &NodeReferenceValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedNodeReferenceValueType(
        &mut self,
        ctx: &ClosedNodeReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openNodeReferenceValueType(
        &mut self,
        ctx: &OpenNodeReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_edgeReferenceValueType(&mut self, ctx: &EdgeReferenceValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedEdgeReferenceValueType(
        &mut self,
        ctx: &ClosedEdgeReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openEdgeReferenceValueType(
        &mut self,
        ctx: &OpenEdgeReferenceValueTypeContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueType}.
     * @param ctx the parse tree
     */
    fn visit_pathValueType(&mut self, ctx: &PathValueTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueTypeName}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeName(&mut self, ctx: &ListValueTypeNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeNameSynonym(&mut self, ctx: &ListValueTypeNameSynonymContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordType}.
     * @param ctx the parse tree
     */
    fn visit_recordType(&mut self, ctx: &RecordTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
     * @param ctx the parse tree
     */
    fn visit_fieldTypesSpecification(&mut self, ctx: &FieldTypesSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldTypeList}.
     * @param ctx the parse tree
     */
    fn visit_fieldTypeList(&mut self, ctx: &FieldTypeListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#notNull}.
     * @param ctx the parse tree
     */
    fn visit_notNull(&mut self, ctx: &NotNullContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldType}.
     * @param ctx the parse tree
     */
    fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchCondition}.
     * @param ctx the parse tree
     */
    fn visit_searchCondition(&mut self, ctx: &SearchConditionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predicate}.
     * @param ctx the parse tree
     */
    fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compOp}.
     * @param ctx the parse tree
     */
    fn visit_compOp(&mut self, ctx: &CompOpContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#existsPredicate}.
     * @param ctx the parse tree
     */
    fn visit_existsPredicate(&mut self, ctx: &ExistsPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullPredicate}.
     * @param ctx the parse tree
     */
    fn visit_nullPredicate(&mut self, ctx: &NullPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_nullPredicatePart2(&mut self, ctx: &NullPredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueTypePredicate}.
     * @param ctx the parse tree
     */
    fn visit_valueTypePredicate(&mut self, ctx: &ValueTypePredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueTypePredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_valueTypePredicatePart2(&mut self, ctx: &ValueTypePredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalizedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_normalizedPredicatePart2(&mut self, ctx: &NormalizedPredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directedPredicate}.
     * @param ctx the parse tree
     */
    fn visit_directedPredicate(&mut self, ctx: &DirectedPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_directedPredicatePart2(&mut self, ctx: &DirectedPredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labeledPredicate}.
     * @param ctx the parse tree
     */
    fn visit_labeledPredicate(&mut self, ctx: &LabeledPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labeledPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_labeledPredicatePart2(&mut self, ctx: &LabeledPredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isLabeledOrColon}.
     * @param ctx the parse tree
     */
    fn visit_isLabeledOrColon(&mut self, ctx: &IsLabeledOrColonContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceDestinationPredicate}.
     * @param ctx the parse tree
     */
    fn visit_sourceDestinationPredicate(
        &mut self,
        ctx: &SourceDestinationPredicateContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReference}.
     * @param ctx the parse tree
     */
    fn visit_nodeReference(&mut self, ctx: &NodeReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourcePredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_sourcePredicatePart2(&mut self, ctx: &SourcePredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_destinationPredicatePart2(&mut self, ctx: &DestinationPredicatePart2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReference}.
     * @param ctx the parse tree
     */
    fn visit_edgeReference(&mut self, ctx: &EdgeReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#all_differentPredicate}.
     * @param ctx the parse tree
     */
    fn visit_all_differentPredicate(&mut self, ctx: &All_differentPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#samePredicate}.
     * @param ctx the parse tree
     */
    fn visit_samePredicate(&mut self, ctx: &SamePredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#property_existsPredicate}.
     * @param ctx the parse tree
     */
    fn visit_property_existsPredicate(&mut self, ctx: &Property_existsPredicateContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code conjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_conjunctiveExprAlt(&mut self, ctx: &ConjunctiveExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code propertyGraphExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_propertyGraphExprAlt(&mut self, ctx: &PropertyGraphExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code multDivExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_multDivExprAlt(&mut self, ctx: &MultDivExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code bindingTableExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableExprAlt(&mut self, ctx: &BindingTableExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code signedExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_signedExprAlt(&mut self, ctx: &SignedExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code isNotExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_isNotExprAlt(&mut self, ctx: &IsNotExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code normalizedPredicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_normalizedPredicateExprAlt(
        &mut self,
        ctx: &NormalizedPredicateExprAltContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code notExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_notExprAlt(&mut self, ctx: &NotExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code valueFunctionExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_valueFunctionExprAlt(&mut self, ctx: &ValueFunctionExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code concatenationExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_concatenationExprAlt(&mut self, ctx: &ConcatenationExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code disjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_disjunctiveExprAlt(&mut self, ctx: &DisjunctiveExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code comparisonExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_comparisonExprAlt(&mut self, ctx: &ComparisonExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primaryExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_primaryExprAlt(&mut self, ctx: &PrimaryExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code addSubtractExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_addSubtractExprAlt(&mut self, ctx: &AddSubtractExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code predicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_predicateExprAlt(&mut self, ctx: &PredicateExprAltContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueFunction}.
     * @param ctx the parse tree
     */
    fn visit_valueFunction(&mut self, ctx: &ValueFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#booleanValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_booleanValueExpression(&mut self, ctx: &BooleanValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterOrByteStringFunction}.
     * @param ctx the parse tree
     */
    fn visit_characterOrByteStringFunction(
        &mut self,
        ctx: &CharacterOrByteStringFunctionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_subCharacterOrByteString(&mut self, ctx: &SubCharacterOrByteStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimSingleCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_trimSingleCharacterOrByteString(
        &mut self,
        ctx: &TrimSingleCharacterOrByteStringContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#foldCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_foldCharacterString(&mut self, ctx: &FoldCharacterStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimMultiCharacterCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_trimMultiCharacterCharacterString(
        &mut self,
        ctx: &TrimMultiCharacterCharacterStringContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalizeCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_normalizeCharacterString(&mut self, ctx: &NormalizeCharacterStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_nodeReferenceValueExpression(
        &mut self,
        ctx: &NodeReferenceValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_edgeReferenceValueExpression(
        &mut self,
        ctx: &EdgeReferenceValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#aggregatingValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_aggregatingValueExpression(
        &mut self,
        ctx: &AggregatingValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_valueExpressionPrimary(&mut self, ctx: &ValueExpressionPrimaryContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedValueExpression(
        &mut self,
        ctx: &ParenthesizedValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_nonParenthesizedValueExpressionPrimary(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimaryContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimarySpecialCase}.
     * @param ctx the parse tree
     */
    fn visit_nonParenthesizedValueExpressionPrimarySpecialCase(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimarySpecialCaseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
     * @param ctx the parse tree
     */
    fn visit_unsignedValueSpecification(
        &mut self,
        ctx: &UnsignedValueSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonNegativeIntegerSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nonNegativeIntegerSpecification(
        &mut self,
        ctx: &NonNegativeIntegerSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalValueSpecification}.
     * @param ctx the parse tree
     */
    fn visit_generalValueSpecification(&mut self, ctx: &GeneralValueSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dynamicParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_dynamicParameterSpecification(
        &mut self,
        ctx: &DynamicParameterSpecificationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_letValueExpression(&mut self, ctx: &LetValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueQueryExpression}.
     * @param ctx the parse tree
     */
    fn visit_valueQueryExpression(&mut self, ctx: &ValueQueryExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseExpression}.
     * @param ctx the parse tree
     */
    fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseAbbreviation}.
     * @param ctx the parse tree
     */
    fn visit_caseAbbreviation(&mut self, ctx: &CaseAbbreviationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseSpecification}.
     * @param ctx the parse tree
     */
    fn visit_caseSpecification(&mut self, ctx: &CaseSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleCase}.
     * @param ctx the parse tree
     */
    fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchedCase}.
     * @param ctx the parse tree
     */
    fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleWhenClause}.
     * @param ctx the parse tree
     */
    fn visit_simpleWhenClause(&mut self, ctx: &SimpleWhenClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchedWhenClause}.
     * @param ctx the parse tree
     */
    fn visit_searchedWhenClause(&mut self, ctx: &SearchedWhenClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elseClause}.
     * @param ctx the parse tree
     */
    fn visit_elseClause(&mut self, ctx: &ElseClauseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseOperand}.
     * @param ctx the parse tree
     */
    fn visit_caseOperand(&mut self, ctx: &CaseOperandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whenOperandList}.
     * @param ctx the parse tree
     */
    fn visit_whenOperandList(&mut self, ctx: &WhenOperandListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whenOperand}.
     * @param ctx the parse tree
     */
    fn visit_whenOperand(&mut self, ctx: &WhenOperandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#result}.
     * @param ctx the parse tree
     */
    fn visit_result(&mut self, ctx: &ResultContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#resultExpression}.
     * @param ctx the parse tree
     */
    fn visit_resultExpression(&mut self, ctx: &ResultExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castSpecification}.
     * @param ctx the parse tree
     */
    fn visit_castSpecification(&mut self, ctx: &CastSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castOperand}.
     * @param ctx the parse tree
     */
    fn visit_castOperand(&mut self, ctx: &CastOperandContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castTarget}.
     * @param ctx the parse tree
     */
    fn visit_castTarget(&mut self, ctx: &CastTargetContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#aggregateFunction}.
     * @param ctx the parse tree
     */
    fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalSetFunction}.
     * @param ctx the parse tree
     */
    fn visit_generalSetFunction(&mut self, ctx: &GeneralSetFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binarySetFunction}.
     * @param ctx the parse tree
     */
    fn visit_binarySetFunction(&mut self, ctx: &BinarySetFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalSetFunctionType}.
     * @param ctx the parse tree
     */
    fn visit_generalSetFunctionType(&mut self, ctx: &GeneralSetFunctionTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binarySetFunctionType}.
     * @param ctx the parse tree
     */
    fn visit_binarySetFunctionType(&mut self, ctx: &BinarySetFunctionTypeContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dependentValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_dependentValueExpression(&mut self, ctx: &DependentValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#independentValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_independentValueExpression(
        &mut self,
        ctx: &IndependentValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#element_idFunction}.
     * @param ctx the parse tree
     */
    fn visit_element_idFunction(&mut self, ctx: &Element_idFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableReference(&mut self, ctx: &BindingVariableReferenceContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_pathValueExpression(&mut self, ctx: &PathValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueConstructor}.
     * @param ctx the parse tree
     */
    fn visit_pathValueConstructor(&mut self, ctx: &PathValueConstructorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn visit_pathValueConstructorByEnumeration(
        &mut self,
        ctx: &PathValueConstructorByEnumerationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementList}.
     * @param ctx the parse tree
     */
    fn visit_pathElementList(&mut self, ctx: &PathElementListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementListStart}.
     * @param ctx the parse tree
     */
    fn visit_pathElementListStart(&mut self, ctx: &PathElementListStartContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementListStep}.
     * @param ctx the parse tree
     */
    fn visit_pathElementListStep(&mut self, ctx: &PathElementListStepContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_listValueExpression(&mut self, ctx: &ListValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_listValueFunction(&mut self, ctx: &ListValueFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimListFunction}.
     * @param ctx the parse tree
     */
    fn visit_trimListFunction(&mut self, ctx: &TrimListFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementsFunction}.
     * @param ctx the parse tree
     */
    fn visit_elementsFunction(&mut self, ctx: &ElementsFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueConstructor}.
     * @param ctx the parse tree
     */
    fn visit_listValueConstructor(&mut self, ctx: &ListValueConstructorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn visit_listValueConstructorByEnumeration(
        &mut self,
        ctx: &ListValueConstructorByEnumerationContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listElementList}.
     * @param ctx the parse tree
     */
    fn visit_listElementList(&mut self, ctx: &ListElementListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listElement}.
     * @param ctx the parse tree
     */
    fn visit_listElement(&mut self, ctx: &ListElementContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordConstructor}.
     * @param ctx the parse tree
     */
    fn visit_recordConstructor(&mut self, ctx: &RecordConstructorContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldsSpecification}.
     * @param ctx the parse tree
     */
    fn visit_fieldsSpecification(&mut self, ctx: &FieldsSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldList}.
     * @param ctx the parse tree
     */
    fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_field(&mut self, ctx: &FieldContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#truthValue}.
     * @param ctx the parse tree
     */
    fn visit_truthValue(&mut self, ctx: &TruthValueContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpression(&mut self, ctx: &NumericValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_numericValueFunction(&mut self, ctx: &NumericValueFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#lengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_lengthExpression(&mut self, ctx: &LengthExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#cardinalityExpression}.
     * @param ctx the parse tree
     */
    fn visit_cardinalityExpression(&mut self, ctx: &CardinalityExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#cardinalityExpressionArgument}.
     * @param ctx the parse tree
     */
    fn visit_cardinalityExpressionArgument(
        &mut self,
        ctx: &CardinalityExpressionArgumentContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#charLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_charLengthExpression(&mut self, ctx: &CharLengthExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_byteLengthExpression(&mut self, ctx: &ByteLengthExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_pathLengthExpression(&mut self, ctx: &PathLengthExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_absoluteValueExpression(&mut self, ctx: &AbsoluteValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#modulusExpression}.
     * @param ctx the parse tree
     */
    fn visit_modulusExpression(&mut self, ctx: &ModulusExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionDividend}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionDividend(
        &mut self,
        ctx: &NumericValueExpressionDividendContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionDivisor}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionDivisor(
        &mut self,
        ctx: &NumericValueExpressionDivisorContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trigonometricFunction}.
     * @param ctx the parse tree
     */
    fn visit_trigonometricFunction(&mut self, ctx: &TrigonometricFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trigonometricFunctionName}.
     * @param ctx the parse tree
     */
    fn visit_trigonometricFunctionName(&mut self, ctx: &TrigonometricFunctionNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmFunction}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmFunction(&mut self, ctx: &GeneralLogarithmFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmBase}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmBase(&mut self, ctx: &GeneralLogarithmBaseContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmArgument}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmArgument(&mut self, ctx: &GeneralLogarithmArgumentContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#commonLogarithm}.
     * @param ctx the parse tree
     */
    fn visit_commonLogarithm(&mut self, ctx: &CommonLogarithmContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#naturalLogarithm}.
     * @param ctx the parse tree
     */
    fn visit_naturalLogarithm(&mut self, ctx: &NaturalLogarithmContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exponentialFunction}.
     * @param ctx the parse tree
     */
    fn visit_exponentialFunction(&mut self, ctx: &ExponentialFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#powerFunction}.
     * @param ctx the parse tree
     */
    fn visit_powerFunction(&mut self, ctx: &PowerFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionBase}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionBase(
        &mut self,
        ctx: &NumericValueExpressionBaseContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionExponent}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionExponent(
        &mut self,
        ctx: &NumericValueExpressionExponentContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#squareRoot}.
     * @param ctx the parse tree
     */
    fn visit_squareRoot(&mut self, ctx: &SquareRootContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#floorFunction}.
     * @param ctx the parse tree
     */
    fn visit_floorFunction(&mut self, ctx: &FloorFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ceilingFunction}.
     * @param ctx the parse tree
     */
    fn visit_ceilingFunction(&mut self, ctx: &CeilingFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_characterStringValueExpression(
        &mut self,
        ctx: &CharacterStringValueExpressionContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteStringValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_byteStringValueExpression(&mut self, ctx: &ByteStringValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimOperands}.
     * @param ctx the parse tree
     */
    fn visit_trimOperands(&mut self, ctx: &TrimOperandsContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimCharacterOrByteStringSource}.
     * @param ctx the parse tree
     */
    fn visit_trimCharacterOrByteStringSource(
        &mut self,
        ctx: &TrimCharacterOrByteStringSourceContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimSpecification}.
     * @param ctx the parse tree
     */
    fn visit_trimSpecification(&mut self, ctx: &TrimSpecificationContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_trimCharacterOrByteString(&mut self, ctx: &TrimCharacterOrByteStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalForm}.
     * @param ctx the parse tree
     */
    fn visit_normalForm(&mut self, ctx: &NormalFormContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#stringLength}.
     * @param ctx the parse tree
     */
    fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression(&mut self, ctx: &DatetimeValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueFunction(&mut self, ctx: &DatetimeValueFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateFunction}.
     * @param ctx the parse tree
     */
    fn visit_dateFunction(&mut self, ctx: &DateFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeFunction}.
     * @param ctx the parse tree
     */
    fn visit_timeFunction(&mut self, ctx: &TimeFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localtimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_localtimeFunction(&mut self, ctx: &LocaltimeFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeFunction(&mut self, ctx: &DatetimeFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localdatetimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_localdatetimeFunction(&mut self, ctx: &LocaldatetimeFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_dateFunctionParameters(&mut self, ctx: &DateFunctionParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_timeFunctionParameters(&mut self, ctx: &TimeFunctionParametersContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_datetimeFunctionParameters(
        &mut self,
        ctx: &DatetimeFunctionParametersContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_durationValueExpression(&mut self, ctx: &DurationValueExpressionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeSubtraction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeSubtraction(&mut self, ctx: &DatetimeSubtractionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeSubtractionParameters}.
     * @param ctx the parse tree
     */
    fn visit_datetimeSubtractionParameters(
        &mut self,
        ctx: &DatetimeSubtractionParametersContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression1}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression1(&mut self, ctx: &DatetimeValueExpression1Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression2}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression2(&mut self, ctx: &DatetimeValueExpression2Context<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_durationValueFunction(&mut self, ctx: &DurationValueFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationFunction}.
     * @param ctx the parse tree
     */
    fn visit_durationFunction(&mut self, ctx: &DurationFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_durationFunctionParameters(
        &mut self,
        ctx: &DurationFunctionParametersContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectName}.
     * @param ctx the parse tree
     */
    fn visit_objectName(&mut self, ctx: &ObjectNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
     * @param ctx the parse tree
     */
    fn visit_objectNameOrBindingVariable(
        &mut self,
        ctx: &ObjectNameOrBindingVariableContext<'input>,
    ) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directoryName}.
     * @param ctx the parse tree
     */
    fn visit_directoryName(&mut self, ctx: &DirectoryNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#schemaName}.
     * @param ctx the parse tree
     */
    fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphName}.
     * @param ctx the parse tree
     */
    fn visit_graphName(&mut self, ctx: &GraphNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#delimitedGraphName}.
     * @param ctx the parse tree
     */
    fn visit_delimitedGraphName(&mut self, ctx: &DelimitedGraphNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeName}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeName(&mut self, ctx: &GraphTypeNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeName}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeName(&mut self, ctx: &NodeTypeNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeName}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeName(&mut self, ctx: &EdgeTypeNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableName}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableName(&mut self, ctx: &BindingTableNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
     * @param ctx the parse tree
     */
    fn visit_delimitedBindingTableName(&mut self, ctx: &DelimitedBindingTableNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureName}.
     * @param ctx the parse tree
     */
    fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelName}.
     * @param ctx the parse tree
     */
    fn visit_labelName(&mut self, ctx: &LabelNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyName}.
     * @param ctx the parse tree
     */
    fn visit_propertyName(&mut self, ctx: &PropertyNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldName}.
     * @param ctx the parse tree
     */
    fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariable}.
     * @param ctx the parse tree
     */
    fn visit_elementVariable(&mut self, ctx: &ElementVariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariable}.
     * @param ctx the parse tree
     */
    fn visit_pathVariable(&mut self, ctx: &PathVariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subpathVariable}.
     * @param ctx the parse tree
     */
    fn visit_subpathVariable(&mut self, ctx: &SubpathVariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariable}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariable(&mut self, ctx: &BindingVariableContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedLiteral}.
     * @param ctx the parse tree
     */
    fn visit_unsignedLiteral(&mut self, ctx: &UnsignedLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLiteral}.
     * @param ctx the parse tree
     */
    fn visit_generalLiteral(&mut self, ctx: &GeneralLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalLiteral}.
     * @param ctx the parse tree
     */
    fn visit_temporalLiteral(&mut self, ctx: &TemporalLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateLiteral}.
     * @param ctx the parse tree
     */
    fn visit_dateLiteral(&mut self, ctx: &DateLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeLiteral}.
     * @param ctx the parse tree
     */
    fn visit_timeLiteral(&mut self, ctx: &TimeLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeLiteral}.
     * @param ctx the parse tree
     */
    fn visit_datetimeLiteral(&mut self, ctx: &DatetimeLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listLiteral}.
     * @param ctx the parse tree
     */
    fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordLiteral}.
     * @param ctx the parse tree
     */
    fn visit_recordLiteral(&mut self, ctx: &RecordLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#identifier}.
     * @param ctx the parse tree
     */
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#regularIdentifier}.
     * @param ctx the parse tree
     */
    fn visit_regularIdentifier(&mut self, ctx: &RegularIdentifierContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeZoneString}.
     * @param ctx the parse tree
     */
    fn visit_timeZoneString(&mut self, ctx: &TimeZoneStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringLiteral}.
     * @param ctx the parse tree
     */
    fn visit_characterStringLiteral(&mut self, ctx: &CharacterStringLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_unsignedNumericLiteral(&mut self, ctx: &UnsignedNumericLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exactNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_exactNumericLiteral(&mut self, ctx: &ExactNumericLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#approximateNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_approximateNumericLiteral(&mut self, ctx: &ApproximateNumericLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedInteger}.
     * @param ctx the parse tree
     */
    fn visit_unsignedInteger(&mut self, ctx: &UnsignedIntegerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedDecimalInteger}.
     * @param ctx the parse tree
     */
    fn visit_unsignedDecimalInteger(&mut self, ctx: &UnsignedDecimalIntegerContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullLiteral}.
     * @param ctx the parse tree
     */
    fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateString}.
     * @param ctx the parse tree
     */
    fn visit_dateString(&mut self, ctx: &DateStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeString}.
     * @param ctx the parse tree
     */
    fn visit_timeString(&mut self, ctx: &TimeStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeString}.
     * @param ctx the parse tree
     */
    fn visit_datetimeString(&mut self, ctx: &DatetimeStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationLiteral}.
     * @param ctx the parse tree
     */
    fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationString}.
     * @param ctx the parse tree
     */
    fn visit_durationString(&mut self, ctx: &DurationStringContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeSynonym}.
     * @param ctx the parse tree
     */
    fn visit_nodeSynonym(&mut self, ctx: &NodeSynonymContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgesSynonym}.
     * @param ctx the parse tree
     */
    fn visit_edgesSynonym(&mut self, ctx: &EdgesSynonymContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeSynonym}.
     * @param ctx the parse tree
     */
    fn visit_edgeSynonym(&mut self, ctx: &EdgeSynonymContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#userDefinedFunction}.
     * @param ctx the parse tree
     */
    fn visit_userDefinedFunction(&mut self, ctx: &UserDefinedFunctionContext<'input>) {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonReservedWords}.
     * @param ctx the parse tree
     */
    fn visit_nonReservedWords(&mut self, ctx: &NonReservedWordsContext<'input>) {
        self.visit_children(ctx)
    }
}

pub trait GQLVisitorCompat<'input>:
    ParseTreeVisitorCompat<'input, Node = GQLParserContextType>
{
    /**
     * Visit a parse tree produced by {@link GQLParser#gqlProgram}.
     * @param ctx the parse tree
     */
    fn visit_gqlProgram(&mut self, ctx: &GqlProgramContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#programActivity}.
     * @param ctx the parse tree
     */
    fn visit_programActivity(&mut self, ctx: &ProgramActivityContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionActivity}.
     * @param ctx the parse tree
     */
    fn visit_sessionActivity(&mut self, ctx: &SessionActivityContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionActivity}.
     * @param ctx the parse tree
     */
    fn visit_transactionActivity(
        &mut self,
        ctx: &TransactionActivityContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endTransactionCommand}.
     * @param ctx the parse tree
     */
    fn visit_endTransactionCommand(
        &mut self,
        ctx: &EndTransactionCommandContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetCommand(&mut self, ctx: &SessionSetCommandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetSchemaClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetSchemaClause(
        &mut self,
        ctx: &SessionSetSchemaClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetGraphClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetGraphClause(
        &mut self,
        ctx: &SessionSetGraphClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetTimeZoneClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetTimeZoneClause(
        &mut self,
        ctx: &SessionSetTimeZoneClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setTimeZoneValue}.
     * @param ctx the parse tree
     */
    fn visit_setTimeZoneValue(&mut self, ctx: &SetTimeZoneValueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetParameterClause(
        &mut self,
        ctx: &SessionSetParameterClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetGraphParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetGraphParameterClause(
        &mut self,
        ctx: &SessionSetGraphParameterClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetBindingTableParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetBindingTableParameterClause(
        &mut self,
        ctx: &SessionSetBindingTableParameterClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetValueParameterClause}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetValueParameterClause(
        &mut self,
        ctx: &SessionSetValueParameterClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionSetParameterName}.
     * @param ctx the parse tree
     */
    fn visit_sessionSetParameterName(
        &mut self,
        ctx: &SessionSetParameterNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionResetCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionResetCommand(
        &mut self,
        ctx: &SessionResetCommandContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionResetArguments}.
     * @param ctx the parse tree
     */
    fn visit_sessionResetArguments(
        &mut self,
        ctx: &SessionResetArgumentsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionCloseCommand}.
     * @param ctx the parse tree
     */
    fn visit_sessionCloseCommand(
        &mut self,
        ctx: &SessionCloseCommandContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sessionParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_sessionParameterSpecification(
        &mut self,
        ctx: &SessionParameterSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#startTransactionCommand}.
     * @param ctx the parse tree
     */
    fn visit_startTransactionCommand(
        &mut self,
        ctx: &StartTransactionCommandContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionCharacteristics}.
     * @param ctx the parse tree
     */
    fn visit_transactionCharacteristics(
        &mut self,
        ctx: &TransactionCharacteristicsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionMode}.
     * @param ctx the parse tree
     */
    fn visit_transactionMode(&mut self, ctx: &TransactionModeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#transactionAccessMode}.
     * @param ctx the parse tree
     */
    fn visit_transactionAccessMode(
        &mut self,
        ctx: &TransactionAccessModeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#rollbackCommand}.
     * @param ctx the parse tree
     */
    fn visit_rollbackCommand(&mut self, ctx: &RollbackCommandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#commitCommand}.
     * @param ctx the parse tree
     */
    fn visit_commitCommand(&mut self, ctx: &CommitCommandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedProcedureSpecification(
        &mut self,
        ctx: &NestedProcedureSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_procedureSpecification(
        &mut self,
        ctx: &ProcedureSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &NestedDataModifyingProcedureSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedQuerySpecification(
        &mut self,
        ctx: &NestedQuerySpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureBody}.
     * @param ctx the parse tree
     */
    fn visit_procedureBody(&mut self, ctx: &ProcedureBodyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableDefinitionBlock}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableDefinitionBlock(
        &mut self,
        ctx: &BindingVariableDefinitionBlockContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableDefinition(
        &mut self,
        ctx: &BindingVariableDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#statementBlock}.
     * @param ctx the parse tree
     */
    fn visit_statementBlock(&mut self, ctx: &StatementBlockContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#statement}.
     * @param ctx the parse tree
     */
    fn visit_statement(&mut self, ctx: &StatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nextStatement}.
     * @param ctx the parse tree
     */
    fn visit_nextStatement(&mut self, ctx: &NextStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_graphVariableDefinition(
        &mut self,
        ctx: &GraphVariableDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedGraphInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedGraphInitializer(
        &mut self,
        ctx: &OptTypedGraphInitializerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphInitializer}.
     * @param ctx the parse tree
     */
    fn visit_graphInitializer(&mut self, ctx: &GraphInitializerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableVariableDefinition(
        &mut self,
        ctx: &BindingTableVariableDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedBindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedBindingTableInitializer(
        &mut self,
        ctx: &OptTypedBindingTableInitializerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableInitializer}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableInitializer(
        &mut self,
        ctx: &BindingTableInitializerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_valueVariableDefinition(
        &mut self,
        ctx: &ValueVariableDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optTypedValueInitializer}.
     * @param ctx the parse tree
     */
    fn visit_optTypedValueInitializer(
        &mut self,
        ctx: &OptTypedValueInitializerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueInitializer}.
     * @param ctx the parse tree
     */
    fn visit_valueInitializer(&mut self, ctx: &ValueInitializerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphExpression}.
     * @param ctx the parse tree
     */
    fn visit_graphExpression(&mut self, ctx: &GraphExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#currentGraph}.
     * @param ctx the parse tree
     */
    fn visit_currentGraph(&mut self, ctx: &CurrentGraphContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableExpression}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableExpression(
        &mut self,
        ctx: &BindingTableExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedBindingTableQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedBindingTableQuerySpecification(
        &mut self,
        ctx: &NestedBindingTableQuerySpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_objectExpressionPrimary(
        &mut self,
        ctx: &ObjectExpressionPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearCatalogModifyingStatement(
        &mut self,
        ctx: &LinearCatalogModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleCatalogModifyingStatement(
        &mut self,
        ctx: &SimpleCatalogModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveCatalogModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveCatalogModifyingStatement(
        &mut self,
        ctx: &PrimitiveCatalogModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createSchemaStatement}.
     * @param ctx the parse tree
     */
    fn visit_createSchemaStatement(
        &mut self,
        ctx: &CreateSchemaStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropSchemaStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropSchemaStatement(
        &mut self,
        ctx: &DropSchemaStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createGraphStatement}.
     * @param ctx the parse tree
     */
    fn visit_createGraphStatement(
        &mut self,
        ctx: &CreateGraphStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openGraphType}.
     * @param ctx the parse tree
     */
    fn visit_openGraphType(&mut self, ctx: &OpenGraphTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ofGraphType}.
     * @param ctx the parse tree
     */
    fn visit_ofGraphType(&mut self, ctx: &OfGraphTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeLikeGraph}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeLikeGraph(
        &mut self,
        ctx: &GraphTypeLikeGraphContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphSource}.
     * @param ctx the parse tree
     */
    fn visit_graphSource(&mut self, ctx: &GraphSourceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropGraphStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropGraphStatement(
        &mut self,
        ctx: &DropGraphStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#createGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn visit_createGraphTypeStatement(
        &mut self,
        ctx: &CreateGraphTypeStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeSource}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeSource(&mut self, ctx: &GraphTypeSourceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#copyOfGraphType}.
     * @param ctx the parse tree
     */
    fn visit_copyOfGraphType(&mut self, ctx: &CopyOfGraphTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dropGraphTypeStatement}.
     * @param ctx the parse tree
     */
    fn visit_dropGraphTypeStatement(
        &mut self,
        ctx: &DropGraphTypeStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callCatalogModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callCatalogModifyingProcedureStatement(
        &mut self,
        ctx: &CallCatalogModifyingProcedureStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearDataModifyingStatement(
        &mut self,
        ctx: &LinearDataModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearDataModifyingStatement(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearDataModifyingStatementBody(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedNestedDataModifyingProcedureSpecification}.
     * @param ctx the parse tree
     */
    fn visit_focusedNestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearDataModifyingStatement(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearDataModifyingStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearDataModifyingStatementBody(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleLinearDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleLinearDataAccessingStatement(
        &mut self,
        ctx: &SimpleLinearDataAccessingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDataAccessingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleDataAccessingStatement(
        &mut self,
        ctx: &SimpleDataAccessingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleDataModifyingStatement(
        &mut self,
        ctx: &SimpleDataModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveDataModifyingStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveDataModifyingStatement(
        &mut self,
        ctx: &PrimitiveDataModifyingStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertStatement}.
     * @param ctx the parse tree
     */
    fn visit_insertStatement(&mut self, ctx: &InsertStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setStatement}.
     * @param ctx the parse tree
     */
    fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setItemList}.
     * @param ctx the parse tree
     */
    fn visit_setItemList(&mut self, ctx: &SetItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setItem}.
     * @param ctx the parse tree
     */
    fn visit_setItem(&mut self, ctx: &SetItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setPropertyItem}.
     * @param ctx the parse tree
     */
    fn visit_setPropertyItem(&mut self, ctx: &SetPropertyItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setAllPropertiesItem}.
     * @param ctx the parse tree
     */
    fn visit_setAllPropertiesItem(
        &mut self,
        ctx: &SetAllPropertiesItemContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setLabelItem}.
     * @param ctx the parse tree
     */
    fn visit_setLabelItem(&mut self, ctx: &SetLabelItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeStatement}.
     * @param ctx the parse tree
     */
    fn visit_removeStatement(&mut self, ctx: &RemoveStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeItemList}.
     * @param ctx the parse tree
     */
    fn visit_removeItemList(&mut self, ctx: &RemoveItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeItem}.
     * @param ctx the parse tree
     */
    fn visit_removeItem(&mut self, ctx: &RemoveItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removePropertyItem}.
     * @param ctx the parse tree
     */
    fn visit_removePropertyItem(
        &mut self,
        ctx: &RemovePropertyItemContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#removeLabelItem}.
     * @param ctx the parse tree
     */
    fn visit_removeLabelItem(&mut self, ctx: &RemoveLabelItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteStatement}.
     * @param ctx the parse tree
     */
    fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteItemList}.
     * @param ctx the parse tree
     */
    fn visit_deleteItemList(&mut self, ctx: &DeleteItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#deleteItem}.
     * @param ctx the parse tree
     */
    fn visit_deleteItem(&mut self, ctx: &DeleteItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callDataModifyingProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callDataModifyingProcedureStatement(
        &mut self,
        ctx: &CallDataModifyingProcedureStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryStatement(
        &mut self,
        ctx: &CompositeQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryExpression}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryExpression(
        &mut self,
        ctx: &CompositeQueryExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#queryConjunction}.
     * @param ctx the parse tree
     */
    fn visit_queryConjunction(&mut self, ctx: &QueryConjunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setOperator}.
     * @param ctx the parse tree
     */
    fn visit_setOperator(&mut self, ctx: &SetOperatorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compositeQueryPrimary}.
     * @param ctx the parse tree
     */
    fn visit_compositeQueryPrimary(
        &mut self,
        ctx: &CompositeQueryPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#linearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_linearQueryStatement(
        &mut self,
        ctx: &LinearQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryStatement(
        &mut self,
        ctx: &FocusedLinearQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryStatementPart}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryStatementPartContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedLinearQueryAndPrimitiveResultStatementPart}.
     * @param ctx the parse tree
     */
    fn visit_focusedLinearQueryAndPrimitiveResultStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedPrimitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn visit_focusedPrimitiveResultStatement(
        &mut self,
        ctx: &FocusedPrimitiveResultStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#focusedNestedQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_focusedNestedQuerySpecification(
        &mut self,
        ctx: &FocusedNestedQuerySpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ambientLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_ambientLinearQueryStatement(
        &mut self,
        ctx: &AmbientLinearQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleLinearQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleLinearQueryStatement(
        &mut self,
        ctx: &SimpleLinearQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleQueryStatement(
        &mut self,
        ctx: &SimpleQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveQueryStatement(
        &mut self,
        ctx: &PrimitiveQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchStatement}.
     * @param ctx the parse tree
     */
    fn visit_matchStatement(&mut self, ctx: &MatchStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleMatchStatement}.
     * @param ctx the parse tree
     */
    fn visit_simpleMatchStatement(
        &mut self,
        ctx: &SimpleMatchStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optionalMatchStatement}.
     * @param ctx the parse tree
     */
    fn visit_optionalMatchStatement(
        &mut self,
        ctx: &OptionalMatchStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#optionalOperand}.
     * @param ctx the parse tree
     */
    fn visit_optionalOperand(&mut self, ctx: &OptionalOperandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchStatementBlock}.
     * @param ctx the parse tree
     */
    fn visit_matchStatementBlock(
        &mut self,
        ctx: &MatchStatementBlockContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callQueryStatement}.
     * @param ctx the parse tree
     */
    fn visit_callQueryStatement(
        &mut self,
        ctx: &CallQueryStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#filterStatement}.
     * @param ctx the parse tree
     */
    fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letStatement}.
     * @param ctx the parse tree
     */
    fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letVariableDefinitionList}.
     * @param ctx the parse tree
     */
    fn visit_letVariableDefinitionList(
        &mut self,
        ctx: &LetVariableDefinitionListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letVariableDefinition}.
     * @param ctx the parse tree
     */
    fn visit_letVariableDefinition(
        &mut self,
        ctx: &LetVariableDefinitionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forStatement}.
     * @param ctx the parse tree
     */
    fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItem}.
     * @param ctx the parse tree
     */
    fn visit_forItem(&mut self, ctx: &ForItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_forItemAlias(&mut self, ctx: &ForItemAliasContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forItemSource}.
     * @param ctx the parse tree
     */
    fn visit_forItemSource(&mut self, ctx: &ForItemSourceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#forOrdinalityOrOffset}.
     * @param ctx the parse tree
     */
    fn visit_forOrdinalityOrOffset(
        &mut self,
        ctx: &ForOrdinalityOrOffsetContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderByAndPageStatement}.
     * @param ctx the parse tree
     */
    fn visit_orderByAndPageStatement(
        &mut self,
        ctx: &OrderByAndPageStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#primitiveResultStatement}.
     * @param ctx the parse tree
     */
    fn visit_primitiveResultStatement(
        &mut self,
        ctx: &PrimitiveResultStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnStatement}.
     * @param ctx the parse tree
     */
    fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_returnStatementBody(
        &mut self,
        ctx: &ReturnStatementBodyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItemList}.
     * @param ctx the parse tree
     */
    fn visit_returnItemList(&mut self, ctx: &ReturnItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItem}.
     * @param ctx the parse tree
     */
    fn visit_returnItem(&mut self, ctx: &ReturnItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#returnItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_returnItemAlias(&mut self, ctx: &ReturnItemAliasContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectStatement}.
     * @param ctx the parse tree
     */
    fn visit_selectStatement(&mut self, ctx: &SelectStatementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItemList}.
     * @param ctx the parse tree
     */
    fn visit_selectItemList(&mut self, ctx: &SelectItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItem}.
     * @param ctx the parse tree
     */
    fn visit_selectItem(&mut self, ctx: &SelectItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_selectItemAlias(&mut self, ctx: &SelectItemAliasContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#havingClause}.
     * @param ctx the parse tree
     */
    fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectStatementBody}.
     * @param ctx the parse tree
     */
    fn visit_selectStatementBody(
        &mut self,
        ctx: &SelectStatementBodyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectGraphMatchList}.
     * @param ctx the parse tree
     */
    fn visit_selectGraphMatchList(
        &mut self,
        ctx: &SelectGraphMatchListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectGraphMatch}.
     * @param ctx the parse tree
     */
    fn visit_selectGraphMatch(&mut self, ctx: &SelectGraphMatchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#selectQuerySpecification}.
     * @param ctx the parse tree
     */
    fn visit_selectQuerySpecification(
        &mut self,
        ctx: &SelectQuerySpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#callProcedureStatement}.
     * @param ctx the parse tree
     */
    fn visit_callProcedureStatement(
        &mut self,
        ctx: &CallProcedureStatementContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureCall}.
     * @param ctx the parse tree
     */
    fn visit_procedureCall(&mut self, ctx: &ProcedureCallContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#inlineProcedureCall}.
     * @param ctx the parse tree
     */
    fn visit_inlineProcedureCall(
        &mut self,
        ctx: &InlineProcedureCallContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#variableScopeClause}.
     * @param ctx the parse tree
     */
    fn visit_variableScopeClause(
        &mut self,
        ctx: &VariableScopeClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableReferenceList}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableReferenceList(
        &mut self,
        ctx: &BindingVariableReferenceListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#namedProcedureCall}.
     * @param ctx the parse tree
     */
    fn visit_namedProcedureCall(
        &mut self,
        ctx: &NamedProcedureCallContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureArgumentList}.
     * @param ctx the parse tree
     */
    fn visit_procedureArgumentList(
        &mut self,
        ctx: &ProcedureArgumentListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureArgument}.
     * @param ctx the parse tree
     */
    fn visit_procedureArgument(&mut self, ctx: &ProcedureArgumentContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#atSchemaClause}.
     * @param ctx the parse tree
     */
    fn visit_atSchemaClause(&mut self, ctx: &AtSchemaClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#useGraphClause}.
     * @param ctx the parse tree
     */
    fn visit_useGraphClause(&mut self, ctx: &UseGraphClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternBindingTable}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternBindingTable(
        &mut self,
        ctx: &GraphPatternBindingTableContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldClause}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldClause(
        &mut self,
        ctx: &GraphPatternYieldClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldItemList}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldItemList(
        &mut self,
        ctx: &GraphPatternYieldItemListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternYieldItem}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternYieldItem(
        &mut self,
        ctx: &GraphPatternYieldItemContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPattern}.
     * @param ctx the parse tree
     */
    fn visit_graphPattern(&mut self, ctx: &GraphPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#matchMode}.
     * @param ctx the parse tree
     */
    fn visit_matchMode(&mut self, ctx: &MatchModeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#repeatableElementsMatchMode}.
     * @param ctx the parse tree
     */
    fn visit_repeatableElementsMatchMode(
        &mut self,
        ctx: &RepeatableElementsMatchModeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#differentEdgesMatchMode}.
     * @param ctx the parse tree
     */
    fn visit_differentEdgesMatchMode(
        &mut self,
        ctx: &DifferentEdgesMatchModeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementBindingsOrElements}.
     * @param ctx the parse tree
     */
    fn visit_elementBindingsOrElements(
        &mut self,
        ctx: &ElementBindingsOrElementsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeBindingsOrEdges}.
     * @param ctx the parse tree
     */
    fn visit_edgeBindingsOrEdges(
        &mut self,
        ctx: &EdgeBindingsOrEdgesContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPatternList}.
     * @param ctx the parse tree
     */
    fn visit_pathPatternList(&mut self, ctx: &PathPatternListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPattern}.
     * @param ctx the parse tree
     */
    fn visit_pathPattern(&mut self, ctx: &PathPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_pathVariableDeclaration(
        &mut self,
        ctx: &PathVariableDeclarationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#keepClause}.
     * @param ctx the parse tree
     */
    fn visit_keepClause(&mut self, ctx: &KeepClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternWhereClause(
        &mut self,
        ctx: &GraphPatternWhereClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertGraphPattern}.
     * @param ctx the parse tree
     */
    fn visit_insertGraphPattern(
        &mut self,
        ctx: &InsertGraphPatternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertPathPatternList}.
     * @param ctx the parse tree
     */
    fn visit_insertPathPatternList(
        &mut self,
        ctx: &InsertPathPatternListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertPathPattern}.
     * @param ctx the parse tree
     */
    fn visit_insertPathPattern(&mut self, ctx: &InsertPathPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertNodePattern}.
     * @param ctx the parse tree
     */
    fn visit_insertNodePattern(&mut self, ctx: &InsertNodePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePattern(&mut self, ctx: &InsertEdgePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePointingLeft(
        &mut self,
        ctx: &InsertEdgePointingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgePointingRight(
        &mut self,
        ctx: &InsertEdgePointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_insertEdgeUndirected(
        &mut self,
        ctx: &InsertEdgeUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#insertElementPatternFiller}.
     * @param ctx the parse tree
     */
    fn visit_insertElementPatternFiller(
        &mut self,
        ctx: &InsertElementPatternFillerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelAndPropertySetSpecification}.
     * @param ctx the parse tree
     */
    fn visit_labelAndPropertySetSpecification(
        &mut self,
        ctx: &LabelAndPropertySetSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathPatternPrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathPatternPrefix(&mut self, ctx: &PathPatternPrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathModePrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathModePrefix(&mut self, ctx: &PathModePrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathMode}.
     * @param ctx the parse tree
     */
    fn visit_pathMode(&mut self, ctx: &PathModeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathSearchPrefix}.
     * @param ctx the parse tree
     */
    fn visit_pathSearchPrefix(&mut self, ctx: &PathSearchPrefixContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#allPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_allPathSearch(&mut self, ctx: &AllPathSearchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathOrPaths}.
     * @param ctx the parse tree
     */
    fn visit_pathOrPaths(&mut self, ctx: &PathOrPathsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#anyPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_anyPathSearch(&mut self, ctx: &AnyPathSearchContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numberOfPaths}.
     * @param ctx the parse tree
     */
    fn visit_numberOfPaths(&mut self, ctx: &NumberOfPathsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#shortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_shortestPathSearch(
        &mut self,
        ctx: &ShortestPathSearchContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#allShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_allShortestPathSearch(
        &mut self,
        ctx: &AllShortestPathSearchContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#anyShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_anyShortestPathSearch(
        &mut self,
        ctx: &AnyShortestPathSearchContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#countedShortestPathSearch}.
     * @param ctx the parse tree
     */
    fn visit_countedShortestPathSearch(
        &mut self,
        ctx: &CountedShortestPathSearchContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#countedShortestGroupSearch}.
     * @param ctx the parse tree
     */
    fn visit_countedShortestGroupSearch(
        &mut self,
        ctx: &CountedShortestGroupSearchContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numberOfGroups}.
     * @param ctx the parse tree
     */
    fn visit_numberOfGroups(&mut self, ctx: &NumberOfGroupsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppePathTerm}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppePathTerm(&mut self, ctx: &PpePathTermContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppeMultisetAlternation}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppeMultisetAlternation(
        &mut self,
        ctx: &PpeMultisetAlternationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppePatternUnion}
     * labeled alternative in {@link GQLParser#pathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_ppePatternUnion(&mut self, ctx: &PpePatternUnionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathTerm}.
     * @param ctx the parse tree
     */
    fn visit_pathTerm(&mut self, ctx: &PathTermContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfPathPrimary(&mut self, ctx: &PfPathPrimaryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfQuantifiedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfQuantifiedPathPrimary(
        &mut self,
        ctx: &PfQuantifiedPathPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pfQuestionedPathPrimary}
     * labeled alternative in {@link GQLParser#pathFactor}.
     * @param ctx the parse tree
     */
    fn visit_pfQuestionedPathPrimary(
        &mut self,
        ctx: &PfQuestionedPathPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppElementPattern}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppElementPattern(&mut self, ctx: &PpElementPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppParenthesizedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppParenthesizedPathPatternExpression(
        &mut self,
        ctx: &PpParenthesizedPathPatternExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code ppSimplifiedPathPatternExpression}
     * labeled alternative in {@link GQLParser#pathPrimary}.
     * @param ctx the parse tree
     */
    fn visit_ppSimplifiedPathPatternExpression(
        &mut self,
        ctx: &PpSimplifiedPathPatternExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPattern}.
     * @param ctx the parse tree
     */
    fn visit_elementPattern(&mut self, ctx: &ElementPatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodePattern}.
     * @param ctx the parse tree
     */
    fn visit_nodePattern(&mut self, ctx: &NodePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternFiller}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternFiller(
        &mut self,
        ctx: &ElementPatternFillerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_elementVariableDeclaration(
        &mut self,
        ctx: &ElementVariableDeclarationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isLabelExpression}.
     * @param ctx the parse tree
     */
    fn visit_isLabelExpression(&mut self, ctx: &IsLabelExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isOrColon}.
     * @param ctx the parse tree
     */
    fn visit_isOrColon(&mut self, ctx: &IsOrColonContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternPredicate}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternPredicate(
        &mut self,
        ctx: &ElementPatternPredicateContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_elementPatternWhereClause(
        &mut self,
        ctx: &ElementPatternWhereClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementPropertySpecification}.
     * @param ctx the parse tree
     */
    fn visit_elementPropertySpecification(
        &mut self,
        ctx: &ElementPropertySpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyKeyValuePairList}.
     * @param ctx the parse tree
     */
    fn visit_propertyKeyValuePairList(
        &mut self,
        ctx: &PropertyKeyValuePairListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyKeyValuePair}.
     * @param ctx the parse tree
     */
    fn visit_propertyKeyValuePair(
        &mut self,
        ctx: &PropertyKeyValuePairContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgePattern}.
     * @param ctx the parse tree
     */
    fn visit_edgePattern(&mut self, ctx: &EdgePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePattern(&mut self, ctx: &FullEdgePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePointingLeft(
        &mut self,
        ctx: &FullEdgePointingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeUndirected(
        &mut self,
        ctx: &FullEdgeUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgePointingRight(
        &mut self,
        ctx: &FullEdgePointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeLeftOrUndirected(
        &mut self,
        ctx: &FullEdgeLeftOrUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeUndirectedOrRight(
        &mut self,
        ctx: &FullEdgeUndirectedOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeLeftOrRight(
        &mut self,
        ctx: &FullEdgeLeftOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fullEdgeAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_fullEdgeAnyDirection(
        &mut self,
        ctx: &FullEdgeAnyDirectionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#abbreviatedEdgePattern}.
     * @param ctx the parse tree
     */
    fn visit_abbreviatedEdgePattern(
        &mut self,
        ctx: &AbbreviatedEdgePatternContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedPathPatternExpression(
        &mut self,
        ctx: &ParenthesizedPathPatternExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subpathVariableDeclaration}.
     * @param ctx the parse tree
     */
    fn visit_subpathVariableDeclaration(
        &mut self,
        ctx: &SubpathVariableDeclarationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedPathPatternWhereClause}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedPathPatternWhereClause(
        &mut self,
        ctx: &ParenthesizedPathPatternWhereClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionNegation}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionNegation(
        &mut self,
        ctx: &LabelExpressionNegationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionDisjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionDisjunction(
        &mut self,
        ctx: &LabelExpressionDisjunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionParenthesized}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionParenthesized(
        &mut self,
        ctx: &LabelExpressionParenthesizedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionWildcard}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionWildcard(
        &mut self,
        ctx: &LabelExpressionWildcardContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionConjunction}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionConjunction(
        &mut self,
        ctx: &LabelExpressionConjunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code labelExpressionName}
     * labeled alternative in {@link GQLParser#labelExpression}.
     * @param ctx the parse tree
     */
    fn visit_labelExpressionName(
        &mut self,
        ctx: &LabelExpressionNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_pathVariableReference(
        &mut self,
        ctx: &PathVariableReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_elementVariableReference(
        &mut self,
        ctx: &ElementVariableReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphPatternQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_graphPatternQuantifier(
        &mut self,
        ctx: &GraphPatternQuantifierContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fixedQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_fixedQuantifier(&mut self, ctx: &FixedQuantifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_generalQuantifier(&mut self, ctx: &GeneralQuantifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#lowerBound}.
     * @param ctx the parse tree
     */
    fn visit_lowerBound(&mut self, ctx: &LowerBoundContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#upperBound}.
     * @param ctx the parse tree
     */
    fn visit_upperBound(&mut self, ctx: &UpperBoundContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPathPatternExpression}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPathPatternExpression(
        &mut self,
        ctx: &SimplifiedPathPatternExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeft}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeft(
        &mut self,
        ctx: &SimplifiedDefaultingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingRight(
        &mut self,
        ctx: &SimplifiedDefaultingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingLeftOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDefaultingAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDefaultingAnyDirection(
        &mut self,
        ctx: &SimplifiedDefaultingAnyDirectionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedContents}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedContents(
        &mut self,
        ctx: &SimplifiedContentsContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPathUnion}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPathUnion(
        &mut self,
        ctx: &SimplifiedPathUnionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedMultisetAlternation}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedMultisetAlternation(
        &mut self,
        ctx: &SimplifiedMultisetAlternationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedFactorLowLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorLowLabel(
        &mut self,
        ctx: &SimplifiedFactorLowLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedConcatenationLabel}
     * labeled alternative in {@link GQLParser#simplifiedTerm}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedConcatenationLabel(
        &mut self,
        ctx: &SimplifiedConcatenationLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedConjunctionLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedConjunctionLabel(
        &mut self,
        ctx: &SimplifiedConjunctionLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code simplifiedFactorHighLabel}
     * labeled alternative in {@link GQLParser#simplifiedFactorLow}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorHighLabel(
        &mut self,
        ctx: &SimplifiedFactorHighLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedFactorHigh}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedFactorHigh(
        &mut self,
        ctx: &SimplifiedFactorHighContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedQuantified}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedQuantified(
        &mut self,
        ctx: &SimplifiedQuantifiedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedQuestioned}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedQuestioned(
        &mut self,
        ctx: &SimplifiedQuestionedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedTertiary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedTertiary(
        &mut self,
        ctx: &SimplifiedTertiaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedDirectionOverride}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedDirectionOverride(
        &mut self,
        ctx: &SimplifiedDirectionOverrideContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeft}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeft(
        &mut self,
        ctx: &SimplifiedOverrideLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideUndirected(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideRight(
        &mut self,
        ctx: &SimplifiedOverrideRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrUndirected}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideUndirectedOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideLeftOrRight}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideLeftOrRight(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedOverrideAnyDirection}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedOverrideAnyDirection(
        &mut self,
        ctx: &SimplifiedOverrideAnyDirectionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedSecondary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedSecondary(
        &mut self,
        ctx: &SimplifiedSecondaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedNegation}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedNegation(
        &mut self,
        ctx: &SimplifiedNegationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simplifiedPrimary}.
     * @param ctx the parse tree
     */
    fn visit_simplifiedPrimary(&mut self, ctx: &SimplifiedPrimaryContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whereClause}.
     * @param ctx the parse tree
     */
    fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldClause}.
     * @param ctx the parse tree
     */
    fn visit_yieldClause(&mut self, ctx: &YieldClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemList}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemList(&mut self, ctx: &YieldItemListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItem}.
     * @param ctx the parse tree
     */
    fn visit_yieldItem(&mut self, ctx: &YieldItemContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemName}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemName(&mut self, ctx: &YieldItemNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#yieldItemAlias}.
     * @param ctx the parse tree
     */
    fn visit_yieldItemAlias(&mut self, ctx: &YieldItemAliasContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupByClause}.
     * @param ctx the parse tree
     */
    fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupingElementList}.
     * @param ctx the parse tree
     */
    fn visit_groupingElementList(
        &mut self,
        ctx: &GroupingElementListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#groupingElement}.
     * @param ctx the parse tree
     */
    fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#emptyGroupingSet}.
     * @param ctx the parse tree
     */
    fn visit_emptyGroupingSet(&mut self, ctx: &EmptyGroupingSetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderByClause}.
     * @param ctx the parse tree
     */
    fn visit_orderByClause(&mut self, ctx: &OrderByClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortSpecificationList}.
     * @param ctx the parse tree
     */
    fn visit_sortSpecificationList(
        &mut self,
        ctx: &SortSpecificationListContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortSpecification}.
     * @param ctx the parse tree
     */
    fn visit_sortSpecification(&mut self, ctx: &SortSpecificationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sortKey}.
     * @param ctx the parse tree
     */
    fn visit_sortKey(&mut self, ctx: &SortKeyContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#orderingSpecification}.
     * @param ctx the parse tree
     */
    fn visit_orderingSpecification(
        &mut self,
        ctx: &OrderingSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullOrdering}.
     * @param ctx the parse tree
     */
    fn visit_nullOrdering(&mut self, ctx: &NullOrderingContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#limitClause}.
     * @param ctx the parse tree
     */
    fn visit_limitClause(&mut self, ctx: &LimitClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#offsetClause}.
     * @param ctx the parse tree
     */
    fn visit_offsetClause(&mut self, ctx: &OffsetClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#offsetSynonym}.
     * @param ctx the parse tree
     */
    fn visit_offsetSynonym(&mut self, ctx: &OffsetSynonymContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#schemaReference}.
     * @param ctx the parse tree
     */
    fn visit_schemaReference(&mut self, ctx: &SchemaReferenceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_absoluteCatalogSchemaReference(
        &mut self,
        ctx: &AbsoluteCatalogSchemaReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogSchemaParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogSchemaParentAndName(
        &mut self,
        ctx: &CatalogSchemaParentAndNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#relativeCatalogSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_relativeCatalogSchemaReference(
        &mut self,
        ctx: &RelativeCatalogSchemaReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predefinedSchemaReference}.
     * @param ctx the parse tree
     */
    fn visit_predefinedSchemaReference(
        &mut self,
        ctx: &PredefinedSchemaReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_absoluteDirectoryPath(
        &mut self,
        ctx: &AbsoluteDirectoryPathContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#relativeDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_relativeDirectoryPath(
        &mut self,
        ctx: &RelativeDirectoryPathContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleDirectoryPath}.
     * @param ctx the parse tree
     */
    fn visit_simpleDirectoryPath(
        &mut self,
        ctx: &SimpleDirectoryPathContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphReference}.
     * @param ctx the parse tree
     */
    fn visit_graphReference(&mut self, ctx: &GraphReferenceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogGraphParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogGraphParentAndName(
        &mut self,
        ctx: &CatalogGraphParentAndNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#homeGraph}.
     * @param ctx the parse tree
     */
    fn visit_homeGraph(&mut self, ctx: &HomeGraphContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeReference(
        &mut self,
        ctx: &GraphTypeReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogGraphTypeParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogGraphTypeParentAndName(
        &mut self,
        ctx: &CatalogGraphTypeParentAndNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableReference}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableReference(
        &mut self,
        ctx: &BindingTableReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureReference}.
     * @param ctx the parse tree
     */
    fn visit_procedureReference(
        &mut self,
        ctx: &ProcedureReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogProcedureParentAndName}.
     * @param ctx the parse tree
     */
    fn visit_catalogProcedureParentAndName(
        &mut self,
        ctx: &CatalogProcedureParentAndNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#catalogObjectParentReference}.
     * @param ctx the parse tree
     */
    fn visit_catalogObjectParentReference(
        &mut self,
        ctx: &CatalogObjectParentReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#referenceParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_referenceParameterSpecification(
        &mut self,
        ctx: &ReferenceParameterSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nestedGraphTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nestedGraphTypeSpecification(
        &mut self,
        ctx: &NestedGraphTypeSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeSpecificationBody}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeSpecificationBody(
        &mut self,
        ctx: &GraphTypeSpecificationBodyContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementTypeList}.
     * @param ctx the parse tree
     */
    fn visit_elementTypeList(&mut self, ctx: &ElementTypeListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_elementTypeSpecification(
        &mut self,
        ctx: &ElementTypeSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeSpecification(
        &mut self,
        ctx: &NodeTypeSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePattern}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePattern(&mut self, ctx: &NodeTypePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePhrase}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePhrase(&mut self, ctx: &NodeTypePhraseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePhraseFiller(
        &mut self,
        ctx: &NodeTypePhraseFillerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeFiller}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeFiller(&mut self, ctx: &NodeTypeFillerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_localNodeTypeAlias(
        &mut self,
        ctx: &LocalNodeTypeAliasContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeImpliedContent(
        &mut self,
        ctx: &NodeTypeImpliedContentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeKeyLabelSet(
        &mut self,
        ctx: &NodeTypeKeyLabelSetContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeLabelSet(&mut self, ctx: &NodeTypeLabelSetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypePropertyTypes(
        &mut self,
        ctx: &NodeTypePropertyTypesContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeSpecification}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeSpecification(
        &mut self,
        ctx: &EdgeTypeSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePattern}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePattern(&mut self, ctx: &EdgeTypePatternContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePhrase}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePhrase(&mut self, ctx: &EdgeTypePhraseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePhraseFiller}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePhraseFiller(
        &mut self,
        ctx: &EdgeTypePhraseFillerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeFiller}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeFiller(&mut self, ctx: &EdgeTypeFillerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeImpliedContent}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeImpliedContent(
        &mut self,
        ctx: &EdgeTypeImpliedContentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeKeyLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeKeyLabelSet(
        &mut self,
        ctx: &EdgeTypeKeyLabelSetContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeLabelSet}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeLabelSet(&mut self, ctx: &EdgeTypeLabelSetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePropertyTypes}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePropertyTypes(
        &mut self,
        ctx: &EdgeTypePropertyTypesContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternDirected}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternDirected(
        &mut self,
        ctx: &EdgeTypePatternDirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternPointingRight(
        &mut self,
        ctx: &EdgeTypePatternPointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternPointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternPointingLeft(
        &mut self,
        ctx: &EdgeTypePatternPointingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypePatternUndirected}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypePatternUndirected(
        &mut self,
        ctx: &EdgeTypePatternUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypePointingRight}.
     * @param ctx the parse tree
     */
    fn visit_arcTypePointingRight(
        &mut self,
        ctx: &ArcTypePointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypePointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_arcTypePointingLeft(
        &mut self,
        ctx: &ArcTypePointingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#arcTypeUndirected}.
     * @param ctx the parse tree
     */
    fn visit_arcTypeUndirected(&mut self, ctx: &ArcTypeUndirectedContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_sourceNodeTypeReference(
        &mut self,
        ctx: &SourceNodeTypeReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationNodeTypeReference}.
     * @param ctx the parse tree
     */
    fn visit_destinationNodeTypeReference(
        &mut self,
        ctx: &DestinationNodeTypeReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeKind}.
     * @param ctx the parse tree
     */
    fn visit_edgeKind(&mut self, ctx: &EdgeKindContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPhrase}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPhrase(
        &mut self,
        ctx: &EndpointPairPhraseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPair}.
     * @param ctx the parse tree
     */
    fn visit_endpointPair(&mut self, ctx: &EndpointPairContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairDirected}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairDirected(
        &mut self,
        ctx: &EndpointPairDirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPointingRight(
        &mut self,
        ctx: &EndpointPairPointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairPointingLeft}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairPointingLeft(
        &mut self,
        ctx: &EndpointPairPointingLeftContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#endpointPairUndirected}.
     * @param ctx the parse tree
     */
    fn visit_endpointPairUndirected(
        &mut self,
        ctx: &EndpointPairUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#connectorPointingRight}.
     * @param ctx the parse tree
     */
    fn visit_connectorPointingRight(
        &mut self,
        ctx: &ConnectorPointingRightContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#connectorUndirected}.
     * @param ctx the parse tree
     */
    fn visit_connectorUndirected(
        &mut self,
        ctx: &ConnectorUndirectedContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_sourceNodeTypeAlias(
        &mut self,
        ctx: &SourceNodeTypeAliasContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationNodeTypeAlias}.
     * @param ctx the parse tree
     */
    fn visit_destinationNodeTypeAlias(
        &mut self,
        ctx: &DestinationNodeTypeAliasContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelSetPhrase}.
     * @param ctx the parse tree
     */
    fn visit_labelSetPhrase(&mut self, ctx: &LabelSetPhraseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelSetSpecification}.
     * @param ctx the parse tree
     */
    fn visit_labelSetSpecification(
        &mut self,
        ctx: &LabelSetSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyTypesSpecification}.
     * @param ctx the parse tree
     */
    fn visit_propertyTypesSpecification(
        &mut self,
        ctx: &PropertyTypesSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyTypeList}.
     * @param ctx the parse tree
     */
    fn visit_propertyTypeList(&mut self, ctx: &PropertyTypeListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyType}.
     * @param ctx the parse tree
     */
    fn visit_propertyType(&mut self, ctx: &PropertyTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyValueType}.
     * @param ctx the parse tree
     */
    fn visit_propertyValueType(&mut self, ctx: &PropertyValueTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableType}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableType(&mut self, ctx: &BindingTableTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code dynamicPropertyValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_dynamicPropertyValueTypeLabel(
        &mut self,
        ctx: &DynamicPropertyValueTypeLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code closedDynamicUnionTypeAtl1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_closedDynamicUnionTypeAtl1(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl1Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code closedDynamicUnionTypeAtl2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_closedDynamicUnionTypeAtl2(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code pathValueTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_pathValueTypeLabel(
        &mut self,
        ctx: &PathValueTypeLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt3}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt3(&mut self, ctx: &ListValueTypeAlt3Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt2}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt2(&mut self, ctx: &ListValueTypeAlt2Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code listValueTypeAlt1}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeAlt1(&mut self, ctx: &ListValueTypeAlt1Context<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code predefinedTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_predefinedTypeLabel(
        &mut self,
        ctx: &PredefinedTypeLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code recordTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_recordTypeLabel(&mut self, ctx: &RecordTypeLabelContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code openDynamicUnionTypeLabel}
     * labeled alternative in {@link GQLParser#valueType}.
     * @param ctx the parse tree
     */
    fn visit_openDynamicUnionTypeLabel(
        &mut self,
        ctx: &OpenDynamicUnionTypeLabelContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#typed}.
     * @param ctx the parse tree
     */
    fn visit_typed(&mut self, ctx: &TypedContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predefinedType}.
     * @param ctx the parse tree
     */
    fn visit_predefinedType(&mut self, ctx: &PredefinedTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#booleanType}.
     * @param ctx the parse tree
     */
    fn visit_booleanType(&mut self, ctx: &BooleanTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringType}.
     * @param ctx the parse tree
     */
    fn visit_characterStringType(
        &mut self,
        ctx: &CharacterStringTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteStringType}.
     * @param ctx the parse tree
     */
    fn visit_byteStringType(&mut self, ctx: &ByteStringTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#minLength}.
     * @param ctx the parse tree
     */
    fn visit_minLength(&mut self, ctx: &MinLengthContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#maxLength}.
     * @param ctx the parse tree
     */
    fn visit_maxLength(&mut self, ctx: &MaxLengthContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fixedLength}.
     * @param ctx the parse tree
     */
    fn visit_fixedLength(&mut self, ctx: &FixedLengthContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericType}.
     * @param ctx the parse tree
     */
    fn visit_numericType(&mut self, ctx: &NumericTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_exactNumericType(&mut self, ctx: &ExactNumericTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_binaryExactNumericType(
        &mut self,
        ctx: &BinaryExactNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#signedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_signedBinaryExactNumericType(
        &mut self,
        ctx: &SignedBinaryExactNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_unsignedBinaryExactNumericType(
        &mut self,
        ctx: &UnsignedBinaryExactNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#verboseBinaryExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_verboseBinaryExactNumericType(
        &mut self,
        ctx: &VerboseBinaryExactNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#decimalExactNumericType}.
     * @param ctx the parse tree
     */
    fn visit_decimalExactNumericType(
        &mut self,
        ctx: &DecimalExactNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#precision}.
     * @param ctx the parse tree
     */
    fn visit_precision(&mut self, ctx: &PrecisionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#scale}.
     * @param ctx the parse tree
     */
    fn visit_scale(&mut self, ctx: &ScaleContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#approximateNumericType}.
     * @param ctx the parse tree
     */
    fn visit_approximateNumericType(
        &mut self,
        ctx: &ApproximateNumericTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalType}.
     * @param ctx the parse tree
     */
    fn visit_temporalType(&mut self, ctx: &TemporalTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalInstantType}.
     * @param ctx the parse tree
     */
    fn visit_temporalInstantType(
        &mut self,
        ctx: &TemporalInstantTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeType}.
     * @param ctx the parse tree
     */
    fn visit_datetimeType(&mut self, ctx: &DatetimeTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localdatetimeType}.
     * @param ctx the parse tree
     */
    fn visit_localdatetimeType(&mut self, ctx: &LocaldatetimeTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateType}.
     * @param ctx the parse tree
     */
    fn visit_dateType(&mut self, ctx: &DateTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeType}.
     * @param ctx the parse tree
     */
    fn visit_timeType(&mut self, ctx: &TimeTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localtimeType}.
     * @param ctx the parse tree
     */
    fn visit_localtimeType(&mut self, ctx: &LocaltimeTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalDurationType}.
     * @param ctx the parse tree
     */
    fn visit_temporalDurationType(
        &mut self,
        ctx: &TemporalDurationTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalDurationQualifier}.
     * @param ctx the parse tree
     */
    fn visit_temporalDurationQualifier(
        &mut self,
        ctx: &TemporalDurationQualifierContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#referenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_referenceValueType(
        &mut self,
        ctx: &ReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#immaterialValueType}.
     * @param ctx the parse tree
     */
    fn visit_immaterialValueType(
        &mut self,
        ctx: &ImmaterialValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullType}.
     * @param ctx the parse tree
     */
    fn visit_nullType(&mut self, ctx: &NullTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#emptyType}.
     * @param ctx the parse tree
     */
    fn visit_emptyType(&mut self, ctx: &EmptyTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_graphReferenceValueType(
        &mut self,
        ctx: &GraphReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedGraphReferenceValueType(
        &mut self,
        ctx: &ClosedGraphReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openGraphReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openGraphReferenceValueType(
        &mut self,
        ctx: &OpenGraphReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableReferenceValueType(
        &mut self,
        ctx: &BindingTableReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_nodeReferenceValueType(
        &mut self,
        ctx: &NodeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedNodeReferenceValueType(
        &mut self,
        ctx: &ClosedNodeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openNodeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openNodeReferenceValueType(
        &mut self,
        ctx: &OpenNodeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_edgeReferenceValueType(
        &mut self,
        ctx: &EdgeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#closedEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_closedEdgeReferenceValueType(
        &mut self,
        ctx: &ClosedEdgeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#openEdgeReferenceValueType}.
     * @param ctx the parse tree
     */
    fn visit_openEdgeReferenceValueType(
        &mut self,
        ctx: &OpenEdgeReferenceValueTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueType}.
     * @param ctx the parse tree
     */
    fn visit_pathValueType(&mut self, ctx: &PathValueTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueTypeName}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeName(&mut self, ctx: &ListValueTypeNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueTypeNameSynonym}.
     * @param ctx the parse tree
     */
    fn visit_listValueTypeNameSynonym(
        &mut self,
        ctx: &ListValueTypeNameSynonymContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordType}.
     * @param ctx the parse tree
     */
    fn visit_recordType(&mut self, ctx: &RecordTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldTypesSpecification}.
     * @param ctx the parse tree
     */
    fn visit_fieldTypesSpecification(
        &mut self,
        ctx: &FieldTypesSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldTypeList}.
     * @param ctx the parse tree
     */
    fn visit_fieldTypeList(&mut self, ctx: &FieldTypeListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#notNull}.
     * @param ctx the parse tree
     */
    fn visit_notNull(&mut self, ctx: &NotNullContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldType}.
     * @param ctx the parse tree
     */
    fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchCondition}.
     * @param ctx the parse tree
     */
    fn visit_searchCondition(&mut self, ctx: &SearchConditionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#predicate}.
     * @param ctx the parse tree
     */
    fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#compOp}.
     * @param ctx the parse tree
     */
    fn visit_compOp(&mut self, ctx: &CompOpContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#existsPredicate}.
     * @param ctx the parse tree
     */
    fn visit_existsPredicate(&mut self, ctx: &ExistsPredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullPredicate}.
     * @param ctx the parse tree
     */
    fn visit_nullPredicate(&mut self, ctx: &NullPredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_nullPredicatePart2(
        &mut self,
        ctx: &NullPredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueTypePredicate}.
     * @param ctx the parse tree
     */
    fn visit_valueTypePredicate(
        &mut self,
        ctx: &ValueTypePredicateContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueTypePredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_valueTypePredicatePart2(
        &mut self,
        ctx: &ValueTypePredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalizedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_normalizedPredicatePart2(
        &mut self,
        ctx: &NormalizedPredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directedPredicate}.
     * @param ctx the parse tree
     */
    fn visit_directedPredicate(&mut self, ctx: &DirectedPredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directedPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_directedPredicatePart2(
        &mut self,
        ctx: &DirectedPredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labeledPredicate}.
     * @param ctx the parse tree
     */
    fn visit_labeledPredicate(&mut self, ctx: &LabeledPredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labeledPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_labeledPredicatePart2(
        &mut self,
        ctx: &LabeledPredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#isLabeledOrColon}.
     * @param ctx the parse tree
     */
    fn visit_isLabeledOrColon(&mut self, ctx: &IsLabeledOrColonContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourceDestinationPredicate}.
     * @param ctx the parse tree
     */
    fn visit_sourceDestinationPredicate(
        &mut self,
        ctx: &SourceDestinationPredicateContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReference}.
     * @param ctx the parse tree
     */
    fn visit_nodeReference(&mut self, ctx: &NodeReferenceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#sourcePredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_sourcePredicatePart2(
        &mut self,
        ctx: &SourcePredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#destinationPredicatePart2}.
     * @param ctx the parse tree
     */
    fn visit_destinationPredicatePart2(
        &mut self,
        ctx: &DestinationPredicatePart2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReference}.
     * @param ctx the parse tree
     */
    fn visit_edgeReference(&mut self, ctx: &EdgeReferenceContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#all_differentPredicate}.
     * @param ctx the parse tree
     */
    fn visit_all_differentPredicate(
        &mut self,
        ctx: &All_differentPredicateContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#samePredicate}.
     * @param ctx the parse tree
     */
    fn visit_samePredicate(&mut self, ctx: &SamePredicateContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#property_existsPredicate}.
     * @param ctx the parse tree
     */
    fn visit_property_existsPredicate(
        &mut self,
        ctx: &Property_existsPredicateContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code conjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_conjunctiveExprAlt(
        &mut self,
        ctx: &ConjunctiveExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code propertyGraphExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_propertyGraphExprAlt(
        &mut self,
        ctx: &PropertyGraphExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code multDivExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_multDivExprAlt(&mut self, ctx: &MultDivExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code bindingTableExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableExprAlt(
        &mut self,
        ctx: &BindingTableExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code signedExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_signedExprAlt(&mut self, ctx: &SignedExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code isNotExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_isNotExprAlt(&mut self, ctx: &IsNotExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code normalizedPredicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_normalizedPredicateExprAlt(
        &mut self,
        ctx: &NormalizedPredicateExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code notExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_notExprAlt(&mut self, ctx: &NotExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code valueFunctionExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_valueFunctionExprAlt(
        &mut self,
        ctx: &ValueFunctionExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code concatenationExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_concatenationExprAlt(
        &mut self,
        ctx: &ConcatenationExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code disjunctiveExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_disjunctiveExprAlt(
        &mut self,
        ctx: &DisjunctiveExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code comparisonExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_comparisonExprAlt(&mut self, ctx: &ComparisonExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code primaryExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_primaryExprAlt(&mut self, ctx: &PrimaryExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code addSubtractExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_addSubtractExprAlt(
        &mut self,
        ctx: &AddSubtractExprAltContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by the {@code predicateExprAlt}
     * labeled alternative in {@link GQLParser#valueExpression}.
     * @param ctx the parse tree
     */
    fn visit_predicateExprAlt(&mut self, ctx: &PredicateExprAltContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueFunction}.
     * @param ctx the parse tree
     */
    fn visit_valueFunction(&mut self, ctx: &ValueFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#booleanValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_booleanValueExpression(
        &mut self,
        ctx: &BooleanValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterOrByteStringFunction}.
     * @param ctx the parse tree
     */
    fn visit_characterOrByteStringFunction(
        &mut self,
        ctx: &CharacterOrByteStringFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_subCharacterOrByteString(
        &mut self,
        ctx: &SubCharacterOrByteStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimSingleCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_trimSingleCharacterOrByteString(
        &mut self,
        ctx: &TrimSingleCharacterOrByteStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#foldCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_foldCharacterString(
        &mut self,
        ctx: &FoldCharacterStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimMultiCharacterCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_trimMultiCharacterCharacterString(
        &mut self,
        ctx: &TrimMultiCharacterCharacterStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalizeCharacterString}.
     * @param ctx the parse tree
     */
    fn visit_normalizeCharacterString(
        &mut self,
        ctx: &NormalizeCharacterStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_nodeReferenceValueExpression(
        &mut self,
        ctx: &NodeReferenceValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeReferenceValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_edgeReferenceValueExpression(
        &mut self,
        ctx: &EdgeReferenceValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#aggregatingValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_aggregatingValueExpression(
        &mut self,
        ctx: &AggregatingValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_valueExpressionPrimary(
        &mut self,
        ctx: &ValueExpressionPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#parenthesizedValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_parenthesizedValueExpression(
        &mut self,
        ctx: &ParenthesizedValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimary}.
     * @param ctx the parse tree
     */
    fn visit_nonParenthesizedValueExpressionPrimary(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimaryContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonParenthesizedValueExpressionPrimarySpecialCase}.
     * @param ctx the parse tree
     */
    fn visit_nonParenthesizedValueExpressionPrimarySpecialCase(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimarySpecialCaseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedValueSpecification}.
     * @param ctx the parse tree
     */
    fn visit_unsignedValueSpecification(
        &mut self,
        ctx: &UnsignedValueSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonNegativeIntegerSpecification}.
     * @param ctx the parse tree
     */
    fn visit_nonNegativeIntegerSpecification(
        &mut self,
        ctx: &NonNegativeIntegerSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalValueSpecification}.
     * @param ctx the parse tree
     */
    fn visit_generalValueSpecification(
        &mut self,
        ctx: &GeneralValueSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dynamicParameterSpecification}.
     * @param ctx the parse tree
     */
    fn visit_dynamicParameterSpecification(
        &mut self,
        ctx: &DynamicParameterSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#letValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_letValueExpression(
        &mut self,
        ctx: &LetValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#valueQueryExpression}.
     * @param ctx the parse tree
     */
    fn visit_valueQueryExpression(
        &mut self,
        ctx: &ValueQueryExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseExpression}.
     * @param ctx the parse tree
     */
    fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseAbbreviation}.
     * @param ctx the parse tree
     */
    fn visit_caseAbbreviation(&mut self, ctx: &CaseAbbreviationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseSpecification}.
     * @param ctx the parse tree
     */
    fn visit_caseSpecification(&mut self, ctx: &CaseSpecificationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleCase}.
     * @param ctx the parse tree
     */
    fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchedCase}.
     * @param ctx the parse tree
     */
    fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#simpleWhenClause}.
     * @param ctx the parse tree
     */
    fn visit_simpleWhenClause(&mut self, ctx: &SimpleWhenClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#searchedWhenClause}.
     * @param ctx the parse tree
     */
    fn visit_searchedWhenClause(
        &mut self,
        ctx: &SearchedWhenClauseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elseClause}.
     * @param ctx the parse tree
     */
    fn visit_elseClause(&mut self, ctx: &ElseClauseContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#caseOperand}.
     * @param ctx the parse tree
     */
    fn visit_caseOperand(&mut self, ctx: &CaseOperandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whenOperandList}.
     * @param ctx the parse tree
     */
    fn visit_whenOperandList(&mut self, ctx: &WhenOperandListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#whenOperand}.
     * @param ctx the parse tree
     */
    fn visit_whenOperand(&mut self, ctx: &WhenOperandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#result}.
     * @param ctx the parse tree
     */
    fn visit_result(&mut self, ctx: &ResultContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#resultExpression}.
     * @param ctx the parse tree
     */
    fn visit_resultExpression(&mut self, ctx: &ResultExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castSpecification}.
     * @param ctx the parse tree
     */
    fn visit_castSpecification(&mut self, ctx: &CastSpecificationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castOperand}.
     * @param ctx the parse tree
     */
    fn visit_castOperand(&mut self, ctx: &CastOperandContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#castTarget}.
     * @param ctx the parse tree
     */
    fn visit_castTarget(&mut self, ctx: &CastTargetContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#aggregateFunction}.
     * @param ctx the parse tree
     */
    fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalSetFunction}.
     * @param ctx the parse tree
     */
    fn visit_generalSetFunction(
        &mut self,
        ctx: &GeneralSetFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binarySetFunction}.
     * @param ctx the parse tree
     */
    fn visit_binarySetFunction(&mut self, ctx: &BinarySetFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalSetFunctionType}.
     * @param ctx the parse tree
     */
    fn visit_generalSetFunctionType(
        &mut self,
        ctx: &GeneralSetFunctionTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#setQuantifier}.
     * @param ctx the parse tree
     */
    fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#binarySetFunctionType}.
     * @param ctx the parse tree
     */
    fn visit_binarySetFunctionType(
        &mut self,
        ctx: &BinarySetFunctionTypeContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dependentValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_dependentValueExpression(
        &mut self,
        ctx: &DependentValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#independentValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_independentValueExpression(
        &mut self,
        ctx: &IndependentValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#element_idFunction}.
     * @param ctx the parse tree
     */
    fn visit_element_idFunction(
        &mut self,
        ctx: &Element_idFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariableReference}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariableReference(
        &mut self,
        ctx: &BindingVariableReferenceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_pathValueExpression(
        &mut self,
        ctx: &PathValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueConstructor}.
     * @param ctx the parse tree
     */
    fn visit_pathValueConstructor(
        &mut self,
        ctx: &PathValueConstructorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn visit_pathValueConstructorByEnumeration(
        &mut self,
        ctx: &PathValueConstructorByEnumerationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementList}.
     * @param ctx the parse tree
     */
    fn visit_pathElementList(&mut self, ctx: &PathElementListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementListStart}.
     * @param ctx the parse tree
     */
    fn visit_pathElementListStart(
        &mut self,
        ctx: &PathElementListStartContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathElementListStep}.
     * @param ctx the parse tree
     */
    fn visit_pathElementListStep(
        &mut self,
        ctx: &PathElementListStepContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_listValueExpression(
        &mut self,
        ctx: &ListValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_listValueFunction(&mut self, ctx: &ListValueFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimListFunction}.
     * @param ctx the parse tree
     */
    fn visit_trimListFunction(&mut self, ctx: &TrimListFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementsFunction}.
     * @param ctx the parse tree
     */
    fn visit_elementsFunction(&mut self, ctx: &ElementsFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueConstructor}.
     * @param ctx the parse tree
     */
    fn visit_listValueConstructor(
        &mut self,
        ctx: &ListValueConstructorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listValueConstructorByEnumeration}.
     * @param ctx the parse tree
     */
    fn visit_listValueConstructorByEnumeration(
        &mut self,
        ctx: &ListValueConstructorByEnumerationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listElementList}.
     * @param ctx the parse tree
     */
    fn visit_listElementList(&mut self, ctx: &ListElementListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listElement}.
     * @param ctx the parse tree
     */
    fn visit_listElement(&mut self, ctx: &ListElementContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordConstructor}.
     * @param ctx the parse tree
     */
    fn visit_recordConstructor(&mut self, ctx: &RecordConstructorContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldsSpecification}.
     * @param ctx the parse tree
     */
    fn visit_fieldsSpecification(
        &mut self,
        ctx: &FieldsSpecificationContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldList}.
     * @param ctx the parse tree
     */
    fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#field}.
     * @param ctx the parse tree
     */
    fn visit_field(&mut self, ctx: &FieldContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#truthValue}.
     * @param ctx the parse tree
     */
    fn visit_truthValue(&mut self, ctx: &TruthValueContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpression(
        &mut self,
        ctx: &NumericValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_numericValueFunction(
        &mut self,
        ctx: &NumericValueFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#lengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_lengthExpression(&mut self, ctx: &LengthExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#cardinalityExpression}.
     * @param ctx the parse tree
     */
    fn visit_cardinalityExpression(
        &mut self,
        ctx: &CardinalityExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#cardinalityExpressionArgument}.
     * @param ctx the parse tree
     */
    fn visit_cardinalityExpressionArgument(
        &mut self,
        ctx: &CardinalityExpressionArgumentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#charLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_charLengthExpression(
        &mut self,
        ctx: &CharLengthExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_byteLengthExpression(
        &mut self,
        ctx: &ByteLengthExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathLengthExpression}.
     * @param ctx the parse tree
     */
    fn visit_pathLengthExpression(
        &mut self,
        ctx: &PathLengthExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#absoluteValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_absoluteValueExpression(
        &mut self,
        ctx: &AbsoluteValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#modulusExpression}.
     * @param ctx the parse tree
     */
    fn visit_modulusExpression(&mut self, ctx: &ModulusExpressionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionDividend}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionDividend(
        &mut self,
        ctx: &NumericValueExpressionDividendContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionDivisor}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionDivisor(
        &mut self,
        ctx: &NumericValueExpressionDivisorContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trigonometricFunction}.
     * @param ctx the parse tree
     */
    fn visit_trigonometricFunction(
        &mut self,
        ctx: &TrigonometricFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trigonometricFunctionName}.
     * @param ctx the parse tree
     */
    fn visit_trigonometricFunctionName(
        &mut self,
        ctx: &TrigonometricFunctionNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmFunction}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmFunction(
        &mut self,
        ctx: &GeneralLogarithmFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmBase}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmBase(
        &mut self,
        ctx: &GeneralLogarithmBaseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLogarithmArgument}.
     * @param ctx the parse tree
     */
    fn visit_generalLogarithmArgument(
        &mut self,
        ctx: &GeneralLogarithmArgumentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#commonLogarithm}.
     * @param ctx the parse tree
     */
    fn visit_commonLogarithm(&mut self, ctx: &CommonLogarithmContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#naturalLogarithm}.
     * @param ctx the parse tree
     */
    fn visit_naturalLogarithm(&mut self, ctx: &NaturalLogarithmContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exponentialFunction}.
     * @param ctx the parse tree
     */
    fn visit_exponentialFunction(
        &mut self,
        ctx: &ExponentialFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#powerFunction}.
     * @param ctx the parse tree
     */
    fn visit_powerFunction(&mut self, ctx: &PowerFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionBase}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionBase(
        &mut self,
        ctx: &NumericValueExpressionBaseContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#numericValueExpressionExponent}.
     * @param ctx the parse tree
     */
    fn visit_numericValueExpressionExponent(
        &mut self,
        ctx: &NumericValueExpressionExponentContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#squareRoot}.
     * @param ctx the parse tree
     */
    fn visit_squareRoot(&mut self, ctx: &SquareRootContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#floorFunction}.
     * @param ctx the parse tree
     */
    fn visit_floorFunction(&mut self, ctx: &FloorFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#ceilingFunction}.
     * @param ctx the parse tree
     */
    fn visit_ceilingFunction(&mut self, ctx: &CeilingFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_characterStringValueExpression(
        &mut self,
        ctx: &CharacterStringValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#byteStringValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_byteStringValueExpression(
        &mut self,
        ctx: &ByteStringValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimOperands}.
     * @param ctx the parse tree
     */
    fn visit_trimOperands(&mut self, ctx: &TrimOperandsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimCharacterOrByteStringSource}.
     * @param ctx the parse tree
     */
    fn visit_trimCharacterOrByteStringSource(
        &mut self,
        ctx: &TrimCharacterOrByteStringSourceContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimSpecification}.
     * @param ctx the parse tree
     */
    fn visit_trimSpecification(&mut self, ctx: &TrimSpecificationContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#trimCharacterOrByteString}.
     * @param ctx the parse tree
     */
    fn visit_trimCharacterOrByteString(
        &mut self,
        ctx: &TrimCharacterOrByteStringContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#normalForm}.
     * @param ctx the parse tree
     */
    fn visit_normalForm(&mut self, ctx: &NormalFormContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#stringLength}.
     * @param ctx the parse tree
     */
    fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression(
        &mut self,
        ctx: &DatetimeValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueFunction(
        &mut self,
        ctx: &DatetimeValueFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateFunction}.
     * @param ctx the parse tree
     */
    fn visit_dateFunction(&mut self, ctx: &DateFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeFunction}.
     * @param ctx the parse tree
     */
    fn visit_timeFunction(&mut self, ctx: &TimeFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localtimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_localtimeFunction(&mut self, ctx: &LocaltimeFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeFunction(&mut self, ctx: &DatetimeFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#localdatetimeFunction}.
     * @param ctx the parse tree
     */
    fn visit_localdatetimeFunction(
        &mut self,
        ctx: &LocaldatetimeFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_dateFunctionParameters(
        &mut self,
        ctx: &DateFunctionParametersContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_timeFunctionParameters(
        &mut self,
        ctx: &TimeFunctionParametersContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_datetimeFunctionParameters(
        &mut self,
        ctx: &DatetimeFunctionParametersContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationValueExpression}.
     * @param ctx the parse tree
     */
    fn visit_durationValueExpression(
        &mut self,
        ctx: &DurationValueExpressionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeSubtraction}.
     * @param ctx the parse tree
     */
    fn visit_datetimeSubtraction(
        &mut self,
        ctx: &DatetimeSubtractionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeSubtractionParameters}.
     * @param ctx the parse tree
     */
    fn visit_datetimeSubtractionParameters(
        &mut self,
        ctx: &DatetimeSubtractionParametersContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression1}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression1(
        &mut self,
        ctx: &DatetimeValueExpression1Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeValueExpression2}.
     * @param ctx the parse tree
     */
    fn visit_datetimeValueExpression2(
        &mut self,
        ctx: &DatetimeValueExpression2Context<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationValueFunction}.
     * @param ctx the parse tree
     */
    fn visit_durationValueFunction(
        &mut self,
        ctx: &DurationValueFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationFunction}.
     * @param ctx the parse tree
     */
    fn visit_durationFunction(&mut self, ctx: &DurationFunctionContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationFunctionParameters}.
     * @param ctx the parse tree
     */
    fn visit_durationFunctionParameters(
        &mut self,
        ctx: &DurationFunctionParametersContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectName}.
     * @param ctx the parse tree
     */
    fn visit_objectName(&mut self, ctx: &ObjectNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#objectNameOrBindingVariable}.
     * @param ctx the parse tree
     */
    fn visit_objectNameOrBindingVariable(
        &mut self,
        ctx: &ObjectNameOrBindingVariableContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#directoryName}.
     * @param ctx the parse tree
     */
    fn visit_directoryName(&mut self, ctx: &DirectoryNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#schemaName}.
     * @param ctx the parse tree
     */
    fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphName}.
     * @param ctx the parse tree
     */
    fn visit_graphName(&mut self, ctx: &GraphNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#delimitedGraphName}.
     * @param ctx the parse tree
     */
    fn visit_delimitedGraphName(
        &mut self,
        ctx: &DelimitedGraphNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#graphTypeName}.
     * @param ctx the parse tree
     */
    fn visit_graphTypeName(&mut self, ctx: &GraphTypeNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeTypeName}.
     * @param ctx the parse tree
     */
    fn visit_nodeTypeName(&mut self, ctx: &NodeTypeNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeTypeName}.
     * @param ctx the parse tree
     */
    fn visit_edgeTypeName(&mut self, ctx: &EdgeTypeNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingTableName}.
     * @param ctx the parse tree
     */
    fn visit_bindingTableName(&mut self, ctx: &BindingTableNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#delimitedBindingTableName}.
     * @param ctx the parse tree
     */
    fn visit_delimitedBindingTableName(
        &mut self,
        ctx: &DelimitedBindingTableNameContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#procedureName}.
     * @param ctx the parse tree
     */
    fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#labelName}.
     * @param ctx the parse tree
     */
    fn visit_labelName(&mut self, ctx: &LabelNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#propertyName}.
     * @param ctx the parse tree
     */
    fn visit_propertyName(&mut self, ctx: &PropertyNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#fieldName}.
     * @param ctx the parse tree
     */
    fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#elementVariable}.
     * @param ctx the parse tree
     */
    fn visit_elementVariable(&mut self, ctx: &ElementVariableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#pathVariable}.
     * @param ctx the parse tree
     */
    fn visit_pathVariable(&mut self, ctx: &PathVariableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#subpathVariable}.
     * @param ctx the parse tree
     */
    fn visit_subpathVariable(&mut self, ctx: &SubpathVariableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#bindingVariable}.
     * @param ctx the parse tree
     */
    fn visit_bindingVariable(&mut self, ctx: &BindingVariableContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedLiteral}.
     * @param ctx the parse tree
     */
    fn visit_unsignedLiteral(&mut self, ctx: &UnsignedLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#generalLiteral}.
     * @param ctx the parse tree
     */
    fn visit_generalLiteral(&mut self, ctx: &GeneralLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#temporalLiteral}.
     * @param ctx the parse tree
     */
    fn visit_temporalLiteral(&mut self, ctx: &TemporalLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateLiteral}.
     * @param ctx the parse tree
     */
    fn visit_dateLiteral(&mut self, ctx: &DateLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeLiteral}.
     * @param ctx the parse tree
     */
    fn visit_timeLiteral(&mut self, ctx: &TimeLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeLiteral}.
     * @param ctx the parse tree
     */
    fn visit_datetimeLiteral(&mut self, ctx: &DatetimeLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#listLiteral}.
     * @param ctx the parse tree
     */
    fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#recordLiteral}.
     * @param ctx the parse tree
     */
    fn visit_recordLiteral(&mut self, ctx: &RecordLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#identifier}.
     * @param ctx the parse tree
     */
    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#regularIdentifier}.
     * @param ctx the parse tree
     */
    fn visit_regularIdentifier(&mut self, ctx: &RegularIdentifierContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeZoneString}.
     * @param ctx the parse tree
     */
    fn visit_timeZoneString(&mut self, ctx: &TimeZoneStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#characterStringLiteral}.
     * @param ctx the parse tree
     */
    fn visit_characterStringLiteral(
        &mut self,
        ctx: &CharacterStringLiteralContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_unsignedNumericLiteral(
        &mut self,
        ctx: &UnsignedNumericLiteralContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#exactNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_exactNumericLiteral(
        &mut self,
        ctx: &ExactNumericLiteralContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#approximateNumericLiteral}.
     * @param ctx the parse tree
     */
    fn visit_approximateNumericLiteral(
        &mut self,
        ctx: &ApproximateNumericLiteralContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedInteger}.
     * @param ctx the parse tree
     */
    fn visit_unsignedInteger(&mut self, ctx: &UnsignedIntegerContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#unsignedDecimalInteger}.
     * @param ctx the parse tree
     */
    fn visit_unsignedDecimalInteger(
        &mut self,
        ctx: &UnsignedDecimalIntegerContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nullLiteral}.
     * @param ctx the parse tree
     */
    fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#dateString}.
     * @param ctx the parse tree
     */
    fn visit_dateString(&mut self, ctx: &DateStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#timeString}.
     * @param ctx the parse tree
     */
    fn visit_timeString(&mut self, ctx: &TimeStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#datetimeString}.
     * @param ctx the parse tree
     */
    fn visit_datetimeString(&mut self, ctx: &DatetimeStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationLiteral}.
     * @param ctx the parse tree
     */
    fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#durationString}.
     * @param ctx the parse tree
     */
    fn visit_durationString(&mut self, ctx: &DurationStringContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nodeSynonym}.
     * @param ctx the parse tree
     */
    fn visit_nodeSynonym(&mut self, ctx: &NodeSynonymContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgesSynonym}.
     * @param ctx the parse tree
     */
    fn visit_edgesSynonym(&mut self, ctx: &EdgesSynonymContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#edgeSynonym}.
     * @param ctx the parse tree
     */
    fn visit_edgeSynonym(&mut self, ctx: &EdgeSynonymContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#userDefinedFunction}.
     * @param ctx the parse tree
     */
    fn visit_userDefinedFunction(
        &mut self,
        ctx: &UserDefinedFunctionContext<'input>,
    ) -> Self::Return {
        self.visit_children(ctx)
    }

    /**
     * Visit a parse tree produced by {@link GQLParser#nonReservedWords}.
     * @param ctx the parse tree
     */
    fn visit_nonReservedWords(&mut self, ctx: &NonReservedWordsContext<'input>) -> Self::Return {
        self.visit_children(ctx)
    }
}

impl<'input, T> GQLVisitor<'input> for T
where
    T: GQLVisitorCompat<'input>,
{
    fn visit_gqlProgram(&mut self, ctx: &GqlProgramContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_gqlProgram(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_programActivity(&mut self, ctx: &ProgramActivityContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_programActivity(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionActivity(&mut self, ctx: &SessionActivityContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionActivity(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_transactionActivity(&mut self, ctx: &TransactionActivityContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_transactionActivity(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endTransactionCommand(&mut self, ctx: &EndTransactionCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endTransactionCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetCommand(&mut self, ctx: &SessionSetCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetSchemaClause(&mut self, ctx: &SessionSetSchemaClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetSchemaClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetGraphClause(&mut self, ctx: &SessionSetGraphClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetGraphClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetTimeZoneClause(&mut self, ctx: &SessionSetTimeZoneClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetTimeZoneClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setTimeZoneValue(&mut self, ctx: &SetTimeZoneValueContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setTimeZoneValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetParameterClause(&mut self, ctx: &SessionSetParameterClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetParameterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetGraphParameterClause(
        &mut self,
        ctx: &SessionSetGraphParameterClauseContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetGraphParameterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetBindingTableParameterClause(
        &mut self,
        ctx: &SessionSetBindingTableParameterClauseContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_sessionSetBindingTableParameterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetValueParameterClause(
        &mut self,
        ctx: &SessionSetValueParameterClauseContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetValueParameterClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionSetParameterName(&mut self, ctx: &SessionSetParameterNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionSetParameterName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionResetCommand(&mut self, ctx: &SessionResetCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionResetCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionResetArguments(&mut self, ctx: &SessionResetArgumentsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionResetArguments(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionCloseCommand(&mut self, ctx: &SessionCloseCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sessionCloseCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sessionParameterSpecification(
        &mut self,
        ctx: &SessionParameterSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_sessionParameterSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_startTransactionCommand(&mut self, ctx: &StartTransactionCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_startTransactionCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_transactionCharacteristics(
        &mut self,
        ctx: &TransactionCharacteristicsContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_transactionCharacteristics(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_transactionMode(&mut self, ctx: &TransactionModeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_transactionMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_transactionAccessMode(&mut self, ctx: &TransactionAccessModeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_transactionAccessMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_rollbackCommand(&mut self, ctx: &RollbackCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_rollbackCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_commitCommand(&mut self, ctx: &CommitCommandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_commitCommand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nestedProcedureSpecification(
        &mut self,
        ctx: &NestedProcedureSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_nestedProcedureSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureSpecification(&mut self, ctx: &ProcedureSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &NestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_nestedDataModifyingProcedureSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nestedQuerySpecification(&mut self, ctx: &NestedQuerySpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nestedQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureBody(&mut self, ctx: &ProcedureBodyContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingVariableDefinitionBlock(
        &mut self,
        ctx: &BindingVariableDefinitionBlockContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_bindingVariableDefinitionBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingVariableDefinition(&mut self, ctx: &BindingVariableDefinitionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingVariableDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_statementBlock(&mut self, ctx: &StatementBlockContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_statementBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_statement(&mut self, ctx: &StatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_statement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nextStatement(&mut self, ctx: &NextStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nextStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphVariableDefinition(&mut self, ctx: &GraphVariableDefinitionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphVariableDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_optTypedGraphInitializer(&mut self, ctx: &OptTypedGraphInitializerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_optTypedGraphInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphInitializer(&mut self, ctx: &GraphInitializerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableVariableDefinition(
        &mut self,
        ctx: &BindingTableVariableDefinitionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableVariableDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_optTypedBindingTableInitializer(
        &mut self,
        ctx: &OptTypedBindingTableInitializerContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_optTypedBindingTableInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableInitializer(&mut self, ctx: &BindingTableInitializerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueVariableDefinition(&mut self, ctx: &ValueVariableDefinitionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueVariableDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_optTypedValueInitializer(&mut self, ctx: &OptTypedValueInitializerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_optTypedValueInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueInitializer(&mut self, ctx: &ValueInitializerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueInitializer(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphExpression(&mut self, ctx: &GraphExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_currentGraph(&mut self, ctx: &CurrentGraphContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_currentGraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableExpression(&mut self, ctx: &BindingTableExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nestedBindingTableQuerySpecification(
        &mut self,
        ctx: &NestedBindingTableQuerySpecificationContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_nestedBindingTableQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_objectExpressionPrimary(&mut self, ctx: &ObjectExpressionPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_objectExpressionPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_linearCatalogModifyingStatement(
        &mut self,
        ctx: &LinearCatalogModifyingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_linearCatalogModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleCatalogModifyingStatement(
        &mut self,
        ctx: &SimpleCatalogModifyingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simpleCatalogModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primitiveCatalogModifyingStatement(
        &mut self,
        ctx: &PrimitiveCatalogModifyingStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_primitiveCatalogModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_createSchemaStatement(&mut self, ctx: &CreateSchemaStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_createSchemaStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dropSchemaStatement(&mut self, ctx: &DropSchemaStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dropSchemaStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_createGraphStatement(&mut self, ctx: &CreateGraphStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_createGraphStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_openGraphType(&mut self, ctx: &OpenGraphTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_openGraphType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ofGraphType(&mut self, ctx: &OfGraphTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ofGraphType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphTypeLikeGraph(&mut self, ctx: &GraphTypeLikeGraphContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphTypeLikeGraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphSource(&mut self, ctx: &GraphSourceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dropGraphStatement(&mut self, ctx: &DropGraphStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dropGraphStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_createGraphTypeStatement(&mut self, ctx: &CreateGraphTypeStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_createGraphTypeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphTypeSource(&mut self, ctx: &GraphTypeSourceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphTypeSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_copyOfGraphType(&mut self, ctx: &CopyOfGraphTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_copyOfGraphType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dropGraphTypeStatement(&mut self, ctx: &DropGraphTypeStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dropGraphTypeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_callCatalogModifyingProcedureStatement(
        &mut self,
        ctx: &CallCatalogModifyingProcedureStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_callCatalogModifyingProcedureStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_linearDataModifyingStatement(
        &mut self,
        ctx: &LinearDataModifyingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_linearDataModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedLinearDataModifyingStatement(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_focusedLinearDataModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedLinearDataModifyingStatementBody(
        &mut self,
        ctx: &FocusedLinearDataModifyingStatementBodyContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_focusedLinearDataModifyingStatementBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedNestedDataModifyingProcedureSpecification(
        &mut self,
        ctx: &FocusedNestedDataModifyingProcedureSpecificationContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_focusedNestedDataModifyingProcedureSpecification(
                self, ctx,
            );
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ambientLinearDataModifyingStatement(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_ambientLinearDataModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ambientLinearDataModifyingStatementBody(
        &mut self,
        ctx: &AmbientLinearDataModifyingStatementBodyContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_ambientLinearDataModifyingStatementBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleLinearDataAccessingStatement(
        &mut self,
        ctx: &SimpleLinearDataAccessingStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_simpleLinearDataAccessingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleDataAccessingStatement(
        &mut self,
        ctx: &SimpleDataAccessingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simpleDataAccessingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleDataModifyingStatement(
        &mut self,
        ctx: &SimpleDataModifyingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simpleDataModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primitiveDataModifyingStatement(
        &mut self,
        ctx: &PrimitiveDataModifyingStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_primitiveDataModifyingStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertStatement(&mut self, ctx: &InsertStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setStatement(&mut self, ctx: &SetStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setItemList(&mut self, ctx: &SetItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setItem(&mut self, ctx: &SetItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setPropertyItem(&mut self, ctx: &SetPropertyItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setPropertyItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setAllPropertiesItem(&mut self, ctx: &SetAllPropertiesItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setAllPropertiesItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setLabelItem(&mut self, ctx: &SetLabelItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setLabelItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_removeStatement(&mut self, ctx: &RemoveStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_removeStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_removeItemList(&mut self, ctx: &RemoveItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_removeItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_removeItem(&mut self, ctx: &RemoveItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_removeItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_removePropertyItem(&mut self, ctx: &RemovePropertyItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_removePropertyItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_removeLabelItem(&mut self, ctx: &RemoveLabelItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_removeLabelItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_deleteStatement(&mut self, ctx: &DeleteStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_deleteStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_deleteItemList(&mut self, ctx: &DeleteItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_deleteItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_deleteItem(&mut self, ctx: &DeleteItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_deleteItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_callDataModifyingProcedureStatement(
        &mut self,
        ctx: &CallDataModifyingProcedureStatementContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_callDataModifyingProcedureStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_compositeQueryStatement(&mut self, ctx: &CompositeQueryStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_compositeQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_compositeQueryExpression(&mut self, ctx: &CompositeQueryExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_compositeQueryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_queryConjunction(&mut self, ctx: &QueryConjunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_queryConjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setOperator(&mut self, ctx: &SetOperatorContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setOperator(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_compositeQueryPrimary(&mut self, ctx: &CompositeQueryPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_compositeQueryPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_linearQueryStatement(&mut self, ctx: &LinearQueryStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_linearQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedLinearQueryStatement(
        &mut self,
        ctx: &FocusedLinearQueryStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_focusedLinearQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedLinearQueryStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryStatementPartContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_focusedLinearQueryStatementPart(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedLinearQueryAndPrimitiveResultStatementPart(
        &mut self,
        ctx: &FocusedLinearQueryAndPrimitiveResultStatementPartContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_focusedLinearQueryAndPrimitiveResultStatementPart(
                self, ctx,
            );
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedPrimitiveResultStatement(
        &mut self,
        ctx: &FocusedPrimitiveResultStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_focusedPrimitiveResultStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_focusedNestedQuerySpecification(
        &mut self,
        ctx: &FocusedNestedQuerySpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_focusedNestedQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ambientLinearQueryStatement(
        &mut self,
        ctx: &AmbientLinearQueryStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_ambientLinearQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleLinearQueryStatement(
        &mut self,
        ctx: &SimpleLinearQueryStatementContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simpleLinearQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleQueryStatement(&mut self, ctx: &SimpleQueryStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simpleQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primitiveQueryStatement(&mut self, ctx: &PrimitiveQueryStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_primitiveQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_matchStatement(&mut self, ctx: &MatchStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_matchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleMatchStatement(&mut self, ctx: &SimpleMatchStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simpleMatchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_optionalMatchStatement(&mut self, ctx: &OptionalMatchStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_optionalMatchStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_optionalOperand(&mut self, ctx: &OptionalOperandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_optionalOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_matchStatementBlock(&mut self, ctx: &MatchStatementBlockContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_matchStatementBlock(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_callQueryStatement(&mut self, ctx: &CallQueryStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_callQueryStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_filterStatement(&mut self, ctx: &FilterStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_filterStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_letStatement(&mut self, ctx: &LetStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_letStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_letVariableDefinitionList(&mut self, ctx: &LetVariableDefinitionListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_letVariableDefinitionList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_letVariableDefinition(&mut self, ctx: &LetVariableDefinitionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_letVariableDefinition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_forStatement(&mut self, ctx: &ForStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_forStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_forItem(&mut self, ctx: &ForItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_forItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_forItemAlias(&mut self, ctx: &ForItemAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_forItemAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_forItemSource(&mut self, ctx: &ForItemSourceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_forItemSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_forOrdinalityOrOffset(&mut self, ctx: &ForOrdinalityOrOffsetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_forOrdinalityOrOffset(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_orderByAndPageStatement(&mut self, ctx: &OrderByAndPageStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_orderByAndPageStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primitiveResultStatement(&mut self, ctx: &PrimitiveResultStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_primitiveResultStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_returnStatement(&mut self, ctx: &ReturnStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_returnStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_returnStatementBody(&mut self, ctx: &ReturnStatementBodyContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_returnStatementBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_returnItemList(&mut self, ctx: &ReturnItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_returnItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_returnItem(&mut self, ctx: &ReturnItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_returnItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_returnItemAlias(&mut self, ctx: &ReturnItemAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_returnItemAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectStatement(&mut self, ctx: &SelectStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectItemList(&mut self, ctx: &SelectItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectItem(&mut self, ctx: &SelectItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectItemAlias(&mut self, ctx: &SelectItemAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectItemAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_havingClause(&mut self, ctx: &HavingClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_havingClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectStatementBody(&mut self, ctx: &SelectStatementBodyContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectStatementBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectGraphMatchList(&mut self, ctx: &SelectGraphMatchListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectGraphMatchList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectGraphMatch(&mut self, ctx: &SelectGraphMatchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectGraphMatch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_selectQuerySpecification(&mut self, ctx: &SelectQuerySpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_selectQuerySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_callProcedureStatement(&mut self, ctx: &CallProcedureStatementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_callProcedureStatement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureCall(&mut self, ctx: &ProcedureCallContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_inlineProcedureCall(&mut self, ctx: &InlineProcedureCallContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_inlineProcedureCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_variableScopeClause(&mut self, ctx: &VariableScopeClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_variableScopeClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingVariableReferenceList(
        &mut self,
        ctx: &BindingVariableReferenceListContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_bindingVariableReferenceList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_namedProcedureCall(&mut self, ctx: &NamedProcedureCallContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_namedProcedureCall(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureArgumentList(&mut self, ctx: &ProcedureArgumentListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureArgumentList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureArgument(&mut self, ctx: &ProcedureArgumentContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_atSchemaClause(&mut self, ctx: &AtSchemaClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_atSchemaClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_useGraphClause(&mut self, ctx: &UseGraphClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_useGraphClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternBindingTable(&mut self, ctx: &GraphPatternBindingTableContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternBindingTable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternYieldClause(&mut self, ctx: &GraphPatternYieldClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternYieldClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternYieldItemList(&mut self, ctx: &GraphPatternYieldItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternYieldItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternYieldItem(&mut self, ctx: &GraphPatternYieldItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternYieldItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPattern(&mut self, ctx: &GraphPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_matchMode(&mut self, ctx: &MatchModeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_matchMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_repeatableElementsMatchMode(
        &mut self,
        ctx: &RepeatableElementsMatchModeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_repeatableElementsMatchMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_differentEdgesMatchMode(&mut self, ctx: &DifferentEdgesMatchModeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_differentEdgesMatchMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementBindingsOrElements(&mut self, ctx: &ElementBindingsOrElementsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementBindingsOrElements(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeBindingsOrEdges(&mut self, ctx: &EdgeBindingsOrEdgesContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeBindingsOrEdges(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathPatternList(&mut self, ctx: &PathPatternListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathPatternList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathPattern(&mut self, ctx: &PathPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathVariableDeclaration(&mut self, ctx: &PathVariableDeclarationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathVariableDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_keepClause(&mut self, ctx: &KeepClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_keepClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternWhereClause(&mut self, ctx: &GraphPatternWhereClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternWhereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertGraphPattern(&mut self, ctx: &InsertGraphPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertGraphPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertPathPatternList(&mut self, ctx: &InsertPathPatternListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertPathPatternList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertPathPattern(&mut self, ctx: &InsertPathPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertPathPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertNodePattern(&mut self, ctx: &InsertNodePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertNodePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertEdgePattern(&mut self, ctx: &InsertEdgePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertEdgePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertEdgePointingLeft(&mut self, ctx: &InsertEdgePointingLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertEdgePointingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertEdgePointingRight(&mut self, ctx: &InsertEdgePointingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertEdgePointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertEdgeUndirected(&mut self, ctx: &InsertEdgeUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_insertEdgeUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_insertElementPatternFiller(
        &mut self,
        ctx: &InsertElementPatternFillerContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_insertElementPatternFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelAndPropertySetSpecification(
        &mut self,
        ctx: &LabelAndPropertySetSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_labelAndPropertySetSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathPatternPrefix(&mut self, ctx: &PathPatternPrefixContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathPatternPrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathModePrefix(&mut self, ctx: &PathModePrefixContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathModePrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathMode(&mut self, ctx: &PathModeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathMode(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathSearchPrefix(&mut self, ctx: &PathSearchPrefixContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathSearchPrefix(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_allPathSearch(&mut self, ctx: &AllPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_allPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathOrPaths(&mut self, ctx: &PathOrPathsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathOrPaths(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_anyPathSearch(&mut self, ctx: &AnyPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_anyPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numberOfPaths(&mut self, ctx: &NumberOfPathsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_numberOfPaths(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_shortestPathSearch(&mut self, ctx: &ShortestPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_shortestPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_allShortestPathSearch(&mut self, ctx: &AllShortestPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_allShortestPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_anyShortestPathSearch(&mut self, ctx: &AnyShortestPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_anyShortestPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_countedShortestPathSearch(&mut self, ctx: &CountedShortestPathSearchContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_countedShortestPathSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_countedShortestGroupSearch(
        &mut self,
        ctx: &CountedShortestGroupSearchContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_countedShortestGroupSearch(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numberOfGroups(&mut self, ctx: &NumberOfGroupsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_numberOfGroups(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppePathTerm(&mut self, ctx: &PpePathTermContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ppePathTerm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppeMultisetAlternation(&mut self, ctx: &PpeMultisetAlternationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ppeMultisetAlternation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppePatternUnion(&mut self, ctx: &PpePatternUnionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ppePatternUnion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathTerm(&mut self, ctx: &PathTermContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathTerm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pfPathPrimary(&mut self, ctx: &PfPathPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pfPathPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pfQuantifiedPathPrimary(&mut self, ctx: &PfQuantifiedPathPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pfQuantifiedPathPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pfQuestionedPathPrimary(&mut self, ctx: &PfQuestionedPathPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pfQuestionedPathPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppElementPattern(&mut self, ctx: &PpElementPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ppElementPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppParenthesizedPathPatternExpression(
        &mut self,
        ctx: &PpParenthesizedPathPatternExpressionContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_ppParenthesizedPathPatternExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ppSimplifiedPathPatternExpression(
        &mut self,
        ctx: &PpSimplifiedPathPatternExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_ppSimplifiedPathPatternExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementPattern(&mut self, ctx: &ElementPatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementPattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodePattern(&mut self, ctx: &NodePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementPatternFiller(&mut self, ctx: &ElementPatternFillerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementPatternFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementVariableDeclaration(
        &mut self,
        ctx: &ElementVariableDeclarationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_elementVariableDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_isLabelExpression(&mut self, ctx: &IsLabelExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_isLabelExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_isOrColon(&mut self, ctx: &IsOrColonContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_isOrColon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementPatternPredicate(&mut self, ctx: &ElementPatternPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementPatternPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementPatternWhereClause(&mut self, ctx: &ElementPatternWhereClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementPatternWhereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementPropertySpecification(
        &mut self,
        ctx: &ElementPropertySpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_elementPropertySpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyKeyValuePairList(&mut self, ctx: &PropertyKeyValuePairListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyKeyValuePairList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyKeyValuePair(&mut self, ctx: &PropertyKeyValuePairContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyKeyValuePair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgePattern(&mut self, ctx: &EdgePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgePattern(&mut self, ctx: &FullEdgePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgePointingLeft(&mut self, ctx: &FullEdgePointingLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgePointingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgeUndirected(&mut self, ctx: &FullEdgeUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgeUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgePointingRight(&mut self, ctx: &FullEdgePointingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgePointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgeLeftOrUndirected(&mut self, ctx: &FullEdgeLeftOrUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgeLeftOrUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgeUndirectedOrRight(&mut self, ctx: &FullEdgeUndirectedOrRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgeUndirectedOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgeLeftOrRight(&mut self, ctx: &FullEdgeLeftOrRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgeLeftOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fullEdgeAnyDirection(&mut self, ctx: &FullEdgeAnyDirectionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fullEdgeAnyDirection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_abbreviatedEdgePattern(&mut self, ctx: &AbbreviatedEdgePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_abbreviatedEdgePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parenthesizedPathPatternExpression(
        &mut self,
        ctx: &ParenthesizedPathPatternExpressionContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_parenthesizedPathPatternExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subpathVariableDeclaration(
        &mut self,
        ctx: &SubpathVariableDeclarationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_subpathVariableDeclaration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parenthesizedPathPatternWhereClause(
        &mut self,
        ctx: &ParenthesizedPathPatternWhereClauseContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_parenthesizedPathPatternWhereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionNegation(&mut self, ctx: &LabelExpressionNegationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionNegation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionDisjunction(
        &mut self,
        ctx: &LabelExpressionDisjunctionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionDisjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionParenthesized(
        &mut self,
        ctx: &LabelExpressionParenthesizedContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionParenthesized(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionWildcard(&mut self, ctx: &LabelExpressionWildcardContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionWildcard(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionConjunction(
        &mut self,
        ctx: &LabelExpressionConjunctionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionConjunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelExpressionName(&mut self, ctx: &LabelExpressionNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelExpressionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathVariableReference(&mut self, ctx: &PathVariableReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathVariableReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementVariableReference(&mut self, ctx: &ElementVariableReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementVariableReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphPatternQuantifier(&mut self, ctx: &GraphPatternQuantifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphPatternQuantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fixedQuantifier(&mut self, ctx: &FixedQuantifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fixedQuantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalQuantifier(&mut self, ctx: &GeneralQuantifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalQuantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lowerBound(&mut self, ctx: &LowerBoundContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_lowerBound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_upperBound(&mut self, ctx: &UpperBoundContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_upperBound(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedPathPatternExpression(
        &mut self,
        ctx: &SimplifiedPathPatternExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedPathPatternExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingLeft(&mut self, ctx: &SimplifiedDefaultingLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDefaultingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDefaultingUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingRight(&mut self, ctx: &SimplifiedDefaultingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDefaultingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrUndirectedContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_simplifiedDefaultingLeftOrUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingUndirectedOrRightContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_simplifiedDefaultingUndirectedOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingLeftOrRight(
        &mut self,
        ctx: &SimplifiedDefaultingLeftOrRightContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDefaultingLeftOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDefaultingAnyDirection(
        &mut self,
        ctx: &SimplifiedDefaultingAnyDirectionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDefaultingAnyDirection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedContents(&mut self, ctx: &SimplifiedContentsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedContents(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedPathUnion(&mut self, ctx: &SimplifiedPathUnionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedPathUnion(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedMultisetAlternation(
        &mut self,
        ctx: &SimplifiedMultisetAlternationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedMultisetAlternation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedFactorLowLabel(&mut self, ctx: &SimplifiedFactorLowLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedFactorLowLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedConcatenationLabel(
        &mut self,
        ctx: &SimplifiedConcatenationLabelContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedConcatenationLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedConjunctionLabel(
        &mut self,
        ctx: &SimplifiedConjunctionLabelContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedConjunctionLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedFactorHighLabel(&mut self, ctx: &SimplifiedFactorHighLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedFactorHighLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedFactorHigh(&mut self, ctx: &SimplifiedFactorHighContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedFactorHigh(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedQuantified(&mut self, ctx: &SimplifiedQuantifiedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedQuantified(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedQuestioned(&mut self, ctx: &SimplifiedQuestionedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedQuestioned(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedTertiary(&mut self, ctx: &SimplifiedTertiaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedTertiary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedDirectionOverride(
        &mut self,
        ctx: &SimplifiedDirectionOverrideContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedDirectionOverride(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideLeft(&mut self, ctx: &SimplifiedOverrideLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedOverrideLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideUndirected(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedOverrideUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideRight(&mut self, ctx: &SimplifiedOverrideRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedOverrideRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideLeftOrUndirected(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrUndirectedContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_simplifiedOverrideLeftOrUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideUndirectedOrRight(
        &mut self,
        ctx: &SimplifiedOverrideUndirectedOrRightContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_simplifiedOverrideUndirectedOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideLeftOrRight(
        &mut self,
        ctx: &SimplifiedOverrideLeftOrRightContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedOverrideLeftOrRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedOverrideAnyDirection(
        &mut self,
        ctx: &SimplifiedOverrideAnyDirectionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedOverrideAnyDirection(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedSecondary(&mut self, ctx: &SimplifiedSecondaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedSecondary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedNegation(&mut self, ctx: &SimplifiedNegationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedNegation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simplifiedPrimary(&mut self, ctx: &SimplifiedPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simplifiedPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_whereClause(&mut self, ctx: &WhereClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_whereClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yieldClause(&mut self, ctx: &YieldClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_yieldClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yieldItemList(&mut self, ctx: &YieldItemListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_yieldItemList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yieldItem(&mut self, ctx: &YieldItemContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_yieldItem(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yieldItemName(&mut self, ctx: &YieldItemNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_yieldItemName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_yieldItemAlias(&mut self, ctx: &YieldItemAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_yieldItemAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_groupByClause(&mut self, ctx: &GroupByClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_groupByClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_groupingElementList(&mut self, ctx: &GroupingElementListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_groupingElementList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_groupingElement(&mut self, ctx: &GroupingElementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_groupingElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_emptyGroupingSet(&mut self, ctx: &EmptyGroupingSetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_emptyGroupingSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_orderByClause(&mut self, ctx: &OrderByClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_orderByClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sortSpecificationList(&mut self, ctx: &SortSpecificationListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sortSpecificationList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sortSpecification(&mut self, ctx: &SortSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sortSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sortKey(&mut self, ctx: &SortKeyContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sortKey(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_orderingSpecification(&mut self, ctx: &OrderingSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_orderingSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nullOrdering(&mut self, ctx: &NullOrderingContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nullOrdering(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_limitClause(&mut self, ctx: &LimitClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_limitClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_offsetClause(&mut self, ctx: &OffsetClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_offsetClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_offsetSynonym(&mut self, ctx: &OffsetSynonymContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_offsetSynonym(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_schemaReference(&mut self, ctx: &SchemaReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_schemaReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_absoluteCatalogSchemaReference(
        &mut self,
        ctx: &AbsoluteCatalogSchemaReferenceContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_absoluteCatalogSchemaReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_catalogSchemaParentAndName(
        &mut self,
        ctx: &CatalogSchemaParentAndNameContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_catalogSchemaParentAndName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_relativeCatalogSchemaReference(
        &mut self,
        ctx: &RelativeCatalogSchemaReferenceContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_relativeCatalogSchemaReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_predefinedSchemaReference(&mut self, ctx: &PredefinedSchemaReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_predefinedSchemaReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_absoluteDirectoryPath(&mut self, ctx: &AbsoluteDirectoryPathContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_absoluteDirectoryPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_relativeDirectoryPath(&mut self, ctx: &RelativeDirectoryPathContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_relativeDirectoryPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleDirectoryPath(&mut self, ctx: &SimpleDirectoryPathContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simpleDirectoryPath(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphReference(&mut self, ctx: &GraphReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_catalogGraphParentAndName(&mut self, ctx: &CatalogGraphParentAndNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_catalogGraphParentAndName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_homeGraph(&mut self, ctx: &HomeGraphContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_homeGraph(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphTypeReference(&mut self, ctx: &GraphTypeReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphTypeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_catalogGraphTypeParentAndName(
        &mut self,
        ctx: &CatalogGraphTypeParentAndNameContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_catalogGraphTypeParentAndName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableReference(&mut self, ctx: &BindingTableReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureReference(&mut self, ctx: &ProcedureReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_catalogProcedureParentAndName(
        &mut self,
        ctx: &CatalogProcedureParentAndNameContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_catalogProcedureParentAndName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_catalogObjectParentReference(
        &mut self,
        ctx: &CatalogObjectParentReferenceContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_catalogObjectParentReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_referenceParameterSpecification(
        &mut self,
        ctx: &ReferenceParameterSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_referenceParameterSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nestedGraphTypeSpecification(
        &mut self,
        ctx: &NestedGraphTypeSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_nestedGraphTypeSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphTypeSpecificationBody(
        &mut self,
        ctx: &GraphTypeSpecificationBodyContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_graphTypeSpecificationBody(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementTypeList(&mut self, ctx: &ElementTypeListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementTypeSpecification(&mut self, ctx: &ElementTypeSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementTypeSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeSpecification(&mut self, ctx: &NodeTypeSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypePattern(&mut self, ctx: &NodeTypePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypePhrase(&mut self, ctx: &NodeTypePhraseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypePhraseFiller(&mut self, ctx: &NodeTypePhraseFillerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypePhraseFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeFiller(&mut self, ctx: &NodeTypeFillerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localNodeTypeAlias(&mut self, ctx: &LocalNodeTypeAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_localNodeTypeAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeImpliedContent(&mut self, ctx: &NodeTypeImpliedContentContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeImpliedContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeKeyLabelSet(&mut self, ctx: &NodeTypeKeyLabelSetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeKeyLabelSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeLabelSet(&mut self, ctx: &NodeTypeLabelSetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeLabelSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypePropertyTypes(&mut self, ctx: &NodeTypePropertyTypesContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypePropertyTypes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeSpecification(&mut self, ctx: &EdgeTypeSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePattern(&mut self, ctx: &EdgeTypePatternContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePattern(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePhrase(&mut self, ctx: &EdgeTypePhraseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePhraseFiller(&mut self, ctx: &EdgeTypePhraseFillerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePhraseFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeFiller(&mut self, ctx: &EdgeTypeFillerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeFiller(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeImpliedContent(&mut self, ctx: &EdgeTypeImpliedContentContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeImpliedContent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeKeyLabelSet(&mut self, ctx: &EdgeTypeKeyLabelSetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeKeyLabelSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeLabelSet(&mut self, ctx: &EdgeTypeLabelSetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeLabelSet(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePropertyTypes(&mut self, ctx: &EdgeTypePropertyTypesContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePropertyTypes(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePatternDirected(&mut self, ctx: &EdgeTypePatternDirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePatternDirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePatternPointingRight(
        &mut self,
        ctx: &EdgeTypePatternPointingRightContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePatternPointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePatternPointingLeft(
        &mut self,
        ctx: &EdgeTypePatternPointingLeftContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePatternPointingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypePatternUndirected(&mut self, ctx: &EdgeTypePatternUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypePatternUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_arcTypePointingRight(&mut self, ctx: &ArcTypePointingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_arcTypePointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_arcTypePointingLeft(&mut self, ctx: &ArcTypePointingLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_arcTypePointingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_arcTypeUndirected(&mut self, ctx: &ArcTypeUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_arcTypeUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sourceNodeTypeReference(&mut self, ctx: &SourceNodeTypeReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sourceNodeTypeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_destinationNodeTypeReference(
        &mut self,
        ctx: &DestinationNodeTypeReferenceContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_destinationNodeTypeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeKind(&mut self, ctx: &EdgeKindContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeKind(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPairPhrase(&mut self, ctx: &EndpointPairPhraseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPairPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPair(&mut self, ctx: &EndpointPairContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPair(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPairDirected(&mut self, ctx: &EndpointPairDirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPairDirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPairPointingRight(&mut self, ctx: &EndpointPairPointingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPairPointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPairPointingLeft(&mut self, ctx: &EndpointPairPointingLeftContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPairPointingLeft(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_endpointPairUndirected(&mut self, ctx: &EndpointPairUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_endpointPairUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_connectorPointingRight(&mut self, ctx: &ConnectorPointingRightContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_connectorPointingRight(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_connectorUndirected(&mut self, ctx: &ConnectorUndirectedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_connectorUndirected(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sourceNodeTypeAlias(&mut self, ctx: &SourceNodeTypeAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sourceNodeTypeAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_destinationNodeTypeAlias(&mut self, ctx: &DestinationNodeTypeAliasContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_destinationNodeTypeAlias(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelSetPhrase(&mut self, ctx: &LabelSetPhraseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelSetPhrase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelSetSpecification(&mut self, ctx: &LabelSetSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelSetSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyTypesSpecification(
        &mut self,
        ctx: &PropertyTypesSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_propertyTypesSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyTypeList(&mut self, ctx: &PropertyTypeListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyType(&mut self, ctx: &PropertyTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyValueType(&mut self, ctx: &PropertyValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableType(&mut self, ctx: &BindingTableTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dynamicPropertyValueTypeLabel(
        &mut self,
        ctx: &DynamicPropertyValueTypeLabelContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_dynamicPropertyValueTypeLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closedDynamicUnionTypeAtl1(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl1Context<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_closedDynamicUnionTypeAtl1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closedDynamicUnionTypeAtl2(
        &mut self,
        ctx: &ClosedDynamicUnionTypeAtl2Context<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_closedDynamicUnionTypeAtl2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathValueTypeLabel(&mut self, ctx: &PathValueTypeLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathValueTypeLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueTypeAlt3(&mut self, ctx: &ListValueTypeAlt3Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueTypeAlt3(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueTypeAlt2(&mut self, ctx: &ListValueTypeAlt2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueTypeAlt2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueTypeAlt1(&mut self, ctx: &ListValueTypeAlt1Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueTypeAlt1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_predefinedTypeLabel(&mut self, ctx: &PredefinedTypeLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_predefinedTypeLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_recordTypeLabel(&mut self, ctx: &RecordTypeLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_recordTypeLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_openDynamicUnionTypeLabel(&mut self, ctx: &OpenDynamicUnionTypeLabelContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_openDynamicUnionTypeLabel(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_typed(&mut self, ctx: &TypedContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_typed(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_predefinedType(&mut self, ctx: &PredefinedTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_predefinedType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_booleanType(&mut self, ctx: &BooleanTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_booleanType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_characterStringType(&mut self, ctx: &CharacterStringTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_characterStringType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_byteStringType(&mut self, ctx: &ByteStringTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_byteStringType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_minLength(&mut self, ctx: &MinLengthContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_minLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_maxLength(&mut self, ctx: &MaxLengthContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_maxLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fixedLength(&mut self, ctx: &FixedLengthContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fixedLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericType(&mut self, ctx: &NumericTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_numericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_exactNumericType(&mut self, ctx: &ExactNumericTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_exactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_binaryExactNumericType(&mut self, ctx: &BinaryExactNumericTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_binaryExactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_signedBinaryExactNumericType(
        &mut self,
        ctx: &SignedBinaryExactNumericTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_signedBinaryExactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedBinaryExactNumericType(
        &mut self,
        ctx: &UnsignedBinaryExactNumericTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedBinaryExactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_verboseBinaryExactNumericType(
        &mut self,
        ctx: &VerboseBinaryExactNumericTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_verboseBinaryExactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_decimalExactNumericType(&mut self, ctx: &DecimalExactNumericTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_decimalExactNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_precision(&mut self, ctx: &PrecisionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_precision(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_scale(&mut self, ctx: &ScaleContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_scale(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_approximateNumericType(&mut self, ctx: &ApproximateNumericTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_approximateNumericType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_temporalType(&mut self, ctx: &TemporalTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_temporalType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_temporalInstantType(&mut self, ctx: &TemporalInstantTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_temporalInstantType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeType(&mut self, ctx: &DatetimeTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localdatetimeType(&mut self, ctx: &LocaldatetimeTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_localdatetimeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dateType(&mut self, ctx: &DateTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dateType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeType(&mut self, ctx: &TimeTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localtimeType(&mut self, ctx: &LocaltimeTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_localtimeType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_temporalDurationType(&mut self, ctx: &TemporalDurationTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_temporalDurationType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_temporalDurationQualifier(&mut self, ctx: &TemporalDurationQualifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_temporalDurationQualifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_referenceValueType(&mut self, ctx: &ReferenceValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_referenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_immaterialValueType(&mut self, ctx: &ImmaterialValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_immaterialValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nullType(&mut self, ctx: &NullTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nullType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_emptyType(&mut self, ctx: &EmptyTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_emptyType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphReferenceValueType(&mut self, ctx: &GraphReferenceValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closedGraphReferenceValueType(
        &mut self,
        ctx: &ClosedGraphReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_closedGraphReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_openGraphReferenceValueType(
        &mut self,
        ctx: &OpenGraphReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_openGraphReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableReferenceValueType(
        &mut self,
        ctx: &BindingTableReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeReferenceValueType(&mut self, ctx: &NodeReferenceValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closedNodeReferenceValueType(
        &mut self,
        ctx: &ClosedNodeReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_closedNodeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_openNodeReferenceValueType(
        &mut self,
        ctx: &OpenNodeReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_openNodeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeReferenceValueType(&mut self, ctx: &EdgeReferenceValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_closedEdgeReferenceValueType(
        &mut self,
        ctx: &ClosedEdgeReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_closedEdgeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_openEdgeReferenceValueType(
        &mut self,
        ctx: &OpenEdgeReferenceValueTypeContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_openEdgeReferenceValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathValueType(&mut self, ctx: &PathValueTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathValueType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueTypeName(&mut self, ctx: &ListValueTypeNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueTypeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueTypeNameSynonym(&mut self, ctx: &ListValueTypeNameSynonymContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueTypeNameSynonym(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_recordType(&mut self, ctx: &RecordTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_recordType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldTypesSpecification(&mut self, ctx: &FieldTypesSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldTypesSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldTypeList(&mut self, ctx: &FieldTypeListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldTypeList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_notNull(&mut self, ctx: &NotNullContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_notNull(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldType(&mut self, ctx: &FieldTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_searchCondition(&mut self, ctx: &SearchConditionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_searchCondition(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_predicate(&mut self, ctx: &PredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_predicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_compOp(&mut self, ctx: &CompOpContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_compOp(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_existsPredicate(&mut self, ctx: &ExistsPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_existsPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nullPredicate(&mut self, ctx: &NullPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nullPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nullPredicatePart2(&mut self, ctx: &NullPredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nullPredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueTypePredicate(&mut self, ctx: &ValueTypePredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueTypePredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueTypePredicatePart2(&mut self, ctx: &ValueTypePredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueTypePredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normalizedPredicatePart2(&mut self, ctx: &NormalizedPredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_normalizedPredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_directedPredicate(&mut self, ctx: &DirectedPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_directedPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_directedPredicatePart2(&mut self, ctx: &DirectedPredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_directedPredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labeledPredicate(&mut self, ctx: &LabeledPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labeledPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labeledPredicatePart2(&mut self, ctx: &LabeledPredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labeledPredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_isLabeledOrColon(&mut self, ctx: &IsLabeledOrColonContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_isLabeledOrColon(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sourceDestinationPredicate(
        &mut self,
        ctx: &SourceDestinationPredicateContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_sourceDestinationPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeReference(&mut self, ctx: &NodeReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_sourcePredicatePart2(&mut self, ctx: &SourcePredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_sourcePredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_destinationPredicatePart2(&mut self, ctx: &DestinationPredicatePart2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_destinationPredicatePart2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeReference(&mut self, ctx: &EdgeReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_all_differentPredicate(&mut self, ctx: &All_differentPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_all_differentPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_samePredicate(&mut self, ctx: &SamePredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_samePredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_property_existsPredicate(&mut self, ctx: &Property_existsPredicateContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_property_existsPredicate(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_conjunctiveExprAlt(&mut self, ctx: &ConjunctiveExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_conjunctiveExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyGraphExprAlt(&mut self, ctx: &PropertyGraphExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyGraphExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_multDivExprAlt(&mut self, ctx: &MultDivExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_multDivExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableExprAlt(&mut self, ctx: &BindingTableExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_signedExprAlt(&mut self, ctx: &SignedExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_signedExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_isNotExprAlt(&mut self, ctx: &IsNotExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_isNotExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normalizedPredicateExprAlt(
        &mut self,
        ctx: &NormalizedPredicateExprAltContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_normalizedPredicateExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_notExprAlt(&mut self, ctx: &NotExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_notExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueFunctionExprAlt(&mut self, ctx: &ValueFunctionExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueFunctionExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_concatenationExprAlt(&mut self, ctx: &ConcatenationExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_concatenationExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_disjunctiveExprAlt(&mut self, ctx: &DisjunctiveExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_disjunctiveExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_comparisonExprAlt(&mut self, ctx: &ComparisonExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_comparisonExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_primaryExprAlt(&mut self, ctx: &PrimaryExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_primaryExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_addSubtractExprAlt(&mut self, ctx: &AddSubtractExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_addSubtractExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_predicateExprAlt(&mut self, ctx: &PredicateExprAltContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_predicateExprAlt(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueFunction(&mut self, ctx: &ValueFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_booleanValueExpression(&mut self, ctx: &BooleanValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_booleanValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_characterOrByteStringFunction(
        &mut self,
        ctx: &CharacterOrByteStringFunctionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_characterOrByteStringFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subCharacterOrByteString(&mut self, ctx: &SubCharacterOrByteStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_subCharacterOrByteString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimSingleCharacterOrByteString(
        &mut self,
        ctx: &TrimSingleCharacterOrByteStringContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_trimSingleCharacterOrByteString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_foldCharacterString(&mut self, ctx: &FoldCharacterStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_foldCharacterString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimMultiCharacterCharacterString(
        &mut self,
        ctx: &TrimMultiCharacterCharacterStringContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_trimMultiCharacterCharacterString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normalizeCharacterString(&mut self, ctx: &NormalizeCharacterStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_normalizeCharacterString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeReferenceValueExpression(
        &mut self,
        ctx: &NodeReferenceValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_nodeReferenceValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeReferenceValueExpression(
        &mut self,
        ctx: &EdgeReferenceValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_edgeReferenceValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_aggregatingValueExpression(
        &mut self,
        ctx: &AggregatingValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_aggregatingValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueExpressionPrimary(&mut self, ctx: &ValueExpressionPrimaryContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueExpressionPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_parenthesizedValueExpression(
        &mut self,
        ctx: &ParenthesizedValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_parenthesizedValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonParenthesizedValueExpressionPrimary(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimaryContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_nonParenthesizedValueExpressionPrimary(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonParenthesizedValueExpressionPrimarySpecialCase(
        &mut self,
        ctx: &NonParenthesizedValueExpressionPrimarySpecialCaseContext<'input>,
    ) {
        let result =
            <Self as GQLVisitorCompat>::visit_nonParenthesizedValueExpressionPrimarySpecialCase(
                self, ctx,
            );
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedValueSpecification(
        &mut self,
        ctx: &UnsignedValueSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedValueSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonNegativeIntegerSpecification(
        &mut self,
        ctx: &NonNegativeIntegerSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_nonNegativeIntegerSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalValueSpecification(&mut self, ctx: &GeneralValueSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalValueSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dynamicParameterSpecification(
        &mut self,
        ctx: &DynamicParameterSpecificationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_dynamicParameterSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_letValueExpression(&mut self, ctx: &LetValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_letValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_valueQueryExpression(&mut self, ctx: &ValueQueryExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_valueQueryExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_caseExpression(&mut self, ctx: &CaseExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_caseExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_caseAbbreviation(&mut self, ctx: &CaseAbbreviationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_caseAbbreviation(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_caseSpecification(&mut self, ctx: &CaseSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_caseSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleCase(&mut self, ctx: &SimpleCaseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simpleCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_searchedCase(&mut self, ctx: &SearchedCaseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_searchedCase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_simpleWhenClause(&mut self, ctx: &SimpleWhenClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_simpleWhenClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_searchedWhenClause(&mut self, ctx: &SearchedWhenClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_searchedWhenClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elseClause(&mut self, ctx: &ElseClauseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elseClause(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_caseOperand(&mut self, ctx: &CaseOperandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_caseOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_whenOperandList(&mut self, ctx: &WhenOperandListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_whenOperandList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_whenOperand(&mut self, ctx: &WhenOperandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_whenOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_result(&mut self, ctx: &ResultContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_result(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_resultExpression(&mut self, ctx: &ResultExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_resultExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_castSpecification(&mut self, ctx: &CastSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_castSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_castOperand(&mut self, ctx: &CastOperandContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_castOperand(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_castTarget(&mut self, ctx: &CastTargetContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_castTarget(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_aggregateFunction(&mut self, ctx: &AggregateFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_aggregateFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalSetFunction(&mut self, ctx: &GeneralSetFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalSetFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_binarySetFunction(&mut self, ctx: &BinarySetFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_binarySetFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalSetFunctionType(&mut self, ctx: &GeneralSetFunctionTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalSetFunctionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_setQuantifier(&mut self, ctx: &SetQuantifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_setQuantifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_binarySetFunctionType(&mut self, ctx: &BinarySetFunctionTypeContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_binarySetFunctionType(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dependentValueExpression(&mut self, ctx: &DependentValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dependentValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_independentValueExpression(
        &mut self,
        ctx: &IndependentValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_independentValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_element_idFunction(&mut self, ctx: &Element_idFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_element_idFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingVariableReference(&mut self, ctx: &BindingVariableReferenceContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingVariableReference(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathValueExpression(&mut self, ctx: &PathValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathValueConstructor(&mut self, ctx: &PathValueConstructorContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathValueConstructor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathValueConstructorByEnumeration(
        &mut self,
        ctx: &PathValueConstructorByEnumerationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_pathValueConstructorByEnumeration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathElementList(&mut self, ctx: &PathElementListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathElementList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathElementListStart(&mut self, ctx: &PathElementListStartContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathElementListStart(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathElementListStep(&mut self, ctx: &PathElementListStepContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathElementListStep(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueExpression(&mut self, ctx: &ListValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueFunction(&mut self, ctx: &ListValueFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimListFunction(&mut self, ctx: &TrimListFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trimListFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementsFunction(&mut self, ctx: &ElementsFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementsFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueConstructor(&mut self, ctx: &ListValueConstructorContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listValueConstructor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listValueConstructorByEnumeration(
        &mut self,
        ctx: &ListValueConstructorByEnumerationContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_listValueConstructorByEnumeration(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listElementList(&mut self, ctx: &ListElementListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listElementList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listElement(&mut self, ctx: &ListElementContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listElement(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_recordConstructor(&mut self, ctx: &RecordConstructorContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_recordConstructor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldsSpecification(&mut self, ctx: &FieldsSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldsSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldList(&mut self, ctx: &FieldListContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldList(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_field(&mut self, ctx: &FieldContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_field(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_truthValue(&mut self, ctx: &TruthValueContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_truthValue(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueExpression(&mut self, ctx: &NumericValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueFunction(&mut self, ctx: &NumericValueFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_lengthExpression(&mut self, ctx: &LengthExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_lengthExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_cardinalityExpression(&mut self, ctx: &CardinalityExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_cardinalityExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_cardinalityExpressionArgument(
        &mut self,
        ctx: &CardinalityExpressionArgumentContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_cardinalityExpressionArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_charLengthExpression(&mut self, ctx: &CharLengthExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_charLengthExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_byteLengthExpression(&mut self, ctx: &ByteLengthExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_byteLengthExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathLengthExpression(&mut self, ctx: &PathLengthExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathLengthExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_absoluteValueExpression(&mut self, ctx: &AbsoluteValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_absoluteValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_modulusExpression(&mut self, ctx: &ModulusExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_modulusExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueExpressionDividend(
        &mut self,
        ctx: &NumericValueExpressionDividendContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueExpressionDividend(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueExpressionDivisor(
        &mut self,
        ctx: &NumericValueExpressionDivisorContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueExpressionDivisor(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trigonometricFunction(&mut self, ctx: &TrigonometricFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trigonometricFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trigonometricFunctionName(&mut self, ctx: &TrigonometricFunctionNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trigonometricFunctionName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalLogarithmFunction(&mut self, ctx: &GeneralLogarithmFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalLogarithmFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalLogarithmBase(&mut self, ctx: &GeneralLogarithmBaseContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalLogarithmBase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalLogarithmArgument(&mut self, ctx: &GeneralLogarithmArgumentContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalLogarithmArgument(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_commonLogarithm(&mut self, ctx: &CommonLogarithmContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_commonLogarithm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_naturalLogarithm(&mut self, ctx: &NaturalLogarithmContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_naturalLogarithm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_exponentialFunction(&mut self, ctx: &ExponentialFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_exponentialFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_powerFunction(&mut self, ctx: &PowerFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_powerFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueExpressionBase(
        &mut self,
        ctx: &NumericValueExpressionBaseContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueExpressionBase(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_numericValueExpressionExponent(
        &mut self,
        ctx: &NumericValueExpressionExponentContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_numericValueExpressionExponent(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_squareRoot(&mut self, ctx: &SquareRootContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_squareRoot(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_floorFunction(&mut self, ctx: &FloorFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_floorFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_ceilingFunction(&mut self, ctx: &CeilingFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_ceilingFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_characterStringValueExpression(
        &mut self,
        ctx: &CharacterStringValueExpressionContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_characterStringValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_byteStringValueExpression(&mut self, ctx: &ByteStringValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_byteStringValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimOperands(&mut self, ctx: &TrimOperandsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trimOperands(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimCharacterOrByteStringSource(
        &mut self,
        ctx: &TrimCharacterOrByteStringSourceContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_trimCharacterOrByteStringSource(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimSpecification(&mut self, ctx: &TrimSpecificationContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trimSpecification(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_trimCharacterOrByteString(&mut self, ctx: &TrimCharacterOrByteStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_trimCharacterOrByteString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_normalForm(&mut self, ctx: &NormalFormContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_normalForm(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_stringLength(&mut self, ctx: &StringLengthContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_stringLength(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeValueExpression(&mut self, ctx: &DatetimeValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeValueFunction(&mut self, ctx: &DatetimeValueFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeValueFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dateFunction(&mut self, ctx: &DateFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dateFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeFunction(&mut self, ctx: &TimeFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localtimeFunction(&mut self, ctx: &LocaltimeFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_localtimeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeFunction(&mut self, ctx: &DatetimeFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_localdatetimeFunction(&mut self, ctx: &LocaldatetimeFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_localdatetimeFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dateFunctionParameters(&mut self, ctx: &DateFunctionParametersContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dateFunctionParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeFunctionParameters(&mut self, ctx: &TimeFunctionParametersContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeFunctionParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeFunctionParameters(
        &mut self,
        ctx: &DatetimeFunctionParametersContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeFunctionParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationValueExpression(&mut self, ctx: &DurationValueExpressionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_durationValueExpression(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeSubtraction(&mut self, ctx: &DatetimeSubtractionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeSubtraction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeSubtractionParameters(
        &mut self,
        ctx: &DatetimeSubtractionParametersContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeSubtractionParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeValueExpression1(&mut self, ctx: &DatetimeValueExpression1Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeValueExpression1(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeValueExpression2(&mut self, ctx: &DatetimeValueExpression2Context<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeValueExpression2(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationValueFunction(&mut self, ctx: &DurationValueFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_durationValueFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationFunction(&mut self, ctx: &DurationFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_durationFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationFunctionParameters(
        &mut self,
        ctx: &DurationFunctionParametersContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_durationFunctionParameters(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_objectName(&mut self, ctx: &ObjectNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_objectName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_objectNameOrBindingVariable(
        &mut self,
        ctx: &ObjectNameOrBindingVariableContext<'input>,
    ) {
        let result = <Self as GQLVisitorCompat>::visit_objectNameOrBindingVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_directoryName(&mut self, ctx: &DirectoryNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_directoryName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_schemaName(&mut self, ctx: &SchemaNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_schemaName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphName(&mut self, ctx: &GraphNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_delimitedGraphName(&mut self, ctx: &DelimitedGraphNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_delimitedGraphName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_graphTypeName(&mut self, ctx: &GraphTypeNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_graphTypeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeTypeName(&mut self, ctx: &NodeTypeNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeTypeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeTypeName(&mut self, ctx: &EdgeTypeNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeTypeName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingTableName(&mut self, ctx: &BindingTableNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingTableName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_delimitedBindingTableName(&mut self, ctx: &DelimitedBindingTableNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_delimitedBindingTableName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_procedureName(&mut self, ctx: &ProcedureNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_procedureName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_labelName(&mut self, ctx: &LabelNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_labelName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_propertyName(&mut self, ctx: &PropertyNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_propertyName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_fieldName(&mut self, ctx: &FieldNameContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_fieldName(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_elementVariable(&mut self, ctx: &ElementVariableContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_elementVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_pathVariable(&mut self, ctx: &PathVariableContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_pathVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_subpathVariable(&mut self, ctx: &SubpathVariableContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_subpathVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_bindingVariable(&mut self, ctx: &BindingVariableContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_bindingVariable(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedLiteral(&mut self, ctx: &UnsignedLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_generalLiteral(&mut self, ctx: &GeneralLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_generalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_temporalLiteral(&mut self, ctx: &TemporalLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_temporalLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dateLiteral(&mut self, ctx: &DateLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dateLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeLiteral(&mut self, ctx: &TimeLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeLiteral(&mut self, ctx: &DatetimeLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_listLiteral(&mut self, ctx: &ListLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_listLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_recordLiteral(&mut self, ctx: &RecordLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_recordLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_identifier(&mut self, ctx: &IdentifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_identifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_regularIdentifier(&mut self, ctx: &RegularIdentifierContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_regularIdentifier(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeZoneString(&mut self, ctx: &TimeZoneStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeZoneString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_characterStringLiteral(&mut self, ctx: &CharacterStringLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_characterStringLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedNumericLiteral(&mut self, ctx: &UnsignedNumericLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedNumericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_exactNumericLiteral(&mut self, ctx: &ExactNumericLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_exactNumericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_approximateNumericLiteral(&mut self, ctx: &ApproximateNumericLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_approximateNumericLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedInteger(&mut self, ctx: &UnsignedIntegerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedInteger(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_unsignedDecimalInteger(&mut self, ctx: &UnsignedDecimalIntegerContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_unsignedDecimalInteger(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nullLiteral(&mut self, ctx: &NullLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nullLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_dateString(&mut self, ctx: &DateStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_dateString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_timeString(&mut self, ctx: &TimeStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_timeString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_datetimeString(&mut self, ctx: &DatetimeStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_datetimeString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationLiteral(&mut self, ctx: &DurationLiteralContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_durationLiteral(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_durationString(&mut self, ctx: &DurationStringContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_durationString(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nodeSynonym(&mut self, ctx: &NodeSynonymContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nodeSynonym(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgesSynonym(&mut self, ctx: &EdgesSynonymContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgesSynonym(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_edgeSynonym(&mut self, ctx: &EdgeSynonymContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_edgeSynonym(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_userDefinedFunction(&mut self, ctx: &UserDefinedFunctionContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_userDefinedFunction(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }

    fn visit_nonReservedWords(&mut self, ctx: &NonReservedWordsContext<'input>) {
        let result = <Self as GQLVisitorCompat>::visit_nonReservedWords(self, ctx);
        *<Self as ParseTreeVisitorCompat>::temp_result(self) = result;
    }
}
