// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Source file cache

use crate::parse::{GetSourceFileByHash, SourceFile};

/// Source file cache, used to keep track of loaded source files
#[derive(Clone, Debug, Default)]
pub struct SourceFileCache {
    /// Source files by hash
    source_files: std::collections::HashMap<u64, std::rc::Rc<SourceFile>>,
    /// Source files by name
    source_files_by_name: std::collections::HashMap<String, std::rc::Rc<SourceFile>>,
}

/// Source file cache, used to keep track of loaded source files
impl SourceFileCache {
    /// Add a new source file to the cache
    pub fn add(&mut self, source_file: std::rc::Rc<SourceFile>) {
        if let std::collections::hash_map::Entry::Vacant(e) =
            self.source_files.entry(source_file.hash())
        {
            e.insert(source_file.clone());
            self.source_files_by_name
                .insert(source_file.filename_as_str().to_string(), source_file);
        }
    }

    /// Get source file by its filename
    pub fn get_source_file_by_name(&self, name: &str) -> Option<&SourceFile> {
        self.source_files_by_name.get(name).map(|s| s.as_ref())
    }
}

impl GetSourceFileByHash for SourceFileCache {
    fn get_source_file_by_hash(&self, hash: u64) -> Option<&SourceFile> {
        self.source_files.get(&hash).map(|s| s.as_ref())
    }
}
