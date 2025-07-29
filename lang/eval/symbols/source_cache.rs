// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::{eval::*, rc::*, src_ref::*, syntax::*};
use std::collections::HashMap;

/// Register of loaded source files and their syntax trees.
///
/// Source file definitions ([`SourceFile`]) are stored in a vector (`Vec<Rc<SourceFile>>`)
/// and mapped by *hash*, *path* and *name* via index to this vector.
///
/// The *root model* (given at creation) will be stored but will only be accessible by hash and path
/// but not by it's qualified name.
#[derive(Default)]
pub struct SourceCache {
    /// External files read from search path.
    externals: Externals,

    by_hash: HashMap<u64, usize>,
    by_path: HashMap<std::path::PathBuf, usize>,
    by_name: HashMap<QualifiedName, usize>,

    /// Loaded, parsed and resolved source files.
    source_files: Vec<Rc<SourceFile>>,

    /// Search paths.
    search_paths: Vec<std::path::PathBuf>,
}

impl SourceCache {
    /// Create new source register.
    pub fn new(root: Rc<SourceFile>, search_paths: &[std::path::PathBuf]) -> Self {
        let mut by_hash = HashMap::new();
        by_hash.insert(root.hash, 0);

        let mut by_path = HashMap::new();
        by_path.insert(root.filename.clone(), 0);

        Self {
            externals: Externals::new(search_paths),
            source_files: vec![root.clone()],
            by_hash,
            by_path,
            // root shall not be found by name
            by_name: HashMap::new(),
            search_paths: search_paths.to_vec(),
        }
    }

    /// Create initial symbol map from externals.
    pub fn create_modules(&self) -> SymbolMap {
        self.externals.create_modules()
    }

    /// Insert a new source file (from externals) into source register.
    ///
    /// The file must lay in one of the search paths given to externals.
    ///
    /// # Arguments
    /// - `source_file`: The loaded source file to store.
    pub fn insert(&mut self, source_file: Rc<SourceFile>) -> EvalResult<QualifiedName> {
        let filename = source_file.filename.clone();
        let name = self.externals.get_name(&filename)?;
        let hash = source_file.hash;
        let index = self.source_files.len();
        log::debug!("caching [{index}] {name} {hash:#x} {filename:?}");
        self.source_files.push(source_file);
        self.by_hash.insert(hash, index);
        self.by_path.insert(filename, index);
        self.by_name.insert(name.clone(), index);
        Ok(name.clone())
    }

    /// Return the qualified name of a file by it's path
    pub fn name_by_path(&mut self, filename: &std::path::Path) -> EvalResult<QualifiedName> {
        Ok(self.externals.get_name(filename)?.clone())
    }

    /// Convenience function to get a source file by from a `SrcReferrer`.
    pub fn get_by_src_ref(&self, referrer: &impl SrcReferrer) -> EvalResult<Rc<SourceFile>> {
        self.get_by_hash(referrer.src_ref().source_hash())
    }

    /// Return a string describing the given source code position.
    pub fn ref_str(&self, referrer: &impl SrcReferrer) -> String {
        format!(
            "{}:{}",
            self.get_by_src_ref(referrer)
                .expect("Source file not found")
                .filename_as_str(),
            referrer.src_ref(),
        )
    }

    /// Find a project file by it's file path.
    pub fn get_by_path(&self, path: &std::path::Path) -> EvalResult<Rc<SourceFile>> {
        let path = path.to_path_buf();
        if let Some(index) = self.by_path.get(&path) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(EvalError::UnknownPath(path))
        }
    }

    /// Get *qualified name* of a file by *hash value*.
    pub fn get_name_by_hash(&self, hash: u64) -> EvalResult<&QualifiedName> {
        match self.get_by_hash(hash) {
            Ok(file) => self.externals.get_name(&file.filename),
            Err(err) => Err(err),
        }
    }

    /// Find a project file by the qualified name which represents the file path.
    pub fn get_by_name(&self, name: &QualifiedName) -> EvalResult<Rc<SourceFile>> {
        if let Some(index) = self.by_name.get(name) {
            Ok(self.source_files[*index].clone())
        } else {
            // if not found in symbol tree we try to find an external file to load
            let (name, path) = self.externals.fetch_external(name)?;
            if self.get_by_path(&path).is_err() {
                Err(EvalError::SymbolMustBeLoaded(name, path))
            } else {
                Err(EvalError::SymbolNotFound(name))
            }
        }
    }

    fn name_from_index(&self, index: usize) -> Option<QualifiedName> {
        self.by_name
            .iter()
            .find(|(_, i)| **i == index)
            .map(|(name, _)| name.clone())
    }

    /// Return search paths of this cache.
    pub fn search_paths(&self) -> &Vec<std::path::PathBuf> {
        &self.search_paths
    }
}

/// Trait that can fetch for a file by it's hash value.
pub trait GetSourceByHash {
    /// Find a project file by it's hash value.
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>>;
}

impl GetSourceByHash for SourceCache {
    /// Find a project file by it's hash value.
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        if let Some(index) = self.by_hash.get(&hash) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(EvalError::UnknownHash(hash))
        }
    }
}

impl std::fmt::Display for SourceCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, source_file) in self.source_files.iter().enumerate() {
            let filename = source_file.filename.clone();
            let name = self
                .name_from_index(index)
                .unwrap_or(QualifiedName::no_ref(vec![]));
            let hash = source_file.hash;
            writeln!(f, "[{index}] {name} {hash:#x} {filename:?}")?;
        }
        Ok(())
    }
}
