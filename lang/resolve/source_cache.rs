// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::{rc_mut::*, resolve::*, src_ref::*, syntax::*};

/// Register of loaded source files
#[derive(Default)]
pub struct SourceCache {
    by_hash: std::collections::HashMap<u64, usize>,
    by_path: std::collections::HashMap<Option<std::path::PathBuf>, usize>,
    by_name: std::collections::HashMap<QualifiedName, usize>,
    source_files: Vec<Rc<SourceFile>>,
}

impl SourceCache {
    /// Create new source register
    pub fn new(root: Rc<SourceFile>) -> Self {
        let mut by_hash = std::collections::HashMap::new();
        by_hash.insert(root.hash(), 0);
        Self {
            source_files: vec![root.clone()],
            by_hash,
            ..Default::default()
        }
    }

    /// Insert a new source file into source register
    /// - `name`: Qualified name which represents the file
    /// - `source_file`: The loaded source file to store
    pub fn insert(&mut self, name: QualifiedName, source_file: Rc<SourceFile>) {
        let hash = source_file.hash();
        let filename = source_file.filename.clone();
        self.source_files.push(source_file);
        let index = self.source_files.len();
        self.by_hash.insert(hash, index);
        self.by_path.insert(filename, index);
        self.by_name.insert(name, index);
    }

    /// Convenience function to get a source file by from a `SrcReferrer`
    pub fn get_by_src_ref(&self, src_ref: &impl SrcReferrer) -> ResolveResult<Rc<SourceFile>> {
        self.get_by_hash(src_ref.src_ref().source_hash())
    }

    /// return a string describing the given source code position
    pub fn ref_str(&self, src_ref: &impl SrcReferrer) -> String {
        format!(
            "{}:{}",
            self.get_by_src_ref(src_ref)
                .expect("Source file not found")
                .filename_as_str(),
            src_ref.src_ref(),
        )
    }

    /// Find a project file by it's file path
    pub fn get_by_path(&self, path: &std::path::Path) -> ResolveResult<Rc<SourceFile>> {
        let path = path.to_path_buf();
        if let Some(index) = self.by_path.get(&Some(path.clone())) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(ResolveError::UnknownPath(path))
        }
    }

    /// Find a project file by the qualified name which represents the file path
    pub fn get_by_name(&self, name: &QualifiedName) -> ResolveResult<Rc<SourceFile>> {
        if let Some(index) = self.by_name.get(name) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(ResolveError::UnknownName(name.clone()))
        }
    }
}

/// Trait that can fetch for a file by it's hash value
pub trait GetSourceByHash {
    /// Find a project file by it's hash value
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>>;
}

impl GetSourceByHash for SourceCache {
    /// Find a project file by it's hash value
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>> {
        if let Some(index) = self.by_hash.get(&hash) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(ResolveError::UnknownHash(hash))
        }
    }
}
