use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Not implemented")]
    NotImplemented,

    #[error("Unknown file extension to export to: {0}")]
    UnknownFileExtension(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("No suitable exporter found for extension: {0}")]
    NoExporterForExtension(String),

    #[error("No filename specified for export")]
    NoFilenameSpecifiedForExport,
}
