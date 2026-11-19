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
        target_type: Option<ValueType>,
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
    Typed(ValueType),
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
    pub target_type: ValueType,
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
    /// Approximate numeric literal, and exact decimals written without the `M` suffix.
    Float(f64),
    /// Exact numeric literal with the `M` suffix (`1.5M`, `10M`), kept as normalized text
    /// (underscores and suffix removed) to avoid any precision loss.
    Decimal(String),
    /// Byte string literal (`X'0A FF'`).
    Bytes(Vec<u8>),
    /// Boolean literal.
    Boolean(bool),
    /// NULL literal.
    Null,
    /// `DATE '...'` literal text.
    Date(String),
    /// `TIME '...'` literal text.
    Time(String),
    /// `DATETIME '...'` / `TIMESTAMP '...'` literal text.
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

/// A declared value type: a [`TypeSpec`] together with its nullability.
///
/// GQL attaches `NOT NULL` to a value type independently from its base type, so `INT` and
/// `INT NOT NULL` share the same [`TypeSpec`] and differ only in [`ValueType::not_null`].
/// Declared types (CAST targets, `IS TYPED` predicates, record and binding-table fields, list
/// elements and property types) use this struct; inferred types use bare [`TypeSpec`]s.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ValueType {
    /// Base type, without nullability.
    pub base: TypeSpec,
    /// True when the declared type excludes the null value (`NOT NULL`).
    pub not_null: bool,
}

impl ValueType {
    /// Creates a value type with explicit nullability.
    pub fn new(base: TypeSpec, not_null: bool) -> Self {
        Self { base, not_null }
    }

    /// Creates a nullable value type (no `NOT NULL` constraint).
    pub fn nullable(base: TypeSpec) -> Self {
        Self::new(base, false)
    }
}

/// A named field of a closed record type or binding-table type.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct FieldType {
    /// Field name.
    pub name: String,
    /// Declared field type.
    pub value_type: ValueType,
}

/// Size of a binary exact numeric (integer) type.
///
/// Explicit bit widths (`INT8`, `UINT64`, `INTEGER128`, ...) are kept apart from the
/// implementation-defined sizes `SMALL`, regular and `BIG`, because GQL does not fix the width
/// of the latter.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum IntegerSize {
    /// 8-bit (`INT8`, `INTEGER8`, `UINT8`).
    Bits8,
    /// 16-bit (`INT16`, `INTEGER16`, `UINT16`).
    Bits16,
    /// 32-bit (`INT32`, `INTEGER32`, `UINT32`).
    Bits32,
    /// 64-bit (`INT64`, `INTEGER64`, `UINT64`).
    Bits64,
    /// 128-bit (`INT128`, `INTEGER128`, `UINT128`).
    Bits128,
    /// 256-bit (`INT256`, `INTEGER256`, `UINT256`).
    Bits256,
    /// `SMALLINT`, `SMALL INTEGER`, `USMALLINT`.
    Small,
    /// `INT`, `INTEGER`, `UINT`, optionally with a binary precision (`INT(p)`).
    Regular {
        /// Optional binary precision.
        precision: Option<u32>,
    },
    /// `BIGINT`, `BIG INTEGER`, `UBIGINT`.
    Big,
}

/// Size of an approximate numeric (floating-point) type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum FloatSize {
    /// `FLOAT16`.
    Bits16,
    /// `FLOAT32`.
    Bits32,
    /// `FLOAT64`.
    Bits64,
    /// `FLOAT128`.
    Bits128,
    /// `FLOAT256`.
    Bits256,
    /// `FLOAT`, optionally with precision and scale.
    Regular {
        /// Optional precision.
        precision: Option<u32>,
        /// Optional scale.
        scale: Option<u32>,
    },
    /// `REAL`.
    Real,
    /// `DOUBLE` / `DOUBLE PRECISION`.
    Double,
}

/// Qualifier of a `DURATION(...)` type.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DurationQualifier {
    /// `YEAR TO MONTH`.
    YearToMonth,
    /// `DAY TO SECOND`.
    DayToSecond,
}

impl DurationQualifier {
    /// Canonical GQL spelling of the qualifier.
    pub fn as_str(&self) -> &'static str {
        match self {
            DurationQualifier::YearToMonth => "YEAR TO MONTH",
            DurationQualifier::DayToSecond => "DAY TO SECOND",
        }
    }
}

/// Represents a type specification (the base type, without nullability; see [`ValueType`]).
///
/// For reference and record types `None` means the open form (`ANY NODE`, `ANY RECORD`, ...)
/// and `Some` the closed form carrying its structural description.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum TypeSpec {
    /// BOOLEAN type.
    Boolean,
    /// Character string type: `STRING[(min,] max)]`, `CHAR(n)` (min = max = n), `VARCHAR(n)`.
    String {
        /// Optional minimum length.
        min_length: Option<usize>,
        /// Optional maximum length.
        max_length: Option<usize>,
    },
    /// Byte string type: `BYTES[(min,] max)]`, `BINARY(n)` (min = max = n), `VARBINARY(n)`.
    Bytes {
        /// Optional minimum length.
        min_length: Option<usize>,
        /// Optional maximum length.
        max_length: Option<usize>,
    },
    /// Binary exact numeric (integer) type, signed or unsigned.
    Integer {
        /// False for `UINT*`, `USMALLINT`, `UBIGINT` and `UNSIGNED ...`.
        signed: bool,
        /// Declared size.
        size: IntegerSize,
    },
    /// DECIMAL type.
    Decimal {
        /// Optional precision.
        precision: Option<u32>,
        /// Optional scale.
        scale: Option<u32>,
    },
    /// Approximate numeric (floating-point) type.
    Float {
        /// Declared size.
        size: FloatSize,
    },
    /// VECTOR type.
    Vector {
        /// Optional vector dimension.
        dimension: Option<usize>,
    },
    /// DATE type.
    Date,
    /// ZONED TIME / TIME WITH TIME ZONE type.
    ZonedTime,
    /// LOCAL TIME / TIME WITHOUT TIME ZONE type.
    LocalTime,
    /// ZONED DATETIME / TIMESTAMP WITH TIME ZONE type.
    ZonedDateTime,
    /// LOCAL DATETIME / TIMESTAMP [WITHOUT TIME ZONE] type.
    LocalDateTime,
    /// DURATION type. `None` is the generic duration produced by inference.
    Duration {
        /// Optional duration qualifier.
        qualifier: Option<DurationQualifier>,
    },
    /// Node reference type: `None` for `[ANY] NODE`, `Some` for a closed node type.
    Node {
        /// Closed node type description.
        node_type: Option<Box<VertexTypeSpec>>,
    },
    /// Edge reference type: `None` for `[ANY] EDGE`, `Some` for a closed edge type.
    Edge {
        /// Closed edge type description.
        edge_type: Option<Box<EdgeTypeSpec>>,
    },
    /// Graph reference type: `None` for `ANY [PROPERTY] GRAPH`, `Some` for a closed graph type.
    Graph {
        /// Optional graph type specification.
        graph_type_spec: Option<Box<GraphTypeSpec>>,
    },
    /// Binding table reference type. `None` is the unspecified table produced by inference.
    BindingTable {
        /// Declared fields.
        fields: Option<Vec<FieldType>>,
    },
    /// PATH type.
    Path,
    /// LIST type.
    List {
        /// Element type, with its own nullability.
        element_type: Box<ValueType>,
        /// Optional max length.
        max_length: Option<usize>,
    },
    /// RECORD type: `None` for `[ANY] RECORD`, `Some` for a closed record type.
    Record {
        /// Declared fields.
        fields: Option<Vec<FieldType>>,
    },
    /// Closed dynamic union type (`A | B`, `ANY <A | B>`).
    Union {
        /// Member types in the union.
        members: Vec<TypeSpec>,
    },
    /// NULL type (contains only the null value).
    Null,
    /// Empty type (`NOTHING`, `NULL NOT NULL`).
    Nothing,
    /// `[ANY] PROPERTY VALUE` type.
    PropertyValue,
    /// `ANY [VALUE]` type, also used for unknown inferred types.
    Any,
}

impl TypeSpec {
    /// Regular signed integer (`INT`), the type inferred for integer literals.
    pub fn integer() -> Self {
        TypeSpec::Integer {
            signed: true,
            size: IntegerSize::Regular { precision: None },
        }
    }

    /// Regular `FLOAT`, the type inferred for approximate numeric literals.
    pub fn float() -> Self {
        TypeSpec::Float {
            size: FloatSize::Regular {
                precision: None,
                scale: None,
            },
        }
    }

    /// Unconstrained `STRING`.
    pub fn string() -> Self {
        TypeSpec::String {
            min_length: None,
            max_length: None,
        }
    }

    /// Unconstrained `BYTES`.
    pub fn bytes() -> Self {
        TypeSpec::Bytes {
            min_length: None,
            max_length: None,
        }
    }

    /// Unconstrained `DECIMAL`.
    pub fn decimal() -> Self {
        TypeSpec::Decimal {
            precision: None,
            scale: None,
        }
    }

    /// Generic `DURATION`.
    pub fn duration() -> Self {
        TypeSpec::Duration { qualifier: None }
    }

    /// Open `RECORD`.
    pub fn record() -> Self {
        TypeSpec::Record { fields: None }
    }

    /// Binding table without declared fields.
    pub fn binding_table() -> Self {
        TypeSpec::BindingTable { fields: None }
    }

    /// List of nullable elements without max length.
    pub fn list_of(element: TypeSpec) -> Self {
        TypeSpec::List {
            element_type: Box::new(ValueType::nullable(element)),
            max_length: None,
        }
    }
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
    pub type_spec: ValueType,
    /// Source range covered by this AST node.
    pub span: Span,
}
