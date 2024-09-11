use std::ops::Deref;

use crate::{diag::*, parse::*};

/// We have a vec of source file diagnostics because we want to keep track of diagnostics for each source file separately
#[derive(Debug, Default)]
pub struct DiagList(Vec<Diag>);

impl DiagList {
    pub fn pretty_print(
        &self,
        w: &mut dyn std::io::Write,
        source_file_by_hash: &impl GetSourceFileByHash,
    ) -> std::io::Result<()> {
        for source_file_diags in &self.0 {
            source_file_diags.pretty_print(w, source_file_by_hash)?;
        }
        Ok(())
    }
}

impl Deref for DiagList {
    type Target = Vec<Diag>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl PushDiag for DiagList {
    fn push_diag(&mut self, diag: Diag) {
        self.0.push(diag);
    }
}
