use crate::resolve::*;

/// Context while resolving a source file
#[derive(Debug)]
pub struct ResolveContext {
    externals: RcMut<Externals>,
}

impl ResolveContext {
    /// Create new resolve context
    pub fn new(externals: RcMut<Externals>) -> Self {
        Self { externals }
    }
    /// return true if a qualified name  can be located in an external use reference
    pub fn check_external(&self, name: QualifiedName) -> ResolveResult<()> {
        self.externals.borrow().fetch_external(name)?;
        Ok(())
    }
}
