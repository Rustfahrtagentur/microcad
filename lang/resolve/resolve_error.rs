// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use thiserror::Error;

use crate::{parse::ParseError, syntax::*};

/// Resolve error.
#[derive(Debug, Error)]
pub enum ResolveError {
    /// To do
    #[error("Not implemented: {0}")]
    Todo(String),

    /// Parse Error.
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),

    /// Can't find a project file by hash.
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Hash is zero
    #[error("Hash is zero")]
    NulHash,

    /// Name of external symbol is unknown.
    #[error("External symbol `{0}` not found")]
    ExternalSymbolNotFound(QualifiedName),

    /// Path of external file is unknown.
    #[error("External path `{0}` not found")]
    ExternalPathNotFound(std::path::PathBuf),

    /// Can't find a project file by it's path.
    #[error("Could not find a file with path {0}")]
    FileNotFound(std::path::PathBuf),

    /// Symbol not found.
    #[error("Symbol {0} not found.")]
    SymbolNotFound(QualifiedName),

    /// Symbol not found (retry to load from external).
    #[error("Symbol {0} must be loaded from {1}")]
    SymbolMustBeLoaded(QualifiedName, std::path::PathBuf),
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
