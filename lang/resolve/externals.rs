// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! External files register

use crate::{resolve::*, syntax::*, MICROCAD_EXTENSIONS};
use derive_more::Deref;
use scan_dir::ScanDir;
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
    pub fn new(search_paths: &[impl AsRef<std::path::Path>]) -> Self {
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
            .filter(|(n, _)| name.is_within(n))
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
        search_paths: &[impl AsRef<std::path::Path>],
    ) -> std::collections::HashMap<QualifiedName, std::path::PathBuf> {
        let mut externals = std::collections::HashMap::new();
        search_paths.iter().for_each(|search_path| {
            Self::scan_path(search_path).iter().for_each(|file| {
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

    /// Return `true` if given path has a valid microcad extension
    fn is_microcad_extension(p: &impl AsRef<std::path::Path>) -> bool {
        p.as_ref()
            .extension()
            .map(|ext| {
                MICROCAD_EXTENSIONS
                    .iter()
                    .any(|extension| *extension == ext)
            })
            .unwrap_or(false)
    }

    /// Return `true` if given path is a file called `mod` plus a valid microcad extension
    fn is_mod_file(p: &std::path::PathBuf) -> bool {
        Self::is_microcad_extension(p)
            && p.file_stem()
                .and_then(|s| s.to_str())
                .is_some_and(|s| s == "mod")
    }

    fn find_mod_dir_files(
        path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<Vec<std::path::PathBuf>> {
        Ok(ScanDir::files().read(path, |iter| {
            iter.map(|(ref entry, _)| entry.path())
                .filter(Self::is_mod_file)
                .collect::<Vec<_>>()
        })?)
    }

    fn find_mod_dir_file(
        path: impl AsRef<std::path::Path>,
    ) -> ResolveResult<Option<std::path::PathBuf>> {
        let files = Self::find_mod_dir_files(path)?;
        if let Some(file) = files.first() {
            match files.len() {
                1 => Ok(Some(file.clone())),
                _ => Err(ResolveError::AmbiguousExternals(files)),
            }
        } else {
            Ok(None)
        }
    }

    /// Scan in a specified path for all available files with one of the given extensions.
    fn scan_path(search_path: &impl AsRef<std::path::Path>) -> std::vec::Vec<std::path::PathBuf> {
        let mut files = ScanDir::files()
            .read(search_path.clone(), |iter| {
                iter.map(|(ref entry, _)| entry.path())
                    .filter(Self::is_microcad_extension)
                    .collect::<Vec<_>>()
            })
            .expect("scan_path failed");

        files.append(
            &mut ScanDir::dirs()
                .read(search_path, |iter| {
                    iter.map(|(ref entry, _)| entry.path())
                        .filter_map(|path| Self::find_mod_dir_file(&path).ok().flatten())
                        .collect::<Vec<_>>()
                })
                .expect("scan_path failed"),
        );

        files
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
    let externals = Externals::new(&["../lib"]);

    assert!(!externals.is_empty());

    log::trace!("{externals}");

    assert!(externals
        .fetch_external(&"std::geo2d::Circle".into())
        .is_ok());

    assert!(externals
        .fetch_external(&"non_std::geo2d::Circle".into())
        .is_err());
}
