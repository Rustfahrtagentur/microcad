use crate::{diag::*, eval::*};

/// Diagnosis trait gives access about collected errors
pub trait Diag {
    /// Pretty print all errors
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result;

    /// Pretty write all errors into a file
    fn write_diagnosis(&self, w: &mut dyn std::io::Write) -> std::io::Result<()> {
        write!(w, "{}", self.diagnosis())
    }

    /// Get pretty printed errors as string
    fn diagnosis(&self) -> String {
        let mut str = String::new();
        self.fmt_diagnosis(&mut str).expect("displayable diagnosis");
        str
    }

    /// Returns true if there are errors
    fn has_errors(&self) -> bool;

    /// Return number of occurred errors
    fn error_count(&self) -> u32;
}

/// Handler for diagnostics
#[derive(Default)]
pub struct DiagHandler {
    /// The list of diagnostics
    diag_list: DiagList,
    /// The current number of errors in the evaluation process
    error_count: u32,
    /// The maximum number of errors. `0` means unlimited number of errors.
    error_limit: u32,
    /// Treat warnings as errors
    warnings_as_errors: bool,
}

/// Handler for diagnostics
impl DiagHandler {
    /// Pretty print all errors
    pub fn pretty_print(
        &self,
        f: &mut dyn std::fmt::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::fmt::Result {
        self.diag_list.pretty_print(f, source_by_hash)
    }

    /// Returns true if there are errors
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }

    /// Return number of occurred errors
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
