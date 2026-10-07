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

grammar GQL;

// GQL grammar used by gql-rs.
//
// This is a combined ANTLR grammar: lower-camel-case rules form the parser,
// while UPPER_CASE rules in the final part of the file form the lexer. The
// numbered headings reproduce the clause titles from ISO/IEC 39075:2024 and
// are kept as navigation anchors.
//
// Keep syntax recognition in this file. Checks that require type information,
// variable scopes, or knowledge of the surrounding statement belong in the
// AST validator instead. After changing a rule, regenerate src/antlr and
// review the CST-to-AST builder that consumes the affected context. The full
// workflow is documented in docs/generating_parser.md.

// -----------------------------------------------------------------------------
// Parser rules
// -----------------------------------------------------------------------------

// Entry point: a complete GQL program must consume EOF. Session commands and
// transaction/procedure activities are separated here so callers cannot accept
// an otherwise valid prefix followed by unparsed input.
//
// 6 <GQL-program>

gqlProgram
    : programActivity sessionCloseCommand? EOF
    | sessionCloseCommand EOF
    ;

programActivity
    : sessionActivity
    | transactionActivity
    ;

sessionActivity
    : sessionResetCommand+
    | sessionSetCommand+ sessionResetCommand*
    ;

transactionActivity
    : startTransactionCommand (procedureSpecification endTransactionCommand?)?
    | procedureSpecification endTransactionCommand?
    | endTransactionCommand
    ;

endTransactionCommand
    : rollbackCommand
    | commitCommand
    ;

// =============================================================================
// 7 Session management
// =============================================================================

// 7.1 <session set command>

sessionSetCommand
    : SESSION SET (sessionSetSchemaClause | sessionSetGraphClause | sessionSetTimeZoneClause | sessionSetParameterClause)
    ;

sessionSetSchemaClause
    : SCHEMA schemaReference
    ;

sessionSetGraphClause
    : PROPERTY? GRAPH graphExpression
    ;

sessionSetTimeZoneClause
    : TIME ZONE setTimeZoneValue
    ;

setTimeZoneValue
    : timeZoneString
    ;

sessionSetParameterClause
    : sessionSetGraphParameterClause
    | sessionSetBindingTableParameterClause
    | sessionSetValueParameterClause
    ;

sessionSetGraphParameterClause
    : PROPERTY? GRAPH sessionSetParameterName optTypedGraphInitializer
    ;

sessionSetBindingTableParameterClause
    : BINDING? TABLE sessionSetParameterName optTypedBindingTableInitializer
    ;

sessionSetValueParameterClause
    : VALUE sessionSetParameterName optTypedValueInitializer
    ;

sessionSetParameterName
    : (IF NOT EXISTS)? sessionParameterSpecification
    ;

// 7.2 <session reset command>

sessionResetCommand
    : SESSION RESET sessionResetArguments?
    ;

sessionResetArguments
    : ALL? (PARAMETERS | CHARACTERISTICS)
    | SCHEMA
    | PROPERTY? GRAPH
    | TIME ZONE
    | PARAMETER? sessionParameterSpecification
    ;

// 7.3 <session close command>

sessionCloseCommand
    : SESSION CLOSE
    ;

// 7.4 <session parameter specification>

sessionParameterSpecification
    : GENERAL_PARAMETER_REFERENCE
    ;

// =============================================================================
// 8 Transaction management
// =============================================================================

// 8.1 <start transaction command>

startTransactionCommand
    : START TRANSACTION transactionCharacteristics?
    ;

// 8.2 <transaction characteristics>

transactionCharacteristics
    : transactionMode (COMMA transactionMode)*
    ;

transactionMode
    : transactionAccessMode
    ;

transactionAccessMode
    : READ ONLY
    | READ WRITE
    ;

// 8.3 <rollback command>

rollbackCommand
    : ROLLBACK
    ;

// 8.4 <commit command>

commitCommand
    : COMMIT
    ;

// =============================================================================
// 9 Procedure specification
// =============================================================================

// 9.1 <procedure specification>

nestedProcedureSpecification
    : LEFT_BRACE procedureSpecification RIGHT_BRACE
    ;

// <catalog-modifying procedure specification>, <data-modifying procedure
// specification>, and <query specification> are identical productions. The
// specification distinguishes them in the BNF, but their concrete syntax is
// ambiguous. The AST builder and validator classify them from the statements
// contained in the <procedure specification>.
procedureSpecification
    : procedureBody
//    : catalogModifyingProcedureSpecification
//    | dataModifyingProcedureSpecification
//    | querySpecification
    ;

//catalogModifyingProcedureSpecification
//    : procedureBody
//    ;

nestedDataModifyingProcedureSpecification
    : LEFT_BRACE procedureBody RIGHT_BRACE
    ;

//dataModifyingProcedureSpecification
//    : procedureBody
//    ;

nestedQuerySpecification
    : LEFT_BRACE procedureBody RIGHT_BRACE
    ;

//querySpecification
//    : procedureBody
//    ;

// 9.2 <procedure body>

procedureBody
    : atSchemaClause? bindingVariableDefinitionBlock? statementBlock
    ;

bindingVariableDefinitionBlock
    : bindingVariableDefinition+
    ;

bindingVariableDefinition
    : graphVariableDefinition
    | bindingTableVariableDefinition
    | valueVariableDefinition
    ;

statementBlock
    : statement nextStatement*
    ;

statement
    : compositeQueryStatement
    | linearCatalogModifyingStatement
    | linearDataModifyingStatement
    ;

nextStatement
    : NEXT yieldClause? statement
    ;

// =============================================================================
// 10 Variable definitions
// =============================================================================

// 10.1 <graph variable definition>

graphVariableDefinition
    : PROPERTY? GRAPH bindingVariable optTypedGraphInitializer
    ;

optTypedGraphInitializer
    : (typed? graphReferenceValueType)? graphInitializer
    ;

graphInitializer
    : EQUALS_OPERATOR graphExpression
    ;

// 10.2 <binding table variable definition>

bindingTableVariableDefinition
    : BINDING? TABLE bindingVariable optTypedBindingTableInitializer
    ;

optTypedBindingTableInitializer
    : (typed? bindingTableReferenceValueType)? bindingTableInitializer
    ;

bindingTableInitializer
    : EQUALS_OPERATOR bindingTableExpression
    ;

// 10.3 <value variable definition>

valueVariableDefinition
    : VALUE bindingVariable optTypedValueInitializer
    ;

optTypedValueInitializer
    : (typed? valueType)? valueInitializer
    ;

valueInitializer
    : EQUALS_OPERATOR valueExpression
    ;

// =============================================================================
// 11 Object expressions
// =============================================================================

// 11.1 <graph expression>

graphExpression
    : graphReference
    | objectExpressionPrimary
    | objectNameOrBindingVariable
    | currentGraph
    ;

currentGraph
    : CURRENT_PROPERTY_GRAPH
    | CURRENT_GRAPH
    ;

// 11.2 <binding table expression>

bindingTableExpression
    : nestedBindingTableQuerySpecification
    | bindingTableReference
    | objectExpressionPrimary
    | objectNameOrBindingVariable
    ;

nestedBindingTableQuerySpecification
    : nestedQuerySpecification
    ;

// 11.3 <object expression primary>

objectExpressionPrimary
    : VARIABLE valueExpressionPrimary
    | parenthesizedValueExpression
    | nonParenthesizedValueExpressionPrimarySpecialCase
    ;

// =============================================================================
// 12 Catalog-modifying statements
// =============================================================================

// 12.1 <linear catalog-modifying statement>

linearCatalogModifyingStatement
    : simpleCatalogModifyingStatement+
    ;

simpleCatalogModifyingStatement
    : primitiveCatalogModifyingStatement
    | callCatalogModifyingProcedureStatement
    ;

primitiveCatalogModifyingStatement
    : createSchemaStatement
    | dropSchemaStatement
    | createGraphStatement
    | dropGraphStatement
    | createGraphTypeStatement
    | dropGraphTypeStatement
    ;

// 12.2 <create schema statement>

createSchemaStatement
    : CREATE SCHEMA (IF NOT EXISTS)? catalogSchemaParentAndName
    ;

// 12.3 <drop schema statement>

dropSchemaStatement
    : DROP SCHEMA (IF EXISTS)? catalogSchemaParentAndName
    ;

// 12.4 <create graph statement>

createGraphStatement
    : CREATE (PROPERTY? GRAPH (IF NOT EXISTS)? | OR REPLACE PROPERTY? GRAPH) catalogGraphParentAndName (openGraphType | ofGraphType) graphSource?
    ;

openGraphType
    : typed? ANY (PROPERTY? GRAPH)?
    ;

ofGraphType
    : graphTypeLikeGraph
    | typed? graphTypeReference
    | typed? (PROPERTY? GRAPH)? nestedGraphTypeSpecification
    ;

graphTypeLikeGraph
    : LIKE graphExpression
    ;

graphSource
    : AS COPY OF graphExpression
    ;

// 12.5 <drop graph statement>

dropGraphStatement
    : DROP PROPERTY? GRAPH (IF EXISTS)? catalogGraphParentAndName
    ;

// 12.6 <create graph type statement>

createGraphTypeStatement
    : CREATE (PROPERTY? GRAPH TYPE (IF NOT EXISTS)? | OR REPLACE PROPERTY? GRAPH TYPE) catalogGraphTypeParentAndName graphTypeSource
    ;

graphTypeSource
    : AS? copyOfGraphType
    | graphTypeLikeGraph
    | AS? nestedGraphTypeSpecification
    ;

copyOfGraphType
    : COPY OF graphTypeReference
    ;

// 12.7 <drop graph type statement>

dropGraphTypeStatement
    : DROP PROPERTY? GRAPH TYPE (IF EXISTS)? catalogGraphTypeParentAndName
    ;

// 12.8 <call catalog-modifying procedure statement>

callCatalogModifyingProcedureStatement
    : callProcedureStatement
    ;

// =============================================================================
// 13 Data-modifying statements
// =============================================================================

// 13.1 <linear data-modifying statement>

linearDataModifyingStatement
    : focusedLinearDataModifyingStatement
    | ambientLinearDataModifyingStatement
    ;

focusedLinearDataModifyingStatement
    : focusedLinearDataModifyingStatementBody
    | focusedNestedDataModifyingProcedureSpecification
    ;

focusedLinearDataModifyingStatementBody
    : useGraphClause simpleLinearDataAccessingStatement primitiveResultStatement?
    ;

focusedNestedDataModifyingProcedureSpecification
    : useGraphClause nestedDataModifyingProcedureSpecification
    ;

ambientLinearDataModifyingStatement
    : ambientLinearDataModifyingStatementBody
    | nestedDataModifyingProcedureSpecification
    ;

ambientLinearDataModifyingStatementBody
    : simpleLinearDataAccessingStatement primitiveResultStatement?
    ;

simpleLinearDataAccessingStatement
    : simpleDataAccessingStatement+
    ;

simpleDataAccessingStatement
    : simpleQueryStatement
    | simpleDataModifyingStatement
    ;

simpleDataModifyingStatement
    : primitiveDataModifyingStatement
    | callDataModifyingProcedureStatement
    ;

primitiveDataModifyingStatement
    : insertStatement
    | setStatement
    | removeStatement
    | deleteStatement
    ;

// 13.2 <insert statement>

insertStatement
    : INSERT insertGraphPattern
    ;

// 13.3 <set statement>

setStatement
    : SET setItemList
    ;

setItemList
    : setItem (COMMA setItem)*
    ;

setItem
    : setPropertyItem
    | setAllPropertiesItem
    | setLabelItem
    ;

setPropertyItem
    : bindingVariableReference PERIOD propertyName EQUALS_OPERATOR valueExpression
    ;

setAllPropertiesItem
    : bindingVariableReference EQUALS_OPERATOR LEFT_BRACE propertyKeyValuePairList? RIGHT_BRACE
    ;

setLabelItem
    : bindingVariableReference isOrColon labelName
    ;

// 13.4 <remove statement>

removeStatement
    : REMOVE removeItemList
    ;

removeItemList
    : removeItem (COMMA removeItem)*
    ;

removeItem
    : removePropertyItem
    | removeLabelItem
    ;

removePropertyItem
    : bindingVariableReference PERIOD propertyName
    ;

removeLabelItem
    : bindingVariableReference isOrColon labelName
    ;

// 13.5 <delete statement>

deleteStatement
    : (DETACH | NODETACH)? DELETE deleteItemList
    ;

deleteItemList
    : deleteItem (COMMA deleteItem)*
    ;

deleteItem
    : valueExpression
    ;

// 13.6 <call data-modifying procedure statement>

callDataModifyingProcedureStatement
    : callProcedureStatement
    ;

// =============================================================================
// 14 Query statements
// =============================================================================

// 14.1 <composite query statement>

compositeQueryStatement
    : compositeQueryExpression
    ;

// 14.2 <composite query expression>

compositeQueryExpression
    : compositeQueryExpression queryConjunction compositeQueryPrimary
    | compositeQueryPrimary
    ;

queryConjunction
    : setOperator
    | OTHERWISE
    ;

setOperator
    : UNION setQuantifier?
    | EXCEPT setQuantifier?
    | INTERSECT setQuantifier?
    ;

compositeQueryPrimary
    : linearQueryStatement
    ;

// 14.3 <linear query statement> and <simple query statement>

linearQueryStatement
    : focusedLinearQueryStatement
    | ambientLinearQueryStatement
    ;

focusedLinearQueryStatement
    : focusedLinearQueryStatementPart* focusedLinearQueryAndPrimitiveResultStatementPart
    | focusedPrimitiveResultStatement
    | focusedNestedQuerySpecification
    | selectStatement
    ;

focusedLinearQueryStatementPart
    : useGraphClause simpleLinearQueryStatement
    ;

focusedLinearQueryAndPrimitiveResultStatementPart
    : useGraphClause simpleLinearQueryStatement primitiveResultStatement
    ;

focusedPrimitiveResultStatement
    : useGraphClause primitiveResultStatement
    ;

focusedNestedQuerySpecification
    : useGraphClause nestedQuerySpecification
    ;

ambientLinearQueryStatement
    : simpleLinearQueryStatement? primitiveResultStatement
    | nestedQuerySpecification
    ;

simpleLinearQueryStatement
    : simpleQueryStatement+
    ;

simpleQueryStatement
    : primitiveQueryStatement
    | callQueryStatement
    ;

primitiveQueryStatement
    : matchStatement
    | letStatement
    | forStatement
    | filterStatement
    | orderByAndPageStatement
    ;

// 14.4 <match statement>

matchStatement
    : simpleMatchStatement
    | optionalMatchStatement
    ;

simpleMatchStatement
    : MATCH graphPatternBindingTable
    ;

optionalMatchStatement
    : OPTIONAL optionalOperand
    ;

optionalOperand
    : simpleMatchStatement
    | LEFT_BRACE matchStatementBlock RIGHT_BRACE
    | LEFT_PAREN matchStatementBlock RIGHT_PAREN
    ;

matchStatementBlock
    : matchStatement+
    ;

// 14.5 <call query statement>

callQueryStatement
    : callProcedureStatement
    ;

// 14.6 <filter statement>

filterStatement
    : FILTER (whereClause | searchCondition)
    ;

// 14.7 <let statement>

letStatement
    : LET letVariableDefinitionList
    ;

letVariableDefinitionList
    : letVariableDefinition (COMMA letVariableDefinition)*
    ;

letVariableDefinition
    : valueVariableDefinition
    | bindingVariable EQUALS_OPERATOR valueExpression
    ;

// 14.8 <for statement>

forStatement
    : FOR forItem forOrdinalityOrOffset?
    ;

forItem
    : forItemAlias forItemSource
    ;

forItemAlias
    : bindingVariable IN
    ;

forItemSource
    : valueExpression
    ;

forOrdinalityOrOffset
    : WITH (ORDINALITY | OFFSET) bindingVariable
    ;

// 14.9 <order by and page statement>

orderByAndPageStatement
    : orderByClause offsetClause? limitClause?
    | offsetClause limitClause?
    | limitClause
    ;

// 14.10 <primitive result statement>

primitiveResultStatement
    : returnStatement orderByAndPageStatement?
    | FINISH
    ;

// 14.11 <return statement>

returnStatement
    : RETURN returnStatementBody
    ;

returnStatementBody
    : setQuantifier? (ASTERISK | returnItemList) groupByClause?
    ;

returnItemList
    : returnItem (COMMA returnItem)*
    ;

returnItem
    : aggregatingValueExpression returnItemAlias?
    ;

returnItemAlias
    : AS identifier
    ;

// 14.12 <select statement>

selectStatement
    : SELECT setQuantifier? (ASTERISK | selectItemList) (selectStatementBody whereClause? groupByClause? havingClause? orderByClause? offsetClause? limitClause?)?
    ;

selectItemList
    : selectItem (COMMA selectItem)*
    ;

selectItem
    : aggregatingValueExpression selectItemAlias?
    ;

selectItemAlias
    : AS identifier
    ;

havingClause
    : HAVING searchCondition
    ;

selectStatementBody
    : FROM (selectGraphMatchList | selectQuerySpecification)
    ;

selectGraphMatchList
    : selectGraphMatch (COMMA selectGraphMatch)*
    ;

selectGraphMatch
    : graphExpression matchStatement
    ;

selectQuerySpecification
    : nestedQuerySpecification
    | graphExpression nestedQuerySpecification
    ;

// =============================================================================
// 15 Procedure calling
// =============================================================================

// 15.1 <call procedure statement> and <procedure call>

callProcedureStatement
    : OPTIONAL? CALL procedureCall
    ;

procedureCall
    : inlineProcedureCall
    | namedProcedureCall
    ;

// 15.2 <inline procedure call>

inlineProcedureCall
    : variableScopeClause? nestedProcedureSpecification
    ;

variableScopeClause
    : LEFT_PAREN bindingVariableReferenceList? RIGHT_PAREN
    ;

bindingVariableReferenceList
    : bindingVariableReference (COMMA bindingVariableReference)*
    ;

// 15.3 <named procedure call>

namedProcedureCall
    : procedureReference LEFT_PAREN procedureArgumentList? RIGHT_PAREN yieldClause?
    ;

procedureArgumentList
    : procedureArgument (COMMA procedureArgument)*
    ;

procedureArgument
    : valueExpression
    ;

// =============================================================================
// 16 Common elements
// =============================================================================

// 16.1 <at schema clause>

atSchemaClause
    : AT schemaReference
    ;

// 16.2 <use graph clause>

useGraphClause
    : USE graphExpression
    ;

// 16.3 <graph pattern binding table>

graphPatternBindingTable
    : graphPattern graphPatternYieldClause?
    ;

graphPatternYieldClause
    : YIELD graphPatternYieldItemList
    ;

graphPatternYieldItemList
    : graphPatternYieldItem (COMMA graphPatternYieldItem)*
    ;

// <element variable reference> and <path variable reference> are identical
// productions: both contain one <binding variable reference>. Consequently a
// <graph pattern yield item> cannot distinguish them syntactically. The parser
// therefore produces bindingVariableReference and leaves classification to the
// AST builder and validator.
graphPatternYieldItem
    : bindingVariableReference
//    : elementVariableReference
//    | pathVariableReference
    ;

// 16.4 <graph pattern>

graphPattern
    : matchMode? pathPatternList keepClause? graphPatternWhereClause?
    ;

matchMode
    : repeatableElementsMatchMode
    | differentEdgesMatchMode
    ;

repeatableElementsMatchMode
    : REPEATABLE elementBindingsOrElements
    ;

differentEdgesMatchMode
    : DIFFERENT edgeBindingsOrEdges
    ;

elementBindingsOrElements
    : ELEMENT BINDINGS?
    | ELEMENTS
    ;

edgeBindingsOrEdges
    : edgeSynonym BINDINGS?
    | edgesSynonym
    ;

pathPatternList
    : pathPattern (COMMA pathPattern)*
    ;

pathPattern
    : pathVariableDeclaration? pathPatternPrefix? pathPatternExpression
    ;

pathVariableDeclaration
    : pathVariable EQUALS_OPERATOR
    ;

keepClause
    : KEEP pathPatternPrefix
    ;

graphPatternWhereClause
    : WHERE searchCondition
    ;

// 16.5 <insert graph pattern>

insertGraphPattern
    : insertPathPatternList
    ;

insertPathPatternList
    : insertPathPattern (COMMA insertPathPattern)*
    ;

insertPathPattern
    : insertNodePattern (insertEdgePattern insertNodePattern)*
    ;

insertNodePattern
    : LEFT_PAREN insertElementPatternFiller? RIGHT_PAREN
    ;

insertEdgePattern
    : insertEdgePointingLeft
    | insertEdgePointingRight
    | insertEdgeUndirected
    ;

insertEdgePointingLeft
    : LEFT_ARROW_BRACKET insertElementPatternFiller? RIGHT_BRACKET_MINUS
    ;

insertEdgePointingRight
    : MINUS_LEFT_BRACKET insertElementPatternFiller? BRACKET_RIGHT_ARROW
    ;

insertEdgeUndirected
    : TILDE_LEFT_BRACKET insertElementPatternFiller? RIGHT_BRACKET_TILDE
    ;

insertElementPatternFiller
    : elementVariableDeclaration labelAndPropertySetSpecification?
    | elementVariableDeclaration? labelAndPropertySetSpecification
    ;

labelAndPropertySetSpecification
    : isOrColon labelSetSpecification elementPropertySpecification?
    | (isOrColon labelSetSpecification)? elementPropertySpecification
    ;

// 16.6 <path pattern prefix>

pathPatternPrefix
    : pathModePrefix
    | pathSearchPrefix
    ;

pathModePrefix
    : pathMode pathOrPaths?
    ;

pathMode
    : WALK
    | TRAIL
    | SIMPLE
    | ACYCLIC
    ;

pathSearchPrefix
    : allPathSearch
    | anyPathSearch
    | shortestPathSearch
    ;

allPathSearch
    : ALL pathMode? pathOrPaths?
    ;

pathOrPaths
    : PATH
    | PATHS
    ;

anyPathSearch
    : ANY numberOfPaths? pathMode? pathOrPaths?
    ;

numberOfPaths
    : nonNegativeIntegerSpecification
    ;

shortestPathSearch
    : allShortestPathSearch
    | anyShortestPathSearch
    | countedShortestPathSearch
    | countedShortestGroupSearch
    ;

allShortestPathSearch
    : ALL SHORTEST pathMode? pathOrPaths?
    ;

anyShortestPathSearch
    : ANY SHORTEST pathMode? pathOrPaths?
    ;

countedShortestPathSearch
    : SHORTEST numberOfPaths pathMode? pathOrPaths?
    ;

countedShortestGroupSearch
    : SHORTEST numberOfGroups? pathMode? pathOrPaths? (GROUP | GROUPS)
    ;

numberOfGroups
    : nonNegativeIntegerSpecification
    ;

// 16.7 <path pattern expression>

pathPatternExpression
    : pathTerm                                              #ppePathTerm
    | pathTerm (MULTISET_ALTERNATION_OPERATOR pathTerm)+    #ppeMultisetAlternation
    | pathTerm (VERTICAL_BAR pathTerm)+                     #ppePatternUnion
    ;

pathTerm
    : pathFactor+
    ;

pathFactor
    : pathPrimary                           #pfPathPrimary
    | pathPrimary graphPatternQuantifier    #pfQuantifiedPathPrimary
    | pathPrimary QUESTION_MARK             #pfQuestionedPathPrimary
    ;

pathPrimary
    : elementPattern                        #ppElementPattern
    | parenthesizedPathPatternExpression    #ppParenthesizedPathPatternExpression
    | simplifiedPathPatternExpression       #ppSimplifiedPathPatternExpression
    ;

elementPattern
    : nodePattern
    | edgePattern
    ;

nodePattern
    : LEFT_PAREN elementPatternFiller RIGHT_PAREN
    ;

elementPatternFiller
    : elementVariableDeclaration? isLabelExpression? elementPatternPredicate?
    ;

elementVariableDeclaration
    : elementVariable
    ;

isLabelExpression
    : isOrColon labelExpression
    ;

isOrColon
    : IS
    | COLON
    ;

elementPatternPredicate
    : elementPatternWhereClause
    | elementPropertySpecification
    ;

elementPatternWhereClause
    : WHERE searchCondition
    ;

elementPropertySpecification
    : LEFT_BRACE propertyKeyValuePairList RIGHT_BRACE
    ;

propertyKeyValuePairList
    : propertyKeyValuePair (COMMA propertyKeyValuePair)*
    ;

propertyKeyValuePair
    : propertyName COLON valueExpression
    ;

edgePattern
    : fullEdgePattern
    | abbreviatedEdgePattern
    ;

fullEdgePattern
    : fullEdgePointingLeft
    | fullEdgeUndirected
    | fullEdgePointingRight
    | fullEdgeLeftOrUndirected
    | fullEdgeUndirectedOrRight
    | fullEdgeLeftOrRight
    | fullEdgeAnyDirection
    ;

fullEdgePointingLeft
    : LEFT_ARROW_BRACKET elementPatternFiller RIGHT_BRACKET_MINUS
    ;

fullEdgeUndirected
    : TILDE_LEFT_BRACKET elementPatternFiller RIGHT_BRACKET_TILDE
    ;

fullEdgePointingRight
    : MINUS_LEFT_BRACKET elementPatternFiller BRACKET_RIGHT_ARROW
    ;

fullEdgeLeftOrUndirected
    : LEFT_ARROW_TILDE_BRACKET elementPatternFiller RIGHT_BRACKET_TILDE
    ;

fullEdgeUndirectedOrRight
    : TILDE_LEFT_BRACKET elementPatternFiller BRACKET_TILDE_RIGHT_ARROW
    ;

fullEdgeLeftOrRight
    : LEFT_ARROW_BRACKET elementPatternFiller BRACKET_RIGHT_ARROW
    ;

fullEdgeAnyDirection
    : MINUS_LEFT_BRACKET elementPatternFiller RIGHT_BRACKET_MINUS
    ;

abbreviatedEdgePattern
    : LEFT_ARROW
    | TILDE
    | RIGHT_ARROW
    | LEFT_ARROW_TILDE
    | TILDE_RIGHT_ARROW
    | LEFT_MINUS_RIGHT
    | MINUS_SIGN
    ;

parenthesizedPathPatternExpression
    : LEFT_PAREN subpathVariableDeclaration? pathModePrefix? pathPatternExpression parenthesizedPathPatternWhereClause? RIGHT_PAREN
    ;

subpathVariableDeclaration
    : subpathVariable EQUALS_OPERATOR
    ;

parenthesizedPathPatternWhereClause
    : WHERE searchCondition
    ;

// 16.8 <label expression>

labelExpression
    : EXCLAMATION_MARK labelExpression                  #labelExpressionNegation
    | labelExpression AMPERSAND labelExpression         #labelExpressionConjunction
    | labelExpression VERTICAL_BAR labelExpression      #labelExpressionDisjunction
    | labelName                                         #labelExpressionName
    | PERCENT                                           #labelExpressionWildcard
    | LEFT_PAREN labelExpression RIGHT_PAREN            #labelExpressionParenthesized
    ;

// 16.9 <path variable reference>

pathVariableReference
    : bindingVariableReference
    ;

// 16.10 <element variable reference>

elementVariableReference
    : bindingVariableReference
    ;

// 16.11 <graph pattern quantifier>

graphPatternQuantifier
    : ASTERISK
    | PLUS_SIGN
    | fixedQuantifier
    | generalQuantifier
    ;

fixedQuantifier
    : LEFT_BRACE unsignedInteger RIGHT_BRACE
    ;

generalQuantifier
    : LEFT_BRACE lowerBound? COMMA upperBound? RIGHT_BRACE
    ;

lowerBound
    : unsignedInteger
    ;

upperBound
    : unsignedInteger
    ;

// 16.12 <simplified path pattern expression>

simplifiedPathPatternExpression
    : simplifiedDefaultingLeft
    | simplifiedDefaultingUndirected
    | simplifiedDefaultingRight
    | simplifiedDefaultingLeftOrUndirected
    | simplifiedDefaultingUndirectedOrRight
    | simplifiedDefaultingLeftOrRight
    | simplifiedDefaultingAnyDirection
    ;

simplifiedDefaultingLeft
    : LEFT_MINUS_SLASH simplifiedContents SLASH_MINUS
    ;

simplifiedDefaultingUndirected
    : TILDE_SLASH simplifiedContents SLASH_TILDE
    ;

simplifiedDefaultingRight
    : MINUS_SLASH simplifiedContents SLASH_MINUS_RIGHT
    ;

simplifiedDefaultingLeftOrUndirected
    : LEFT_TILDE_SLASH simplifiedContents SLASH_TILDE
    ;

simplifiedDefaultingUndirectedOrRight
    : TILDE_SLASH simplifiedContents SLASH_TILDE_RIGHT
    ;

simplifiedDefaultingLeftOrRight
    : LEFT_MINUS_SLASH simplifiedContents SLASH_MINUS_RIGHT
    ;

simplifiedDefaultingAnyDirection
    : MINUS_SLASH simplifiedContents SLASH_MINUS
    ;

simplifiedContents
    : simplifiedTerm
    | simplifiedPathUnion
    | simplifiedMultisetAlternation
    ;

simplifiedPathUnion
    : simplifiedTerm VERTICAL_BAR simplifiedTerm (VERTICAL_BAR simplifiedTerm)*
    ;

simplifiedMultisetAlternation
    : simplifiedTerm MULTISET_ALTERNATION_OPERATOR simplifiedTerm (MULTISET_ALTERNATION_OPERATOR simplifiedTerm)*
    ;

simplifiedTerm
    : simplifiedFactorLow                        #simplifiedFactorLowLabel
    | simplifiedTerm simplifiedFactorLow      #simplifiedConcatenationLabel
    ;

simplifiedFactorLow
    : simplifiedFactorHigh                                     #simplifiedFactorHighLabel
    | simplifiedFactorLow AMPERSAND simplifiedFactorHigh #simplifiedConjunctionLabel
    ;

simplifiedFactorHigh
    : simplifiedTertiary
    | simplifiedQuantified
    | simplifiedQuestioned
    ;

simplifiedQuantified
    : simplifiedTertiary graphPatternQuantifier
    ;

simplifiedQuestioned
    : simplifiedTertiary QUESTION_MARK
    ;

simplifiedTertiary
    : simplifiedDirectionOverride
    | simplifiedSecondary
    ;

simplifiedDirectionOverride
    : simplifiedOverrideLeft
    | simplifiedOverrideUndirected
    | simplifiedOverrideRight
    | simplifiedOverrideLeftOrUndirected
    | simplifiedOverrideUndirectedOrRight
    | simplifiedOverrideLeftOrRight
    | simplifiedOverrideAnyDirection
    ;

simplifiedOverrideLeft
    : LEFT_ANGLE_BRACKET simplifiedSecondary
    ;

simplifiedOverrideUndirected
    : TILDE simplifiedSecondary
    ;

simplifiedOverrideRight
    : simplifiedSecondary RIGHT_ANGLE_BRACKET
    ;

simplifiedOverrideLeftOrUndirected
    : LEFT_ARROW_TILDE simplifiedSecondary
    ;

simplifiedOverrideUndirectedOrRight
    : TILDE simplifiedSecondary RIGHT_ANGLE_BRACKET
    ;

simplifiedOverrideLeftOrRight
    : LEFT_ANGLE_BRACKET simplifiedSecondary RIGHT_ANGLE_BRACKET
    ;

simplifiedOverrideAnyDirection
    : MINUS_SIGN simplifiedSecondary
    ;

simplifiedSecondary
    : simplifiedPrimary
    | simplifiedNegation
    ;

simplifiedNegation
    : EXCLAMATION_MARK simplifiedPrimary
    ;

simplifiedPrimary
    : labelName
    | LEFT_PAREN simplifiedContents RIGHT_PAREN
    ;

// 16.13 <where clause>

whereClause
    : WHERE searchCondition
    ;

// 16.14 <yield clause>

yieldClause
    : YIELD yieldItemList
    ;

yieldItemList
    : yieldItem (COMMA yieldItem)*
    ;

yieldItem
    : (yieldItemName yieldItemAlias?)
    ;

yieldItemName
    : fieldName
    ;

yieldItemAlias
    : AS bindingVariable
    ;

// 16.15 <group by clause>

groupByClause
    : GROUP BY groupingElementList
    ;

groupingElementList
    : groupingElement (COMMA groupingElement)*
    | emptyGroupingSet
    ;

groupingElement
    : bindingVariableReference
    ;

emptyGroupingSet
    : LEFT_PAREN RIGHT_PAREN
    ;

// 16.16 <order by clause>

orderByClause
    : ORDER BY sortSpecificationList
    ;

// 16.17 <sort specification list>

sortSpecificationList
    : sortSpecification (COMMA sortSpecification)*
    ;

sortSpecification
    : sortKey orderingSpecification? nullOrdering?
    ;

sortKey
    : aggregatingValueExpression
    ;

orderingSpecification
    : ASC
    | ASCENDING
    | DESC
    | DESCENDING
    ;

nullOrdering
    : NULLS FIRST
    | NULLS LAST
    ;

// 16.18 <limit clause>

limitClause
    : LIMIT nonNegativeIntegerSpecification
    ;

// 16.19 <offset clause>

offsetClause
    : offsetSynonym nonNegativeIntegerSpecification
    ;

offsetSynonym
    : OFFSET
    | SKIP_RESERVED_WORD
    ;

// =============================================================================
// 17 Object references
// =============================================================================

// 17.1 <schema reference> and <catalog schema parent and name>

schemaReference
    : absoluteCatalogSchemaReference
    | relativeCatalogSchemaReference
    | referenceParameterSpecification
    ;

absoluteCatalogSchemaReference
    : SOLIDUS
    | absoluteDirectoryPath schemaName
    ;

catalogSchemaParentAndName
    : absoluteDirectoryPath schemaName
    ;

relativeCatalogSchemaReference
    : predefinedSchemaReference
    | relativeDirectoryPath schemaName
    ;

predefinedSchemaReference
    : HOME_SCHEMA
    | CURRENT_SCHEMA
    | PERIOD
    ;

absoluteDirectoryPath
    : SOLIDUS simpleDirectoryPath?
    ;

relativeDirectoryPath
    : DOUBLE_PERIOD (SOLIDUS DOUBLE_PERIOD)* SOLIDUS simpleDirectoryPath?
    ;

simpleDirectoryPath
    : (directoryName SOLIDUS)+
    ;

// 17.2 <graph reference> and <catalog graph parent and name>

graphReference
    : catalogObjectParentReference graphName
    | delimitedGraphName
    | homeGraph
    | referenceParameterSpecification
    ;

catalogGraphParentAndName
    : catalogObjectParentReference? graphName
    ;

homeGraph
    : HOME_PROPERTY_GRAPH
    | HOME_GRAPH
    ;

// 17.3 <graph type reference> and <catalog graph type parent and name>

graphTypeReference
    : catalogGraphTypeParentAndName
    | referenceParameterSpecification
    ;

catalogGraphTypeParentAndName
    : catalogObjectParentReference? graphTypeName
    ;

// 17.4 <binding table reference> and <catalog binding table parent and name>

bindingTableReference
    : catalogObjectParentReference bindingTableName
    | delimitedBindingTableName
    | referenceParameterSpecification
    ;

// 17.5 <procedure reference> and <catalog procedure parent and name>

procedureReference
    : catalogProcedureParentAndName
    | referenceParameterSpecification
    ;

catalogProcedureParentAndName
    : catalogObjectParentReference? procedureName
    ;

// 17.6 <catalog object parent reference>

catalogObjectParentReference
    : schemaReference SOLIDUS? (objectName PERIOD)*
    |  (objectName PERIOD)+
    ;

// 17.7 <reference parameter specification>

referenceParameterSpecification
    : SUBSTITUTED_PARAMETER_REFERENCE
    ;

// =============================================================================
// 18 Type elements
// =============================================================================

// 18.1 <nested graph type specification>

nestedGraphTypeSpecification
    : LEFT_BRACE graphTypeSpecificationBody RIGHT_BRACE
    ;

graphTypeSpecificationBody
    : elementTypeList
    ;

elementTypeList
    : elementTypeSpecification (COMMA elementTypeSpecification)*
    ;

elementTypeSpecification
    : nodeTypeSpecification
    | edgeTypeSpecification
    ;

// 18.2 <node type specification>

nodeTypeSpecification
    : nodeTypePattern
    | nodeTypePhrase
    ;

nodeTypePattern
    : (nodeSynonym TYPE? nodeTypeName)? LEFT_PAREN localNodeTypeAlias? nodeTypeFiller? RIGHT_PAREN
    ;

nodeTypePhrase
    : nodeSynonym TYPE? nodeTypePhraseFiller (AS localNodeTypeAlias)?
    ;

nodeTypePhraseFiller
    : nodeTypeName nodeTypeFiller?
    | nodeTypeFiller
    ;

nodeTypeFiller
    : nodeTypeKeyLabelSet nodeTypeImpliedContent?
    | nodeTypeImpliedContent
    ;

localNodeTypeAlias
    : regularIdentifier
    ;

nodeTypeImpliedContent
    : nodeTypeLabelSet
    | nodeTypePropertyTypes
    | nodeTypeLabelSet nodeTypePropertyTypes
    ;

nodeTypeKeyLabelSet
    : labelSetPhrase? IMPLIES
    ;

nodeTypeLabelSet
    : labelSetPhrase
    ;

nodeTypePropertyTypes
    : propertyTypesSpecification
    ;

// 18.3 <edge type specification>

edgeTypeSpecification
    : edgeTypePattern
    | edgeTypePhrase
    ;

edgeTypePattern
    : (edgeKind? edgeSynonym TYPE? edgeTypeName)? (edgeTypePatternDirected | edgeTypePatternUndirected)
    ;

edgeTypePhrase
    : edgeKind edgeSynonym TYPE? edgeTypePhraseFiller endpointPairPhrase
    ;

edgeTypePhraseFiller
    : edgeTypeName edgeTypeFiller?
    | edgeTypeFiller
    ;

edgeTypeFiller
    : edgeTypeKeyLabelSet edgeTypeImpliedContent?
    | edgeTypeImpliedContent
    ;

edgeTypeImpliedContent
    : edgeTypeLabelSet
    | edgeTypePropertyTypes
    | edgeTypeLabelSet edgeTypePropertyTypes
    ;

edgeTypeKeyLabelSet
    : labelSetPhrase? IMPLIES
    ;

edgeTypeLabelSet
    : labelSetPhrase
    ;

edgeTypePropertyTypes
    : propertyTypesSpecification
    ;

edgeTypePatternDirected
    : edgeTypePatternPointingRight
    | edgeTypePatternPointingLeft
    ;

edgeTypePatternPointingRight
    : sourceNodeTypeReference arcTypePointingRight destinationNodeTypeReference
    ;

edgeTypePatternPointingLeft
    : destinationNodeTypeReference arcTypePointingLeft sourceNodeTypeReference
    ;

edgeTypePatternUndirected
    : sourceNodeTypeReference arcTypeUndirected destinationNodeTypeReference
    ;

arcTypePointingRight
    : MINUS_LEFT_BRACKET edgeTypeFiller BRACKET_RIGHT_ARROW
    ;

arcTypePointingLeft
    : LEFT_ARROW_BRACKET edgeTypeFiller RIGHT_BRACKET_MINUS
    ;

arcTypeUndirected
    : TILDE_LEFT_BRACKET edgeTypeFiller RIGHT_BRACKET_TILDE
    ;

sourceNodeTypeReference
    : LEFT_PAREN sourceNodeTypeAlias RIGHT_PAREN
    | LEFT_PAREN nodeTypeFiller? RIGHT_PAREN
    ;

destinationNodeTypeReference
    : LEFT_PAREN destinationNodeTypeAlias RIGHT_PAREN
    | LEFT_PAREN nodeTypeFiller? RIGHT_PAREN
    ;

edgeKind
    : DIRECTED
    | UNDIRECTED
    ;

endpointPairPhrase
    : CONNECTING endpointPair
    ;

endpointPair
    : endpointPairDirected
    | endpointPairUndirected
    ;

endpointPairDirected
    : endpointPairPointingRight
    | endpointPairPointingLeft
    ;

endpointPairPointingRight
    : LEFT_PAREN sourceNodeTypeAlias connectorPointingRight destinationNodeTypeAlias RIGHT_PAREN
    ;

endpointPairPointingLeft
    : LEFT_PAREN destinationNodeTypeAlias LEFT_ARROW sourceNodeTypeAlias RIGHT_PAREN
    ;

endpointPairUndirected
    : LEFT_PAREN sourceNodeTypeAlias connectorUndirected destinationNodeTypeAlias RIGHT_PAREN
    ;

connectorPointingRight
    : TO
    | RIGHT_ARROW
    ;

connectorUndirected
    : TO
    | TILDE
    ;

sourceNodeTypeAlias
    : regularIdentifier
    ;

destinationNodeTypeAlias
    : regularIdentifier
    ;

// 18.4 <label set phrase> and <label set specification>

labelSetPhrase
    : LABEL labelName
    | LABELS labelSetSpecification
    | isOrColon labelSetSpecification
    ;

labelSetSpecification
    : labelName (AMPERSAND labelName)*
    ;

// 18.5 <property types specification>

propertyTypesSpecification
    : LEFT_BRACE propertyTypeList? RIGHT_BRACE
    ;

propertyTypeList
    : propertyType (COMMA propertyType)*
    ;

// 18.6 <property type>

propertyType
    : propertyName typed? propertyValueType
    ;

// 18.7 <property value type>

propertyValueType
    : valueType
    ;

// 18.8 <binding table type>

bindingTableType
    : BINDING? TABLE fieldTypesSpecification
    ;

// 18.9 <value type>

valueType
    : predefinedType                                                                                                                              #predefinedTypeLabel
    // <constructed value type>
    | pathValueType                                                                                                                               #pathValueTypeLabel
    | listValueTypeName LEFT_ANGLE_BRACKET valueType RIGHT_ANGLE_BRACKET (LEFT_BRACKET maxLength RIGHT_BRACKET)? notNull?     #listValueTypeAlt1
    | valueType listValueTypeName (LEFT_BRACKET maxLength RIGHT_BRACKET)? notNull?                                                    #listValueTypeAlt2
    | listValueTypeName (LEFT_BRACKET maxLength RIGHT_BRACKET)? notNull?                                                                #listValueTypeAlt3
    | recordType                                                                                                                                   #recordTypeLabel
    // <dynamic union type>
    | ANY VALUE? notNull?                                                                                                                        #openDynamicUnionTypeLabel
    | ANY? PROPERTY VALUE notNull?                                                                                                             #dynamicPropertyValueTypeLabel
    // <closed dynamic union type>
    | ANY VALUE? LEFT_ANGLE_BRACKET valueType (VERTICAL_BAR valueType)* RIGHT_ANGLE_BRACKET                                         #closedDynamicUnionTypeAtl1
    | valueType VERTICAL_BAR valueType                                                                                                        #closedDynamicUnionTypeAtl2
    ;

typed
    : DOUBLE_COLON
    | TYPED
    ;

predefinedType
    : booleanType
    | characterStringType
    | byteStringType
    | numericType
    | temporalType
    | referenceValueType
    | immaterialValueType
    ;

booleanType
    : (BOOL | BOOLEAN) notNull?
    ;

characterStringType
    : STRING (LEFT_PAREN (minLength COMMA)? maxLength RIGHT_PAREN)? notNull?
    | CHAR (LEFT_PAREN fixedLength RIGHT_PAREN)? notNull?
    | VARCHAR (LEFT_PAREN maxLength RIGHT_PAREN)? notNull?
    ;

byteStringType
    : BYTES (LEFT_PAREN (minLength COMMA)? maxLength RIGHT_PAREN)? notNull?
    | BINARY (LEFT_PAREN fixedLength RIGHT_PAREN)? notNull?
    | VARBINARY (LEFT_PAREN maxLength RIGHT_PAREN)? notNull?
    ;

minLength
    : unsignedInteger
    ;

maxLength
    : unsignedInteger
    ;

fixedLength
    : unsignedInteger
    ;

numericType
    : exactNumericType
    | approximateNumericType
    ;

exactNumericType
    : binaryExactNumericType
    | decimalExactNumericType
    ;

binaryExactNumericType
    : signedBinaryExactNumericType
    | unsignedBinaryExactNumericType
    ;

signedBinaryExactNumericType
    : INT8 notNull?
    | INT16 notNull?
    | INT32 notNull?
    | INT64 notNull?
    | INT128 notNull?
    | INT256 notNull?
    | SMALLINT notNull?
    | INT (LEFT_PAREN precision RIGHT_PAREN)? notNull?
    | BIGINT notNull?
    | SIGNED? verboseBinaryExactNumericType
    ;

unsignedBinaryExactNumericType
    : UINT8 notNull?
    | UINT16 notNull?
    | UINT32 notNull?
    | UINT64 notNull?
    | UINT128 notNull?
    | UINT256 notNull?
    | USMALLINT notNull?
    | UINT (LEFT_PAREN precision RIGHT_PAREN)? notNull?
    | UBIGINT notNull?
    | UNSIGNED verboseBinaryExactNumericType
    ;

verboseBinaryExactNumericType
    : INTEGER8 notNull?
    | INTEGER16 notNull?
    | INTEGER32 notNull?
    | INTEGER64 notNull?
    | INTEGER128 notNull?
    | INTEGER256 notNull?
    | SMALL INTEGER notNull?
    | INTEGER (LEFT_PAREN precision RIGHT_PAREN)? notNull?
    | BIG INTEGER notNull?
    ;

decimalExactNumericType
    : (DECIMAL | DEC) (LEFT_PAREN precision (COMMA scale)? RIGHT_PAREN notNull?)?
    ;

precision
    : unsignedDecimalInteger
    ;

scale
    : unsignedDecimalInteger
    ;

approximateNumericType
    : FLOAT16 notNull?
    | FLOAT32 notNull?
    | FLOAT64 notNull?
    | FLOAT128 notNull?
    | FLOAT256 notNull?
    | FLOAT (LEFT_PAREN precision (COMMA scale)? RIGHT_PAREN)? notNull?
    | REAL notNull?
    | DOUBLE PRECISION? notNull?
    ;

temporalType
    : temporalInstantType
    | temporalDurationType
    ;

temporalInstantType
    : datetimeType
    | localdatetimeType
    | dateType
    | timeType
    | localtimeType
    ;

datetimeType
    : ZONED DATETIME notNull?
    | TIMESTAMP WITH TIME ZONE notNull?
    ;

localdatetimeType
    : LOCAL DATETIME notNull?
    | TIMESTAMP (WITHOUT TIME ZONE)? notNull?
    ;

dateType
    : DATE notNull?
    ;

timeType
    : ZONED TIME notNull?
    | TIME WITH TIME ZONE notNull?
    ;

localtimeType
    : LOCAL TIME notNull?
    | TIME WITHOUT TIME ZONE notNull?
    ;

temporalDurationType
    : DURATION LEFT_PAREN temporalDurationQualifier RIGHT_PAREN notNull?
    ;

temporalDurationQualifier
    : YEAR TO MONTH
    | DAY TO SECOND
    ;

referenceValueType
    : graphReferenceValueType
    | bindingTableReferenceValueType
    | nodeReferenceValueType
    | edgeReferenceValueType
    ;

immaterialValueType
    : nullType
    | emptyType
    ;

nullType
    : NULL_KW
    ;

emptyType
    : NULL_KW notNull
    | NOTHING
    ;

graphReferenceValueType
    : openGraphReferenceValueType
    | closedGraphReferenceValueType
    ;

closedGraphReferenceValueType
    : PROPERTY? GRAPH nestedGraphTypeSpecification notNull?
    ;

openGraphReferenceValueType
    : ANY PROPERTY? GRAPH notNull?
    ;

bindingTableReferenceValueType
    : bindingTableType notNull?
    ;

nodeReferenceValueType
    : openNodeReferenceValueType
    | closedNodeReferenceValueType
    ;

closedNodeReferenceValueType
    : nodeTypeSpecification notNull?
    ;

openNodeReferenceValueType
    : ANY? nodeSynonym notNull?
    ;

edgeReferenceValueType
    : openEdgeReferenceValueType
    | closedEdgeReferenceValueType
    ;

closedEdgeReferenceValueType
    : edgeTypeSpecification notNull?
    ;

openEdgeReferenceValueType
    : ANY? edgeSynonym notNull?
    ;

pathValueType
    : PATH notNull?
    ;

listValueTypeName
    : listValueTypeNameSynonym
    ;

listValueTypeNameSynonym
    : LIST
    | ARRAY
    ;

recordType
    : ANY? RECORD notNull?
    | RECORD? fieldTypesSpecification notNull?
    ;

fieldTypesSpecification
    : LEFT_BRACE fieldTypeList? RIGHT_BRACE
    ;

fieldTypeList
    : fieldType (COMMA fieldType)*
    ;

notNull
    :  NOT NULL_KW
    ;

// 18.10 <field type>

fieldType
    : fieldName typed? valueType
    ;

// =============================================================================
// 19 Predicates
// =============================================================================

// 19.1 <search condition>

searchCondition
    : booleanValueExpression
    ;

// 19.2 <predicate>

predicate
    : existsPredicate
    | nullPredicate
    | valueTypePredicate
    | directedPredicate
    | labeledPredicate
    | sourceDestinationPredicate
    | all_differentPredicate
    | samePredicate
    | property_existsPredicate
    ;

// 19.3 <comparison predicate>

// The <comparison predicate> productions moved to valueExpression
// to avoid left mutually recursive productions.

compOp
    : EQUALS_OPERATOR
    | NOT_EQUALS_OPERATOR
    | LEFT_ANGLE_BRACKET
    | RIGHT_ANGLE_BRACKET
    | LESS_THAN_OR_EQUALS_OPERATOR
    | GREATER_THAN_OR_EQUALS_OPERATOR
    ;

// 19.4 <exists predicate>

existsPredicate
    : EXISTS (LEFT_BRACE graphPattern RIGHT_BRACE | LEFT_PAREN graphPattern RIGHT_PAREN | LEFT_BRACE matchStatementBlock RIGHT_BRACE | LEFT_PAREN matchStatementBlock RIGHT_PAREN | nestedQuerySpecification)
    ;

// 19.5 <null predicate>

nullPredicate
    : valueExpressionPrimary nullPredicatePart2
    ;

nullPredicatePart2
    : IS NOT? NULL_KW
    ;

// 19.6 <value type predicate>

valueTypePredicate
    : valueExpressionPrimary valueTypePredicatePart2
    ;

valueTypePredicatePart2
    : IS NOT? typed valueType
    ;

// 19.7 <normalized predicate>

normalizedPredicatePart2
    : IS NOT? normalForm? NORMALIZED
    ;

// 19.8 <directed predicate>

directedPredicate
    : elementVariableReference directedPredicatePart2
    ;

directedPredicatePart2
    : IS NOT? DIRECTED
    ;

// 19.9 <labeled predicate>

labeledPredicate
    : elementVariableReference labeledPredicatePart2
    ;

labeledPredicatePart2
    : isLabeledOrColon labelExpression
    ;

isLabeledOrColon
    : IS NOT? LABELED
    | COLON
    ;

// 19.10 <source/destination predicate>

sourceDestinationPredicate
    : nodeReference sourcePredicatePart2
    | nodeReference destinationPredicatePart2
    ;

nodeReference
    : elementVariableReference
    ;

sourcePredicatePart2
    : IS NOT? SOURCE OF edgeReference
    ;

destinationPredicatePart2
    : IS NOT? DESTINATION OF edgeReference
    ;

edgeReference
    : elementVariableReference
    ;

// 19.11 <all_different predicate>

all_differentPredicate
    : ALL_DIFFERENT LEFT_PAREN elementVariableReference COMMA elementVariableReference (COMMA elementVariableReference)* RIGHT_PAREN
    ;

// 19.12 <same predicate>

samePredicate
    : SAME LEFT_PAREN elementVariableReference COMMA elementVariableReference (COMMA elementVariableReference)* RIGHT_PAREN
    ;

// 19.13 <property_exists predicate>

property_existsPredicate
    : PROPERTY_EXISTS LEFT_PAREN elementVariableReference COMMA propertyName RIGHT_PAREN
    ;

// =============================================================================
// 20 Value expressions and specifications
// =============================================================================

// 20.1 <value expression>

// The specification presents several expression families separately. They are
// intentionally consolidated here to avoid ambiguity caused by several rules
// sharing valueExpressionPrimary. The AST builder restores the semantic node
// distinctions after parsing.

valueExpression
    // Numeric, datetime and duration types all support roughly the same expressions. So here
    // we define a rule that deals with all of them. It is up to the implementation to post
    // process the sytnax tree and flag invalid type and function combinations.
    : sign = (PLUS_SIGN | MINUS_SIGN) valueExpression                       #signedExprAlt
    | valueExpression operator = (ASTERISK | SOLIDUS) valueExpression       #multDivExprAlt
    | valueExpression operator = (PLUS_SIGN | MINUS_SIGN) valueExpression   #addSubtractExprAlt
    // Character strings, byte strings, lists and paths all support the same concatenation
    // operator. So here we define a rule that deals with all of them. Of course the types
    // cannot be combined. So it is up to implementation to post process the sytax tree
    // and flag invalid type and function combinations.
    | valueExpression CONCATENATION_OPERATOR valueExpression                #concatenationExprAlt
    // The comparisonPredicate productions moved here to eliminate left mutual recursion.
    | valueExpression compOp valueExpression                                #comparisonExprAlt //comparisonPredicatePart2
    | predicate                                                             #predicateExprAlt
    // The normalizedPredicate productions moved here to eliminate left mutual recursion.
    | valueExpression normalizedPredicatePart2                              #normalizedPredicateExprAlt
    // Boolean value expression included here.
    | NOT valueExpression                                                   #notExprAlt
    | valueExpression IS NOT? truthValue                                    #isNotExprAlt
    | valueExpression AND valueExpression                                   #conjunctiveExprAlt
    | valueExpression operator = (OR | XOR) valueExpression                 #disjunctiveExprAlt
    | PROPERTY? GRAPH graphExpression                                       #propertyGraphExprAlt
    | BINDING? TABLE bindingTableExpression                                 #bindingTableExprAlt
    | valueFunction                                                         #valueFunctionExprAlt
    | valueExpressionPrimary                                                #primaryExprAlt
    ;

valueFunction
    : numericValueFunction
    | datetimeSubtraction
    | datetimeValueFunction
    | durationValueFunction
    | characterOrByteStringFunction
    | listValueFunction
    | userDefinedFunction
    ;

booleanValueExpression
    : valueExpression
    ;

characterOrByteStringFunction
    : subCharacterOrByteString
    | trimSingleCharacterOrByteString
    | foldCharacterString
    | trimMultiCharacterCharacterString
    | normalizeCharacterString
    ;

subCharacterOrByteString
    : (LEFT | RIGHT) LEFT_PAREN valueExpression COMMA stringLength RIGHT_PAREN
    ;

trimSingleCharacterOrByteString
    : TRIM LEFT_PAREN trimOperands RIGHT_PAREN
    ;

foldCharacterString
    : (UPPER | LOWER) LEFT_PAREN valueExpression RIGHT_PAREN
    ;

trimMultiCharacterCharacterString
    : (BTRIM | LTRIM | RTRIM) LEFT_PAREN valueExpression (COMMA valueExpression)? RIGHT_PAREN
    ;

normalizeCharacterString
    : NORMALIZE LEFT_PAREN valueExpression (COMMA normalForm)? RIGHT_PAREN
    ;

nodeReferenceValueExpression
    : valueExpressionPrimary
    ;

edgeReferenceValueExpression
    : valueExpressionPrimary
    ;

aggregatingValueExpression
    : valueExpression
    ;

// 20.2 <value expression primary>

valueExpressionPrimary
    : parenthesizedValueExpression
    | aggregateFunction
    | unsignedValueSpecification
// List and record literals are redundantly/ambiguously part of the literal production.
//    | listValueConstructor
//    | recordConstructor
    | pathValueConstructor
    | valueExpressionPrimary PERIOD propertyName      // <propertyReference
    | valueQueryExpression
    | caseExpression
    | castSpecification
    | element_idFunction
    | letValueExpression
    | bindingVariableReference
    ;

parenthesizedValueExpression
    : LEFT_PAREN valueExpression RIGHT_PAREN
    ;

nonParenthesizedValueExpressionPrimary
    : nonParenthesizedValueExpressionPrimarySpecialCase
    | bindingVariableReference
    ;

nonParenthesizedValueExpressionPrimarySpecialCase
    : aggregateFunction
    | unsignedValueSpecification
// List and record literals are redundantly/ambiguously part of the literal production.
//    | listValueConstructor
//    | recordConstructor
    | pathValueConstructor
    | valueExpressionPrimary PERIOD propertyName      // <property reference>
    | valueQueryExpression
    | caseExpression
    | castSpecification
    | element_idFunction
    | letValueExpression
    ;

// 20.3 <value specification>

unsignedValueSpecification
    : unsignedLiteral
    | generalValueSpecification
    ;

nonNegativeIntegerSpecification
    : unsignedInteger
    | dynamicParameterSpecification
    ;

generalValueSpecification
    : dynamicParameterSpecification
    | SESSION_USER
    ;

// 20.4 <dynamic parameter specification>

dynamicParameterSpecification
    : GENERAL_PARAMETER_REFERENCE
    ;

// 20.5 <let value expression>

letValueExpression
    : LET letVariableDefinitionList IN valueExpression END
    ;

// 20.6 <value query expression>

valueQueryExpression
    : VALUE nestedQuerySpecification
    ;

// 20.7 <case expression>

caseExpression
    : caseAbbreviation
    | caseSpecification
    ;

caseAbbreviation
    : NULLIF LEFT_PAREN valueExpression COMMA valueExpression RIGHT_PAREN
    | COALESCE LEFT_PAREN valueExpression (COMMA valueExpression)+ RIGHT_PAREN
    ;

caseSpecification
    : simpleCase
    | searchedCase
    ;

simpleCase
    : CASE caseOperand simpleWhenClause+ elseClause? END
    ;

searchedCase
    : CASE searchedWhenClause+ elseClause? END
    ;

simpleWhenClause
    : WHEN whenOperandList THEN result
    ;

searchedWhenClause
    : WHEN searchCondition THEN result
    ;

elseClause
    : ELSE result
    ;

caseOperand
    : nonParenthesizedValueExpressionPrimary
    | elementVariableReference
    ;

whenOperandList
    : whenOperand (COMMA whenOperand)*
    ;

whenOperand
    : nonParenthesizedValueExpressionPrimary
    | compOp valueExpression    //comparisonPredicatePart2
    | nullPredicatePart2
    | valueTypePredicatePart2
    | normalizedPredicatePart2
    | directedPredicatePart2
    | labeledPredicatePart2
    | sourcePredicatePart2
    | destinationPredicatePart2
    ;

result
    : resultExpression
    | nullLiteral
    ;

resultExpression
    : valueExpression
    ;

// 20.8 <cast specification>

castSpecification
    : CAST LEFT_PAREN castOperand AS castTarget RIGHT_PAREN
    ;

castOperand
    : valueExpression
    | nullLiteral
    ;

castTarget
    : valueType
    ;

// 20.9 <aggregate function>

aggregateFunction
    : COUNT LEFT_PAREN ASTERISK RIGHT_PAREN
    | generalSetFunction
    | binarySetFunction
    ;

generalSetFunction
    : generalSetFunctionType LEFT_PAREN setQuantifier? valueExpression RIGHT_PAREN
    ;

binarySetFunction
    : binarySetFunctionType LEFT_PAREN dependentValueExpression COMMA independentValueExpression RIGHT_PAREN
    ;

generalSetFunctionType
    : AVG
    | COUNT
    | MAX
    | MIN
    | SUM
    | COLLECT_LIST
    | STDDEV_SAMP
    | STDDEV_POP
    ;

setQuantifier
    : DISTINCT
    | ALL
    ;

binarySetFunctionType
    : PERCENTILE_CONT
    | PERCENTILE_DISC
    ;

dependentValueExpression
    : setQuantifier? numericValueExpression
    ;

independentValueExpression
    : numericValueExpression
    ;

// 20.10 <element_id function>

element_idFunction
    : ELEMENT_ID LEFT_PAREN elementVariableReference RIGHT_PAREN
    ;

// 20.11 <property reference>

// 20.12 <binding variable reference>

bindingVariableReference
    : bindingVariable
    ;

// 20.13 <path value expression>
//
// The path value expression was combined with list and string value expressions.
// See listStringOrPathValueExpression.

pathValueExpression
    : valueExpression
    ;

// 20.14 <path value constructor>

pathValueConstructor
    : pathValueConstructorByEnumeration
    ;

pathValueConstructorByEnumeration
    : PATH LEFT_BRACKET pathElementList RIGHT_BRACKET
    ;

pathElementList
    : pathElementListStart pathElementListStep*
    ;

pathElementListStart
    : nodeReferenceValueExpression
    ;

pathElementListStep
    : COMMA edgeReferenceValueExpression COMMA nodeReferenceValueExpression
    ;

// 20.15 <list value expression>

// The list value expression was combined with path and string value expressions.
// See listStringOrPathValueExpression.

listValueExpression
    : valueExpression
    ;

// 20.16 <list value function>

// Note: ByteString functions were moved to characterByteStringOrListFunction, some alternatives
// apply to characterString, byteString and list. Breaking them out separately resulted in
// ambiguity.

listValueFunction
    : trimListFunction
    | elementsFunction
    ;

trimListFunction
    : TRIM LEFT_PAREN listValueExpression COMMA numericValueExpression RIGHT_PAREN
    ;

elementsFunction
    : ELEMENTS LEFT_PAREN pathValueExpression RIGHT_PAREN
    ;

// 20.17 <list value constructor>

listValueConstructor
    : listValueConstructorByEnumeration
    ;

listValueConstructorByEnumeration
    : listValueTypeName? LEFT_BRACKET listElementList? RIGHT_BRACKET
    ;

listElementList
    : listElement (COMMA listElement)*
    ;

listElement
    : valueExpression
    ;

// 20.18 <record constructor>

recordConstructor
    : RECORD? fieldsSpecification
    ;

fieldsSpecification
    : LEFT_BRACE fieldList? RIGHT_BRACE
    ;

fieldList
    : field (COMMA field)*
    ;

// 20.19 <field>

field
    : fieldName COLON valueExpression
    ;

// 20.20 <boolean value expression>

// Most of <boolean value expression> is incorporated in valueExpression

truthValue
    : BOOLEAN_LITERAL
    ;

// 20.21 <numeric value expression>

numericValueExpression
    : sign = (PLUS_SIGN | MINUS_SIGN) numericValueExpression
    | numericValueExpression operator = (ASTERISK | SOLIDUS) numericValueExpression
    | numericValueExpression operator = (PLUS_SIGN | MINUS_SIGN) numericValueExpression
    | valueExpressionPrimary
    | numericValueFunction
    ;

// 20.22 <numeric value function>

numericValueFunction
    : lengthExpression
    | cardinalityExpression
    | absoluteValueExpression
    | modulusExpression
    | trigonometricFunction
    | generalLogarithmFunction
    | commonLogarithm
    | naturalLogarithm
    | exponentialFunction
    | powerFunction
    | squareRoot
    | floorFunction
    | ceilingFunction
    ;

lengthExpression
    : charLengthExpression
    | byteLengthExpression
    | pathLengthExpression
    ;

cardinalityExpression
    : CARDINALITY LEFT_PAREN cardinalityExpressionArgument RIGHT_PAREN
    | SIZE LEFT_PAREN listValueExpression RIGHT_PAREN
    ;

cardinalityExpressionArgument
    : valueExpression
    ;

charLengthExpression
    : (CHAR_LENGTH | CHARACTER_LENGTH) LEFT_PAREN characterStringValueExpression RIGHT_PAREN
    ;

byteLengthExpression
    : (BYTE_LENGTH | OCTET_LENGTH) LEFT_PAREN byteStringValueExpression RIGHT_PAREN
    ;

pathLengthExpression
    : PATH_LENGTH LEFT_PAREN pathValueExpression RIGHT_PAREN
    ;

// absoluteValueExpression applies to both numeric types and duration types. They have the same syntax.
absoluteValueExpression
    : ABS LEFT_PAREN valueExpression RIGHT_PAREN
    ;

modulusExpression
    : MOD LEFT_PAREN numericValueExpressionDividend COMMA numericValueExpressionDivisor RIGHT_PAREN
    ;

numericValueExpressionDividend
    : numericValueExpression
    ;

numericValueExpressionDivisor
    : numericValueExpression
    ;

trigonometricFunction
    : trigonometricFunctionName LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

trigonometricFunctionName
    : SIN
    | COS
    | TAN
    | COT
    | SINH
    | COSH
    | TANH
    | ASIN
    | ACOS
    | ATAN
    | DEGREES
    | RADIANS
    ;

generalLogarithmFunction
    : LOG_KW LEFT_PAREN generalLogarithmBase COMMA generalLogarithmArgument RIGHT_PAREN
    ;

generalLogarithmBase
    : numericValueExpression
    ;

generalLogarithmArgument
    : numericValueExpression
    ;

commonLogarithm
    : LOG10 LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

naturalLogarithm
    : LN LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

exponentialFunction
    : EXP LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

powerFunction
    : POWER LEFT_PAREN numericValueExpressionBase COMMA numericValueExpressionExponent RIGHT_PAREN
    ;

numericValueExpressionBase
    : numericValueExpression
    ;

numericValueExpressionExponent
    : numericValueExpression
    ;

squareRoot
    : SQRT LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

floorFunction
    : FLOOR LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

ceilingFunction
    : (CEIL | CEILING) LEFT_PAREN numericValueExpression RIGHT_PAREN
    ;

// 20.23 <string value expression>

// The string value expressions were combined with list and path value expressions.

characterStringValueExpression
    : valueExpression
    ;

byteStringValueExpression
    : valueExpression
    ;

// 20.24 <character string function>

// Note: String functions were moved to characterByteStringOrListFunction, some alternatives
// apply to characterString, byteString and list. Breaking them out separately resulted in
// ambiguity.

trimOperands
    : (trimSpecification? trimCharacterOrByteString? FROM)? trimCharacterOrByteStringSource
    ;

trimCharacterOrByteStringSource
    : valueExpression
    ;

trimSpecification
    : LEADING
    | TRAILING
    | BOTH
    ;

trimCharacterOrByteString
    : valueExpression
    ;

normalForm
    : NFC
    | NFD
    | NFKC
    | NFKD
    ;

stringLength
    : numericValueExpression
    ;

// 20.25 <byte string function>

// Note: ByteString functions were moved to characterByteStringOrListFunction, some alternatives
// apply to characterString, byteString and list. Breaking them out separately resulted in
// ambiguity.

// 20.26 <datetime value expression>

// The implementation should enforce that the data type is a datetime value.
datetimeValueExpression
     : valueExpression
     ;

// 20.27 <datetime value function>

datetimeValueFunction
    : dateFunction
    | timeFunction
    | datetimeFunction
    | localtimeFunction
    | localdatetimeFunction
    ;

dateFunction
    : CURRENT_DATE
    | DATE LEFT_PAREN dateFunctionParameters? RIGHT_PAREN
    ;

timeFunction
    : CURRENT_TIME
    | ZONED_TIME LEFT_PAREN timeFunctionParameters? RIGHT_PAREN
    ;

localtimeFunction
    : LOCAL_TIME (LEFT_PAREN timeFunctionParameters? RIGHT_PAREN)?
    ;

datetimeFunction
    : CURRENT_TIMESTAMP
    | ZONED_DATETIME LEFT_PAREN datetimeFunctionParameters? RIGHT_PAREN
    ;

localdatetimeFunction
    : LOCAL_TIMESTAMP
    | LOCAL_DATETIME LEFT_PAREN datetimeFunctionParameters? RIGHT_PAREN
    ;

dateFunctionParameters
    : dateString
    | recordConstructor
    ;

timeFunctionParameters
    : timeString
    | recordConstructor
    ;

datetimeFunctionParameters
    : datetimeString
    | recordConstructor
    ;

// 20.28 <duration value expression>

// The implementation should enforce that the data type is a duration value.
durationValueExpression
    : valueExpression
    ;

datetimeSubtraction
    : DURATION_BETWEEN LEFT_PAREN datetimeSubtractionParameters RIGHT_PAREN temporalDurationQualifier?
    ;

datetimeSubtractionParameters
    : datetimeValueExpression1 COMMA datetimeValueExpression2
    ;

datetimeValueExpression1
    : datetimeValueExpression
    ;

datetimeValueExpression2
    : datetimeValueExpression
    ;

// 20.29 <duration value function>

durationValueFunction
    : durationFunction
    | absoluteValueExpression
    ;

durationFunction
    : DURATION LEFT_PAREN durationFunctionParameters RIGHT_PAREN
    ;

durationFunctionParameters
    : durationString
    | recordConstructor
    ;

// =============================================================================
// 21 Lexical elements
// =============================================================================
//
// The first lexical-element productions are parser rules because they combine
// tokens into names, variables, and literals used by the syntax above.

// 21.1 Names and variables

objectName
    : identifier
    ;

objectNameOrBindingVariable
    : regularIdentifier
    ;

directoryName
    : identifier
    ;

schemaName
    : identifier
    ;

graphName
    : regularIdentifier
    | delimitedGraphName
    ;

delimitedGraphName
    // DELIMITED_IDENTIFIER
    : DOUBLE_QUOTED_CHARACTER_SEQUENCE
    | ACCENT_QUOTED_CHARACTER_SEQUENCE
    ;

graphTypeName
    : identifier
    ;

nodeTypeName
    : identifier
    ;

edgeTypeName
    : identifier
    ;

bindingTableName
    : regularIdentifier
    | delimitedBindingTableName
    ;

delimitedBindingTableName
     // DELIMITED_IDENTIFIER
     : DOUBLE_QUOTED_CHARACTER_SEQUENCE
     | ACCENT_QUOTED_CHARACTER_SEQUENCE
     ;

procedureName
    : identifier
    ;

labelName
    : identifier
    ;

propertyName
    : identifier
    ;

fieldName
    : identifier
    ;

elementVariable
    : bindingVariable
    ;

pathVariable
    : bindingVariable
    ;

subpathVariable
    : regularIdentifier
    ;

bindingVariable
    : regularIdentifier
    ;

// 21.2 <literal>

unsignedLiteral
    : unsignedNumericLiteral
    | generalLiteral
    ;

generalLiteral
    : BOOLEAN_LITERAL
    | characterStringLiteral
    | BYTE_STRING_LITERAL
    | temporalLiteral
    | durationLiteral
    | nullLiteral
    | listLiteral
    | recordLiteral
    ;

temporalLiteral
    : dateLiteral
    | timeLiteral
    | datetimeLiteral
//    | sqlDatetimeLiteral
    ;

dateLiteral
    : DATE dateString
    ;

timeLiteral
    : TIME timeString
    ;

datetimeLiteral
    : (DATETIME | TIMESTAMP) datetimeString
    ;

listLiteral
    : listValueConstructorByEnumeration
    ;

recordLiteral
    : recordConstructor
    ;

identifier
    : regularIdentifier
    // DELIMITED_IDENTIFIER
    | DOUBLE_QUOTED_CHARACTER_SEQUENCE
    | ACCENT_QUOTED_CHARACTER_SEQUENCE
    ;

regularIdentifier
    : REGULAR_IDENTIFIER
    | nonReservedWords
    ;

timeZoneString
    : characterStringLiteral
    ;

characterStringLiteral
    : SINGLE_QUOTED_CHARACTER_SEQUENCE
    | DOUBLE_QUOTED_CHARACTER_SEQUENCE
    ;

unsignedNumericLiteral
    : exactNumericLiteral
    | approximateNumericLiteral
    ;

exactNumericLiteral
    : UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_EXACT_NUMBER_SUFFIX
    | UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_EXACT_NUMBER_SUFFIX
    | UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITHOUT_SUFFIX
    | UNSIGNED_DECIMAL_INTEGER_WITH_EXACT_NUMBER_SUFFIX
    | unsignedInteger
    ;

approximateNumericLiteral
    : UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX
    | UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITHOUT_SUFFIX
    | UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX
    | UNSIGNED_DECIMAL_INTEGER_WITH_APPROXIMATE_NUMBER_SUFFIX
    ;

unsignedInteger
    : UNSIGNED_DECIMAL_INTEGER
    | UNSIGNED_HEXADECIMAL_INTEGER
    | UNSIGNED_OCTAL_INTEGER
    | UNSIGNED_BINARY_INTEGER
    ;

unsignedDecimalInteger
    : UNSIGNED_DECIMAL_INTEGER
    ;

nullLiteral
    : NULL_KW
    ;

dateString
    : characterStringLiteral
    ;

timeString
    : characterStringLiteral
    ;

datetimeString
    : characterStringLiteral
    ;

durationLiteral
    : DURATION durationString
//    | sqlIntervalLiteral
    ;

durationString
    : characterStringLiteral
    ;

nodeSynonym
    : NODE
    | VERTEX
    ;

edgesSynonym
    : EDGES
    | RELATIONSHIPS
    ;

edgeSynonym
    : EDGE
    | RELATIONSHIP
    ;

// -----------------------------------------------------------------------------
// Lexer rules
// -----------------------------------------------------------------------------
//
// Lexer rules start here. Rule order matters: ANTLR chooses the longest match,
// then the first rule when matches have equal length. Keep keyword tokens above
// REGULAR_IDENTIFIER so reserved words are not emitted as identifiers.
//
// Keywords are case-insensitive through the A..Z fragments below. This grammar
// targets the ANTLR 4.8 Rust generator, which does not support the newer
// caseInsensitive grammar option.

// 21.1 Names and variables
//
// Lexer continuation of the names-and-variables subclause.

IMPLIES
    : RIGHT_DOUBLE_ARROW
    | I M P L I E S
    ;

fragment PARAMETER_NAME
    : SEPARATED_IDENTIFIER
    ;

// NTUR-DB extension: user-defined functions may use an object-name prefix.
userDefinedFunction
    : (objectName PERIOD)* identifier LEFT_PAREN (valueExpression (COMMA valueExpression)*)? RIGHT_PAREN
    ;

// 21.2 <literal>

nonReservedWords
    : ACYCLIC
    | BINDING
    | BINDINGS
    | CONNECTING
    | DESTINATION
    | DIFFERENT
    | DIRECTED
    | EDGE
    | EDGES
    | ELEMENT
    | ELEMENTS
    | FIRST
    | GRAPH
    | GROUPS
    | KEEP
    | LABEL
    | LABELED
    | LABELS
    | LAST
    | NFC
    | NFD
    | NFKC
    | NFKD
    | NO
    | NODE
    | NORMALIZED
    | ONLY
    | ORDINALITY
    | PROPERTY
    | READ
    | RELATIONSHIP
    | RELATIONSHIPS
    | REPEATABLE
    | SHORTEST
    | SIMPLE
    | SOURCE
    | TABLE
    | TO
    | TRAIL
    | TRANSACTION
    | TYPE
    | UNDIRECTED
    | VERTEX
    | WALK
    | WITHOUT
    | WRITE
    | ZONE
    ;

BOOLEAN_LITERAL
    : T R U E
    | F A L S E
    | U N K N O W N
    ;

SINGLE_QUOTED_CHARACTER_SEQUENCE
    : NO_ESCAPE? UNBROKEN_SINGLE_QUOTED_CHARACTER_SEQUENCE
    ;

DOUBLE_QUOTED_CHARACTER_SEQUENCE
    : NO_ESCAPE? UNBROKEN_DOUBLE_QUOTED_CHARACTER_SEQUENCE
    ;

ACCENT_QUOTED_CHARACTER_SEQUENCE
    :NO_ESCAPE? UNBROKEN_ACCENT_QUOTED_CHARACTER_SEQUENCE
    ;

NO_ESCAPE
    : COMMERCIAL_AT
    ;

fragment UNBROKEN_SINGLE_QUOTED_CHARACTER_SEQUENCE
    : QUOTE SINGLE_QUOTED_CHARACTER_REPRESENTATION* QUOTE
    ;

fragment UNBROKEN_DOUBLE_QUOTED_CHARACTER_SEQUENCE
    : DOUBLE_QUOTE DOUBLE_QUOTED_CHARACTER_REPRESENTATION* DOUBLE_QUOTE
    ;

fragment UNBROKEN_ACCENT_QUOTED_CHARACTER_SEQUENCE
    : GRAVE_ACCENT ACCENT_QUOTED_CHARACTER_REPRESENTATION* GRAVE_ACCENT
    ;

fragment SINGLE_QUOTED_CHARACTER_REPRESENTATION:
	(ESCAPED_CHARACTER | ~['\\\r\n] | '\'\'')+
	;

fragment DOUBLE_QUOTED_CHARACTER_REPRESENTATION:
	(ESCAPED_CHARACTER | ~["\\\r\n] | '""')+
	;

fragment ACCENT_QUOTED_CHARACTER_REPRESENTATION:
	(ESCAPED_CHARACTER | ~[`\\\r\n] | '``')+
	;

fragment ESCAPED_CHARACTER
    : ESCAPED_REVERSE_SOLIDUS
	| ESCAPED_QUOTE
	| ESCAPED_DOUBLE_QUOTE
	| ESCAPED_GRAVE_ACCENT
	| ESCAPED_TAB
	| ESCAPED_BACKSPACE
	| ESCAPED_NEW_LINE
	| ESCAPED_CARRIAGE_RETURN
	| ESCAPED_FORM_FEED
	| ESCAPED_UNICODE4_DIGIT_VALUE
	| ESCAPED_UNICODE6_DIGIT_VALUE
	;

fragment ESCAPED_REVERSE_SOLIDUS: REVERSE_SOLIDUS REVERSE_SOLIDUS;
fragment ESCAPED_QUOTE: REVERSE_SOLIDUS QUOTE;
fragment ESCAPED_DOUBLE_QUOTE: REVERSE_SOLIDUS DOUBLE_QUOTE;
fragment ESCAPED_GRAVE_ACCENT: REVERSE_SOLIDUS GRAVE_ACCENT;
fragment ESCAPED_TAB: REVERSE_SOLIDUS 't';
fragment ESCAPED_BACKSPACE: REVERSE_SOLIDUS 'b';
fragment ESCAPED_NEW_LINE: REVERSE_SOLIDUS 'n';
fragment ESCAPED_CARRIAGE_RETURN: REVERSE_SOLIDUS 'r';
fragment ESCAPED_FORM_FEED: REVERSE_SOLIDUS 'f';
fragment ESCAPED_UNICODE4_DIGIT_VALUE:
	START_UNICODE4 HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT;
fragment ESCAPED_UNICODE6_DIGIT_VALUE:
	START_UNICODE6 HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT HEX_DIGIT;
fragment START_UNICODE4: REVERSE_SOLIDUS 'u';
fragment START_UNICODE6: REVERSE_SOLIDUS 'U';

BYTE_STRING_LITERAL
    : 'X' QUOTE SPACE* (HEX_DIGIT SPACE* HEX_DIGIT SPACE*)* QUOTE
    ;

UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_EXACT_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION EXACT_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITHOUT_SUFFIX
    : UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION
    ;

UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION APPROXIMATE_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_EXACT_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_IN_COMMON_NOTATION EXACT_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITHOUT_SUFFIX
    : UNSIGNED_DECIMAL_IN_COMMON_NOTATION
    ;

UNSIGNED_DECIMAL_IN_COMMON_NOTATION_WITH_APPROXIMATE_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_IN_COMMON_NOTATION APPROXIMATE_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_INTEGER_WITH_EXACT_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_INTEGER EXACT_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_INTEGER_WITH_APPROXIMATE_NUMBER_SUFFIX
    : UNSIGNED_DECIMAL_INTEGER APPROXIMATE_NUMBER_SUFFIX
    ;

UNSIGNED_DECIMAL_INTEGER
    : DIGIT (UNDERSCORE? DIGIT)*
    ;

fragment EXACT_NUMBER_SUFFIX
    : 'M'
    ;

fragment UNSIGNED_DECIMAL_IN_SCIENTIFIC_NOTATION
    : MANTISSA E EXPONENT
    ;

fragment MANTISSA
    : UNSIGNED_DECIMAL_IN_COMMON_NOTATION
    | UNSIGNED_DECIMAL_INTEGER
    ;

fragment EXPONENT
    : SIGNED_DECIMAL_INTEGER
    ;

fragment UNSIGNED_DECIMAL_IN_COMMON_NOTATION
    : UNSIGNED_DECIMAL_INTEGER (PERIOD UNSIGNED_DECIMAL_INTEGER?)
    | PERIOD UNSIGNED_DECIMAL_INTEGER
    ;

fragment SIGNED_DECIMAL_INTEGER
    : (PLUS_SIGN | MINUS_SIGN)? UNSIGNED_DECIMAL_INTEGER
    ;

UNSIGNED_HEXADECIMAL_INTEGER
    : START_HEX ('_'? HEX_DIGIT)+
    ;
fragment START_HEX: '0x';

UNSIGNED_OCTAL_INTEGER
    : START_OCTAL ('_'? OCTAL_DIGIT)+
    ;
fragment START_OCTAL: '0o';

UNSIGNED_BINARY_INTEGER
    : START_BIN ('_'? BINARY_DIGIT)+
    ;
fragment START_BIN: '0b';

fragment APPROXIMATE_NUMBER_SUFFIX
    : F
    | D
    ;

// 21.3 <token>, <separator>, and <identifier>

// Case-insensitive single-letter fragments (ANTLR 4.8 compatible; no 'caseInsensitive' option support)
fragment A : [aA] ;
fragment B : [bB] ;
fragment C : [cC] ;
fragment D : [dD] ;
fragment E : [eE] ;
fragment F : [fF] ;
fragment G : [gG] ;
fragment H : [hH] ;
fragment I : [iI] ;
fragment J : [jJ] ;
fragment K : [kK] ;
fragment L : [lL] ;
fragment M : [mM] ;
fragment N : [nN] ;
fragment O : [oO] ;
fragment P : [pP] ;
fragment Q : [qQ] ;
fragment R : [rR] ;
fragment S : [sS] ;
fragment T : [tT] ;
fragment U : [uU] ;
fragment V : [vV] ;
fragment W : [wW] ;
fragment X : [xX] ;
fragment Y : [yY] ;
fragment Z : [zZ] ;

// Reserved words. These tokens cannot be consumed as regular identifiers
// unless a parser rule explicitly permits them.
ABS: A B S;
ACOS: A C O S;
ALL: A L L;
ALL_DIFFERENT: A L L '_' D I F F E R E N T;
AND: A N D;
ANY: A N Y;
ARRAY: A R R A Y;
AS: A S;
ASC: A S C;
ASCENDING: A S C E N D I N G;
ASIN: A S I N;
AT: A T;
ATAN: A T A N;
AVG: A V G;
BIG: B I G;
BIGINT: B I G I N T;
BINARY: B I N A R Y;
BOOL: B O O L;
BOOLEAN: B O O L E A N;
BOTH: B O T H;
BTRIM: B T R I M;
BY: B Y;
BYTE_LENGTH: B Y T E '_' L E N G T H;
BYTES: B Y T E S;
CALL: C A L L;
CARDINALITY: C A R D I N A L I T Y;
CASE: C A S E;
CAST: C A S T;
CEIL: C E I L;
CEILING: C E I L I N G;
CHAR: C H A R;
CHAR_LENGTH: C H A R '_' L E N G T H;
CHARACTER_LENGTH: C H A R A C T E R '_' L E N G T H;
CHARACTERISTICS: C H A R A C T E R I S T I C S;
CLOSE: C L O S E;
COALESCE: C O A L E S C E;
COLLECT_LIST: C O L L E C T '_' L I S T;
COMMIT: C O M M I T;
COPY: C O P Y;
COS: C O S;
COSH: C O S H;
COT: C O T;
COUNT: C O U N T;
CREATE: C R E A T E;
CURRENT_DATE: C U R R E N T '_' D A T E;
CURRENT_GRAPH: C U R R E N T '_' G R A P H;
CURRENT_PROPERTY_GRAPH: C U R R E N T '_' P R O P E R T Y '_' G R A P H;
CURRENT_SCHEMA: C U R R E N T '_' S C H E M A;
CURRENT_TIME: C U R R E N T '_' T I M E;
CURRENT_TIMESTAMP: C U R R E N T '_' T I M E S T A M P;
DATE: D A T E;
DATETIME: D A T E T I M E;
DAY: D A Y;
DEC: D E C;
DECIMAL: D E C I M A L;
DEGREES: D E G R E E S;
DELETE: D E L E T E;
DESC: D E S C;
DESCENDING: D E S C E N D I N G;
DETACH: D E T A C H;
DISTINCT: D I S T I N C T;
DOUBLE: D O U B L E;
DROP: D R O P;
DURATION: D U R A T I O N;
DURATION_BETWEEN: D U R A T I O N '_' B E T W E E N;
ELEMENT_ID: E L E M E N T '_' I D;
ELSE: E L S E;
END: E N D;
EXCEPT: E X C E P T;
EXISTS: E X I S T S;
EXP: E X P;
FILTER: F I L T E R;
FINISH: F I N I S H;
FLOAT: F L O A T;
FLOAT16: F L O A T '1' '6';
FLOAT32: F L O A T '3' '2';
FLOAT64: F L O A T '6' '4';
FLOAT128: F L O A T '1' '2' '8';
FLOAT256: F L O A T '2' '5' '6';
FLOOR: F L O O R;
FOR: F O R;
FROM: F R O M;
GROUP: G R O U P;
HAVING: H A V I N G;
HOME_GRAPH: H O M E '_' G R A P H;
HOME_PROPERTY_GRAPH: H O M E '_' P R O P E R T Y '_' G R A P H;
HOME_SCHEMA: H O M E '_' S C H E M A;
HOUR: H O U R;
IF: I F;
IN: I N;
INSERT: I N S E R T;
INT: I N T;
INTEGER: I N T E G E R;
INT8: I N T '8';
INTEGER8: I N T E G E R '8';
INT16: I N T '1' '6';
INTEGER16: I N T E G E R '1' '6';
INT32: I N T '3' '2';
INTEGER32: I N T E G E R '3' '2';
INT64: I N T '6' '4';
INTEGER64: I N T E G E R '6' '4';
INT128: I N T '1' '2' '8';
INTEGER128: I N T E G E R '1' '2' '8';
INT256: I N T '2' '5' '6';
INTEGER256: I N T E G E R '2' '5' '6';
INTERSECT: I N T E R S E C T;
INTERVAL: I N T E R V A L;
IS: I S;
LEADING: L E A D I N G;
LEFT: L E F T;
LET: L E T;
LIKE: L I K E;
LIMIT: L I M I T;
LIST: L I S T;
LN: L N;
LOCAL: L O C A L;
LOCAL_DATETIME: L O C A L '_' D A T E T I M E;
LOCAL_TIME: L O C A L '_' T I M E;
LOCAL_TIMESTAMP: L O C A L '_' T I M E S T A M P;
LOG_KW: L O G;
LOG10: L O G '1' '0';
LOWER: L O W E R;
LTRIM: L T R I M;
MATCH: M A T C H;
MAX: M A X;
MIN: M I N;
MINUTE: M I N U T E;
MOD: M O D;
MONTH: M O N T H;
NEXT: N E X T;
NODETACH: N O D E T A C H;
NORMALIZE: N O R M A L I Z E;
NOT: N O T;
NOTHING: N O T H I N G;
NULL_KW: N U L L;            // NULL is a commonly used macro in C++.
NULLS: N U L L S;
NULLIF: N U L L I F;
OCTET_LENGTH: O C T E T '_' L E N G T H;
OF: O F;
OFFSET: O F F S E T;
OPTIONAL: O P T I O N A L;
OR: O R;
ORDER: O R D E R;
OTHERWISE: O T H E R W I S E;
PARAMETER: P A R A M E T E R;
PARAMETERS: P A R A M E T E R S;
PATH: P A T H;
PATH_LENGTH: P A T H '_' L E N G T H;
PATHS: P A T H S;
PERCENTILE_CONT: P E R C E N T I L E '_' C O N T;
PERCENTILE_DISC: P E R C E N T I L E '_' D I S C;
POWER: P O W E R;
PRECISION: P R E C I S I O N;
PROPERTY_EXISTS: P R O P E R T Y '_' E X I S T S;
RADIANS: R A D I A N S;
REAL: R E A L;
RECORD: R E C O R D;
REMOVE: R E M O V E;
REPLACE: R E P L A C E;
RESET: R E S E T;
RETURN: R E T U R N;
RIGHT: R I G H T;
ROLLBACK: R O L L B A C K;
RTRIM: R T R I M;
SAME: S A M E;
SCHEMA: S C H E M A;
SECOND: S E C O N D;
SELECT: S E L E C T;
SESSION: S E S S I O N;
SESSION_USER: S E S S I O N '_' U S E R;
SET: S E T;
SIGNED: S I G N E D;
SIN: S I N;
SINH: S I N H;
SIZE: S I Z E;
SKIP_RESERVED_WORD: S K I P;
SMALL: S M A L L;
SMALLINT: S M A L L I N T;
SQRT: S Q R T;
START: S T A R T;
STDDEV_POP: S T D D E V '_' P O P;
STDDEV_SAMP: S T D D E V '_' S A M P;
STRING: S T R I N G;
SUM: S U M;
TAN: T A N;
TANH: T A N H;
THEN: T H E N;
TIME: T I M E;
TIMESTAMP: T I M E S T A M P;
TRAILING: T R A I L I N G;
TRIM: T R I M;
TYPED: T Y P E D;
UBIGINT: U B I G I N T;
UINT: U I N T;
UINT8: U I N T '8';
UINT16: U I N T '1' '6';
UINT32: U I N T '3' '2';
UINT64: U I N T '6' '4';
UINT128: U I N T '1' '2' '8';
UINT256: U I N T '2' '5' '6';
UNION: U N I O N;
UNSIGNED: U N S I G N E D;
UPPER: U P P E R;
USE: U S E;
USMALLINT: U S M A L L I N T;
VALUE: V A L U E;
VARBINARY: V A R B I N A R Y;
VARCHAR: V A R C H A R;
VARIABLE: V A R I A B L E;
WHEN: W H E N;
WHERE: W H E R E;
WITH: W I T H;
XOR: X O R;
YEAR: Y E A R;
YIELD: Y I E L D;
ZONED: Z O N E D;
ZONED_DATETIME: Z O N E D '_' D A T E T I M E;
ZONED_TIME: Z O N E D '_' T I M E;

// Pre-reserved words. They are tokenized separately to preserve the categories
// defined by the language specification.
ABSTRACT: A B S T R A C T;
AGGREGATE: A G G R E G A T E;
AGGREGATES: A G G R E G A T E S;
ALTER: A L T E R;
CATALOG: C A T A L O G;
CLEAR: C L E A R;
CLONE: C L O N E;
CONSTRAINT: C O N S T R A I N T;
CURRENT_ROLE: C U R R E N T '_' R O L E;
CURRENT_USER: C U R R E N T '_' U S E R;
DATA: D A T A;
DIRECTORY: D I R E C T O R Y;
DRYRUN: D R Y R U N;
EXACT: E X A C T;
EXISTING: E X I S T I N G;
FUNCTION: F U N C T I O N;
GQLSTATUS: G Q L S T A T U S;
GRANT: G R A N T;
INSTANT: I N S T A N T;
INFINITY_KW: I N F I N I T Y;            // INFINITY is a commonly used macro in C++
NUMBER: N U M B E R;
NUMERIC: N U M E R I C;
ON: O N;
OPEN: O P E N;
PARTITION: P A R T I T I O N;
PROCEDURE: P R O C E D U R E;
PRODUCT: P R O D U C T;
PROJECT: P R O J E C T;
QUERY: Q U E R Y;
RECORDS: R E C O R D S;
REFERENCE: R E F E R E N C E;
RENAME: R E N A M E;
REVOKE: R E V O K E;
SUBSTRING: S U B S T R I N G;
SYSTEM_USER: S Y S T E M '_' U S E R;
TEMPORAL: T E M P O R A L;
UNIQUE: U N I Q U E;
UNIT: U N I T;
VALUES: V A L U E S;

// Non-reserved words. Parser rules may accept these tokens where identifiers
// are allowed.
ACYCLIC: A C Y C L I C;
BINDING: B I N D I N G;
BINDINGS: B I N D I N G S;
CONNECTING: C O N N E C T I N G;
DESTINATION: D E S T I N A T I O N;
DIFFERENT: D I F F E R E N T;
DIRECTED: D I R E C T E D;
EDGE: E D G E;
EDGES: E D G E S;
ELEMENT: E L E M E N T;
ELEMENTS: E L E M E N T S;
FIRST: F I R S T;
GRAPH: G R A P H;
GROUPS: G R O U P S;
KEEP: K E E P;
LABEL: L A B E L;
LABELED: L A B E L E D;
LABELS: L A B E L S;
LAST: L A S T;
NFC: N F C;
NFD: N F D;
NFKC: N F K C;
NFKD: N F K D;
NO: N O;
NODE: N O D E;
NORMALIZED: N O R M A L I Z E D;
ONLY: O N L Y;
ORDINALITY: O R D I N A L I T Y;
PROPERTY: P R O P E R T Y;
READ: R E A D;
RELATIONSHIP: R E L A T I O N S H I P;
RELATIONSHIPS: R E L A T I O N S H I P S;
REPEATABLE: R E P E A T A B L E;
SHORTEST: S H O R T E S T;
SIMPLE: S I M P L E;
SOURCE: S O U R C E;
TABLE: T A B L E;
TO: T O;
TRAIL: T R A I L;
TRANSACTION: T R A N S A C T I O N;
TYPE: T Y P E;
UNDIRECTED: U N D I R E C T E D;
VERTEX: V E R T E X;
WALK: W A L K;
WITHOUT: W I T H O U T;
WRITE: W R I T E;
ZONE: Z O N E;

fragment SEPARATED_IDENTIFIER
    : DELIMITED_IDENTIFIER
    | EXTENDED_IDENTIFIER
    ;

REGULAR_IDENTIFIER
    : IDENTIFIER_START IDENTIFIER_EXTEND*
    ;

fragment EXTENDED_IDENTIFIER
    : IDENTIFIER_EXTEND+
    ;

fragment DELIMITED_IDENTIFIER
    : DOUBLE_QUOTED_CHARACTER_SEQUENCE
    | ACCENT_QUOTED_CHARACTER_SEQUENCE
    ;

SUBSTITUTED_PARAMETER_REFERENCE
    : DOUBLE_DOLLAR_SIGN PARAMETER_NAME
    ;

GENERAL_PARAMETER_REFERENCE
    : DOLLAR_SIGN PARAMETER_NAME
    ;

fragment IDENTIFIER_START
    : ID_Start
    | Pc
    ;

fragment IDENTIFIER_EXTEND
    : ID_Continue
    ;

fragment ID_Start
    : [\p{ID_Start}]
    ;

fragment ID_Continue
    : [\p{ID_Continue}]
    ;

MULTISET_ALTERNATION_OPERATOR: '|+|';

BRACKET_RIGHT_ARROW: ']->';
BRACKET_TILDE_RIGHT_ARROW: ']~>';
CONCATENATION_OPERATOR: '||';
DOUBLE_COLON: '::';
DOUBLE_DOLLAR_SIGN: '$$';
DOUBLE_PERIOD: '..';
GREATER_THAN_OR_EQUALS_OPERATOR: '>=';
LEFT_ARROW: '<-';
LEFT_ARROW_TILDE: '<~';
LEFT_ARROW_BRACKET: '<-[';
LEFT_ARROW_TILDE_BRACKET: '<~[';
LEFT_MINUS_RIGHT: '<->';
LEFT_MINUS_SLASH: '<-/';
LEFT_TILDE_SLASH: '<~/';
LESS_THAN_OR_EQUALS_OPERATOR: '<=';
MINUS_LEFT_BRACKET: '-[';
MINUS_SLASH: '-/';
NOT_EQUALS_OPERATOR: '<>';
RIGHT_ARROW: '->';
RIGHT_BRACKET_MINUS: ']-';
RIGHT_BRACKET_TILDE: ']~';
RIGHT_DOUBLE_ARROW: '=>';
SLASH_MINUS: '/-';
SLASH_MINUS_RIGHT: '/->';
SLASH_TILDE: '/~';
SLASH_TILDE_RIGHT: '/~>';
TILDE_LEFT_BRACKET: '~[';
TILDE_RIGHT_ARROW: '~>';
TILDE_SLASH: '~/';

// 21.4 <GQL terminal character>

AMPERSAND: '&';
ASTERISK: '*';
COLON: ':';
COMMA: ',';
COMMERCIAL_AT: '@';
DOLLAR_SIGN: '$';
DOUBLE_QUOTE: '"';
EQUALS_OPERATOR: '=';
EXCLAMATION_MARK: '!';
RIGHT_ANGLE_BRACKET: '>';
GRAVE_ACCENT: '`';
LEFT_BRACE: '{';
LEFT_BRACKET: '[';
LEFT_PAREN: '(';
LEFT_ANGLE_BRACKET: '<';
MINUS_SIGN: '-';
PERCENT: '%';
PERIOD: '.';
PLUS_SIGN: '+';
QUESTION_MARK: '?';
QUOTE: '\'';
REVERSE_SOLIDUS: '\\';
RIGHT_BRACE: '}';
RIGHT_BRACKET: ']';
RIGHT_PAREN: ')';
SOLIDUS: '/';
TILDE: '~';
UNDERSCORE: '_';
VERTICAL_BAR: '|';

fragment HEX_DIGIT
    : [0-9a-fA-F]
    ;

fragment DIGIT
    : [0-9]
    ;

fragment OCTAL_DIGIT
    : [0-7]
    ;

fragment BINARY_DIGIT
    : [0-1]
    ;

// Whitespace and comments are retained on ANTLR's hidden channel. They do not
// participate in parser decisions, but remain available to tools that inspect
// the token stream.
SP
  : (WHITESPACE)+
  -> channel(HIDDEN)
  ;

WHITESPACE
    : SPACE
    | TAB
    | LF
    | VT
    | FF
    | CR
    | FS
    | GS
    | RS
    | US
    | '\u1680'
    | '\u180e'
    | '\u2000'
    | '\u2001'
    | '\u2002'
    | '\u2003'
    | '\u2004'
    | '\u2005'
    | '\u2006'
    | '\u2008'
    | '\u2009'
    | '\u200a'
    | '\u2028'
    | '\u2029'
    | '\u205f'
    | '\u3000'
    | '\u00a0'
    | '\u2007'
    | '\u202f'
    ;

BRACKETED_COMMENT: '/*' .*? '*/' -> channel(HIDDEN);

SIMPLE_COMMENT_SOLIDUS: '//' ~[\r\n]* -> channel(HIDDEN);

SIMPLE_COMMENT_MINUS: '--' ~[\r\n]* -> channel(HIDDEN);

fragment GS : [\u001D];

fragment FS : [\u001C];

fragment CR : [\r];

fragment Sc : [\p{Sc}];

fragment SPACE : [ ];

fragment Pc : [\p{Pc}];

fragment TAB : [\t];

fragment LF : [\n];

fragment VT : [\u000B];

fragment US : [\u001F];

fragment FF: [\f];

fragment RS: [\u001E];
