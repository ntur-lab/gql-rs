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

use crate::ast::Span;
use crate::error_handler::error::{ErrorContext, GqlError};

#[test]
fn gql_error_constructors_and_display_cover_all_context_shapes() {
    let parse_error = GqlError::parse("parse failed");
    assert_eq!(parse_error.message(), "parse failed");
    assert_eq!(
        parse_error.context(),
        &ErrorContext {
            span: None,
            code: None,
        }
    );
    assert_eq!(format!("{}", parse_error), "Parse error: parse failed");

    let parse_span = Span {
        start_line: 1,
        start_col: 2,
        end_line: 1,
        end_col: 8,
    };
    let parse_at_error = GqlError::parse_at("parse at span", parse_span.clone());
    assert_eq!(parse_at_error.message(), "parse at span");
    assert_eq!(
        parse_at_error.context(),
        &ErrorContext {
            span: Some(parse_span.clone()),
            code: None,
        }
    );
    assert_eq!(
        format!("{}", parse_at_error),
        "Parse error: parse at span [line 1:2 to 1:8]"
    );

    let validation_error = GqlError::validation("validation failed");
    assert_eq!(validation_error.message(), "validation failed");
    assert_eq!(
        validation_error.context(),
        &ErrorContext {
            span: Some(Span::default()),
            code: None,
        }
    );
    assert_eq!(
        format!("{}", validation_error),
        "Validation error: validation failed"
    );

    let validation_span = Span {
        start_line: 3,
        start_col: 4,
        end_line: 3,
        end_col: 10,
    };
    let validation_at_error =
        GqlError::validation_at("validation at span", validation_span.clone());
    assert_eq!(validation_at_error.message(), "validation at span");
    assert_eq!(
        validation_at_error.context(),
        &ErrorContext {
            span: Some(validation_span.clone()),
            code: None,
        }
    );
    assert_eq!(
        format!("{}", validation_at_error),
        "Validation error: validation at span [line 3:4 to 3:10]"
    );

    let validation_with_code = GqlError::validation_with_code("validation coded", "E_VALID");
    assert_eq!(validation_with_code.message(), "validation coded");
    assert_eq!(
        validation_with_code.context(),
        &ErrorContext {
            span: Some(Span::default()),
            code: Some("E_VALID".to_string()),
        }
    );
    assert_eq!(
        format!("{}", validation_with_code),
        "Validation error: validation coded [code=E_VALID]"
    );

    let generic_error = GqlError::generic("generic failed");
    assert_eq!(generic_error.message(), "generic failed");
    assert_eq!(
        generic_error.context(),
        &ErrorContext {
            span: None,
            code: None,
        }
    );
    assert_eq!(format!("{}", generic_error), "Error: generic failed");
}
