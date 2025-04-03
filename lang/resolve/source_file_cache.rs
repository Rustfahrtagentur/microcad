// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::{rc_mut::*, resolve::*, syntax::*};

#[derive(Default)]
pub struct SourceFileCache {
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

    /// Find a project file by it's hash value
    pub fn get_by_hash(&self, hash: u64) -> BuildResult<Rc<SourceFile>> {
        if let Some(index) = self.by_hash.get(&hash) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(BuildError::UnknownHash(hash))
        }
    }

    /// Find a project file by it's file path
    pub fn get_by_path(&self, path: &std::path::Path) -> BuildResult<Rc<SourceFile>> {
        let path = path.to_path_buf();
        if let Some(index) = self.by_path.get(&Some(path.clone())) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(BuildError::UnknownPath(path))
        }
    }

    /// Find a project file by the qualified name which represents the file path
    pub fn get_by_name(&self, name: &QualifiedName) -> BuildResult<Rc<SourceFile>> {
        if let Some(index) = self.by_name.get(name) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(BuildError::UnknownName(name.clone()))
        }
    }
}
