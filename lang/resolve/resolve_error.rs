// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Resolve error

use thiserror::Error;

use crate::{diag::*, parse::*, src_ref::*, syntax::*};

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

    /// Property is not allowed at this place
    #[error("Defining a property is not allowed here ({0})")]
    PropertyNotAllowed(SrcRef),

    /// Symbol is not a value
    #[error("Symbol {0} is not a value")]
    NotAValue(QualifiedName),

    /// Declaration of property not allowed here
    #[error("Declaration of {0} not allowed within {1}")]
    DeclNotAllowed(Identifier, QualifiedName),

    /// I/O Error
    #[error("I/O Error: {0}")]
    IoError(#[from] std::io::Error),

    /// Sternal module file not found
    #[error("External module file not found for '{0}'")]
    ExternalNotFound(Identifier),

    /// Sternal module file not found
    #[error("Ambiguous external module files found for '{0}': {1:?}")]
    AmbiguousExternal(Identifier, Vec<std::path::PathBuf>),

    /// Sternal module file not found
    #[error("Ambiguous external module files found {0:?}")]
    AmbiguousExternals(Vec<std::path::PathBuf>),

    /// Ambiguous symbol was added
    #[error("Ambiguous symbol added: {0}")]
    AmbiguousSymbol(Identifier),

    /// ScanDir Error
    #[error("ScanDir Error: {0}")]
    ScanDirError(#[from] scan_dir::Error),

    /// Invalid path.
    #[error("Invalid path: {0:?}")]
    InvalidPath(std::path::PathBuf),

    /// Diagnostic error
    #[error("Diagnostic error: {0}")]
    DiagError(#[from] DiagError),

    /// Statement is not supported in this context.
    #[error("{0} is not available within {1}")]
    StatementNotSupported(String, String),

    /// Given symbol has not children which can be used.
    #[error("No symbols found to use in {0}")]
    NoSymbolsToUse(QualifiedName),

    /// Stack underflow.
    #[error("Stack underflow")]
    StackUnderflow,

    /// Stack is unexpectedly empty.
    #[error("Stack is empty")]
    StackEmpty,

    /// Alias leads to itself.
    #[error("Alias leads to itself: {0}")]
    CircularAlias(String),
}

/// Result type of any resolve.
pub type ResolveResult<T> = std::result::Result<T, ResolveError>;
