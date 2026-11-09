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

use super::gqllexer::GQLLexer;
use super::gqllistener::GQLListener;
use super::gqlparser::{GQLParser, GQLParserContext, GQLParserContextType, ruleNames};
use antlr_rust::common_token_stream::CommonTokenStream;
use antlr_rust::tree::{ParseTreeListener, ParseTreeWalker};
use antlr_rust::{InputStream, Parser};
use cucumber::gherkin::{Feature, GherkinEnv};
use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Default)]
struct RuleCoverage {
    covered: BTreeSet<usize>,
}

impl<'input> ParseTreeListener<'input, GQLParserContextType> for RuleCoverage {
    fn enter_every_rule(&mut self, context: &dyn GQLParserContext<'input>) {
        let rule_index = context.get_rule_index();
        if rule_index < ruleNames.len() {
            self.covered.insert(rule_index);
        }
    }
}

impl<'input> GQLListener<'input> for RuleCoverage {}

#[test]
fn tck_covers_all_reachable_parser_rules() {
    let feature_root = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/tck/features");
    let mut feature_paths = Vec::new();
    collect_feature_paths(&feature_root, &mut feature_paths);
    feature_paths.sort();

    let mut coverage = RuleCoverage::default();
    let mut query_count = 0;

    for path in feature_paths {
        let feature = Feature::parse_path(&path, GherkinEnv::default())
            .unwrap_or_else(|error| panic!("cannot parse {}: {error}", path.display()));
        for scenario in feature
            .scenarios
            .iter()
            .chain(feature.rules.iter().flat_map(|rule| rule.scenarios.iter()))
        {
            for query in scenario
                .steps
                .iter()
                .filter_map(|step| step.docstring.as_deref())
            {
                query_count += 1;
                let input = InputStream::new(query.trim());
                let lexer = GQLLexer::new(input);
                let token_stream = CommonTokenStream::new(lexer);
                let mut parser = GQLParser::new(token_stream);
                parser.remove_error_listeners();
                let tree = parser
                    .gqlProgram()
                    .unwrap_or_else(|error| panic!("cannot parse TCK query: {error:?}\n{query}"));
                coverage = *ParseTreeWalker::<GQLParserContextType, dyn GQLListener<'_>>::walk(
                    Box::new(coverage),
                    tree.as_ref(),
                );
            }
        }
    }

    let missing = ruleNames
        .iter()
        .enumerate()
        .filter_map(|(index, name)| (!coverage.covered.contains(&index)).then_some(*name))
        .collect::<Vec<_>>();

    let structurally_unreachable = [
        "nestedDataModifyingProcedureSpecification",
        "focusedNestedDataModifyingProcedureSpecification",
        "callDataModifyingProcedureStatement",
        "pathVariableReference",
        "listValueConstructor",
        "durationValueExpression",
    ];

    eprintln!(
        "TCK parser-rule coverage: {}/{} rules across {} queries",
        coverage.covered.len(),
        ruleNames.len(),
        query_count
    );
    eprintln!(
        "Uncovered parser rules ({}): {}",
        missing.len(),
        missing.join(", ")
    );

    assert_eq!(
        missing, structurally_unreachable,
        "the set of uncovered parser rules changed; update the TCK or justify the grammar rule as structurally unreachable"
    );
}

fn collect_feature_paths(directory: &Path, paths: &mut Vec<PathBuf>) {
    let entries = fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("cannot read {}: {error}", directory.display()));

    for entry in entries {
        let path = entry.expect("cannot read TCK directory entry").path();
        if path.is_dir() {
            collect_feature_paths(&path, paths);
        } else if path.extension().and_then(|extension| extension.to_str()) == Some("feature") {
            paths.push(path);
        }
    }
}
