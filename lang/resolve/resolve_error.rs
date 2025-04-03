// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use crate::{parse::*, syntax::*};
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

    /// Error while parsing a file
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),

    /// Can't find a project file by hash
    #[error("Could not find a file with hash {0}")]
    UnknownHash(u64),

    /// Can't find a project file by it's path
    #[error("Could not find a file with path {0}")]
    UnknownPath(std::path::PathBuf),

    /// Can't find a project file by it's qualified name
    #[error("Could not find a file with name {0}")]
    UnknownName(QualifiedName),
}

/// Result type of resolve
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
