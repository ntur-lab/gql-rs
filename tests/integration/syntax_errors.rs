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

use gql_parser::error_handler::error::GqlError;
use gql_parser::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer};

#[test]
fn analyzer_surfaces_syntax_errors_through_the_error_listener() {
	let mut analyzer = DefaultGQLQueryAnalyzer::new();

	let result = analyzer.exec("MATCH (");

	match result {
		Err(GqlError::Parse { message, context }) => {
			assert!(message.contains("Syntax error at line 1, column 7:"));
			assert_eq!(context.span, None);
			assert_eq!(context.code, None);
		}
		other => panic!("Expected parse error from syntax failure, got: {:?}", other),
	}

	assert_eq!(analyzer.get_query(), None);
	assert!(analyzer.get_ast().is_none());
}
