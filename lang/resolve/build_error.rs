// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad project

use crate::{parse::*, resolve::*};

use thiserror::*;

/// Resolve error
#[derive(Debug, Error)]
pub enum BuildError {
    /// Error while parsing a file
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),

    /// Error while resolving symbols
    #[error("Resolve Error: {0}")]
    ResolveError(#[from] ResolveError),

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

/// Result type of a build
pub type BuildResult<T> = std::result::Result<T, BuildError>;
