use crate::diag::*;

/// We have a vec of source file diagnostics because we want to keep track of diagnostics for each source file separately
#[derive(Debug, Default)]
pub struct DiagList(Vec<Diagnostic>);

impl DiagList {
    /// Pretty print this list of diagnostics
    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_by_hash: &impl GetSourceByHash,
    ) -> std::io::Result<()> {
        self.0
            .iter()
            .try_for_each(|diag| diag.pretty_print(w, source_by_hash))
    }
}

impl std::ops::Deref for DiagList {
    type Target = Vec<Diagnostic>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PushDiag for DiagList {
    fn push_diag(&mut self, diag: Diagnostic) -> crate::eval::EvalResult<()> {
        self.0.push(diag);
        Ok(())
    }
}
