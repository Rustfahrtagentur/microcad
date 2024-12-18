// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tree dump exporter

use std::{fs::File, path::PathBuf};

use crate::*;
use microcad_lang::objecttree::ObjectNode;

/// Export a node into tree dump file
pub struct TreeDumpExporter {
    filename: PathBuf,
}

impl Exporter for TreeDumpExporter {
    fn from_settings(settings: &ExportSettings) -> Result<Self, CoreError>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(if let Some(filename) = settings.filename() {
                filename
            } else {
                return Err(CoreError::NoFilenameSpecifiedForExport);
            }),
        })
    }

    fn export(&mut self, node: ObjectNode) -> Result<(), CoreError> {
        // TODO Make this a separate function
        let path = std::path::absolute(&self.filename)?;

        if let Some(containing_dir) = path.parent() {
            // If we want to export "/home/user/export.svg", "/home/user" must exist.
            if !containing_dir.exists() {
                return Err(microcad_core::CoreError::DirectoryDoesNotExist(
                    containing_dir.to_path_buf(),
                ));
            }
        } else {
            panic!("Tried to write to root!");
        }

        let file = File::create(&path)?;
        let mut writer = std::io::BufWriter::new(&file);
        microcad_lang::objecttree::dump(&mut writer, node)?;
        Ok(())
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["tree.dump"]
    }
}
