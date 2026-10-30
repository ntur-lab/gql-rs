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

//! Structured error values returned by `gql-rs`.
//!
//! Errors can be displayed directly for users or inspected by matching on
//! [`crate::GqlError`].
//!
//! # Example
//!
//! ```rust
//! use gql_rs::{DefaultGQLQueryAnalyzer, GQLQueryAnalyzer, GqlError};
//!
//! let mut analyzer = DefaultGQLQueryAnalyzer::new();
//!
//! match analyzer.exec("MATCH (") {
//!     Err(GqlError::Parse { message, context }) => {
//!         eprintln!("{message}");
//!         eprintln!("source span: {:?}", context.span);
//!     }
//!     Err(other) => eprintln!("{other}"),
//!     Ok(_) => unreachable!(),
//! }
//! ```
//!
use crate::ast::model::Span;
use std::fmt;

/// Extra metadata attached to parser/validator errors.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct ErrorContext {
    /// Source range associated with the diagnostic, when available.
    pub span: Option<Span>,
    /// Optional machine-readable diagnostic code.
    pub code: Option<String>,
}

/// Represents an error that can occur during parsing and semantic analysis.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GqlError {
    /// Error produced by syntax/CST parsing.
    Parse {
        /// Human-readable description of the parsing failure.
        message: String,
        /// Structured diagnostic context associated with the failure.
        context: ErrorContext,
    },
    /// Error produced by semantic validation.
    Validation {
        /// Human-readable description of the validation failure.
        message: String,
        /// Structured diagnostic context associated with the failure.
        context: ErrorContext,
    },
    /// Generic fallback error.
    Generic {
        /// Human-readable description of the failure.
        message: String,
        /// Structured diagnostic context associated with the failure.
        context: ErrorContext,
    },
}

impl GqlError {
    /// Creates a new parse error with the given message and optional span.
    pub fn parse(message: impl Into<String>) -> Self {
        Self::Parse {
            message: message.into(),
            context: ErrorContext::default(),
        }
    }

    /// Creates a new parse error with the given message and span.
    pub fn parse_at(message: impl Into<String>, span: Span) -> Self {
        Self::Parse {
            message: message.into(),
            context: ErrorContext {
                span: Some(span),
                code: None,
            },
        }
    }

    /// Creates a new validation error with the given message and optional span.
    pub fn validation(message: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            context: ErrorContext {
                span: Some(Span::default()),
                code: None,
            },
        }
    }

    /// Creates a new validation error with the given message and span.
    pub fn validation_at(message: impl Into<String>, span: Span) -> Self {
        Self::Validation {
            message: message.into(),
            context: ErrorContext {
                span: Some(span),
                code: None,
            },
        }
    }

    /// Creates a new validation error with the given message, span, and code.
    pub fn validation_with_code(message: impl Into<String>, code: impl Into<String>) -> Self {
        Self::Validation {
            message: message.into(),
            context: ErrorContext {
                span: Some(Span::default()),
                code: Some(code.into()),
            },
        }
    }

    /// Creates a new generic error with the given message.
    pub fn generic(message: impl Into<String>) -> Self {
        Self::Generic {
            message: message.into(),
            context: ErrorContext::default(),
        }
    }

    /// Getter methods for error message and context fields for easier access in handling code.
    pub fn message(&self) -> &str {
        match self {
            GqlError::Parse { message, .. }
            | GqlError::Validation { message, .. }
            | GqlError::Generic { message, .. } => message,
        }
    }

    /// Getter for error context, which may contain span and code information.
    pub fn context(&self) -> &ErrorContext {
        match self {
            GqlError::Parse { context, .. }
            | GqlError::Validation { context, .. }
            | GqlError::Generic { context, .. } => context,
        }
    }

    // Helper to render error context (span, code) in a consistent format for Display.
    fn render_context(context: &ErrorContext) -> String {
        let mut parts: Vec<String> = Vec::new();

        if let Some(span) = &context.span {
            let has_real_position = span.start_line != 0
                || span.start_col != 0
                || span.end_line != 0
                || span.end_col != 0;
            if has_real_position {
            parts.push(format!(
                "line {}:{} to {}:{}",
                span.start_line, span.start_col, span.end_line, span.end_col
            ));
            }
        }

        if let Some(code) = &context.code {
            parts.push(format!("code={}", code));
        }

        if parts.is_empty() {
            String::new()
        } else {
            format!(" [{}]", parts.join(", "))
        }
    }
}

impl fmt::Display for GqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GqlError::Parse { message, context } => {
                write!(f, "Parse error: {}{}", message, Self::render_context(context))
            }
            GqlError::Validation { message, context } => {
                write!(
                    f,
                    "Validation error: {}{}",
                    message,
                    Self::render_context(context)
                )
            }
            GqlError::Generic { message, context } => {
                write!(f, "Error: {}{}", message, Self::render_context(context))
            }
        }
    }
}

impl std::error::Error for GqlError {}
