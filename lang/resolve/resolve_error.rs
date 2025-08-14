// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use thiserror::Error;

use crate::parse::ParseError;

/// Resolve error.
#[derive(Debug, Error)]
pub enum ResolveError {
    /// To do
    #[error("Not implemented: {0}")]
    Todo(String),

    /// Parse Error.
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
