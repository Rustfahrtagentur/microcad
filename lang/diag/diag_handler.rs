// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*};

/// Handler for diagnostics.
#[derive(Default)]
pub struct DiagHandler {
    /// The list of diagnostics per source file.
    diag_list: DiagList,
    /// The current number of overall errors in the evaluation process.
    error_count: u32,
    /// The maximum number of collected errors until abort
    /// (`0` means unlimited number of errors).
    error_limit: u32,
    /// Treat warnings as errors if `true`.
    warnings_as_errors: bool,
}

/// Handler for diagnostics.
impl DiagHandler {
    /// Pretty print all errors of all files.
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::fmt::Result {
        self.diag_list.pretty_print(f, source_by_hash)
    }

    /// Return overall number of occurred errors.
    pub fn error_count(&self) -> u32 {
        self.error_count
    }
}

impl PushDiag for DiagHandler {
    fn push_diag(&mut self, diag: super::Diagnostic) -> crate::eval::EvalResult<()> {
        use super::Diagnostic;
        match &diag {
            Diagnostic::Error(_) => {
                self.error_count += 1;
            }
            Diagnostic::Warning(_) => {
                if self.warnings_as_errors {
                    self.error_count += 1;
                }
            }
            _ => (),
        }

        self.diag_list.push_diag(diag)?;

        if self.error_limit > 0 && self.error_count > self.error_limit {
            self.error(
                SrcRef(None),
                Box::new(EvalError::ErrorLimitReached(self.error_limit)),
            )?;
            Err(EvalError::ErrorLimitReached(self.error_limit))
        } else {
            Ok(())
        }
    }
}
