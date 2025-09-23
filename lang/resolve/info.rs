use crate::{resolve::*, syntax::*, MICROCAD_EXTENSIONS};

/// Return `true` if given path has a valid microcad extension
pub(super) fn is_microcad_extension(p: &std::path::PathBuf) -> bool {
    p.extension()
        .map(|ext| {
            MICROCAD_EXTENSIONS
                .iter()
                .any(|extension| *extension == ext)
        })
        .unwrap_or(false)
}

/// Return `true` if given path is a file called `mod` plus a valid microcad extension
pub(super) fn is_mod_file(p: &std::path::PathBuf) -> bool {
    is_microcad_extension(p)
        && p.file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s == "mod")
}

/// Returns a closure which matches the file stem of a [path] with [id].
pub(super) fn matches_id(id: &Identifier) -> impl Fn(&std::path::PathBuf) -> bool {
    |p| {
        p.file_stem()
            .and_then(|s| s.to_str())
            .is_some_and(|s| s == &id.to_string())
    }
}

/// Find a module file.
///
/// Module files might be on of the following:
///
/// - <path>`/`<id>`.`*ext*
/// - <path>`/`<id>`/mod.`*ext*
///
/// *ext* = any valid microcad file extension.
pub(super) fn find_mod_file(
    path: impl AsRef<std::path::Path>,
    id: &Identifier,
) -> ResolveResult<std::path::PathBuf> {
    let dir = find_source_file(
        path.as_ref().with_file_name(id.to_string()),
        &Identifier::no_ref("mod"),
    );
    let file = find_source_file(path, id);

    match (file, dir) {
        (Ok(_), Ok(_)) => todo!("ambiguous"),
        (Ok(file), Err(ResolveError::ExternalNotFound(..)))
        | (Err(ResolveError::ExternalNotFound(..)), Ok(file)) => Ok(file),
        (_, Err(err)) | (Err(err), _) => Err(err),
    }
}

/// Find a module file at the [path].
///
/// File stem must match [id] and have a valid microcad file extension:
///
/// - <path>`/`<id>`.`*ext*
///
pub(super) fn find_source_file(
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
        .filter(is_microcad_extension)
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

pub(super) fn find_source_files(
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
        .filter(is_microcad_extension)
        .collect())
}
