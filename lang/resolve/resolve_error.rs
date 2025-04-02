// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use crate::syntax::*;
use thiserror::Error;

/// Resolve error
#[derive(Debug, Error)]
pub enum ResolveError {
    /// Name of external symbol is unknown
    #[error("External symbol {0} not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown
    #[error("External path '{0}' not found")]
    ExternalPathNotFound(std::path::PathBuf),
}

/// Result type of resolve
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
