// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::{rc::*, resolve::*, src_ref::*, syntax::*};
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
        by_path.insert(root.filename(), 0);

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
    pub fn insert(&mut self, source_file: Rc<SourceFile>) -> ResolveResult<QualifiedName> {
        let filename = source_file.filename();
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
    pub fn name_by_path(&mut self, filename: &std::path::Path) -> ResolveResult<QualifiedName> {
        Ok(self.externals.get_name(filename)?.clone())
    }

    /// Convenience function to get a source file by from a `SrcReferrer`.
    pub fn get_by_src_ref(&self, referrer: &impl SrcReferrer) -> ResolveResult<Rc<SourceFile>> {
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
    pub fn get_by_path(&self, path: &std::path::Path) -> ResolveResult<Rc<SourceFile>> {
        let path = path.to_path_buf();
        if let Some(index) = self.by_path.get(&path) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(ResolveError::FileNotFound(path))
        }
    }

    /// Get *qualified name* of a file by *hash value*.
    pub fn get_name_by_hash(&self, hash: u64) -> ResolveResult<&QualifiedName> {
        match self.get_by_hash(hash) {
            Ok(file) => self.externals.get_name(&file.filename()),
            Err(err) => Err(err),
        }
    }

    /// Find a project file by the qualified name which represents the file path.
    pub fn get_by_name(&self, name: &QualifiedName) -> ResolveResult<Rc<SourceFile>> {
        if let Some(index) = self.by_name.get(name) {
            Ok(self.source_files[*index].clone())
        } else {
            // if not found in symbol tree we try to find an external file to load
            match self.externals.fetch_external(name) {
                Ok((name, path)) => {
                    if self.get_by_path(&path).is_err() {
                        return Err(ResolveError::SymbolMustBeLoaded(name, path));
                    }
                }
                Err(ResolveError::ExternalSymbolNotFound(_)) => (),
                Err(err) => return Err(err),
            }
            Err(ResolveError::SymbolNotFound(name.clone()))
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
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>>;
}

impl GetSourceByHash for SourceCache {
    /// Find a project file by it's hash value.
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>> {
        if let Some(index) = self.by_hash.get(&hash) {
            Ok(self.source_files[*index].clone())
        } else if hash == 0 {
            Err(ResolveError::NulHash)
        } else {
            Err(ResolveError::UnknownHash(hash))
        }
    }
}

impl std::fmt::Display for SourceCache {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, source_file) in self.source_files.iter().enumerate() {
            let filename = source_file.filename_as_str();
            let name = self
                .name_from_index(index)
                .unwrap_or(QualifiedName::no_ref(vec![]));
            let hash = source_file.hash;
            writeln!(f, "[{index}] {name} {hash:#x} {filename}")?;
        }
        Ok(())
    }
}
