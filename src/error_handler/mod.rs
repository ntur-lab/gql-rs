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

//! Errors produced while parsing, building, and validating GQL programs.
//!
//! [`crate::GqlError`] distinguishes three failure categories:
//!
//! - parsing and syntax errors;
//! - semantic validation errors;
//! - generic crate-level errors.
//!
//! Every error contains a human-readable message and an
//! [`ErrorContext`](crate::error_handler::error::ErrorContext). Context can
//! include a source span and a diagnostic code when that information is
//! available.

/// Structured error and diagnostic context types.
pub mod error;

// Internal crate modules.
pub(crate) mod error_listener;

#[cfg(test)]
mod tests;
