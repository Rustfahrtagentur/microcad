// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External files register

use crate::{resolve::*, syntax::*, MICROCAD_EXTENSIONS};
use derive_more::Deref;

/// External files register.
///
/// A map of *qualified name* -> *source file path* which is generated at creation
/// by scanning in the given `search_paths`.
#[derive(Debug, Default, Deref)]
pub struct Externals(std::collections::HashMap<QualifiedName, std::path::PathBuf>);

impl Externals {
    /// Creates externals list.
    ///
    /// Recursively scans given `search_paths` for µcad files but files will not be loaded.
    /// # Arguments
    /// - `search_paths`: Paths to search for any external files.
    pub fn new(search_paths: &[std::path::PathBuf]) -> Self {
        let no_search_paths = search_paths.is_empty();
        let new = Self(Self::search_externals(search_paths));
        if no_search_paths {
            log::info!("No external search paths were given");
        } else if new.is_empty() {
            log::warn!("Did not find any externals in any search path");
        } else {
            log::info!("Found {} external modules", new.len());
            log::trace!("Externals:\n{new}");
        }
        new
    }

    /// Search for an external file which may include a given qualified name.
    ///
    /// # Arguments
    /// - `name`: Qualified name expected to find.
    pub fn fetch_external(
        &self,
        name: &QualifiedName,
    ) -> ResolveResult<(QualifiedName, std::path::PathBuf)> {
        log::trace!("fetching {name} from externals");

        if let Some(found) = self
            .0
            .iter()
            // filter all files which might include name
            .filter(|(n, _)| name.is_sub_of(n))
            // find the file which has the longest name match
            .max_by_key(|(name, _)| name.len())
            // clone the references
            .map(|(name, path)| ((*name).clone(), (*path).clone()))
        {
            return Ok(found);
        }

        Err(ResolveError::ExternalSymbolNotFound(name.clone()))
    }

    /// Get qualified name by path
    pub fn get_name(&self, path: &std::path::Path) -> ResolveResult<&QualifiedName> {
        match self.0.iter().find(|(_, p)| p.as_path() == path) {
            Some((name, _)) => {
                log::trace!("got name of {path:?} => {name}");
                Ok(name)
            }
            None => Err(ResolveError::ExternalPathNotFound(path.to_path_buf())),
        }
    }

    /// Searches for external source code files (*external modules*) in given *search paths*.
    fn search_externals(
        search_paths: &[std::path::PathBuf],
    ) -> std::collections::HashMap<QualifiedName, std::path::PathBuf> {
        let mut externals = std::collections::HashMap::new();
        search_paths.iter().for_each(|search_path| {
            Self::scan_path(search_path.clone(), MICROCAD_EXTENSIONS)
                .iter()
                .for_each(|file| {
                    externals.insert(
                        (*file
                            .strip_prefix(search_path)
                            .expect("cannot strip search path from file name")
                            .with_extension(""))
                        .into(),
                        file.canonicalize().expect("path not found"),
                    );
                });
        });

        externals
    }

    /// Scan in a specified path for all available files with one of the given extensions.
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
        let mut v = self.0.iter().collect::<Vec<_>>();
        // sort for better readability
        v.sort();
        v.iter()
            .try_for_each(|file| writeln!(f, "{} => {}", file.0, file.1.to_string_lossy()))
    }
}

#[test]
fn resolve_external_file() {
    let externals = Externals::new(&["../lib".into()]);

    assert!(!externals.is_empty());

    log::trace!("{externals}");

    assert!(externals
        .fetch_external(&"std::geo2d::Circle".into())
        .is_ok());

    assert!(externals
        .fetch_external(&"non_std::geo2d::Circle".into())
        .is_err());
}
