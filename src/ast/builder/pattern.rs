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

use super::*;

impl GQLAstBuilder {
    /// Extracts all graph patterns represented by a MATCH statement, preserving optionality.
    pub(super) fn extract_graph_patterns<'input>(
        match_statement: &MatchStatementContextAll<'input>,
    ) -> GqlResult<Vec<(Rc<GraphPatternContextAll<'input>>, bool)>> {
        if let Some(simple_match) = match_statement.simpleMatchStatement() {
            let graph_pattern = simple_match
                .graphPatternBindingTable()
                .and_then(|binding_table| binding_table.graphPattern())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: graphPattern missing in MATCH".into())
                })?;

            return Ok(vec![(graph_pattern, false)]);
        }

        let operand = match_statement
            .optionalMatchStatement()
            .and_then(|optional_match| optional_match.optionalOperand())
            .ok_or_else(|| {
                Self::parse_err("CST not valid: optionalOperand missing in OPTIONAL MATCH".into())
            })?;

        if let Some(simple_match) = operand.simpleMatchStatement() {
            let graph_pattern = simple_match
                .graphPatternBindingTable()
                .and_then(|binding_table| binding_table.graphPattern())
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: graphPattern missing in OPTIONAL MATCH".into())
                })?;

            return Ok(vec![(graph_pattern, true)]);
        }

        if let Some(match_block) = operand.matchStatementBlock() {
            let nested_matches = match_block.matchStatement_all();
            if nested_matches.is_empty() {
                return Self::parse_error(
                    "CST not valid: empty matchStatementBlock in OPTIONAL MATCH",
                );
            }

            let mut flattened = Vec::new();
            for nested_match in nested_matches {
                let nested_patterns = Self::extract_graph_patterns(&nested_match)?;
                for (graph_pattern, _nested_optional) in nested_patterns {
                    flattened.push((graph_pattern, true));
                }
            }

            return Ok(flattened);
        }

        Self::parse_error("CST not valid: unsupported OPTIONAL operand")
    }

    /// Function to build a PropertyAccess from a PropertyAccessContext, extracting the object and property expressions from the context, building the corresponding Expressions for each, and returning the resulting PropertyAccess. If the CST structure does not match the expected format for a property access, it returns a parse error indicating that the CST is not valid.
    pub(super) fn extract_graph_pattern<'input>(
        match_statement: &MatchStatementContextAll<'input>,
    ) -> GqlResult<(Rc<GraphPatternContextAll<'input>>, bool)> {
        let mut patterns = Self::extract_graph_patterns(match_statement)?;
        if patterns.len() != 1 {
            return Self::parse_error(
                "CST not valid: expected a single graph pattern but got multiple",
            );
        }

        Ok(patterns.remove(0))
    }

    /// Function to build a PropertyAccess from a PropertyAccessContext, extracting the object and property expressions from the context, building the corresponding Expressions for each, and returning the resulting PropertyAccess. If the CST structure does not match the expected format for a property access, it returns a parse error indicating that the CST is not valid.
    pub(super) fn build_match_clause(
        graph_pattern: &GraphPatternContextAll,
        optional: bool,
    ) -> GqlResult<MatchClause> {
        let span = Self::extract_span(graph_pattern);

        let patterns = graph_pattern
            .pathPatternList()
            .ok_or_else(|| {
                Self::parse_err("CST not valid: pathPatternList missing in graphPattern".into())
            })?
            .pathPattern_all()
            .into_iter()
            .map(|path_pattern| Self::build_path_pattern_from_context(&path_pattern))
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(MatchClause {
            optional,
            match_clause: MatchClauseItem {
                patterns,
                span: span.clone(),
            },
            span: span.clone(),
        })
    }

    /// Function to build a PropertyAccess from a PropertyAccessContext, extracting the object and property expressions from the context, building the corresponding Expressions for each, and returning the resulting PropertyAccess. If the CST structure does not match the expected format for a property access, it returns a parse error indicating that the CST is not valid.
    pub(super) fn build_path_pattern_from_context(
        path_pattern: &PathPatternContextAll,
    ) -> GqlResult<PathPattern> {
        let span = Self::extract_span(path_pattern);

        let path_expression = path_pattern.pathPatternExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: pathPatternExpression missing".into())
        })?;
        let expression_flavor = Self::path_expression_flavor(&path_expression);

        let search_prefix = path_pattern
            .pathPatternPrefix()
            .and_then(|prefix| prefix.pathSearchPrefix());

        let path_search = path_pattern
            .pathPatternPrefix()
            .and_then(|prefix| prefix.pathSearchPrefix())
            .and_then(|search_prefix| Self::build_path_search_from_prefix(&search_prefix));

        let path_mode = path_pattern
            .pathPatternPrefix()
            .and_then(|prefix| prefix.pathModePrefix())
            .and_then(|mode_prefix| mode_prefix.pathMode())
            .map(|mode_ctx| {
                if mode_ctx.WALK().is_some() {
                    PathMode::Walk
                } else if mode_ctx.TRAIL().is_some() {
                    PathMode::Trail
                } else if mode_ctx.SIMPLE().is_some() {
                    PathMode::Simple
                } else {
                    PathMode::Acyclic
                }
            })
            .or_else(|| {
                search_prefix.and_then(|sp| Self::extract_path_mode_from_search_prefix(&sp))
            });

        let mut alternatives: Vec<Vec<PatternElement>> = Vec::new();
        for path_term in Self::extract_path_terms(path_pattern)? {
            let mut term_elements: Vec<PatternElement> = Vec::new();
            for path_factor in path_term.pathFactor_all() {
                term_elements.extend(Self::build_elements_from_factor(&path_factor)?);
            }
            alternatives.push(term_elements);
        }

        if alternatives.is_empty() {
            return Self::parse_error("CST not valid: pathPatternExpression has no path terms");
        }

        let elements = alternatives.first().cloned().unwrap_or_default();

        let alternation = match expression_flavor {
            PathExpressionFlavor::Single => None,
            PathExpressionFlavor::Union => Some(PathAlternation {
                operator: PathAlternationOperator::Union,
                alternatives,
                span: span.clone(),
            }),
            PathExpressionFlavor::MultisetAlternation => Some(PathAlternation {
                operator: PathAlternationOperator::MultisetAlternation,
                alternatives,
                span: span.clone(),
            }),
        };

        let mut path_pattern_ast = PathPattern {
            variable: path_pattern
                .pathVariableDeclaration()
                .and_then(|declaration| declaration.pathVariable())
                .map(|path_variable| path_variable.get_text()),
            path_mode,
            path_search,
            simplified_pattern: Self::extract_simplified_pattern_text(path_pattern),
            alternation,
            elements,
            span: span.clone(),
        };

        Self::desugar_path_pattern(&mut path_pattern_ast)?;

        Ok(path_pattern_ast)
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn extract_path_mode_from_search_prefix(
        search_prefix: &PathSearchPrefixContextAll,
    ) -> Option<PathMode> {
        let text = search_prefix.get_text().to_ascii_uppercase();
        if text.contains("WALK") {
            return Some(PathMode::Walk);
        }
        if text.contains("TRAIL") {
            return Some(PathMode::Trail);
        }
        if text.contains("SIMPLE") {
            return Some(PathMode::Simple);
        }
        if text.contains("ACYCLIC") {
            return Some(PathMode::Acyclic);
        }
        None
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn desugar_path_pattern(path_pattern: &mut PathPattern) -> GqlResult<()> {
        if path_pattern.simplified_pattern.is_none() {
            return Ok(());
        }

        path_pattern.elements = Self::normalize_simplified_path_elements(
            std::mem::take(&mut path_pattern.elements),
            path_pattern.span.clone(),
        );
        Self::validate_alternating_node_edge(&path_pattern.elements)?;

        if let Some(alternation) = &mut path_pattern.alternation {
            for elements in &mut alternation.alternatives {
                *elements = Self::normalize_simplified_path_elements(
                    std::mem::take(elements),
                    path_pattern.span.clone(),
                );
                Self::validate_alternating_node_edge(elements)?;
            }
        }

        if let Some(first_alternative) = path_pattern
            .alternation
            .as_ref()
            .and_then(|alternation| alternation.alternatives.first())
        {
            path_pattern.elements = first_alternative.clone();
        }

        Ok(())
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn validate_alternating_node_edge(elements: &[PatternElement]) -> GqlResult<()> {
        if elements.is_empty() {
            return Self::parse_error("Simplified path desugaring produced an empty pattern");
        }

        for pair in elements.windows(2) {
            let left_is_node = matches!(pair[0], PatternElement::Node(_));
            let right_is_node = matches!(pair[1], PatternElement::Node(_));
            if left_is_node == right_is_node {
                return Self::parse_error(
                    "Simplified path desugaring failed to produce alternating Node/Edge elements",
                );
            }
        }

        Ok(())
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn normalize_simplified_path_elements(
        elements: Vec<PatternElement>,
        span: Span,
    ) -> Vec<PatternElement> {
        let mut normalized = Vec::new();

        for element in elements {
            if normalized.is_empty() {
                if matches!(element, PatternElement::Edge(_)) {
                    normalized.push(Self::anonymous_path_node(span.clone()));
                }
                normalized.push(element);
                continue;
            }

            if matches!(normalized.last(), Some(PatternElement::Edge(_)))
                && matches!(element, PatternElement::Edge(_))
            {
                normalized.push(Self::anonymous_path_node(span.clone()));
            }

            normalized.push(element);
        }

        if matches!(normalized.last(), Some(PatternElement::Edge(_))) {
            normalized.push(Self::anonymous_path_node(span.clone()));
        }

        normalized
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn anonymous_path_node(span: Span) -> PatternElement {
        PatternElement::Node(Node {
            id: None,
            labels: None,
            properties: None,
            where_clause: None,
            span: span.clone(),
        })
    }

    /// Helper function to determine the flavor of a path expression based on its structure, classifying it as Single, Union, or MultisetAlternation depending on the presence of union or alternation operators in the expression. This classification is used to preserve the semantics of path expressions during AST construction and validation.
    pub(super) fn extract_simplified_pattern_text(
        path_pattern: &PathPatternContextAll,
    ) -> Option<String> {
        let path_expression = path_pattern.pathPatternExpression()?;
        let path_terms = Self::extract_path_terms_from_expression(&path_expression).ok()?;

        for path_term in path_terms {
            for path_factor in path_term.pathFactor_all() {
                match &*path_factor {
                    PathFactorContextAll::PfPathPrimaryContext(ctx) => {
                        if let Some(path_primary) = ctx.pathPrimary()
                            && let PathPrimaryContextAll::PpSimplifiedPathPatternExpressionContext(
                                simplified_ctx,
                            ) = &*path_primary
                            && let Some(simplified_expr) =
                                simplified_ctx.simplifiedPathPatternExpression()
                        {
                            return Some(simplified_expr.get_text());
                        }
                    }
                    PathFactorContextAll::PfQuantifiedPathPrimaryContext(ctx) => {
                        if let Some(path_primary) = ctx.pathPrimary()
                            && let PathPrimaryContextAll::PpSimplifiedPathPatternExpressionContext(
                                simplified_ctx,
                            ) = &*path_primary
                            && let Some(simplified_expr) =
                                simplified_ctx.simplifiedPathPatternExpression()
                        {
                            return Some(simplified_expr.get_text());
                        }
                    }
                    PathFactorContextAll::PfQuestionedPathPrimaryContext(ctx) => {
                        if let Some(path_primary) = ctx.pathPrimary()
                            && let PathPrimaryContextAll::PpSimplifiedPathPatternExpressionContext(
                                simplified_ctx,
                            ) = &*path_primary
                            && let Some(simplified_expr) =
                                simplified_ctx.simplifiedPathPatternExpression()
                        {
                            return Some(simplified_expr.get_text());
                        }
                    }
                    _ => {}
                }
            }
        }

        None
    }

    /// Function to build a PathSearch from a PathSearchPrefixContext
    pub(super) fn build_path_search_from_prefix(
        prefix: &PathSearchPrefixContextAll,
    ) -> Option<PathSearch> {
        if prefix.allPathSearch().is_some() {
            return Some(PathSearch::All);
        }
        if prefix.anyPathSearch().is_some() {
            return Some(PathSearch::Any);
        }

        if let Some(shortest) = prefix.shortestPathSearch() {
            if shortest.allShortestPathSearch().is_some() {
                return Some(PathSearch::AllShortest);
            }
            if shortest.anyShortestPathSearch().is_some() {
                return Some(PathSearch::AnyShortest);
            }
            if let Some(counted) = shortest.countedShortestPathSearch() {
                let count = counted
                    .numberOfPaths()
                    .and_then(|n| n.nonNegativeIntegerSpecification())
                    .and_then(|n| n.unsignedInteger())
                    .and_then(|u| u.get_text().parse::<u32>().ok());
                return Some(PathSearch::CountedShortestPaths { count });
            }
            if let Some(grouped) = shortest.countedShortestGroupSearch() {
                let groups = grouped
                    .numberOfGroups()
                    .and_then(|n| n.nonNegativeIntegerSpecification())
                    .and_then(|n| n.unsignedInteger())
                    .and_then(|u| u.get_text().parse::<u32>().ok());
                return Some(PathSearch::CountedShortestGroups { groups });
            }
            return Some(PathSearch::Shortest);
        }

        None
    }

    /// Function to extract a pathTerm from a PathPatternContext, with validation that it is present and unique
    pub(super) fn build_quantifier_from_context(
        ctx: &GraphPatternQuantifierContextAll,
    ) -> GqlResult<PathQuantifier> {
        if ctx.ASTERISK().is_some() {
            return Ok(PathQuantifier::Range {
                min: 0,
                max: u32::MAX,
            });
        }
        if ctx.PLUS_SIGN().is_some() {
            return Ok(PathQuantifier::AtLeast(1));
        }
        if let Some(fixed) = ctx.fixedQuantifier() {
            let n = fixed
                .unsignedInteger()
                .ok_or_else(|| Self::parse_err("fixedQuantifier missing unsignedInteger".into()))?
                .get_text()
                .parse::<u32>()
                .map_err(|_| Self::parse_err("Invalid fixed quantifier".into()))?;
            return Ok(PathQuantifier::Exact(n));
        }
        if let Some(general) = ctx.generalQuantifier() {
            let min = general
                .lowerBound()
                .and_then(|l| l.unsignedInteger())
                .map(|i| i.get_text().parse::<u32>())
                .transpose()
                .map_err(|_| Self::parse_err("Invalid lower bound in quantifier".into()))?
                .unwrap_or(0);

            let max = general
                .upperBound()
                .and_then(|u| u.unsignedInteger())
                .map(|i| i.get_text().parse::<u32>())
                .transpose()
                .map_err(|_| Self::parse_err("Invalid upper bound in quantifier".into()))?;

            return Ok(match (min, max) {
                (0, None) => PathQuantifier::Range {
                    min: 0,
                    max: u32::MAX,
                },
                (0, Some(m)) => PathQuantifier::AtMost(m),
                (m, None) => PathQuantifier::AtLeast(m),
                (m, Some(n)) => PathQuantifier::Range { min: m, max: n },
            });
        }

        Self::parse_error("Unrecognized graphPatternQuantifier")
    }

    /// Function to build one or more PatternElement from a PathPrimaryContext.
    pub(super) fn build_elements_from_primary(
        path_primary: &PathPrimaryContextAll,
    ) -> GqlResult<Vec<PatternElement>> {
        match path_primary {
            PathPrimaryContextAll::PpElementPatternContext(ctx) => {
                let element_pattern = ctx.elementPattern().ok_or_else(|| {
                    Self::parse_err("CST not valid: elementPattern missing in pathPrimary".into())
                })?;

                if let Some(node_pattern) = element_pattern.nodePattern() {
                    Ok(vec![PatternElement::Node(Self::build_node_from_context(
                        &node_pattern,
                    )?)])
                } else if let Some(edge_pattern) = element_pattern.edgePattern() {
                    Ok(vec![PatternElement::Edge(Self::build_edge_from_context(
                        &edge_pattern,
                    )?)])
                } else {
                    Self::parse_error("CST not valid: elementPattern without node or edge")
                }
            }
            PathPrimaryContextAll::PpParenthesizedPathPatternExpressionContext(ctx) => {
                let parenthesized = ctx.parenthesizedPathPatternExpression().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: missing parenthesizedPathPatternExpression".into(),
                    )
                })?;
                let expression = parenthesized.pathPatternExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: missing pathPatternExpression".into())
                })?;

                let mut elements: Vec<PatternElement> = Vec::new();
                for path_term in Self::extract_path_terms_from_expression(&expression)? {
                    for path_factor in path_term.pathFactor_all() {
                        elements.extend(Self::build_elements_from_factor(&path_factor)?);
                    }
                }
                Ok(elements)
            }
            PathPrimaryContextAll::PpSimplifiedPathPatternExpressionContext(ctx) => {
                let simplified = ctx.simplifiedPathPatternExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: missing simplifiedPathPatternExpression".into())
                })?;
                Self::build_elements_from_simplified_path_context(&simplified)
            }
            _ => Self::parse_error("Unsupported pathPrimary variant"),
        }
    }

    /// Function to build one or more PatternElement from a SimplifiedPathPatternExpressionContext, handling the various forms of simplified path expressions and applying the appropriate default edge direction based on the specific variant of simplified expression. This function recursively processes the structure of the simplified path pattern to construct the corresponding PatternElement instances that represent the desugared path pattern in the AST.
    pub(super) fn build_elements_from_simplified_path_context(
        simplified: &SimplifiedPathPatternExpressionContextAll,
    ) -> GqlResult<Vec<PatternElement>> {
        let span = Self::extract_span(simplified);

        let (default_direction, contents) = if let Some(x) = simplified.simplifiedDefaultingLeft() {
            (EdgeDirection::Incoming, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingUndirected() {
            (EdgeDirection::Undirected, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingRight() {
            (EdgeDirection::Outgoing, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingLeftOrUndirected() {
            (EdgeDirection::Both, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingUndirectedOrRight() {
            (EdgeDirection::Both, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingLeftOrRight() {
            (EdgeDirection::Both, x.simplifiedContents())
        } else if let Some(x) = simplified.simplifiedDefaultingAnyDirection() {
            (EdgeDirection::Both, x.simplifiedContents())
        } else {
            return Self::parse_error("CST not valid: unknown simplified path defaulting");
        };

        let contents = contents
            .ok_or_else(|| Self::parse_err("CST not valid: simplifiedContents missing".into()))?;

        let atoms = Self::build_simplified_atoms_from_contents(&contents, default_direction)?;
        if atoms.is_empty() {
            return Self::parse_error("Empty simplified path pattern");
        }

        let mut elements = Vec::with_capacity(atoms.len() * 2 + 1);
        elements.push(PatternElement::Node(Node {
            id: None,
            labels: None,
            properties: None,
            where_clause: None,
            span: span.clone(),
        }));

        for atom in atoms {
            elements.push(PatternElement::Edge(Edge {
                id: None,
                labels: atom.labels,
                properties: None,
                where_clause: None,
                direction: atom.direction,
                quantifier: atom.quantifier,
                span: span.clone(),
            }));
            elements.push(PatternElement::Node(Node {
                id: None,
                labels: None,
                properties: None,
                where_clause: None,
                span: span.clone(),
            }));
        }

        Ok(elements)
    }

    /// Function to build a list of SimplifiedEdgeAtom from a SimplifiedContentsContext, handling the different possible structures of simplified contents (term, multiset alternation, path union) and applying the appropriate default edge direction to each atom based on the context in which it appears. This function recursively processes the structure of the simplified contents to construct the corresponding SimplifiedEdgeAtom instances that represent the desugared edge patterns in the AST.
    pub(super) fn build_simplified_atoms_from_contents(
        contents: &SimplifiedContentsContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        if let Some(term) = contents.simplifiedTerm() {
            return Self::build_simplified_atoms_from_term(&term, default_direction);
        }

        if let Some(multiset) = contents.simplifiedMultisetAlternation() {
            return Self::build_simplified_atoms_from_multiset_alternation(
                &multiset,
                default_direction,
            );
        }

        if let Some(union) = contents.simplifiedPathUnion() {
            return Self::build_simplified_atoms_from_path_union(&union, default_direction);
        }

        Self::parse_error("CST not valid: unsupported simplifiedContents variant")
    }

    /// Function to build a SimplifiedEdgeAtom from a SimplifiedFactorHighContext, determining the appropriate edge direction and quantifier based on the specific structure of the simplified factor (e.g., whether it is a tertiary expression, a quantified expression, or a questioned expression) and applying the default edge direction as needed. This function recursively processes the structure of the simplified factor to construct the corresponding SimplifiedEdgeAtom that represents the desugared edge pattern in the AST.
    pub(super) fn build_simplified_atom_from_factor_high(
        high: &SimplifiedFactorHighContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<SimplifiedEdgeAtom> {
        if let Some(tertiary) = high.simplifiedTertiary() {
            return Self::build_simplified_atom_from_tertiary(&tertiary, default_direction, None);
        }

        if let Some(q) = high.simplifiedQuantified() {
            let tertiary = q.simplifiedTertiary().ok_or_else(|| {
                Self::parse_err("CST not valid: simplifiedTertiary missing in quantified".into())
            })?;
            let quantifier = q
                .graphPatternQuantifier()
                .map(|ctx| Self::build_quantifier_from_context(&ctx))
                .transpose()?;
            return Self::build_simplified_atom_from_tertiary(
                &tertiary,
                default_direction,
                quantifier,
            );
        }

        if let Some(qm) = high.simplifiedQuestioned() {
            let tertiary = qm.simplifiedTertiary().ok_or_else(|| {
                Self::parse_err("CST not valid: simplifiedTertiary missing in questioned".into())
            })?;
            return Self::build_simplified_atom_from_tertiary(
                &tertiary,
                default_direction,
                Some(PathQuantifier::Optional),
            );
        }

        Self::parse_error("CST not valid: unsupported simplifiedFactorHigh variant")
    }

    /// Function to build a SimplifiedEdgeAtom from a SimplifiedTertiaryContext, determining the appropriate edge direction based on any direction overrides present in the context and applying the default edge direction as needed, as well as extracting any label expressions from the secondary context if present. This function constructs the corresponding SimplifiedEdgeAtom that represents the desugared edge pattern in the AST based on the structure of the simplified tertiary expression.
    pub(super) fn build_simplified_atom_from_tertiary(
        tertiary: &SimplifiedTertiaryContextAll,
        default_direction: EdgeDirection,
        quantifier: Option<PathQuantifier>,
    ) -> GqlResult<SimplifiedEdgeAtom> {
        let mut direction = default_direction;
        let mut labels: Option<Expression> = None;

        if let Some(override_ctx) = tertiary.simplifiedDirectionOverride() {
            direction = Self::direction_from_simplified_override(&override_ctx);
            if let Some(sec) = Self::secondary_from_override(&override_ctx) {
                labels = Self::label_expr_from_simplified_secondary(&sec);
            }
        } else if let Some(sec) = tertiary.simplifiedSecondary() {
            labels = Self::label_expr_from_simplified_secondary(&sec);
        }

        Ok(SimplifiedEdgeAtom {
            direction,
            labels,
            quantifier,
        })
    }

    /// Helper function to extract the secondary context from a simplified direction override context, which may contain additional label information that can be used to construct the labels for the corresponding edge atom in the AST.
    pub(super) fn direction_from_simplified_override(
        ctx: &SimplifiedDirectionOverrideContextAll,
    ) -> EdgeDirection {
        if ctx.simplifiedOverrideLeft().is_some() {
            EdgeDirection::Incoming
        } else if ctx.simplifiedOverrideUndirected().is_some() {
            EdgeDirection::Undirected
        } else if ctx.simplifiedOverrideRight().is_some() {
            EdgeDirection::Outgoing
        } else {
            EdgeDirection::Both
        }
    }

    /// Helper function to extract the secondary context from a simplified direction override context, which may contain additional label information that can be used to construct the labels for the corresponding edge atom in the AST.
    pub(super) fn label_expr_from_simplified_secondary(
        sec: &SimplifiedSecondaryContextAll,
    ) -> Option<Expression> {
        let span = Self::extract_span(sec);

        if let Some(primary) = sec.simplifiedPrimary() {
            return Self::label_expr_from_simplified_primary(&primary);
        }

        if let Some(neg) = sec.simplifiedNegation()
            && let Some(primary) = neg.simplifiedPrimary()
            && let Some(label_expr) = Self::label_expr_from_simplified_primary(&primary)
        {
            return Some(Expression::Unary(UnaryExpression {
                operator: Operator::Not,
                expression: Box::new(label_expr),
                span: span.clone(),
            }));
        }

        None
    }

    /// Helper function to extract a label expression from a simplified primary context, which may represent a simple label or a parenthesized expression that can be recursively processed to construct more complex label expressions for edge atoms in the AST.  
    pub(super) fn label_expr_from_simplified_primary(
        primary: &SimplifiedPrimaryContextAll,
    ) -> Option<Expression> {
        if let Some(label) = primary.labelName() {
            return Some(Expression::Literal(Literal::String(Self::identifier_text(
                &label.get_text(),
            ))));
        }

        if let Some(contents) = primary.simplifiedContents() {
            let atoms =
                Self::build_simplified_atoms_from_contents(&contents, EdgeDirection::Both).ok()?;

            if atoms.len() != 1 {
                return None;
            }

            return atoms.into_iter().next().and_then(|atom| atom.labels);
        }

        None
    }

    /// Function to build a list of SimplifiedEdgeAtom from a SimplifiedTermContext, handling both the base case of a simplified factor and the recursive case of a concatenation of terms, applying the default edge direction as needed and combining the resulting atoms into a single list that represents the desugared edge pattern in the AST.
    pub(super) fn build_simplified_atoms_from_term(
        term: &SimplifiedTermContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        match term {
            SimplifiedTermContextAll::SimplifiedFactorLowLabelContext(ctx) => {
                let low = ctx.simplifiedFactorLow().ok_or_else(|| {
                    Self::parse_err("CST not valid: simplifiedFactorLow missing".into())
                })?;
                Self::build_simplified_atoms_from_factor_low(&low, default_direction)
            }
            SimplifiedTermContextAll::SimplifiedConcatenationLabelContext(ctx) => {
                let left_term = ctx.simplifiedTerm().ok_or_else(|| {
                    Self::parse_err("CST not valid: simplifiedTerm missing in concatenation".into())
                })?;
                let right_low = ctx.simplifiedFactorLow().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: simplifiedFactorLow missing in concatenation".into(),
                    )
                })?;

                let mut left =
                    Self::build_simplified_atoms_from_term(&left_term, default_direction.clone())?;
                let mut right =
                    Self::build_simplified_atoms_from_factor_low(&right_low, default_direction)?;
                left.append(&mut right);
                Ok(left)
            }
            _ => Self::parse_error("CST not valid: unsupported simplifiedTerm variant"),
        }
    }

    pub(super) fn build_simplified_atoms_from_factor_low(
        low: &SimplifiedFactorLowContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        let span = Self::extract_span(low);

        match low {
            SimplifiedFactorLowContextAll::SimplifiedFactorHighLabelContext(ctx) => {
                let high = ctx.simplifiedFactorHigh().ok_or_else(|| {
                    Self::parse_err("CST not valid: simplifiedFactorHigh missing".into())
                })?;
                let atom = Self::build_simplified_atom_from_factor_high(&high, default_direction)?;
                Ok(vec![atom])
            }
            SimplifiedFactorLowContextAll::SimplifiedConjunctionLabelContext(ctx) => {
                let left = ctx.simplifiedFactorLow().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: simplifiedFactorLow missing in conjunction".into(),
                    )
                })?;
                let right = ctx.simplifiedFactorHigh().ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: simplifiedFactorHigh missing in conjunction".into(),
                    )
                })?;

                let mut left_atoms =
                    Self::build_simplified_atoms_from_factor_low(&left, default_direction.clone())?;
                if left_atoms.len() != 1 {
                    return Self::parse_error(
                        "Simplified conjunction currently supports a single edge atom on each side",
                    );
                }

                let mut left_atom = left_atoms.remove(0);
                let right_atom =
                    Self::build_simplified_atom_from_factor_high(&right, default_direction)?;

                if left_atom.direction != right_atom.direction
                    || left_atom.quantifier != right_atom.quantifier
                {
                    return Self::parse_error(
                        "Simplified conjunction requires homogeneous direction and quantifier",
                    );
                }

                left_atom.labels = Self::merge_conjunctive_labels(
                    left_atom.labels,
                    right_atom.labels,
                    span.clone(),
                );

                Ok(vec![left_atom])
            }
            _ => Self::parse_error("CST not valid: unsupported simplifiedFactorLow variant"),
        }
    }

    pub(super) fn merge_conjunctive_labels(
        left: Option<Expression>,
        right: Option<Expression>,
        span: Span,
    ) -> Option<Expression> {
        match (left, right) {
            (None, other) => other,
            (other, None) => other,
            (Some(l), Some(r)) => Some(Expression::Binary(BinaryExpression {
                left: Box::new(l),
                operator: Operator::And,
                right: Box::new(r),
                span: span.clone(),
            })),
        }
    }

    pub(super) fn build_simplified_atoms_from_multiset_alternation(
        multiset: &SimplifiedMultisetAlternationContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        let terms = multiset.simplifiedTerm_all();
        if terms.is_empty() {
            return Self::parse_error("CST not valid: empty simplified multiset alternation");
        }

        Self::reduce_simplified_alternative_terms(
            terms,
            default_direction,
            "Simplified multiset alternation",
        )
    }

    pub(super) fn build_simplified_atoms_from_path_union(
        union: &SimplifiedPathUnionContextAll,
        default_direction: EdgeDirection,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        let terms = union.simplifiedTerm_all();
        if terms.is_empty() {
            return Self::parse_error("CST not valid: empty simplified path union");
        }

        Self::reduce_simplified_alternative_terms(terms, default_direction, "Simplified path UNION")
    }

    pub(super) fn reduce_simplified_alternative_terms(
        terms: Vec<Rc<SimplifiedTermContextAll>>,
        default_direction: EdgeDirection,
        family_name: &str,
    ) -> GqlResult<Vec<SimplifiedEdgeAtom>> {
        let span = if let Some(first) = terms.first() {
            Self::extract_span(&**first)
        } else {
            Span::default()
        };

        let mut alternatives: Vec<Vec<SimplifiedEdgeAtom>> = terms
            .into_iter()
            .map(|term| Self::build_simplified_atoms_from_term(&term, default_direction.clone()))
            .collect::<GqlResult<Vec<_>>>()?;

        if alternatives.len() == 1 {
            return Ok(alternatives.remove(0));
        }

        let expected_hops = alternatives.first().map(|atoms| atoms.len()).unwrap_or(0);
        if alternatives
            .iter()
            .any(|atoms| atoms.len() != expected_hops)
        {
            return Self::parse_error(format!(
                "{} requires alternatives with equal hop length",
                family_name
            ));
        }

        let mut merged = alternatives.remove(0);

        for alternative in alternatives {
            for (idx, atom) in alternative.into_iter().enumerate() {
                let slot = merged.get_mut(idx).ok_or_else(|| {
                    Self::parse_err("CST not valid: alternative hop index out of bounds".into())
                })?;

                if atom.direction != slot.direction || atom.quantifier != slot.quantifier {
                    return Self::parse_error(format!(
                        "{} requires homogeneous direction and quantifier",
                        family_name
                    ));
                }

                slot.labels =
                    Self::merge_alternative_labels(slot.labels.clone(), atom.labels, span.clone());
            }
        }

        Ok(merged)
    }

    pub(super) fn merge_alternative_labels(
        left: Option<Expression>,
        right: Option<Expression>,
        span: Span,
    ) -> Option<Expression> {
        match (left, right) {
            (None, _) | (_, None) => None,
            (Some(l), Some(r)) => Some(Expression::Binary(BinaryExpression {
                left: Box::new(l),
                operator: Operator::Or,
                right: Box::new(r),
                span: span.clone(),
            })),
        }
    }

    pub(super) fn secondary_from_override<'input>(
        ctx: &SimplifiedDirectionOverrideContextAll<'input>,
    ) -> Option<Rc<SimplifiedSecondaryContextAll<'input>>> {
        if let Some(x) = ctx.simplifiedOverrideLeft() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideUndirected() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideRight() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideLeftOrUndirected() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideUndirectedOrRight() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideLeftOrRight() {
            return x.simplifiedSecondary();
        }
        if let Some(x) = ctx.simplifiedOverrideAnyDirection() {
            return x.simplifiedSecondary();
        }
        None
    }

    /// Function to extract all pathTerm occurrences from a PathPatternContext.
    pub(super) fn extract_path_terms<'input>(
        path_pattern: &PathPatternContextAll<'input>,
    ) -> GqlResult<Vec<Rc<PathTermContextAll<'input>>>> {
        let path_expression = path_pattern.pathPatternExpression().ok_or_else(|| {
            Self::parse_err("CST not valid: pathPatternExpression missing".into())
        })?;

        Self::extract_path_terms_from_expression(&path_expression)
    }

    pub(super) fn path_expression_flavor<'input>(
        path_expression: &PathPatternExpressionContextAll<'input>,
    ) -> PathExpressionFlavor {
        match path_expression {
            PathPatternExpressionContextAll::PpePathTermContext(_) => PathExpressionFlavor::Single,
            PathPatternExpressionContextAll::PpePatternUnionContext(_) => {
                PathExpressionFlavor::Union
            }
            PathPatternExpressionContextAll::PpeMultisetAlternationContext(_) => {
                PathExpressionFlavor::MultisetAlternation
            }
            _ => PathExpressionFlavor::Single,
        }
    }

    pub(super) fn extract_path_terms_from_expression<'input>(
        path_expression: &PathPatternExpressionContextAll<'input>,
    ) -> GqlResult<Vec<Rc<PathTermContextAll<'input>>>> {
        match path_expression {
            PathPatternExpressionContextAll::PpePathTermContext(context) => {
                context.pathTerm().map(|t| vec![t]).ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: pathTerm missing in pathPatternExpression".into(),
                    )
                })
            }
            PathPatternExpressionContextAll::PpePatternUnionContext(context) => {
                Ok(context.pathTerm_all())
            }
            PathPatternExpressionContextAll::PpeMultisetAlternationContext(context) => {
                Ok(context.pathTerm_all())
            }
            _ => Ok(Vec::new()),
        }
    }

    pub(super) fn build_elements_from_factor(
        path_factor: &PathFactorContextAll,
    ) -> GqlResult<Vec<PatternElement>> {
        match path_factor {
            PathFactorContextAll::PfPathPrimaryContext(ctx) => {
                let primary = ctx.pathPrimary().ok_or_else(|| {
                    Self::parse_err("CST not valid: pathPrimary missing in pathFactor".into())
                })?;
                Self::build_elements_from_primary(&primary)
            }
            PathFactorContextAll::PfQuantifiedPathPrimaryContext(ctx) => {
                let primary = ctx.pathPrimary().ok_or_else(|| {
                    Self::parse_err("CST not valid: pathPrimary missing in pathFactor".into())
                })?;
                let quantifier_ctx = ctx.graphPatternQuantifier().ok_or_else(|| {
                    Self::parse_err("CST not valid: graphPatternQuantifier missing".into())
                })?;

                let quantifier = Self::build_quantifier_from_context(&quantifier_ctx)?;
                let mut elements = Self::build_elements_from_primary(&primary)?;

                if let Some(PatternElement::Edge(edge)) = elements
                    .iter_mut()
                    .rev()
                    .find(|element| matches!(element, PatternElement::Edge(_)))
                {
                    edge.quantifier = Some(quantifier);
                }

                Ok(elements)
            }
            PathFactorContextAll::PfQuestionedPathPrimaryContext(ctx) => {
                let primary = ctx.pathPrimary().ok_or_else(|| {
                    Self::parse_err("CST not valid: pathPrimary missing in pathFactor".into())
                })?;
                let mut elements = Self::build_elements_from_primary(&primary)?;

                if let Some(PatternElement::Edge(edge)) = elements
                    .iter_mut()
                    .rev()
                    .find(|element| matches!(element, PatternElement::Edge(_)))
                {
                    edge.quantifier = Some(PathQuantifier::Optional);
                }

                Ok(elements)
            }
            _ => Self::parse_error("Unsupported pathFactor variant"),
        }
    }

    /// Function to build a Node from a NodePatternContext
    pub(super) fn build_node_from_context(node_pattern: &NodePatternContextAll) -> GqlResult<Node> {
        let span = Self::extract_span(node_pattern);

        let filler = node_pattern.elementPatternFiller().ok_or_else(|| {
            Self::parse_err("CST not valid: elementPatternFiller missing in nodePattern".into())
        })?;

        let id = filler
            .elementVariableDeclaration()
            .map(|variable_declaration| variable_declaration.get_text());

        let labels = filler
            .isLabelExpression()
            .and_then(|is_label_expr| is_label_expr.labelExpression())
            .map(|label_expr| Self::build_label_expression_from_context(&label_expr))
            .transpose()?;

        let properties = Self::build_properties_from_filler(&filler)?;

        let where_clause = filler
            .elementPatternPredicate()
            .and_then(|predicate| predicate.elementPatternWhereClause())
            .and_then(|where_ctx| where_ctx.searchCondition())
            .and_then(|search_cond| search_cond.booleanValueExpression())
            .and_then(|bool_expr| bool_expr.valueExpression())
            .map(|expr_ctx| Self::build_expression_from_context(&expr_ctx))
            .transpose()?
            .map(Box::new);

        Ok(Node {
            id,
            labels,
            properties,
            where_clause,
            span: span.clone(),
        })
    }

    /// Function to build an Edge from an EdgePatternContext
    pub(super) fn build_edge_from_context(edge_pattern: &EdgePatternContextAll) -> GqlResult<Edge> {
        let span = Self::extract_span(edge_pattern);

        let edge_text = edge_pattern.get_text();
        let direction = if edge_text.contains("<-") && edge_text.contains("->") {
            EdgeDirection::Both
        } else if edge_text.contains("->") {
            EdgeDirection::Outgoing
        } else if edge_text.contains("<-") {
            EdgeDirection::Incoming
        } else {
            EdgeDirection::Undirected
        };

        let filler = Self::extract_edge_pattern_filler(edge_pattern)?;
        let (id, labels, properties, where_clause) = if let Some(filler) = filler {
            let id = filler
                .elementVariableDeclaration()
                .map(|variable_declaration| variable_declaration.get_text());

            let labels = filler
                .isLabelExpression()
                .and_then(|is_label_expr| is_label_expr.labelExpression())
                .map(|label_expr| Self::build_label_expression_from_context(&label_expr))
                .transpose()?;

            let properties = Self::build_properties_from_filler(&filler)?;

            let where_clause = filler
                .elementPatternPredicate()
                .and_then(|predicate| predicate.elementPatternWhereClause())
                .and_then(|where_ctx| where_ctx.searchCondition())
                .and_then(|search_cond| search_cond.booleanValueExpression())
                .and_then(|bool_expr| bool_expr.valueExpression())
                .map(|expr_ctx| Self::build_expression_from_context(&expr_ctx))
                .transpose()?
                .map(Box::new);

            (id, labels, properties, where_clause)
        } else {
            (None, None, None, None)
        };

        Ok(Edge {
            id,
            labels,
            properties,
            direction,
            quantifier: None,
            where_clause,
            span: span.clone(),
        })
    }

    /// Helper function to build properties from a filler
    pub(super) fn build_properties_from_filler(
        filler: &ElementPatternFillerContextAll,
    ) -> GqlResult<Option<PropertyMap>> {
        filler
            .elementPatternPredicate()
            .and_then(|predicate| predicate.elementPropertySpecification())
            .map(|spec| Self::build_property_map_from_context(&spec))
            .transpose()
    }

    /// Function to build a property map from a context
    pub(super) fn build_property_map_from_context(
        spec: &ElementPropertySpecificationContextAll,
    ) -> GqlResult<PropertyMap> {
        let span = Self::extract_span(spec);

        let list = spec.propertyKeyValuePairList().ok_or_else(|| {
            Self::parse_err(
                "CST not valid: propertyKeyValuePairList missing in elementPropertySpecification"
                    .into(),
            )
        })?;

        let properties = list
            .propertyKeyValuePair_all()
            .into_iter()
            .map(|pair| {
                let key = pair
                    .propertyName()
                    .map(|name| Self::identifier_text(&name.get_text()))
                    .ok_or_else(|| {
                        Self::parse_err(
                            "CST non valido: propertyName mancante in propertyKeyValuePair".into(),
                        )
                    })?;

                let value_ctx = pair.valueExpression().ok_or_else(|| {
                    Self::parse_err(
                        "CST non valido: valueExpression mancante in propertyKeyValuePair".into(),
                    )
                })?;

                let value = Self::build_expression_from_context(&value_ctx)?;
                Ok(PropertyItem {
                    key,
                    value,
                    span: span.clone(),
                })
            })
            .collect::<GqlResult<Vec<_>>>()?;

        Ok(PropertyMap {
            properties,
            span: span.clone(),
        })
    }

    /// Function to extract an edge pattern filler from an edge pattern context
    pub(super) fn extract_edge_pattern_filler<'input>(
        edge_pattern: &EdgePatternContextAll<'input>,
    ) -> GqlResult<Option<Rc<ElementPatternFillerContextAll<'input>>>> {
        if let Some(full) = edge_pattern.fullEdgePattern() {
            return Self::extract_full_edge_pattern_filler(full).map(Some);
        }
        if edge_pattern.abbreviatedEdgePattern().is_some() {
            return Ok(None);
        }
        Self::parse_error(
            "CST not valid: edgePattern without fullEdgePattern or abbreviatedEdgePattern",
        )
    }

    /// Function to extract an edge pattern filler from a full edge pattern context, trying all possible edge types and validating that the filler is present
    pub(super) fn extract_full_edge_pattern_filler(
        full: Rc<FullEdgePatternContextAll>,
    ) -> GqlResult<Rc<ElementPatternFillerContextAll>> {
        full.fullEdgePointingLeft()
            .and_then(|ctx| ctx.elementPatternFiller())
            .or_else(|| {
                full.fullEdgeUndirected()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .or_else(|| {
                full.fullEdgePointingRight()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .or_else(|| {
                full.fullEdgeLeftOrUndirected()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .or_else(|| {
                full.fullEdgeUndirectedOrRight()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .or_else(|| {
                full.fullEdgeLeftOrRight()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .or_else(|| {
                full.fullEdgeAnyDirection()
                    .and_then(|ctx| ctx.elementPatternFiller())
            })
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: elementPatternFiller missing in fullEdgePattern".into(),
                )
            })
    }

    /// Function to build a label expression from a LabelExpressionContext, handling different types of label expressions (name, wildcard, negation, conjunction, disjunction, parentheses) and extracting the relevant components from the CST.
    pub(super) fn build_label_expression_from_context(
        label_expression: &LabelExpressionContextAll,
    ) -> GqlResult<Expression> {
        let span = Self::extract_span(label_expression);

        match label_expression {
            LabelExpressionContextAll::LabelExpressionNameContext(ctx) => {
                let label = ctx
                    .labelName()
                    .map(|name| Self::identifier_text(&name.get_text()))
                    .ok_or_else(|| Self::parse_err("CST not valid: labelName missing".into()))?;
                Ok(Expression::Literal(Literal::String(label)))
            }
            LabelExpressionContextAll::LabelExpressionWildcardContext(_) => {
                Ok(Expression::Literal(Literal::String("%".to_string())))
            }
            LabelExpressionContextAll::LabelExpressionNegationContext(ctx) => {
                let inner = ctx.labelExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: labelExpression missing in negation".into())
                })?;
                let expression = Self::build_label_expression_from_context(&inner)?;
                Ok(Expression::Unary(UnaryExpression {
                    operator: Operator::Not,
                    expression: Box::new(expression),
                    span: span.clone(),
                }))
            }
            LabelExpressionContextAll::LabelExpressionConjunctionContext(ctx) => {
                let left_ctx = ctx.labelExpression(0).ok_or_else(|| {
                    Self::parse_err("CST not valid: left operand missing in conjunction".into())
                })?;
                let right_ctx = ctx.labelExpression(1).ok_or_else(|| {
                    Self::parse_err("CST not valid: right operand missing in conjunction".into())
                })?;

                let left = Self::build_label_expression_from_context(&left_ctx)?;
                let right = Self::build_label_expression_from_context(&right_ctx)?;

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    operator: Operator::And,
                    right: Box::new(right),
                    span: span.clone(),
                }))
            }
            LabelExpressionContextAll::LabelExpressionDisjunctionContext(ctx) => {
                let left_ctx = ctx.labelExpression(0).ok_or_else(|| {
                    Self::parse_err("CST not valid: left operand missing in disjunction".into())
                })?;
                let right_ctx = ctx.labelExpression(1).ok_or_else(|| {
                    Self::parse_err("CST not valid: right operand missing in disjunction".into())
                })?;

                let left = Self::build_label_expression_from_context(&left_ctx)?;
                let right = Self::build_label_expression_from_context(&right_ctx)?;

                Ok(Expression::Binary(BinaryExpression {
                    left: Box::new(left),
                    operator: Operator::Or,
                    right: Box::new(right),
                    span: span.clone(),
                }))
            }
            LabelExpressionContextAll::LabelExpressionParenthesizedContext(ctx) => {
                let inner = ctx.labelExpression().ok_or_else(|| {
                    Self::parse_err("CST not valid: labelExpression missing in parentheses".into())
                })?;
                Self::build_label_expression_from_context(&inner)
            }
            _ => Self::parse_error("Type of labelExpression not supported"),
        }
    }

    /// Function to build a WhereClause from a GraphPatternWhereClauseContext
    pub(super) fn build_where_clause_from_context(
        graph_pattern_where_clause: &GraphPatternWhereClauseContextAll,
    ) -> GqlResult<WhereClause> {
        let span = Self::extract_span(graph_pattern_where_clause);

        let condition = graph_pattern_where_clause
            .searchCondition()
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: searchCondition missing in graphPatternWhereClause".into(),
                )
            })?
            .booleanValueExpression()
            .and_then(|expression| expression.valueExpression())
            .map(|expression| Self::build_expression_from_context(&expression))
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: booleanValueExpression missing in searchCondition".into(),
                )
            })??;

        Ok(WhereClause {
            condition,
            span: span.clone(),
        })
    }

    /// Function to build a FilterClause from a FilterStatementContext
    pub(super) fn build_filter_clause_from_context(
        filter: &FilterStatementContextAll,
    ) -> GqlResult<FilterClause> {
        let span = Self::extract_span(filter);

        if let Some(where_clause) = filter.whereClause() {
            let condition = where_clause
                .searchCondition()
                .ok_or_else(|| {
                    Self::parse_err("CST not valid: searchCondition missing in whereClause".into())
                })?
                .booleanValueExpression()
                .and_then(|expression| expression.valueExpression())
                .map(|expression| Self::build_expression_from_context(&expression))
                .ok_or_else(|| {
                    Self::parse_err(
                        "CST not valid: booleanValueExpression missing in whereClause".into(),
                    )
                })??;
            return Ok(FilterClause {
                condition,
                span: span.clone(),
            });
        }

        let condition = filter
            .searchCondition()
            .ok_or_else(|| {
                Self::parse_err("CST not valid: searchCondition missing in filterStatement".into())
            })?
            .booleanValueExpression()
            .and_then(|expression| expression.valueExpression())
            .map(|expression| Self::build_expression_from_context(&expression))
            .ok_or_else(|| {
                Self::parse_err(
                    "CST not valid: booleanValueExpression missing in searchCondition".into(),
                )
            })??;

        Ok(FilterClause {
            condition,
            span: span.clone(),
        })
    }

    /// Function to build a ForClause from a ForStatementContext, extracting the variable and expression components from the CST and constructing the corresponding AST node.
    pub(super) fn build_for_clause_from_context(
        for_stmt: &ForStatementContextAll,
    ) -> GqlResult<ForClause> {
        let span = Self::extract_span(for_stmt);

        let for_item: Rc<
            antlr_rust::parser_rule_context::BaseParserRuleContext<'_, ForItemContextExt<'_>>,
        > = for_stmt.forItem().ok_or_else(|| {
            Self::parse_err("CST not valid: forItem missing in forStatement".into())
        })?;

        let variable = for_item
            .forItemAlias()
            .and_then(|a| a.bindingVariable())
            .map(|v| v.get_text().to_string())
            .ok_or_else(|| {
                Self::parse_err("CST not valid: bindingVariable missing in forItemAlias".into())
            })?;

        let expression = for_item
            .forItemSource()
            .and_then(|s| s.valueExpression())
            .map(|e| Self::build_expression_from_context(&e))
            .ok_or_else(|| {
                Self::parse_err("CST not valid: valueExpression missing in forItemSource".into())
            })??;

        let ordinality_or_offset_variable = for_stmt
            .forOrdinalityOrOffset()
            .and_then(|ctx| ctx.bindingVariable())
            .map(|ctx| ctx.get_text());

        Ok(ForClause {
            variable,
            expression,
            ordinality_or_offset_variable,
            span: span.clone(),
        })
    }
}
