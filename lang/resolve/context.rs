use crate::diag::*;

#[derive(Default)]
pub struct Context {
    /// Source file diagnostics.
    diag_handler: DiagHandler,
}
