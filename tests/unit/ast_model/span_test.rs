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

use gql_parser::ast::Span;

#[test]
fn span_default_is_the_unknown_source_position() {
    let span = Span::default();

    assert_eq!(span.start_line, 0);
    assert_eq!(span.start_col, 0);
    assert_eq!(span.end_line, 0);
    assert_eq!(span.end_col, 0);
}

#[test]
fn span_json_round_trip_preserves_every_coordinate() {
    let span = Span {
        start_line: 3,
        start_col: 5,
        end_line: 7,
        end_col: 11,
    };

    let json = serde_json::to_value(&span).expect("span should serialize");
    let decoded: Span = serde_json::from_value(json).expect("span should deserialize");

    assert_eq!(decoded, span);
}

