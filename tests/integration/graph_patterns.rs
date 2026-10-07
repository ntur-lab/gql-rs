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

use gql_parser::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};
use gql_parser::ast::model::*;

mod graph_pattern_tests {
    use super::*;









        #[test]
        fn test_path_quantifier_exact_maps_to_exact_quantifier() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->{2}(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::Exact(2))));
        }

        #[test]
        fn test_path_quantifier_at_least_maps_to_at_least_quantifier() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->{2,}(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::AtLeast(2))));
        }

        #[test]
        fn test_path_quantifier_at_most_maps_to_at_most_quantifier() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->{,5}(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::AtMost(5))));
        }

        #[test]
        fn test_path_quantifier_plus_maps_to_at_least_one() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->+(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::AtLeast(1))));
        }

        #[test]
        fn test_path_quantifier_star_maps_to_unbounded_range() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->*(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(
                edge.quantifier,
                Some(PathQuantifier::Range {
                    min: 0,
                    max: u32::MAX
                })
            ));
        }

        #[test]
        fn test_path_quantifier_bounded_range_maps_to_range_quantifier() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH (a)-[e]->{2,5}(b) RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(
                edge.quantifier,
                Some(PathQuantifier::Range { min: 2, max: 5 })
            ));
        }

        #[test]
        fn test_parenthesized_path_with_optional_quantifier_maps_to_optional_edge() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH ((a)-[e]->(b))? RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::Optional)));
        }

        #[test]
        fn test_parenthesized_path_with_plus_quantifier_maps_to_at_least_one() {
            let mut analyzer = DefaultGQLQueryAnalyzer::new();
            analyzer
                .exec("MATCH ((a)-[e]->(b))+ RETURN a")
                .expect("query should parse");

            let ast = analyzer.get_ast().expect("ast must be available");
            let Statement::Query(Query::Basic(query)) = &ast.statement else {
                panic!("expected basic query statement");
            };

            let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
                panic!("expected MATCH as first pipeline step");
            };

            let pattern = match_clause
                .match_clause
                .patterns
                .first()
                .expect("pattern should exist");

            let edge = pattern
                .elements
                .iter()
                .find_map(|element| match element {
                    PatternElement::Edge(edge) => Some(edge),
                    _ => None,
                })
                .expect("edge should exist");

            assert!(matches!(edge.quantifier, Some(PathQuantifier::AtLeast(1))));
        }

    #[test]
    fn test_path_mode_search_incompatibility_walk_shortest() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH ALL SHORTEST WALK PATHS (n)-[e]->(m) RETURN n");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("WALK is not compatible with SHORTEST"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_path_mode_cycle_constraint_acyclic_self_loop() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH ACYCLIC (n)-[e]->(n) RETURN n");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("does not allow immediate self-loop"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_path_quantifier_rejects_invalid_range_bounds() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH (n)-[]->{5,2}(m) RETURN n, m");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("lower bound 5 is greater than upper bound 2"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_path_quantifier_rejects_open_range_in_simple_mode() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        let result = analyzer.exec("MATCH SIMPLE (n)-[]->{1,}(m) RETURN n, m");

        match result {
            Err(GqlError::Validation { message: msg, .. }) => {
                assert!(
                    msg.contains("SIMPLE/ACYCLIC path modes do not allow open quantifiers"),
                    "Unexpected validation error: {}",
                    msg
                );
            }
            other => panic!("Expected validation error, got: {:?}", other),
        }
    }

    #[test]
    fn test_simplified_negated_label_maps_to_unary_not_expression() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH -/ !Person /-> RETURN 1")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let edge_labels = pattern
            .elements
            .iter()
            .find_map(|element| match element {
                PatternElement::Edge(edge) => edge.labels.as_ref(),
                _ => None,
            })
            .expect("edge labels should exist");

        let Expression::Unary(unary) = edge_labels else {
            panic!("expected unary expression for negated label");
        };
        assert_eq!(unary.operator, Operator::Not);

        let Expression::Literal(Literal::String(label)) = unary.expression.as_ref() else {
            panic!("expected string literal inside negated label expression");
        };
        assert_eq!(label, "Person");
    }

    #[test]
    fn test_simplified_conjunction_and_negation_map_to_binary_and_expression() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH -/ Person & !Employee /-> RETURN 1")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let edge_labels = pattern
            .elements
            .iter()
            .find_map(|element| match element {
                PatternElement::Edge(edge) => edge.labels.as_ref(),
                _ => None,
            })
            .expect("edge labels should exist");

        let Expression::Binary(binary) = edge_labels else {
            panic!("expected binary expression for conjunction");
        };
        assert_eq!(binary.operator, Operator::And);

        let Expression::Literal(Literal::String(left_label)) = binary.left.as_ref() else {
            panic!("expected string literal on left side");
        };
        assert_eq!(left_label, "Person");

        let Expression::Unary(unary_right) = binary.right.as_ref() else {
            panic!("expected unary NOT on right side");
        };
        assert_eq!(unary_right.operator, Operator::Not);

        let Expression::Literal(Literal::String(right_label)) = unary_right.expression.as_ref() else {
            panic!("expected string literal inside right-side negation");
        };
        assert_eq!(right_label, "Employee");
    }

    #[test]
    fn test_simplified_parenthesized_union_maps_to_binary_or_expression() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH -/ (Person|Employee) /-> RETURN 1")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let edge_labels = pattern
            .elements
            .iter()
            .find_map(|element| match element {
                PatternElement::Edge(edge) => edge.labels.as_ref(),
                _ => None,
            })
            .expect("edge labels should exist");

        let Expression::Binary(binary) = edge_labels else {
            panic!("expected binary OR expression for parenthesized union");
        };
        assert_eq!(binary.operator, Operator::Or);

        let Expression::Literal(Literal::String(left_label)) = binary.left.as_ref() else {
            panic!("expected literal on OR left side");
        };
        assert_eq!(left_label, "Person");

        let Expression::Literal(Literal::String(right_label)) = binary.right.as_ref() else {
            panic!("expected literal on OR right side");
        };
        assert_eq!(right_label, "Employee");
    }

    #[test]
    fn test_simplified_multihop_union_merges_labels_per_hop() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH -/ Person Employee|Customer Supplier /-> RETURN 1")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let edges = pattern
            .elements
            .iter()
            .filter_map(|element| match element {
                PatternElement::Edge(edge) => Some(edge),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(edges.len(), 2, "expected two edges for a two-hop simplified pattern");

        let first_labels = edges[0].labels.as_ref().expect("first edge labels should exist");
        let Expression::Binary(first_or) = first_labels else {
            panic!("expected OR labels on first edge");
        };
        assert_eq!(first_or.operator, Operator::Or);

        let Expression::Literal(Literal::String(first_left)) = first_or.left.as_ref() else {
            panic!("expected literal on first OR left");
        };
        assert_eq!(first_left, "Person");

        let Expression::Literal(Literal::String(first_right)) = first_or.right.as_ref() else {
            panic!("expected literal on first OR right");
        };
        assert_eq!(first_right, "Customer");

        let second_labels = edges[1].labels.as_ref().expect("second edge labels should exist");
        let Expression::Binary(second_or) = second_labels else {
            panic!("expected OR labels on second edge");
        };
        assert_eq!(second_or.operator, Operator::Or);

        let Expression::Literal(Literal::String(second_left)) = second_or.left.as_ref() else {
            panic!("expected literal on second OR left");
        };
        assert_eq!(second_left, "Employee");

        let Expression::Literal(Literal::String(second_right)) = second_or.right.as_ref() else {
            panic!("expected literal on second OR right");
        };
        assert_eq!(second_right, "Supplier");
    }

    #[test]
    fn test_exists_optional_block_preserves_all_nested_matches() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (n) WHERE EXISTS { OPTIONAL { MATCH (a:Person) MATCH (b:Movie) } } RETURN n")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let filter_clause = query
            .pipeline
            .iter()
            .find_map(|step| match step {
                QueryStep::Filter(filter) => Some(filter),
                _ => None,
            })
            .expect("expected WHERE/FILTER step");

        let Expression::Subquery(subquery_expr) = &filter_clause.condition else {
            panic!("expected EXISTS to map to subquery expression");
        };
        assert!(subquery_expr.is_exists, "EXISTS subquery flag should be true");

        let Query::Basic(exists_query) = subquery_expr.query.as_ref() else {
            panic!("expected EXISTS subquery to be a basic query");
        };

        let nested_matches = exists_query
            .pipeline
            .iter()
            .filter_map(|step| match step {
                QueryStep::Match(m) => Some(m),
                _ => None,
            })
            .collect::<Vec<_>>();

        assert_eq!(
            nested_matches.len(),
            2,
            "expected both nested MATCH statements to be preserved"
        );
        assert!(
            nested_matches.iter().all(|m| m.optional),
            "nested matches from OPTIONAL block must be marked optional"
        );
    }

    #[test]
    fn test_path_pattern_union_sets_alternation_metadata() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (a)-[e1]->(b) | (c)-[e2]->(d) RETURN a")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let alternation = pattern
            .alternation
            .as_ref()
            .expect("expected alternation metadata");

        assert_eq!(alternation.operator, PathAlternationOperator::Union);
        assert_eq!(alternation.alternatives.len(), 2);
    }

    #[test]
    fn test_path_pattern_multiset_sets_alternation_metadata() {
        let mut analyzer = DefaultGQLQueryAnalyzer::new();
        analyzer
            .exec("MATCH (a)-[e1]->(b) |+| (c)-[e2]->(d) RETURN a")
            .expect("query should parse");

        let ast = analyzer.get_ast().expect("ast must be available");
        let Statement::Query(Query::Basic(query)) = &ast.statement else {
            panic!("expected basic query statement");
        };

        let Some(QueryStep::Match(match_clause)) = query.pipeline.first() else {
            panic!("expected MATCH as first pipeline step");
        };

        let pattern = match_clause
            .match_clause
            .patterns
            .first()
            .expect("pattern should exist");

        let alternation = pattern
            .alternation
            .as_ref()
            .expect("expected alternation metadata");

        assert_eq!(
            alternation.operator,
            PathAlternationOperator::MultisetAlternation
        );
        assert_eq!(alternation.alternatives.len(), 2);
    }
}
