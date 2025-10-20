// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use derive_more::Deref;

use crate::{parse::*, rc::*, resolve::*, src_ref::*, syntax::*};
use std::collections::HashMap;

/// Register of loaded source files and their syntax trees.
///
/// Source file definitions ([`SourceFile`]) are stored in a vector (`Vec<Rc<SourceFile>>`)
/// and mapped by *hash*, *path* and *name* via index to this vector.
///
/// The *root model* (given at creation) will be stored but will only be accessible by hash and path
/// but not by it's qualified name.
#[derive(Default, Deref)]
pub struct Sources {
    /// External files read from search path.
    externals: Externals,

    by_hash: HashMap<u64, usize>,
    by_path: HashMap<std::path::PathBuf, usize>,
    by_name: HashMap<QualifiedName, usize>,

    //root source file.
    root: Rc<SourceFile>,

    /// External source files.
    #[deref]
    pub source_files: Vec<Rc<SourceFile>>,

    /// Search paths.
    search_paths: Vec<std::path::PathBuf>,
}

impl Sources {
    /// Create source cache
    ///
    /// Inserts the `root` file and loads all files from `search_paths`.
    pub fn load(
        root: Rc<SourceFile>,
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> ResolveResult<Self> {
        let mut source_files = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_hash = HashMap::new();
        let mut by_path = HashMap::new();

        by_hash.insert(root.hash, 0);
        by_path.insert(root.filename(), 0);
        by_name.insert(root.name.clone(), 0);
        source_files.push(root.clone());

        // search for external source files
        let externals = Externals::new(search_paths)?;

        log::trace!("Externals:\n{externals}");

        // load all external source files into cache
        externals
            .iter()
            .try_for_each(|(name, path)| -> Result<(), ParseError> {
                let source_file = SourceFile::load_with_name(path.clone(), name.clone())?;
                let index = source_files.len();
                by_hash.insert(source_file.hash, index);
                by_path.insert(source_file.filename(), index);
                by_name.insert(name.clone(), index);
                source_files.push(source_file);
                Ok(())
            })?;

        Ok(Self {
            externals,
            root,
            source_files,
            by_hash,
            by_path,
            by_name,
            search_paths: search_paths
                .iter()
                .map(|path| path.as_ref().canonicalize().expect("valid path"))
                .collect(),
        })
    }

    /// Return root file.
    pub fn root(&self) -> Rc<SourceFile> {
        self.root.clone()
    }

    /// Insert a file to the sources.
    pub fn insert(&mut self, source_file: Rc<SourceFile>) {
        let index = self.source_files.len();
        self.source_files.push(source_file.clone());
        self.by_hash.insert(source_file.hash, index);
        self.by_path.insert(source_file.filename(), index);
        self.by_name.insert(source_file.name.clone(), index);
    }

    /// Return the qualified name of a file by it's path
    pub fn generate_name_from_path(
        &self,
        file_path: &std::path::Path,
    ) -> ResolveResult<QualifiedName> {
        // check root file name
        if self.root.filename() == file_path {
            return Ok(QualifiedName::from_id(self.root.id()));
        }

        // check file names relative to search paths
        let path = if let Some(path) = self
            .search_paths
            .iter()
            .find_map(|path| file_path.strip_prefix(path).ok())
        {
            path.with_extension("")
        }
        // check file names relative to project root directory
        else if let Some(root_dir) = self.root_dir() {
            if let Ok(path) = file_path.strip_prefix(root_dir) {
                path.with_extension("")
            } else {
                return Err(ResolveError::InvalidPath(file_path.to_path_buf()));
            }
        } else {
            return Err(ResolveError::InvalidPath(file_path.to_path_buf()));
        };

        // check if file is a mod file then it gets it"s name from the parent directory
        let path = if path
            .iter()
            .next_back()
            .map(|s| s.to_string_lossy().to_string())
            == Some("mod".into())
        {
            path.parent()
        } else {
            Some(path.as_path())
        };

        // get name from path which was found
        if let Some(path) = path {
            Ok(path
                .iter()
                .map(|name| Identifier::no_ref(name.to_string_lossy().as_ref()))
                .collect())
        } else {
            Err(ResolveError::InvalidPath(file_path.to_path_buf()))
        }
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

    fn root_dir(&self) -> Option<std::path::PathBuf> {
        self.root.filename().parent().map(|p| p.to_path_buf())
    }

    /// Load another source file into cache.
    pub fn load_mod_file(
        &mut self,
        parent_path: impl AsRef<std::path::Path>,
        id: &Identifier,
    ) -> ResolveResult<Rc<SourceFile>> {
        log::trace!("load_file: {:?} {id}", parent_path.as_ref());
        let file_path = find_mod_file_by_id(parent_path, id)?;
        let name = self.generate_name_from_path(&file_path)?;
        let source_file = SourceFile::load_with_name(&file_path, name)?;
        self.insert(source_file.clone());
        Ok(source_file)
    }

    /// Reload an existing file
    pub fn update_file(
        &mut self,
        path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<Rc<SourceFile>> {
        let path = path.as_ref().to_path_buf();
        if let Some(index) = self.by_path.get(&path) {
            let old_source_file = self.source_files[*index].clone();
            let name = old_source_file.name.clone();
            let new_source_file = SourceFile::load_with_name(path, name)?;
            self.by_hash.remove(&old_source_file.hash);
            self.by_hash.insert(new_source_file.hash, *index);
            self.source_files[*index] = new_source_file;
            Ok(old_source_file)
        } else {
            Err(ResolveError::FileNotFound(path))
        }
    }
}

/// Trait that can fetch for a file by it's hash value.
pub trait GetSourceByHash {
    /// Find a project file by it's hash value.
    fn get_by_hash(&self, hash: u64) -> ResolveResult<Rc<SourceFile>>;
}

impl GetSourceByHash for Sources {
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

impl std::fmt::Display for Sources {
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

impl std::fmt::Debug for Sources {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (index, source_file) in self.source_files.iter().enumerate() {
            let filename = source_file.filename_as_str();
            let name = self
                .name_from_index(index)
                .unwrap_or(QualifiedName::no_ref(vec![]));
            let hash = source_file.hash;
            writeln!(f, "[{index}] {name:?} {hash:#x} {filename}")?;
        }
        Ok(())
    }
}
