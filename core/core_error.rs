// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Core error

use thiserror::Error;

/// Core error
#[derive(Debug, Error)]
pub enum CoreError {
    /// Not implemented
    #[error("Not implemented")]
    NotImplemented,

    /// Unknown file extension to export
    #[error("Unknown file extension to export to: {0}")]
    UnknownFileExtension(String),

    /// IO error
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    /// Path error
    #[error("Path error: {0:?}")]
    PathError(#[from] core::convert::Infallible),

    /// No suitable exporter found
    #[error("No suitable exporter found for `{0}`")]
    NoSuitableExporterFound(String),

    /// No filename specified for export
    #[error("No filename specified for export")]
    NoFilenameSpecifiedForExport,

    /// Directory does not exist
    #[error("Directory does not exist: {0}")]
    DirectoryDoesNotExist(std::path::PathBuf),

    /// Invalid rendering precision
    #[error("Invalid rendering precision: {0}")]
    InvalidRenderPrecision(String),

    /// Parse float error
    #[error("Parse float error: {0}")]
    Error(#[from] std::num::ParseFloatError),

    /// Export missing filename
    #[error("Export missing filename")]
    ExportMissingFilename,

    /// Cannot detect export format from extension
    #[error("Cannot detect export format from extension")]
    CannotDetectExportFormatFromExtension,

    /// Unexpected direction value
    #[error("Bad direction '{0}'")]
    BadDirection(String),

    /// Unexpected alignment value
    #[error("Bad alignment '{0}'")]
    BadAlignment(String),
}

/// Core result type
pub type CoreResult<T> = std::result::Result<T, CoreError>;
