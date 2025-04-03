use crate::resolve::*;

/// Context while resolving a source file
#[derive(Debug)]
pub struct ResolveContext<'a> {
    externals: &'a mut Externals,
}

impl<'a> ResolveContext<'a> {
    /// Create new resolve context
    pub fn new(externals: &'a mut Externals) -> Self {
        Self { externals }
    }
    /// return true if a qualified name  can be located in an external use reference
    pub fn check_external(&mut self, name: QualifiedName) -> ResolveResult<()> {
        self.externals.use_external(name)
    }
}
