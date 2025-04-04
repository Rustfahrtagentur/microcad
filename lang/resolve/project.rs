// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! µcad project

use crate::resolve::*;

/// A project bundling all dependant files of a root source file to a project which can be evaluated
#[allow(unused)]
pub struct Project {
    files: SourceCache,
}

impl Project {
    /// Load and parse a root source file and all it's externals it dependents from
    /// - `root_file`: The root source file path
    /// - `search_paths`: Paths to search for external source files
    ///   (see [crate::MICROCAD_EXTENSIONS] for accepted file extensions)
    pub fn load(root_file: impl AsRef<std::path::Path>) -> ResolveResult<Self> {
        // load root file from path
        let root_file = SourceFile::load(root_file)?;
        root_file.resolve(None)?;

        Ok(Self {
            files: SourceCache::new(root_file),
        })
    }
}
