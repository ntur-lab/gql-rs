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
use crate::ast::model::Span;
use serde::{Deserialize, Serialize};

/// Represents an expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Expression {
    /// Binary operator expression.
    Binary(BinaryExpression),
    /// Unary operator expression.
    Unary(UnaryExpression),
    /// Literal value expression.
    Literal(Literal),
    /// Variable reference expression.
    Variable {
        /// Name of the referenced binding variable.
        name: String,
        /// Source range covered by this AST node.
        span: Span,
    },
    /// Property access expression.
    PropertyAccess(PropertyAccess),
    /// Function call expression.
    FunctionCall(FunctionCall),
    /// IS predicate expression.
    IsPredicate(IsPredicateExpression),
    /// Array literal expression.
    Array(ArrayExpression),
    /// Map literal expression.
    Map(Vec<PropertyItem>),
    /// CASE expression.
    Case(CaseExpression),
    /// Subquery expression.
    Subquery(SubqueryExpression),
    /// CAST expression.
    Cast(CastExpression),
    /// Inline LET expression.
    InlineLet(InlineLetExpression),
    /// Path constructor expression.
    PathConstructor(Vec<Expression>),
    /// WHEN operand expression.
    WhenOperand(WhenOperandExpression),
}

/// Represents a WHEN operand expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WhenOperandExpression {
    /// Specific WHEN operand kind.
    pub kind: WhenOperandKind,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the different kinds of operands
/// that can be used in a WHEN clause of a CASE expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WhenOperandKind {
    /// Comparison operand against a right-hand expression.
    Comparison {
        /// Comparison operator.
        operator: Operator,
        /// Right-hand expression.
        right: Box<Expression>,
    },
    /// NULL predicate operand.
    NullPredicate {
        /// True when predicate is negated.
        negated: bool,
    },
    /// Value-type predicate operand.
    ValueTypePredicate {
        /// True when predicate is negated.
        negated: bool,
        /// Optional target type.
        target_type: Option<TypeSpec>,
    },
    /// Normalization-form predicate operand.
    NormalizedPredicate {
        /// True when predicate is negated.
        negated: bool,
        /// Optional normalization form.
        normal_form: Option<String>,
    },
    /// Directedness predicate operand.
    DirectedPredicate {
        /// True when predicate is negated.
        negated: bool,
    },
    /// Label predicate operand.
    LabeledPredicate {
        /// True when predicate is negated.
        negated: bool,
        /// Optional label expression text.
        label_expression: Option<String>,
    },
    /// Source predicate operand.
    SourcePredicate {
        /// True when predicate is negated.
        negated: bool,
        /// Optional edge reference.
        edge_reference: Option<String>,
    },
    /// Destination predicate operand.
    DestinationPredicate {
        /// True when predicate is negated.
        negated: bool,
        /// Optional edge reference.
        edge_reference: Option<String>,
    },
}

/// Represents a binary expression in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BinaryExpression {
    /// Left-hand expression.
    pub left: Box<Expression>,
    /// Binary operator.
    pub operator: Operator,
    /// Right-hand expression.
    pub right: Box<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an operator.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Operator {
    /// Addition operator.
    Plus,
    /// Subtraction operator.
    Minus,
    /// Multiplication operator.
    Star,
    /// Division operator.
    Slash,
    /// Modulo operator.
    Percent,
    /// Exponentiation operator.
    Caret,
    /// String concatenation operator.
    Concat,

    /// Equality operator.
    Equal,
    /// Inequality operator.
    NotEqual,
    /// Less-than operator.
    LessThan,
    /// Less-than-or-equal operator.
    LessEqual,
    /// Greater-than operator.
    GreaterThan,
    /// Greater-than-or-equal operator.
    GreaterEqual,
    /// Regular-expression match operator.
    Regex,

    /// Logical AND operator.
    And,
    /// Logical OR operator.
    Or,
    /// Logical NOT operator.
    Not,
    /// Logical XOR operator.
    Xor,
}

/// Represents a unary expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnaryExpression {
    /// Unary operator.
    pub operator: Operator,
    /// Operand expression.
    pub expression: Box<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a property access.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyAccess {
    /// Object/variable being accessed.
    pub object: String,
    /// Property key name.
    pub property: String,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a function call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    /// Function name.
    pub name: String,
    /// DISTINCT/ALL qualifier.
    pub distinct: DistinctQualifier,
    /// Function arguments.
    pub arguments: Vec<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a CASE expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CaseExpression {
    /// CASE payload kind.
    pub case_type: CaseType,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the different types of CASE expressions.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CaseType {
    /// Simple CASE variant.
    Simple(SimpleCaseExpression),
    /// Searched CASE variant.
    Searched(SearchedCaseExpression),
}

/// Represents a simple CASE expression in a GQL query,
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleCaseExpression {
    /// Test expression.
    pub test_expression: Box<Expression>,
    /// WHEN branches.
    pub when_branches: Vec<SimpleWhenBranch>,
    /// Optional ELSE expression.
    pub else_expression: Option<Box<Expression>>,
}

/// Represents a WHEN branch in a simple CASE expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimpleWhenBranch {
    /// Values tested by this branch.
    pub when_values: Vec<Expression>,
    /// Result expression when matched.
    pub then_expression: Box<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a searched CASE expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchedCaseExpression {
    /// WHEN branches.
    pub when_branches: Vec<SearchedWhenBranch>,
    /// Optional ELSE expression.
    pub else_expression: Option<Box<Expression>>,
}

/// Represents a WHEN branch in a searched CASE expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SearchedWhenBranch {
    /// Branch condition.
    pub condition: Box<Expression>,
    /// Result expression when condition is true.
    pub then_expression: Box<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an IS predicate expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IsPredicateExpression {
    /// Checked expression.
    pub expression: Box<Expression>,
    /// True when predicate is negated.
    pub negated: bool,
    /// Predicate kind.
    pub predicate: IsPredicateKind,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents the specific kind of IS predicate.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IsPredicateKind {
    /// IS TRUE.
    True,
    /// IS FALSE.
    False,
    /// IS UNKNOWN.
    Unknown,
    /// IS NULL.
    Null,
    /// IS OF TYPE predicate.
    Typed(TypeSpec),
}

/// Represents a subquery expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubqueryExpression {
    /// Nested query body.
    pub query: Box<Query>,
    /// Optional projection expression.
    pub expression: Option<Box<Expression>>,
    /// True for EXISTS-subquery shape.
    pub is_exists: bool,
    /// True when predicate is negated.
    pub negated: bool,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a CAST expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CastExpression {
    /// Expression being cast.
    pub expression: Box<Expression>,
    /// Target type.
    pub target_type: TypeSpec,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an inline LET expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InlineLetExpression {
    /// Variable definitions evaluated in order.
    pub variable_definitions: Vec<VariableDefinition>,
    /// Final expression evaluated in local scope.
    pub evaluation_expression: Box<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a literal value in a GQL query.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Literal {
    /// String literal.
    String(String),
    /// Integer literal.
    Integer(i64),
    /// Floating-point literal.
    Float(f64),
    /// Boolean literal.
    Boolean(bool),
    /// NULL literal.
    Null,
    /// Date/time literal text.
    DateTime(String),
    /// Duration literal text.
    Duration(String),
    /// Time-window literal text.
    TimeWindow(String),
    /// Vector literal values.
    Vector(Vec<f64>),
    /// Nested list literal values.
    List(Vec<Literal>),
    /// Parameter reference expression
    Parameter(String),
}

/// Represents an array expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArrayExpression {
    /// Array elements.
    pub elements: Vec<Expression>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a type specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeSpec {
    /// BOOLEAN type.
    Boolean,
    /// STRING type.
    String {
        /// Optional max length.
        max_length: Option<usize>,
    },
    /// BYTES type.
    Bytes {
        /// Optional max length.
        max_length: Option<usize>,
    },
    /// DECIMAL type.
    Decimal {
        /// Optional precision.
        precision: Option<u8>,
        /// Optional scale.
        scale: Option<u8>,
    },
    /// INTEGER type.
    Integer,
    /// BIGINT type.
    BigInt,
    /// SMALLINT type.
    SmallInt,
    /// INT128 type.
    Int128,
    /// INT256 type.
    Int256,
    /// FLOAT type.
    Float {
        /// Optional precision.
        precision: Option<u8>,
    },
    /// FLOAT32 type.
    Float32,
    /// REAL type.
    Real,
    /// DOUBLE type.
    Double,
    /// VECTOR type.
    Vector {
        /// Optional vector dimension.
        dimension: Option<usize>,
    },
    /// DATE type.
    Date,
    /// TIME type.
    Time {
        /// Optional fractional precision.
        precision: Option<u8>,
        /// Whether timezone is required.
        with_timezone: bool,
    },
    /// TIMESTAMP type.
    Timestamp {
        /// Optional fractional precision.
        precision: Option<u8>,
        /// Whether timezone is required.
        with_timezone: bool,
    },
    /// ZONED TIME type.
    ZonedTime {
        /// Optional precision.
        precision: Option<u8>,
    },
    /// ZONED DATETIME type.
    ZonedDateTime {
        /// Optional precision.
        precision: Option<u8>,
    },
    /// LOCAL TIME type.
    LocalTime {
        /// Optional precision.
        precision: Option<u8>,
    },
    /// LOCAL DATETIME type.
    LocalDateTime {
        /// Optional precision.
        precision: Option<u8>,
    },
    /// DURATION type.
    Duration {
        /// Optional precision.
        precision: Option<u8>,
        /// Optional qualifier text.
        qualifier: Option<String>,
    },
    /// REFERENCE type.
    Reference {
        /// Optional referenced target type.
        target_type: Option<Box<TypeSpec>>,
    },
    /// PATH type.
    Path,
    /// LIST type.
    List {
        /// Element type.
        element_type: Box<TypeSpec>,
        /// Optional max length.
        max_length: Option<usize>,
    },
    /// RECORD type.
    Record,
    /// GRAPH type.
    Graph {
        /// Optional graph type specification.
        graph_type_spec: Option<Box<GraphTypeSpec>>,
    },
    /// BINDING TABLE type.
    BindingTable,
    /// UNION type.
    Union {
        /// Member types in the union.
        members: Vec<TypeSpec>,
    },
    /// ANY type.
    Any,
}

/// Represents a graph type specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GraphTypeSpec {
    /// Vertex type declarations.
    pub vertex_types: Vec<VertexTypeSpec>,
    /// Edge type declarations.
    pub edge_types: Vec<EdgeTypeSpec>,
}

/// Represents a vertex type specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VertexTypeSpec {
    /// Optional vertex identifier.
    pub identifier: Option<String>,
    /// Optional vertex labels.
    pub labels: Option<LabelExpression>,
    /// Optional vertex properties.
    pub properties: Option<PropertyTypeList>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents an edge type specification.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EdgeTypeSpec {
    /// Optional edge identifier.
    pub identifier: Option<String>,
    /// Optional edge labels.
    pub labels: Option<LabelExpression>,
    /// Optional edge properties.
    pub properties: Option<PropertyTypeList>,
    /// Optional source vertex type.
    pub source_vertex: Option<String>,
    /// Optional destination vertex type.
    pub destination_vertex: Option<String>,
    /// Source range covered by this AST node.
    pub span: Span,
}

/// Represents a label expression.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabelExpression {
    /// Conjunctive/disjunctive label terms.
    pub terms: Vec<LabelTerm>,
}

/// Represents a label term in Label expression.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LabelTerm {
    /// Factors within the term.
    pub factors: Vec<LabelFactor>,
}

/// Represents a label factor in a label term.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum LabelFactor {
    /// Label identifier.
    Identifier {
        /// Label name.
        name: String,
        /// Source range covered by this AST node.
        span: Span,
    },
    /// Wildcard label.
    Wildcard,
    /// Parenthesized label expression.
    Parenthesized(Box<LabelExpression>),
}

/// Represents a list of property type declarations.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyTypeList {
    /// Property type declarations.
    pub properties: Vec<PropertyTypeDecl>,
}

/// Represents a property type declaration.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PropertyTypeDecl {
    /// Property name.
    pub name: String,
    /// Property type specification.
    pub type_spec: TypeSpec,
    /// Source range covered by this AST node.
    pub span: Span,
}
