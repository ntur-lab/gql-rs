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

use crate::error_handler::error::GqlError;
use antlr_rust::error_listener::ErrorListener;
use antlr_rust::errors::ANTLRError;
use antlr_rust::recognizer::Recognizer;
use std::cell::RefCell;
use std::rc::Rc;

/// Custom error listener for capturing syntax errors during ANTLR parsing of GQL queries.
pub struct GqlErrorListener {
    pub errors: Rc<RefCell<Vec<GqlError>>>,
}

impl GqlErrorListener {
    pub fn new() -> Self {
        Self {
            errors: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

impl<'input, T: Recognizer<'input>> ErrorListener<'input, T> for GqlErrorListener {
    fn syntax_error(
        &self,
        _recognizer: &T,
        _offending_symbol: Option<
            &<T::TF as antlr_rust::token_factory::TokenFactory<'input>>::Inner,
        >,
        line: isize,
        column: isize,
        msg: &str,
        _error: Option<&ANTLRError>,
    ) {
        self.errors.borrow_mut().push(GqlError::parse(format!(
            "Syntax error at line {}, column {}: {}",
            line, column, msg
        )));
    }
}
