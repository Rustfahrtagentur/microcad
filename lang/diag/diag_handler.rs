use crate::{diag::*, eval::*, syntax::*};

/// Handler for diagnostics
#[derive(Default)]
pub struct DiagHandler {
    /// The list of diagnostics
    diag_list: DiagList,

    /// The current number of errors in the evaluation process
    pub error_count: u32,

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
        source_file_by_hash: &impl GetSourceFileByHash,
    ) -> std::io::Result<()> {
        self.diag_list.pretty_print(w, source_file_by_hash)
    }

    /// Pretty print all errors into a string
    pub fn pretty_print_to_string(
        &self,
        source_file_by_hash: &impl GetSourceFileByHash,
    ) -> std::io::Result<String> {
        let mut s = Vec::new();
        let mut w = std::io::BufWriter::new(&mut s);
        self.diag_list.pretty_print(&mut w, source_file_by_hash)?;
        let w = w.into_inner().expect("could not pretty print errors");
        Ok(String::from_utf8(w.to_vec()).expect("could not pretty print errors"))
    }

    /// Returns true if there are errors
    pub fn has_errors(&self) -> bool {
        self.error_count > 0
    }
}

impl PushDiag for DiagHandler {
    fn push_diag(&mut self, diag: super::Diag) -> crate::eval::EvalResult<()> {
        use super::Diag;
        match &diag {
            Diag::Error(_) => {
                self.error_count += 1;
            }
            Diag::Warning(_) => {
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
