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


/// This module exposes the AST model and the internal CST-to-AST and validation layers.
///
/// The public entry point is `QueryAST`, while the implementation is split into:
/// - `model/` for the typed AST structures;
/// - `builder/` for CST-to-AST conversion;
/// - `validator/` for semantic validation.
pub mod model;

// Internal crate modules.
// Internal CST -> AST builder layer.
pub(crate) mod builder;
// Internal semantic validation layer.
pub(crate) mod validator;


pub use model::*;