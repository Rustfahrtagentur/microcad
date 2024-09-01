//! Remember source code position for diagnosis

use crate::src_ref::*;

enum Level {
    Error,
    Warning,
    Info,
}

impl std::fmt::Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Level::Error => write!(f, "error"),
            Level::Warning => write!(f, "warning"),
            Level::Info => write!(f, "info"),
        }
    }
}

struct Diagnostic {
    pub src_ref: SrcRef,
    pub message: String,
    pub level: Level,
}

impl Diagnostic {
    pub fn new(src_ref: SrcRef, message: String, level: Level) -> Self {
        Self {
            src_ref,
            message,
            level,
        }
    }

    pub fn info(src_ref: SrcRef, message: String) -> Self {
        Self::new(src_ref, message, Level::Info)
    }

    pub fn warning(src_ref: SrcRef, message: String) -> Self {
        Self::new(src_ref, message, Level::Warning)
    }

    pub fn error(src_ref: SrcRef, message: String) -> Self {
        Self::new(src_ref, message, Level::Error)
    }

    pub fn pretty_print(&self, w: &mut dyn std::fmt::Write, source_file: impl AsRef<std::path::Path>) -> std::fmt::Result {
        writeln!(
            w,
            "{}: {}",
            self.level,
            self.message
        )?;
        writeln!(
            w,
            "  --> {}:{}",
            source_file.as_ref().display(),
            self.src_ref.at().unwrap(),
        )?;

        Ok(())
    }
}



/// Diagnostics for a single source file
struct SourceFileDiagnostics {
    source_file: std::path::PathBuf,
    diagnostics: Vec<Diagnostic>,
}

impl SourceFileDiagnostics {
    pub fn new(source_file: std::path::PathBuf) -> Self {
        Self {
            source_file,
            diagnostics: Vec::new(),
        }
    }

    pub fn add(&mut self, diagnostic: Diagnostic) {
        self.diagnostics.push(diagnostic);
    }
}
