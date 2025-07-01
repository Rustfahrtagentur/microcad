// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Builtin FileIoInterface

use std::{collections::HashMap, rc::Rc};

use crate::Id;

/// The [`FileIoInterface`] defines an interface for file import and export.
pub trait FileIoInterface {
    /// Return the id for this IO interface.
    fn id(&self) -> Id;

    /// Return file extensions this IO interface supports.
    fn file_extensions(&self) -> Vec<Id> {
        vec![self.id()]
    }
}

/// Registry to store file IO handlers by id and by file extension.
pub(crate) struct FileIoRegistry<T> {
    /// File IO by ID.
    by_id: std::collections::HashMap<Id, T>,
    /// File IO by file extension.
    by_file_extension: std::collections::HashMap<Id, Vec<T>>,
}

impl<T> Default for FileIoRegistry<T> {
    fn default() -> Self {
        Self {
            by_id: HashMap::default(),
            by_file_extension: HashMap::default(),
        }
    }
}

impl<T: FileIoInterface + ?Sized> FileIoRegistry<Rc<T>> {
    /// Add new importer to the registry.
    ///
    /// TODO Error handling.
    pub fn insert(&mut self, rc: Rc<T>) {
        let id = rc.id();
        assert!(!id.is_empty());

        if self.by_id.contains_key(&id) {
            panic!("Importer already exists");
        }

        self.by_id.insert(id, rc.clone());

        let extensions = rc.file_extensions();
        for ext in extensions {
            if !ext.is_empty() && self.by_file_extension.contains_key(&ext) {
                self.by_file_extension
                    .get_mut(&ext)
                    .expect("Exporter list")
                    .push(rc.clone());
            } else {
                self.by_file_extension.insert(ext, vec![rc.clone()]);
            }
        }
    }

    /// Get file IO by filename.
    pub fn by_filename(&self, filename: impl AsRef<std::path::Path>) -> Vec<Rc<T>> {
        let ext: Id = filename
            .as_ref()
            .extension()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .into();

        self.by_file_extension
            .get(&ext)
            .cloned()
            .unwrap_or_default()
    }

    /// Get file IO by id.
    pub fn by_id(&self, id: &Id) -> Option<Rc<T>> {
        self.by_id.get(id).cloned()
    }
}
