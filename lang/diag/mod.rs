// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handling of diagnostics with source code references

mod diag_handler;
mod diag_list;
mod level;

pub use diag_handler::*;
pub use diag_list::*;
pub use level::*;

use crate::{eval::*, src_ref::*, syntax::*};

/// A trait to add diagnostics with different levels conveniently
pub trait PushDiag {
    /// Push a diagnostic message (must be implemented)
    fn push_diag(&mut self, diag: Diagnostic) -> EvalResult<()>;

    /// Push new trace message
    fn trace(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Trace(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic trace message");
    }
    /// Push new informative message
    fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Info(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic info message");
    }
    /// Push new warning
    fn warning(
        &mut self,
        src: impl SrcReferrer,
        error: Box<dyn std::error::Error>,
    ) -> EvalResult<()> {
        self.push_diag(Diagnostic::Warning(Refer::new(error, src.src_ref())))
    }
    /// Push new error
    fn error(
        &mut self,
        src: impl SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> EvalResult<()> {
        let err = Diagnostic::Error(Refer::new(Box::new(error), src.src_ref()));
        log::error!("{err}");
        self.push_diag(err)
    }
}

/// Diagnostic message with source code reference
#[derive(Debug)]
pub enum Diagnostic {
    /// Trace message with source code reference attached
    Trace(Refer<String>),
    /// Informative message with source code reference attached
    Info(Refer<String>),
    /// Warning with source code reference attached
    Warning(Refer<Box<dyn std::error::Error>>),
    /// Error with source code reference and optional stack trace attached
    Error(Refer<Box<dyn std::error::Error>>),
}

impl Diagnostic {
    /// Get diagnostic level
    pub fn level(&self) -> Level {
        match self {
            Diagnostic::Trace(_) => Level::Trace,
            Diagnostic::Info(_) => Level::Info,
            Diagnostic::Warning(_) => Level::Warning,
            Diagnostic::Error(_) => Level::Error,
        }
    }

    /// get message (errors will be serialized)
    pub fn message(&self) -> String {
        match self {
            Diagnostic::Trace(msg) | Diagnostic::Info(msg) => msg.to_string(),
            Diagnostic::Warning(err) | Diagnostic::Error(err) => err.to_string(),
        }
    }

    /// Pretty print the diagnostic
    ///
    /// This will print the diagnostic to the given writer, including the source code reference
    ///
    /// # Arguments
    ///
    /// * `w` - The writer to write to
    /// * `source_file_by_hash` - Hash provider to get the source file by hash
    ///
    /// This will print:
    ///
    /// ```text
    /// error: This is an error
    ///   ---> filename:1:8
    ///     |
    ///  1  | module circle(radius: length) {}
    ///     |        ^^^^^^
    /// ```
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::fmt::Result {
        let src_ref = self.src_ref();
        let source_file = source_by_hash.get_by_hash(src_ref.source_hash());

        fn make_relative(path: &std::path::Path) -> String {
            let current_dir = std::env::current_dir().expect("current dir");
            if let Ok(path) = path.canonicalize() {
                pathdiff::diff_paths(path, current_dir)
                    .expect("related paths:\n  {path:?}\n  {current_dir:?}")
            } else {
                path.to_path_buf()
            }
            .to_string_lossy()
            .to_string()
        }
        match &src_ref {
            SrcRef(None) => writeln!(f, "{}: {}", self.level(), self.message())?,
            SrcRef(Some(src_ref)) => {
                writeln!(f, "{}: {}", self.level(), self.message())?;
                writeln!(
                    f,
                    "  ---> {}:{}",
                    source_file
                        .as_ref()
                        .map(|sf| make_relative(&sf.filename))
                        .unwrap_or(SourceFile::NO_FILE.to_string()),
                    src_ref.at
                )?;
                writeln!(f, "     |",)?;

                let line = source_file
                    .as_ref()
                    .map(|sf| sf.get_line(src_ref.at.line - 1).unwrap_or("<no line>"))
                    .unwrap_or(SourceFile::NO_FILE);

                writeln!(f, "{: >4} | {}", src_ref.at.line, line)?;
                writeln!(
                    f,
                    "{: >4} | {}",
                    "",
                    " ".repeat(src_ref.at.col - 1)
                        + &"^".repeat(src_ref.range.len().min(line.len())),
                )?;
                writeln!(f, "     |",)?;
            }
        }

        // Print stack trace
        if let Diagnostic::Error(_) = self {
            //stack.pretty_print(w, source_file_by_hash)?
        }

        Ok(())
    }
}

impl SrcReferrer for Diagnostic {
    fn src_ref(&self) -> SrcRef {
        match self {
            Diagnostic::Trace(message) => message.src_ref(),
            Diagnostic::Info(message) => message.src_ref(),
            Diagnostic::Warning(error) => error.src_ref(),
            Diagnostic::Error(error) => error.src_ref(),
        }
    }
}

impl std::fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diagnostic::Trace(message) => write!(f, "trace: {}: {message}", self.src_ref()),
            Diagnostic::Info(message) => write!(f, "info: {}: {message}", self.src_ref()),
            Diagnostic::Warning(error) => write!(f, "warning: {}: {error}", self.src_ref()),
            Diagnostic::Error(error) => write!(f, "error: {}: {error}", self.src_ref()),
        }
    }
}
