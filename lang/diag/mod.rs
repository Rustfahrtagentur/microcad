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
    fn push_diag(&mut self, diag: Diag) -> EvalResult<()>;

    /// Push new trace message
    fn trace(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diag::Trace(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic trace message");
    }
    /// Push new informative message
    fn info(&mut self, src: impl SrcReferrer, message: String) {
        self.push_diag(Diag::Info(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic info message");
    }
    /// Push new warning
    fn warning(
        &mut self,
        src: impl SrcReferrer,
        error: Box<dyn std::error::Error>,
    ) -> EvalResult<()> {
        self.push_diag(Diag::Warning(Refer::new(error, src.src_ref())))
    }
    /// Push new error
    fn error(
        &mut self,
        src: impl SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> EvalResult<()> {
        self.push_diag(Diag::Error(Refer::new(Box::new(error), src.src_ref())))
    }
}

/// Diagnostic message with source code reference
#[derive(Debug)]
pub enum Diag {
    /// Trace message with source code reference attached
    Trace(Refer<String>),
    /// Informative message with source code reference attached
    Info(Refer<String>),
    /// Warning with source code reference attached
    Warning(Refer<Box<dyn std::error::Error>>),
    /// Error with source code reference and optional stack trace attached
    Error(Refer<Box<dyn std::error::Error>>),
}

impl Diag {
    /// Get diagnostic level
    pub fn level(&self) -> Level {
        match self {
            Diag::Trace(_) => Level::Trace,
            Diag::Info(_) => Level::Info,
            Diag::Warning(_) => Level::Warning,
            Diag::Error(_) => Level::Error,
        }
    }

    /// get message (errors will be serialized)
    pub fn message(&self) -> String {
        match self {
            Diag::Trace(msg) | Diag::Info(msg) => msg.to_string(),
            Diag::Warning(err) | Diag::Error(err) => err.to_string(),
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
        w: &mut dyn std::io::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::io::Result<()> {
        let src_ref = self.src_ref();
        let source_file = source_by_hash.get_by_hash(src_ref.source_hash());

        match &src_ref {
            SrcRef(None) => writeln!(w, "{}: {}", self.level(), self.message())?,
            SrcRef(Some(src_ref)) => {
                writeln!(w, "{}: {}", self.level(), self.message())?;
                writeln!(
                    w,
                    "  ---> {}:{}",
                    source_file
                        .as_ref()
                        .map(|sf| sf.filename_as_str())
                        .unwrap_or(SourceFile::NO_FILE),
                    src_ref.at
                )?;
                writeln!(w, "     |",)?;

                let line = source_file
                    .as_ref()
                    .map(|sf| sf.get_line(src_ref.at.line - 1).unwrap_or("<no line>"))
                    .unwrap_or(SourceFile::NO_FILE);

                writeln!(w, "{: >4} | {}", src_ref.at.line, line)?;
                writeln!(
                    w,
                    "{: >4} | {}",
                    "",
                    " ".repeat(src_ref.at.col - 1)
                        + &"^".repeat(src_ref.range.len().min(line.len())),
                )?;
                writeln!(w, "     |",)?;
            }
        }

        // Print stack trace
        if let Diag::Error(_) = self {
            //stack.pretty_print(w, source_file_by_hash)?
        }

        Ok(())
    }
}

impl SrcReferrer for Diag {
    fn src_ref(&self) -> SrcRef {
        match self {
            Diag::Trace(message) => message.src_ref(),
            Diag::Info(message) => message.src_ref(),
            Diag::Warning(error) => error.src_ref(),
            Diag::Error(error) => error.src_ref(),
        }
    }
}

impl std::fmt::Display for Diag {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Diag::Trace(message) => write!(f, "trace: {}: {message}", self.src_ref()),
            Diag::Info(message) => write!(f, "info: {}: {message}", self.src_ref()),
            Diag::Warning(error) => write!(f, "warning: {}: {error}", self.src_ref()),
            Diag::Error(error) => write!(f, "error: {}: {error}", self.src_ref()),
        }
    }
}
