// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Handling of diagnostic errors.
//!
//! While *evaluation* µcad is collecting [`Diagnostic`] messages.
//!
//! This is done in [`DiagHandler`] by providing the following traits:
//!
//! - [`PushDiag`]: Collects error in [`DiagHandler`]
//! - [`Diag`]: Get diagnostic messages

mod diag_error;
mod diag_handler;
mod diag_list;
mod diagnostic;
mod level;

pub use diag_error::*;
pub use diag_handler::*;
pub use diag_list::*;
pub use diagnostic::*;
pub use level::*;

use crate::src_ref::*;

/// A trait to add diagnostics with different levels conveniently.
pub trait PushDiag {
    /// Push a diagnostic message (must be implemented).
    fn push_diag(&mut self, diag: Diagnostic) -> DiagResult<()>;

    /// Push new trace message.
    fn trace(&mut self, src: &impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Trace(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic trace message");
    }
    /// Push new informative message.
    fn info(&mut self, src: &impl SrcReferrer, message: String) {
        self.push_diag(Diagnostic::Info(Refer::new(message, src.src_ref())))
            .expect("could not push diagnostic info message");
    }
    /// Push new warning.
    fn warning(
        &mut self,
        src: &impl SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> DiagResult<()> {
        let err = Diagnostic::Warning(Refer::new(error.into(), src.src_ref()));
        if cfg!(feature = "ansi-color") {
            log::warn!("{}", color_print::cformat!("<y,s>{err}</>"));
        } else {
            log::warn!("{err}");
        }
        self.push_diag(err)
    }
    /// Push new error.
    fn error(
        &mut self,
        src: &impl SrcReferrer,
        error: impl std::error::Error + 'static,
    ) -> DiagResult<()> {
        let err = Diagnostic::Error(Refer::new(error.into(), src.src_ref()));
        if cfg!(feature = "ansi-color") {
            log::error!("{}", color_print::cformat!("<r,s>{err}</>"));
        } else {
            log::error!("{err}");
        }
        self.push_diag(err)
    }
}

/// Diagnosis trait gives access about collected errors.
pub trait Diag {
    /// Pretty print all errors.
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Pretty write all errors into a file.
    fn write_diagnosis(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}", self.diagnosis())
    }

    /// Get pretty printed errors as string.
    fn diagnosis(&self) -> String {
        let mut str = String::new();
        self.fmt_diagnosis(&mut str).expect("displayable diagnosis");
        str
    }

    /// Returns true if there are warnings.
    fn has_warnings(&self) -> bool {
        self.warning_count() > 0
    }

    /// Return number of occurred warnings.
    fn warning_count(&self) -> u32;

    /// Returns true if there are errors.
    fn has_errors(&self) -> bool {
        self.error_count() > 0
    }

    /// Return number of occurred errors.
    fn error_count(&self) -> u32;

    /// Return all lines with errors
    fn error_lines(&self) -> std::collections::HashSet<usize>;

    /// Return all lines with warnings
    fn warning_lines(&self) -> std::collections::HashSet<usize>;
}

/// Trait to write something with Display trait into a file.
pub trait WriteToFile: std::fmt::Display {
    /// Write something to a file.
    fn write_to_file(&self, filename: &impl AsRef<std::path::Path>) -> std::io::Result<()> {
        use std::io::Write;
        let file = std::fs::File::create(filename)?;
        let mut writer = std::io::BufWriter::new(file);
        write!(writer, "{self}")
    }
}
