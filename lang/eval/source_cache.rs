// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::{eval::*, rc_mut::*, src_ref::*, syntax::*};
use log::*;

/// Register of loaded source files and their syntax trees
#[derive(Default)]
pub struct SourceCache {
    /// External files read from search path
    externals: Externals,

    by_hash: std::collections::HashMap<u64, usize>,
    by_path: std::collections::HashMap<std::path::PathBuf, usize>,
    by_name: std::collections::HashMap<QualifiedName, usize>,

    /// Loaded, parsed and resolved source files
    source_files: Vec<Rc<SourceFile>>,
}

impl SourceCache {
    /// Create new source register
    pub fn new(root: Rc<SourceFile>, search_paths: Vec<std::path::PathBuf>) -> Self {
        let mut by_hash = std::collections::HashMap::new();
        by_hash.insert(root.hash(), 0);
        Self {
            externals: Externals::new(search_paths),
            source_files: vec![root.clone()],
            by_hash,
            ..Default::default()
        }
    }

    /// Create initial namespace tree from externals
    pub fn create_namespaces(&self) -> SymbolMap {
        self.externals.create_namespaces()
    }

    /// Insert a new source file into source register
    /// - `name`: Qualified name which represents the file
    /// - `source_file`: The loaded source file to store
    pub fn insert(&mut self, source_file: Rc<SourceFile>) -> EvalResult<QualifiedName> {
        let name = self.externals.get_name(&source_file.filename)?;
        let hash = source_file.hash();
        let filename = source_file.filename.clone();
        let index = self.source_files.len();
        debug!("caching [{index}] {name} {hash:#x} {filename:?}");
        self.source_files.push(source_file);
        self.by_hash.insert(hash, index);
        self.by_path.insert(filename, index);
        self.by_name.insert(name.clone(), index);
        Ok(name.clone())
    }

    /// Convenience function to get a source file by from a `SrcReferrer`
    pub fn get_by_src_ref(&self, src_ref: &impl SrcReferrer) -> EvalResult<Rc<SourceFile>> {
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
    pub fn get_by_path(&self, path: &std::path::Path) -> EvalResult<Rc<SourceFile>> {
        let path = path.to_path_buf();
        if let Some(index) = self.by_path.get(&path) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(EvalError::UnknownPath(path))
        }
    }

    /// Find a project file by the qualified name which represents the file path
    pub fn get_by_name(&self, name: &QualifiedName) -> EvalResult<Rc<SourceFile>> {
        if let Some(index) = self.by_name.get(name) {
            Ok(self.source_files[*index].clone())
        } else {
            // if not found in symbol tree we try to find an external file to load
            let external = self.externals.fetch_external(name)?;
            Err(EvalError::SymbolMustBeLoaded(
                self.externals.get_name(external)?.clone(),
                external.clone(),
            ))
        }
    }
}

/// Trait that can fetch for a file by it's hash value
pub trait GetSourceByHash {
    /// Find a project file by it's hash value
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>>;
}

impl GetSourceByHash for SourceCache {
    /// Find a project file by it's hash value
    fn get_by_hash(&self, hash: u64) -> EvalResult<Rc<SourceFile>> {
        if let Some(index) = self.by_hash.get(&hash) {
            Ok(self.source_files[*index].clone())
        } else {
            Err(EvalError::UnknownHash(hash))
        }
    }
}
