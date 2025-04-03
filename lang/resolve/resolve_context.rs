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
}

impl std::fmt::Display for ResolveContext<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.externals)
    }
}
