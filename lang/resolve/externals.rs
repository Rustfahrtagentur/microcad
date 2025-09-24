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
                    make_symbol_name(
                        file.strip_prefix(search_path)
                            .expect("cannot strip search path from file name"),
                    ),
                    file.canonicalize().expect("path not found"),
                );
            });
        });

        externals
    }

    /// Scan in a specified path for all available files with one of the given extensions.
    fn scan_path(search_path: impl AsRef<std::path::Path>) -> Vec<std::path::PathBuf> {
        use scan_dir::ScanDir;
        let mut files = ScanDir::files()
            .read(search_path.as_ref(), |iter| {
                iter.map(|(entry, _)| entry.path())
                    .filter(is_microcad_file)
                    .collect::<Vec<_>>()
            })
            .expect("scan_path failed");

        files.append(
            &mut ScanDir::dirs()
                .read(search_path, |iter| {
                    iter.map(|(entry, _)| entry.path())
                        .filter_map(|p| match find_mod_dir_file(p) {
                            Ok(Some(file)) => Some(file),
                            _ => None,
                        })
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

fn make_symbol_name(relative_path: impl AsRef<std::path::Path>) -> QualifiedName {
    let path = relative_path.as_ref();
    let stem = path.file_stem().map(|s| s.to_string_lossy().to_string());
    let name = if stem == Some("mod".into()) {
        path.parent().expect("mod file without parent folder")
    } else {
        path
    };
    name.iter()
        .map(|id| Identifier::no_ref(id.to_string_lossy().as_ref()))
        .collect()
}

fn find_mod_dir_files(path: impl AsRef<std::path::Path>) -> ResolveResult<Vec<std::path::PathBuf>> {
    Ok(scan_dir::ScanDir::files().read(path, |iter| {
        iter.map(|(ref entry, _)| entry.path())
            .filter(is_mod_file)
            .collect::<Vec<_>>()
    })?)
}

fn find_mod_dir_file(
    path: impl AsRef<std::path::Path>,
) -> ResolveResult<Option<std::path::PathBuf>> {
    let files = find_mod_dir_files(path)?;
    if let Some(file) = files.first() {
        match files.len() {
            1 => Ok(Some(file.clone())),
            _ => Err(ResolveError::AmbiguousExternals(files)),
        }
    } else {
        Ok(None)
    }
}

/// Return `true` if given path has a valid microcad extension
fn is_microcad_file(p: &std::path::PathBuf) -> bool {
    p.is_file()
        && p.extension()
            .map(|ext| {
                MICROCAD_EXTENSIONS
                    .iter()
                    .any(|extension| *extension == ext)
            })
            .unwrap_or(false)
}

/// Return `true` if given path is a file called `mod` plus a valid microcad extension
fn is_mod_file(p: &std::path::PathBuf) -> bool {
    is_microcad_file(p)
        && p.file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s == "mod")
}

/// Find a module file at the [path].
///
/// File stem must match [id] and have a valid microcad file extension:
///
/// - <path>`/`<id>`.`*ext*
///
pub fn find_source_file(
    path: impl AsRef<std::path::Path>,
    id: &Identifier,
) -> ResolveResult<std::path::PathBuf> {
    // Can"t really use ScanDir here because we need to be aware of ambiguity
    use std::fs;
    let files: Vec<_> = fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter_map(|p| {
            if p.is_file() {
                Some(p)
            } else if p.is_symlink() {
                todo!("symlink as external")
            } else {
                None
            }
        })
        .filter(is_microcad_file)
        .filter(matches_id(id))
        .collect();

    if let Some(file) = files.first() {
        match files.len() {
            1 => Ok(file.clone()),
            _ => Err(ResolveError::AmbiguousExternal(id.clone(), files)),
        }
    } else {
        Err(ResolveError::ExternalNotFound(id.clone()))
    }
}

pub fn find_source_files(
    path: impl AsRef<std::path::Path>,
) -> ResolveResult<Vec<std::path::PathBuf>> {
    // Can"t really use ScanDir here because we need to be aware of ambiguity
    Ok(std::fs::read_dir(path)?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .filter_map(|p| {
            if p.is_file() {
                Some(p)
            } else if p.is_symlink() {
                todo!("symlink as external")
            } else {
                None
            }
        })
        .filter(is_microcad_file)
        .collect())
}

/// Returns a closure which matches the file stem of a [path] with [id].
fn matches_id(id: &Identifier) -> impl Fn(&std::path::PathBuf) -> bool {
    |p| {
        p.file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s == &id.to_string())
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
