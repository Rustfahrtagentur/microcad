use crate::{diag::*, eval::*};

/// Diagnosis trait gives access about collected errors
pub trait Diag {
    /// Pretty print all errors
    fn pretty_print(&self, w: &mut dyn std::io::Write) -> std::io::Result<()>;

    /// Pretty print all errors into a string
    fn pretty_print_to_string(&self) -> std::io::Result<String>;

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
        w: &mut dyn std::io::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::io::Result<()> {
        self.diag_list.pretty_print(w, source_by_hash)
    }

    /// Pretty print all errors into a string
    pub fn pretty_print_to_string(
        &self,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::io::Result<String> {
        let mut s = Vec::new();
        let mut w = std::io::BufWriter::new(&mut s);
        self.diag_list.pretty_print(&mut w, source_by_hash)?;
        let w = w
            .into_inner()
            .expect("write error while pretty printing errors");
        Ok(String::from_utf8(w.to_vec()).expect("UTF-8 error while pretty printing errors"))
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
