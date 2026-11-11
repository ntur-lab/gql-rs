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

#[macro_use]
#[path = "support.rs"]
mod support;

#[path = "catalog_and_session_it.rs"]
mod catalog_and_session_it;
#[path = "expressions_it.rs"]
mod expressions_it;
#[path = "graph_patterns_it.rs"]
mod graph_patterns_it;
#[path = "query_composition_it.rs"]
mod query_composition_it;
#[path = "set_operations_it.rs"]
mod set_operations_it;
#[path = "stateless_api_it.rs"]
mod stateless_api;
#[path = "string_literal_it.rs"]
mod string_literal_it;
#[path = "syntax_errors_it.rs"]
mod syntax_errors_it;
#[path = "transactions_it.rs"]
mod transactions_it;
