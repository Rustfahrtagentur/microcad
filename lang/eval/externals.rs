// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External files register

use crate::{eval::*, rc_mut::RcMut, syntax::*};

/// External files register
#[derive(Debug, Default)]
pub struct Externals(std::collections::HashMap<QualifiedName, std::path::PathBuf>);

impl Externals {
    /// Create new resolve context
    pub fn new(search_paths: Vec<std::path::PathBuf>) -> Self {
        Self(Self::search_externals(search_paths))
    }

    pub fn create_namespaces(&self) -> SymbolMap {
        println!("{self}");

        let mut map = SymbolMap::new();

        let basenames = self
            .iter()
            .filter_map(|(name, _)| name.basename())
            .collect::<Vec<_>>();

        basenames.iter().for_each(|basename| {
            let node_id = basename.first().expect("Non-empty qualified name");

            let parent_symbol = if let Some(symbol) = map.get(node_id.id()) {
                symbol.clone()
            } else {
                SymbolNode::new_namespace(node_id.clone())
            };

            let basename = basename.remove_first();
            self._create_namespaces(basename, parent_symbol.clone());

            map.insert(node_id.id().clone(), parent_symbol);
        });

        map
    }

    fn _create_namespaces(
        &self,
        name: QualifiedName,
        parent: RcMut<SymbolNode>,
    ) -> Option<RcMut<SymbolNode>> {
        if name.is_empty() {
            return None;
        }

        println!("Name: {name}");

        let node_id = name.first().expect("Non-empty qualified name");

        if let Some(child) = parent.borrow().fetch(node_id.id()) {
            return Some(child.clone());
        }

        let child = SymbolNode::new_namespace(node_id.clone());
        SymbolNode::insert_child(&parent, child.clone());

        self._create_namespaces(name.remove_first(), child.clone());
        Some(child)
    }

    /// search for an external file which may include a given qualified name
    pub fn fetch_external(&self, name: &QualifiedName) -> EvalResult<&std::path::PathBuf> {
        let mut result: EvalResult<&std::path::PathBuf> =
            Err(EvalError::ExternalSymbolNotFound(name.clone()));
        for (namespace, path) in self.0.iter() {
            if name.is_sub_of(namespace) {
                if let Ok(alt_path) = result {
                    return Err(EvalError::AmbiguousExternal(alt_path.clone(), path.clone()));
                }
                result = Ok(path);
            }
        }
        result
    }

    /// get qualified name by path
    pub fn get_name(&self, path: &std::path::Path) -> EvalResult<&QualifiedName> {
        match self.0.iter().find(|(_, p)| p.as_path() == path) {
            Some((name, _)) => {
                eprintln!("get_name({path:?}) = {name}");

                Ok(name)
            }
            None => Err(EvalError::ExternalPathNotFound(path.to_path_buf())),
        }
    }

    /// searches for external source code files (external modules) in some search paths
    fn search_externals(
        search_paths: Vec<std::path::PathBuf>,
    ) -> std::collections::HashMap<QualifiedName, std::path::PathBuf> {
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
                        file.canonicalize().expect("path not found"),
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
            .try_for_each(|file| writeln!(f, "{} => {}", file.0, file.1.to_string_lossy()))
    }
}

impl std::ops::Deref for Externals {
    type Target = std::collections::HashMap<QualifiedName, std::path::PathBuf>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

#[test]
fn resolve_external_file() {
    let externals = Externals::new(vec!["../lib".into()]);

    assert!(!externals.is_empty());

    println!("{externals}");

    assert!(
        externals
            .fetch_external(&"std::geo2d::circle".into())
            .is_ok()
    );

    assert!(
        externals
            .fetch_external(&"non_std::geo2d::circle".into())
            .is_err()
    );
}

#[test]
fn create_namespaces() {
    let externals = Externals::new(vec!["../lib".into()]);

    assert!(!externals.is_empty());

    let namespaces = externals.create_namespaces();

    println!("{namespaces:#?}");
}
