// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External files register

use crate::{resolve::*, syntax::*};

/// File reference including path an flag about usage
#[derive(Debug)]
pub struct FileRef {
    path: std::path::PathBuf,
    used: bool,
}

impl FileRef {
    /// create new file reference
    pub fn new(path: std::path::PathBuf) -> Self {
        Self { path, used: false }
    }
}

/// External files register
#[derive(Debug)]
pub struct Externals(std::collections::HashMap<QualifiedName, FileRef>);

impl Externals {
    /// Create new resolve context
    pub fn new(search_paths: Vec<std::path::PathBuf>) -> RcMut<Self> {
        RcMut::new(Self(Self::search_externals(search_paths)))
    }

    /// search for an external file which may include a given qualified name
    pub fn fetch_external(&self, name: QualifiedName) -> ResolveResult<&std::path::PathBuf> {
        for (namespace, file_ref) in self.0.iter() {
            if name.is_sub_of(namespace) {
                eprintln!("found {name} in {namespace}");
                return Ok(&file_ref.path);
            }
        }
        Err(ResolveError::ExternalSymbolNotFound(name))
    }

    /// get qualified name by path
    pub fn get_name(&self, path: &std::path::Path) -> ResolveResult<&QualifiedName> {
        match self
            .0
            .iter()
            .find(|(_, file_ref)| file_ref.path.as_path() == path)
        {
            Some((name, _)) => Ok(name),
            None => Err(ResolveError::ExternalPathNotFound(path.to_path_buf())),
        }
    }

    pub fn get_used_files(&self) -> Vec<&std::path::PathBuf> {
        self.0
            .iter()
            .filter(|file| file.1.used)
            .map(|file| &file.1.path)
            .collect()
    }

    /// searches for external source code files (external modules) in some search paths
    fn search_externals(
        search_paths: Vec<std::path::PathBuf>,
    ) -> std::collections::HashMap<QualifiedName, FileRef> {
        let mut externals = std::collections::HashMap::new();
        search_paths.iter().for_each(|search_path| {
            Self::scan_path(search_path.clone(), crate::MICROCAD_EXTENSIONS)
                .iter()
                .for_each(|file| {
                    externals.insert(
                        Self::into_qualified_name(
                            &file
                                .strip_prefix(search_path.clone())
                                .expect("cannot strip search path from file name")
                                .with_extension(""),
                        ),
                        FileRef::new(file.canonicalize().expect("path not found")),
                    );
                });
        });
        externals
    }

    /// convert a path (of an external source code file) into a qualified name
    fn into_qualified_name(path: &std::path::Path) -> QualifiedName {
        use crate::src_ref::*;

        QualifiedName(
            path.iter()
                .map(|id| {
                    Identifier(Refer {
                        value: id.to_string_lossy().into_owned().into(),
                        src_ref: SrcRef(None),
                    })
                })
                .collect(),
        )
    }

    /// scan in a specified path for all available files with one of the given extensions
    fn scan_path(
        search_path: std::path::PathBuf,
        extensions: &[&str],
    ) -> std::vec::Vec<std::path::PathBuf> {
        use scan_dir::ScanDir;

        ScanDir::files()
            .walk(search_path, |iter| {
                iter.filter(|(_, name)| {
                    extensions.iter().any(|extension| name.ends_with(extension))
                })
                .map(|(ref entry, _)| entry.path())
                .collect::<Vec<_>>()
            })
            .expect("scan_path failed")
    }
}

impl std::fmt::Display for Externals {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0
            .iter()
            .try_for_each(|file| writeln!(f, "{} => {}", file.0, file.1.path.to_string_lossy()))
    }
}

impl std::ops::Deref for Externals {
    type Target = std::collections::HashMap<QualifiedName, FileRef>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn resolve_external_file() {
    let externals = Externals::new(vec!["../lib".into()]);
    let externals = externals.borrow();

    assert!(!externals.is_empty());

    println!("{externals}");

    assert!(externals
        .fetch_external(QualifiedName::from("std::geo2d::circle"))
        .is_ok());

    assert!(externals
        .fetch_external(QualifiedName::from("non_std::geo2d::circle"))
        .is_err());
}
