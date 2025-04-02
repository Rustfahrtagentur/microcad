use crate::{parse::*, resolve::*};

use thiserror::*;

/// Resolve error
#[derive(Debug, Error)]
enum BuildError {
    #[error("Parse Error: {0}")]
    ParseError(#[from] ParseError),

    #[error("Resolve Error: {0}")]
    ResolveError(#[from] ResolveError),
}

/// Result type of a build
pub type BuildResult<T> = std::result::Result<T, BuildError>;

struct Project {
    search_paths: Vec<std::path::PathBuf>,
    externals: RcMut<Externals>,
    files: SourceFileCache,
}

#[derive(Default)]
struct SourceFileCache {
    by_hash: std::collections::HashMap<u64, usize>,
    by_path: std::collections::HashMap<Option<std::path::PathBuf>, usize>,
    by_name: std::collections::HashMap<QualifiedName, usize>,
    source_files: Vec<Rc<SourceFile>>,
}

impl SourceFileCache {
    pub fn new() -> Self {
        Self {
            ..Default::default()
        }
    }
    pub fn insert(&mut self, name: QualifiedName, source_file: Rc<SourceFile>) {
        let hash = source_file.hash();
        let filename = source_file.filename.clone();
        self.source_files.push(source_file);
        let index = self.source_files.len();
        self.by_hash.insert(hash, index);
        self.by_path.insert(filename, index);
        self.by_name.insert(name, index);
    }
}

impl Project {
    pub fn load(
        path: &std::path::Path,
        search_paths: Vec<std::path::PathBuf>,
    ) -> BuildResult<Self> {
        let source_file = SourceFile::load(path)?;
        let externals = Externals::new(search_paths.clone());
        let context = &mut ResolveContext::new(externals.clone());
        source_file.resolve(None, context)?;

        let mut files = SourceFileCache::new();
        files.insert(QualifiedName::new(), source_file);

        Ok(Self {
            files,
            search_paths,
            externals,
        })
    }
}
