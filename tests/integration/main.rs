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

#[macro_use]
#[path = "support.rs"]
mod support;

#[path = "catalog_and_session.rs"]
mod catalog_and_session;
#[path = "expressions.rs"]
mod expressions;
#[path = "graph_patterns.rs"]
mod graph_patterns;
#[path = "query_composition.rs"]
mod query_composition;
#[path = "set_operations.rs"]
mod set_operations;
#[path = "string_literal.rs"]
mod string_literal;
#[path = "syntax_errors.rs"]
mod syntax_errors;
#[path = "transactions.rs"]
mod transactions;
