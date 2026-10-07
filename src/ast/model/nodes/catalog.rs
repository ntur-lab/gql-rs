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
use serde::{Deserialize, Serialize};
use crate::ast::model::Span;

/// Top-level GQL statement categories represented by this AST.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Statement {
    /// Query statement tree.
    Query(Query),
    /// Data manipulation statement tree.
    Data(DataStatement),
    /// NEXT chained statement.
    Next(NextStatement),
    /// LET variable-definition statement.
    Let(LetStatement),
    /// Session command statement.
    Session(SessionStatement),
    /// Transaction command statement.
    Transaction(TransactionStatement),
    /// Catalog DDL statement block.
    Catalog(CatalogStatementBlock),
}

impl Statement {
    /// Derives the transaction requirement needed to execute this statement.
    pub fn transaction_intent(&self) -> TransactionIntent {
        match self {
            Statement::Query(query) => query.transaction_intent(),
            Statement::Data(_) => TransactionIntent::DataWrite,
            Statement::Next(next) => next.transaction_intent(),
            Statement::Let(_) => TransactionIntent::ReadOnly,
            Statement::Session(_) => TransactionIntent::ReadOnly,
            Statement::Transaction(_) => TransactionIntent::ReadOnly,
            Statement::Catalog(_) => TransactionIntent::CatalogWrite,
        }
    }

    /// Convenience helper used by runtimes that branch on write requirements.
    pub fn requires_write_tx(&self) -> bool {
        self.transaction_intent().requires_write_tx()
    }
}

/// Represents a block of catalog statements.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogStatementBlock {
    /// Statements in the block.
    pub statements: Vec<CatalogStatement>,
}

/// Represents a catalog statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CatalogStatement {
    /// CREATE SCHEMA statement.
    CreateSchema(CreateSchema),
    /// DROP SCHEMA statement.
    DropSchema(DropSchema),
    /// CREATE GRAPH statement.
    CreateGraph(CreateGraph),
    /// DROP GRAPH statement.
    DropGraph(DropGraph),
    /// CREATE GRAPH TYPE statement.
    CreateGraphType(CreateGraphType),
    /// DROP GRAPH TYPE statement.
    DropGraphType(DropGraphType),
    /// Catalog CALL statement preserved in catalog block order.
    Call(CallStatement),
    /// Raw/untyped catalog statement fallback.
    Raw(RawCatalogStatement),
}

/// Represents a raw catalog statement,
/// used as a fallback for statements that are parsed
/// but not fully modeled yet in dedicated typed AST variants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RawCatalogStatement {
    /// Original statement text.
    pub text: String,
    /// Optional parsing/build detail when the statement falls back to raw text.
    pub parse_error: Option<String>,
    /// Source span for the fallback location.
    pub span: Span,
}

/// Represents a CREATE SCHEMA DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateSchema {
    /// True when IF NOT EXISTS is present.
    pub if_not_exists: bool,
    /// Target schema name.
    pub target: CatalogSchemaName,
}

/// Represents a DROP SCHEMA DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropSchema {
    /// True when IF EXISTS is present.
    pub if_exists: bool,
    /// Target schema name.
    pub target: CatalogSchemaName,
}

/// Represents a CREATE GRAPH DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGraph {
    /// True when OR REPLACE is present.
    pub or_replace: bool,
    /// True when PROPERTY GRAPH keywords are present.
    pub property_graph_keyword: bool,
    /// True when IF NOT EXISTS is present.
    pub if_not_exists: bool,
    /// Target graph object name.
    pub target: CatalogObjectName,
    /// Graph type clause for the created graph.
    pub graph_type_clause: CreateGraphTypeClause,
    /// Optional graph source clause.
    pub graph_source: Option<GraphSourceClause>,
}

/// Represents a DROP GRAPH DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropGraph {
    /// True when PROPERTY GRAPH keywords are present.
    pub property_graph_keyword: bool,
    /// True when IF EXISTS is present.
    pub if_exists: bool,
    /// Target graph object name.
    pub target: CatalogObjectName,
}

/// Represents a CREATE GRAPH TYPE DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateGraphType {
    /// True when OR REPLACE is present.
    pub or_replace: bool,
    /// True when PROPERTY GRAPH keywords are present.
    pub property_graph_keyword: bool,
    /// True when IF NOT EXISTS is present.
    pub if_not_exists: bool,
    /// Target graph type object name.
    pub target: CatalogObjectName,
    /// Source clause used to build the graph type.
    pub source: GraphTypeSourceClause,
}

/// Represents a DROP GRAPH TYPE DDL statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropGraphType {
    /// True when PROPERTY GRAPH keywords are present.
    pub property_graph_keyword: bool,
    /// True when IF EXISTS is present.
    pub if_exists: bool,
    /// Target graph type object name.
    pub target: CatalogObjectName,
}

/// Represents a catalog schema name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogSchemaName {
    /// Optional absolute directory path for the schema.
    pub absolute_directory_path: String,
    /// Schema identifier.
    pub schema_name: String,
    /// Source span for schema name diagnostics.
    pub span: Span,
}

/// Represents a catalog object name.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CatalogObjectName {
    /// Optional parent qualifier.
    pub parent: Option<String>,
    /// Object identifier.
    pub name: String,
    /// Source span for object name diagnostics.
    pub span: Span,
}

/// Represents a graph type clause in a CREATE GRAPH statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CreateGraphTypeClause {
    /// OPEN graph type clause.
    Open(OpenGraphTypeClause),
    /// OF graph type clause.
    Of(OfGraphTypeClause),
}

/// Represents an open graph type clause in a CREATE GRAPH statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenGraphTypeClause {
    /// True when TYPED modifier is present.
    pub typed: bool,
    /// True when PROPERTY GRAPH keywords are present.
    pub property_graph_keyword: bool,
    /// True when GRAPH keyword is present.
    pub graph_keyword: bool,
}

/// Represents an OF graph type clause in a CREATE GRAPH statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum OfGraphTypeClause {
    /// LIKE <graph expression> variant.
    LikeGraph {
        /// Graph expression used as template.
        graph_expression: GraphExpressionClause,
    },
    /// TYPED reference to an existing graph type.
    TypedReference {
        /// True when TYPED modifier is present.
        typed: bool,
        /// Referenced graph type name.
        graph_type_reference: String,
    },
    /// Nested inline graph type specification.
    TypedNestedSpecification {
        /// True when TYPED modifier is present.
        typed: bool,
        /// True when PROPERTY GRAPH keywords are present.
        property_graph_keyword: bool,
        /// True when GRAPH keyword is present.
        graph_keyword: bool,
        /// Parsed nested graph type specification.
        nested_graph_type_specification: NestedGraphTypeSpecification,
    },
}

/// Represents a graph source clause in a CREATE GRAPH statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphSourceClause {
    /// Graph expression used as source.
    pub graph_expression: GraphExpressionClause,
}

/// Represents a graph type source clause in a CREATE GRAPH TYPE statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphTypeSourceClause {
    /// COPY OF <graph type> variant.
    CopyOf {
        /// True when WITH AS keywords are present.
        with_as_keyword: bool,
        /// Referenced graph type name.
        graph_type_reference: String,
    },
    /// LIKE <graph expression> variant.
    LikeGraph {
        /// Graph expression used as template.
        graph_expression: GraphExpressionClause,
    },
    /// Inline nested graph type specification variant.
    NestedSpecification {
        /// True when WITH AS keywords are present.
        with_as_keyword: bool,
        /// Parsed nested graph type specification.
        nested_graph_type_specification: NestedGraphTypeSpecification,
    },
}

/// Represents a nested graph type specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NestedGraphTypeSpecification {
    /// Typed list of node and edge type specifications.
    pub element_type_specifications: Vec<ElementTypeSpecification>,
}

/// Represents an element type specification in a nested graph type.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ElementTypeSpecification {
    /// Node type specification element.
    Node(NodeTypeSpecification),
    /// Edge type specification element.
    Edge(EdgeTypeSpecification),
}

/// Represents a node type specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTypeSpecification {
    /// Original node type specification text.
    pub text: String,
    /// Optional local alias inferred from specification.
    pub local_alias: Option<String>,
    /// Optional declared node type name.
    pub type_name: Option<String>,
    /// Optional typed property specification payload.
    pub property_types: Option<PropertyTypesSpecification>,
}

/// Represents an edge type specification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EdgeTypeSpecification {
    /// Original edge type specification text.
    pub text: String,
    /// Optional declared edge type name.
    pub type_name: Option<String>,
    /// Optional source endpoint alias when inferable.
    pub source_alias: Option<String>,
    /// Optional destination endpoint alias when inferable.
    pub destination_alias: Option<String>,
    /// True for directed edge specification when inferable.
    pub directed: Option<bool>,
    /// Optional typed property specification payload.
    pub property_types: Option<PropertyTypesSpecification>,
}

/// Represents a property types specification block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTypesSpecification {
    /// Parsed property type entries.
    pub properties: Vec<PropertyTypeSpecification>,
}

/// Represents a single property type entry.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyTypeSpecification {
    /// Property name.
    pub name: String,
    /// True when the TYPED marker is present.
    pub typed: bool,
    /// Property value type text.
    pub value_type: String,
    /// Parsed typed representation when build-time mapping succeeds.
    pub parsed_value_type: Option<TypeSpec>,
    /// Source span for the property declaration.
    pub span: Span,
}

/// Represents a graph expression clause.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GraphExpressionClause {
    /// Parsed graph expression kind.
    pub kind: GraphExpressionKind,
    /// Original graph expression text.
    pub text: String,
    /// Parsed reference when kind is GraphReference.
    pub graph_reference: Option<String>,
    /// Parsed name/variable when kind is ObjectNameOrBindingVariable.
    pub object_name_or_binding_variable: Option<String>,
    /// Source span for the graph expression.
    pub span: Span,
}

/// Represents the different kinds of graph expressions that can be used.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GraphExpressionKind {
    /// Graph reference expression.
    GraphReference,
    /// Object expression primary.
    ObjectExpressionPrimary,
    /// Object name or binding variable expression.
    ObjectNameOrBindingVariable,
    /// CURRENT GRAPH expression.
    CurrentGraph,
}

/// Represents a session command statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionStatement {
    /// Parsed session command kind.
    pub kind: SessionCommandKind,
    /// Optional typed payload for SESSION SET clauses.
    pub set_clauses: Vec<SessionSetClause>,
    /// Optional typed payload for SESSION RESET clauses.
    pub reset_clauses: Vec<SessionResetClause>,
}

/// Represents supported session command kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionCommandKind {
    /// SESSION SET command.
    Set,
    /// SESSION RESET command.
    Reset,
    /// SESSION CLOSE command.
    Close,
    /// Generic session activity command.
    Activity,
}

/// Represents a SESSION SET clause payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionSetClause {
    /// SESSION SET SCHEMA <schema reference>
    Schema {
        /// Schema reference text.
        schema_reference: String,
    },
    /// SESSION SET [PROPERTY] GRAPH <graph expression>
    Graph {
        /// True when PROPERTY keyword is present.
        property_graph_keyword: bool,
        /// Target graph expression.
        graph_expression: GraphExpressionClause,
    },
    /// SESSION SET TIME ZONE <value>
    TimeZone {
        /// Time zone value payload.
        value: SessionTimeZoneValue,
    },
    /// SESSION SET <parameter clause>
    Parameter(SessionSetParameterClause),
}

/// Represents a SESSION RESET clause payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionResetClause {
    /// Reset target payload.
    pub target: SessionResetTarget,
}

/// Represents a SESSION RESET target.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionResetTarget {
    /// RESET [ALL] PARAMETERS|CHARACTERISTICS
    ParametersOrCharacteristics {
        /// True when ALL is present.
        all: bool,
        /// Reset collection target.
        target: SessionResetCollectionTarget,
    },
    /// RESET SCHEMA
    Schema,
    /// RESET [PROPERTY] GRAPH
    Graph {
        /// True when PROPERTY keyword is present.
        property_graph_keyword: bool,
    },
    /// RESET TIME ZONE
    TimeZone,
    /// RESET [PARAMETER] <parameter spec>
    Parameter {
        /// True when PARAMETER keyword is present.
        has_parameter_keyword: bool,
        /// Parameter specification text.
        parameter_specification: String,
    },
}

/// Represents the target of RESET [ALL] PARAMETERS|CHARACTERISTICS.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionResetCollectionTarget {
    /// PARAMETERS target.
    Parameters,
    /// CHARACTERISTICS target.
    Characteristics,
}

/// Represents a SESSION SET parameter clause payload.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSetParameterClause {
    /// Parameter clause kind.
    pub kind: SessionSetParameterKind,
    /// Parameter name and options.
    pub parameter_name: SessionSetParameterName,
    /// Optional initializer text.
    pub initializer: Option<String>,
    /// Optional initializer parsed as graph expression.
    pub graph_initializer: Option<GraphExpressionClause>,
    /// Optional initializer parsed as value expression.
    pub value_initializer: Option<Expression>,
    /// Optional initializer parsed as binding table expression text.
    pub binding_table_initializer: Option<String>,
}

/// Represents SESSION SET parameter clause kinds.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionSetParameterKind {
    /// [PROPERTY] GRAPH parameter clause.
    Graph {
        /// True when PROPERTY keyword is present.
        property_graph_keyword: bool,
    },
    /// [BINDING] TABLE parameter clause.
    BindingTable {
        /// True when BINDING keyword is present.
        binding_keyword: bool,
    },
    /// VALUE parameter clause.
    Value,
}

/// Represents SESSION SET parameter name metadata.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SessionSetParameterName {
    /// True when IF NOT EXISTS is present.
    pub if_not_exists: bool,
    /// Parameter specification text.
    pub parameter_specification: String,
    /// Source span for the parameter name.
    pub span: Span,
}

/// Represents a SESSION TIME ZONE value.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SessionTimeZoneValue {
    /// Time zone string literal.
    Literal(String),
}

/// Represents a transaction command statement.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransactionStatement {
    /// Parsed transaction command kind.
    pub kind: TransactionCommandKind,
    /// Optional transaction mode for START TRANSACTION.
    pub mode: Option<TransactionMode>,
    /// Optional transaction characteristics for START TRANSACTION.
    pub body: Option<Box<Statement>>,
    /// Optional end command kind for START TRANSACTION to link with its closing command in the AST.
    pub end_command: Option<TransactionCommandKind>,
}

/// Represents supported transaction command kinds.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionCommandKind {
    /// START TRANSACTION command.
    Start,
    /// COMMIT command.
    Commit,
    /// ROLLBACK command.
    Rollback,
    /// Generic transaction activity command.
    Activity,
}

/// Represents supported transaction access modes.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionMode {
    /// READ ONLY
    ReadOnly,
    /// READ WRITE
    ReadWrite,
}
