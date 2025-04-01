use crate::{resolve::*, syntax::*};

/// Context while resolving a source file
#[derive(Debug)]
pub struct ResolveContext {
    externals: std::collections::HashMap<QualifiedName, std::path::PathBuf>,
}

impl ResolveContext {
    /// Create new resolve context
    pub fn new(search_paths: Vec<std::path::PathBuf>) -> Self {
        Self {
            externals: Self::search_externals(search_paths),
        }
    }

    /// search for an external file which may include a given qualified name
    pub fn fetch_external(&self, name: QualifiedName) -> ResolveResult<&std::path::PathBuf> {
        for (namespace, path) in self.externals.iter() {
            if name.is_sub_of(namespace) {
                eprintln!("found {name} in {namespace}");
                return Ok(path);
            }
        }
        Err(ResolveError::ExternalSymbolNotFound(name))
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

impl std::fmt::Display for ResolveContext {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.externals
            .iter()
            .try_for_each(|file| writeln!(f, "{} => {}", file.0, file.1.to_string_lossy()))
    }
}

#[test]
fn resolve_external_file() {
    let context = ResolveContext::new(vec!["../lib".into()]);

    assert!(!context.externals.is_empty());

    println!("{context}");

    assert!(context
        .fetch_external(QualifiedName::from("std::geo2d::circle"))
        .is_ok());

    assert!(context
        .fetch_external(QualifiedName::from("non_std::geo2d::circle"))
        .is_err());
}
