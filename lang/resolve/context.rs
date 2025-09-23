use crate::{diag::*, resolve::*};

#[derive(Default)]
pub struct Context {
    // Root node identifier.
    root: Identifier,
    /// external source files.
    sources: Sources,
    /// Source file diagnostics.
    diag_handler: DiagHandler,
}

impl Context {
    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Root source file definition.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn new(root: SourceFile, search_paths: impl AsRef<std::path::Path>) -> Self {
        log::debug!("Creating resolve context");

        // put all together
        Self {
            root,
            sources: Sources::search(search_paths),
            diag_handler: Default::default(),
        }
    }

    /// Create a new context from a source file.
    ///
    /// # Arguments
    /// - `root`: Path to the root file to load.
    /// - `builtin`: The builtin library.
    /// - `search_paths`: Paths to search for external libraries (e.g. the standard library).
    pub fn from_source(
        root: impl AsRef<std::path::Path> + std::fmt::Debug,
        search_paths: &[std::path::PathBuf],
    ) -> ResolveResult<Self> {
        let root = SourceFile::load(root)?;
        let root_id = root.id();
        let sources = Sources::load(root, search_paths)?;
        Ok(Self::new(root_id, sources))
    }
}

impl Diag for Context {
    fn fmt_diagnosis(&self, f: &mut dyn std::fmt::Write) -> std::fmt::Result {
        self.diag_handler.pretty_print(f, &self.sources)
    }

    fn error_count(&self) -> u32 {
        self.diag_handler.error_count()
    }

    fn error_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_handler.error_lines()
    }

    fn warning_lines(&self) -> std::collections::HashSet<usize> {
        self.diag_handler.warning_lines()
    }
}
