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
    source_files: Vec<Rc<SourceFile>>,

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
    ) -> ParseResult<Self> {
        let mut source_files = Vec::new();
        let mut by_name = HashMap::new();
        let mut by_hash = HashMap::new();
        let mut by_path = HashMap::new();

        by_hash.insert(root.hash, 0);
        by_path.insert(root.filename(), 0);
        by_name.insert(root.name.clone(), 0);
        source_files.push(root.clone());

        // search for external source files
        let externals = Externals::new(search_paths);

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
                .map(|path| path.as_ref().to_path_buf())
                .collect(),
        })
    }

    /// Return root file.
    pub fn root(&self) -> Rc<SourceFile> {
        self.root.clone()
    }

    /// Creates symbol map from externals.
    fn create_modules(externals: &Externals) -> SymbolMap {
        let mut map = SymbolMap::new();
        externals.iter().for_each(|(basename, _)| {
            let (id, name) = basename.split_first();
            let module = match map.get(&id) {
                Some(symbol) => symbol.clone(),
                _ => Symbol::new(
                    SymbolDefinition::Module(ModuleDefinition::new(Visibility::Public, id.clone())),
                    None,
                ),
            };
            Self::recursive_create_modules(&module, &name);
            map.insert(id.clone(), module);
        });
        map
    }

    fn recursive_create_modules(parent: &Symbol, name: &QualifiedName) -> Option<Symbol> {
        if name.is_empty() {
            return None;
        }

        let node_id = name.first().expect("Non-empty qualified name");
        if let Some(child) = parent.get(node_id) {
            return Some(child.clone());
        }

        let child = Symbol::new(
            SymbolDefinition::Module(ModuleDefinition::new(Visibility::Public, node_id.clone())),
            None,
        );
        Symbol::add_child(parent, child.clone());

        Self::recursive_create_modules(&child, &name.remove_first());
        Some(child)
    }

    /// Create a symbol out of all sources (without resolving them).
    pub fn symbolize(&self) -> ResolveResult<SymbolMap> {
        let symbols = self
            .iter()
            .map(
                |source| match (self.name_from_path(&source.filename()), source.symbolize()) {
                    (Ok(name), Ok(symbol)) => Ok((name, symbol)),
                    (Ok(_), Err(err)) | (Err(err), _) => Err(err),
                },
            )
            .collect::<ResolveResult<Vec<_>>>()?;

        let mut symbol_map = SymbolMap::default();
        for (name, symbol) in symbols {
            if let Some(id) = name.single_identifier() {
                symbol_map.insert(id.clone(), symbol);
            } else {
                todo!()
            }
        }
        Ok(symbol_map)
    }

    /// Create initial symbol map from externals.
    pub fn resolve(&self) -> ResolveResult<SymbolMap> {
        let mut symbols = Self::create_modules(&self.externals);
        symbols.insert(
            self.root().id(),
            Symbol::new(SymbolDefinition::SourceFile(self.root()), None),
        );

        self.source_files
            .iter()
            .try_for_each(|source_file| -> Result<(), ResolveError> {
                let name = &source_file.name;
                log::trace!(
                    "{resolve} file {path:?} [{name}]",
                    resolve = crate::mark!(RESOLVE),
                    path = source_file.filename(),
                );
                let symbol = todo!(); //source_file.resolve()?;

                // search module where to place loaded source file into
                let target = symbols.search(name)?;
                target.move_children(&symbol);

                Ok(())
            })?;

        Ok(symbols)
    }

    /// Return the qualified name of a file by it's path
    pub fn name_from_path(&self, filename: &std::path::Path) -> ResolveResult<QualifiedName> {
        if self.root.filename() == filename {
            Ok(QualifiedName::from_id(self.root.id()))
        } else {
            Ok(self.externals.get_name(filename)?.clone())
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
    pub fn get_name_by_hash(&self, hash: u64) -> ResolveResult<QualifiedName> {
        match self.get_by_hash(hash) {
            Ok(file) => self.name_from_path(&file.filename()),
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
            writeln!(f, "[{index}] {name:?} {hash:#x} {filename}")?;
        }
        Ok(())
    }
}
