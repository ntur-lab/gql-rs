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

/// Represents a pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathPattern {
    /// Optional path variable binding.
    pub variable: Option<String>,
    /// Optional path mode constraint.
    pub path_mode: Option<PathMode>,
    /// Optional path search strategy.
    pub path_search: Option<PathSearch>,
    /// Optional simplified textual pattern.
    pub simplified_pattern: Option<String>,
    /// Optional alternation metadata.
    pub alternation: Option<PathAlternation>,
    /// Ordered path elements (node/edge sequence).
    pub elements: Vec<PatternElement>,
    pub span: Span,
}

/// Represents explicit alternation metadata for a path pattern expression.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PathAlternation {
    /// Alternation operator.
    pub operator: PathAlternationOperator,
    /// Alternative element sequences.
    pub alternatives: Vec<Vec<PatternElement>>,
    pub span: Span,
}

/// Represents the alternation operator used in a path expression.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PathAlternationOperator {
    /// Union alternation.
    Union,
    /// Multiset alternation.
    MultisetAlternation,
}

/// Represents the different path search strategies.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PathSearch {
    /// ALL paths.
    All,
    /// ANY path.
    Any,
    /// SHORTEST path.
    Shortest,
    /// ALL SHORTEST paths.
    AllShortest,
    /// ANY SHORTEST path.
    AnyShortest,
    /// SHORTEST with path count.
    CountedShortestPaths {
        /// Optional number of shortest paths.
        count: Option<u32>,
    },
    /// SHORTEST with group count.
    CountedShortestGroups {
        /// Optional number of shortest groups.
        groups: Option<u32>,
    },
}

/// Represents the different path modes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PathMode {
    /// WALK mode.
    Walk,
    /// TRAIL mode.
    Trail,
    /// SIMPLE mode.
    Simple,
    /// ACYCLIC mode.
    Acyclic,
}

/// Represents an element in a Path pattern, 
/// basic building block of a pattern which can be either a node or an edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PatternElement {
    /// Node element.
    Node(Node),
    /// Edge element.
    Edge(Edge),
}

/// Represents a node in a Path pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Node {
    /// Optional node identifier.
    pub id: Option<String>,
    /// Optional node label expression.
    pub labels: Option<Expression>,
    /// Optional node properties.
    pub properties: Option<PropertyMap>,
    /// Optional node-local WHERE predicate.
    pub where_clause: Option<Box<Expression>>,
    pub span: Span, 
}

/// Represents an edge in a Path pattern.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Edge {
    /// Optional edge identifier.
    pub id: Option<String>,
    /// Optional edge label expression.
    pub labels: Option<Expression>,
    /// Optional edge properties.
    pub properties: Option<PropertyMap>,
    /// Optional edge-local WHERE predicate.
    pub where_clause: Option<Box<Expression>>,
    /// Edge direction.
    pub direction: EdgeDirection,
    /// Optional path quantifier.
    pub quantifier: Option<PathQuantifier>,
    pub span: Span, 
}

/// Represents the different directions that can be specified for an edge.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EdgeDirection {
    /// Outgoing edge.
    Outgoing,
    /// Incoming edge.
    Incoming,
    /// Bidirectional edge.
    Both,
    /// Undirected edge.
    Undirected,
}

/// Represents the different quantifiers that can be specified for variable-length paths.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PathQuantifier {
    /// Optional quantifier (`?`).
    Optional,
    /// Exact repetition count.
    Exact(u32),
    /// Inclusive repetition range.
    Range {
        /// Minimum repetition count.
        min: u32,
        /// Maximum repetition count.
        max: u32,
    },
    /// Minimum repetition count.
    AtLeast(u32),
    /// Maximum repetition count.
    AtMost(u32),
}

/// Represents a map of properties for a node or edge.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyMap {
    /// Property assignments.
    pub properties: Vec<PropertyItem>,
    pub span: Span,
}

/// Represents a property item in a property map.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PropertyItem {
    /// Property key.
    pub key: String,
    /// Property value expression.
    pub value: Expression,
    pub span: Span,
}