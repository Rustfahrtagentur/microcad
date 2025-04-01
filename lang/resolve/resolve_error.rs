// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use crate::syntax::*;
use thiserror::Error;

/// Resolve error
#[derive(Debug, Error)]
pub enum ResolveError {
    /// Custom evaluation error
    #[error("{0}")]
    ExternalSymbolNotFound(QualifiedName),
}

/// Result type of resolve
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
