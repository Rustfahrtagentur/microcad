// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{diag::*, eval::*, resolve::*};

/// Handler for diagnostics.
#[derive(Default)]
pub struct DiagHandler {
    /// The list of diagnostics per source file.
    diag_list: DiagList,
    /// The current number of overall errors in the evaluation process.
    error_count: u32,
    /// The maximum number of collected errors until abort
    /// (`0` means unlimited number of errors).
    error_limit: Option<u32>,
    /// `true` after the first time error limit was reached
    error_limit_reached: bool,
    /// Treat warnings as errors if `true`.
    warnings_as_errors: bool,
}

/// Handler for diagnostics.
impl DiagHandler {
    /// Create new diag handler.
    pub fn new(error_limit: Option<u32>, warnings_as_errors: bool) -> Self {
        Self {
            error_limit,
            warnings_as_errors,
            ..Default::default()
        }
    }

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

    /// return lines with errors
    pub fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_list
            .iter()
            .filter_map(|d| {
                if d.level() == Level::Error {
                    d.line()
                } else {
                    None
                }
            })
            .collect()
    }

    /// return lines with warnings
    pub fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_list
            .iter()
            .filter_map(|d| {
                if d.level() == Level::Warning {
                    d.line()
                } else {
                    None
                }
            })
            .collect()
    }
}

impl PushDiag for DiagHandler {
    fn push_diag(&mut self, diag: super::Diagnostic) -> crate::eval::EvalResult<()> {
        if let Some(error_limit) = self.error_limit {
            if self.error_count >= error_limit && !self.error_limit_reached {
                self.error(
                    &SrcRef(None),
                    Box::new(EvalError::ErrorLimitReached(error_limit)),
                )?;
                self.error_limit_reached = true;
            }
            return Err(EvalError::ErrorLimitReached(error_limit));
        }

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

        self.diag_list.push_diag(diag)
    }
}
