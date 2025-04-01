use crate::{resolve::*, syntax::*};

/// Context while resolving a source file
#[derive(Debug)]
pub struct ResolveContext {
    externals: std::collections::HashMap<QualifiedName, std::path::PathBuf>,
}

impl ResolveContext {
    /// Create new resolve context
    pub fn new(search_paths: Vec<std::path::PathBuf>) -> Self {
        let mut externals = std::collections::HashMap::new();
        for search_path in search_paths {
            for file in Self::scan_path(search_path.clone(), crate::MICROCAD_EXTENSIONS) {
                externals.insert(
                    Self::into_qualified_name(
                        &file
                            .strip_prefix(search_path.clone())
                            .expect("cannot strip search path from file name")
                            .with_extension(""),
                    ),
                    file.canonicalize().expect("path not found"),
                );
            }
        }
        Self { externals }
    }

    /// search for an external file which may include a given qualified name
    pub fn fetch_external(&self, name: QualifiedName) -> ResolveResult<&std::path::PathBuf> {
        for (namespace, path) in self.externals.iter() {
            if name.is_sub_of(namespace) {
                return Ok(path);
            }
        }
        Err(ResolveError::ExternalSymbolNotFound(name))
    }

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
}
