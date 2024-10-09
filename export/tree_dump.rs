// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Tree dump exporter

use std::{fs::File, path::PathBuf};

use microcad_render::Node;

use crate::*;

/// Export a node into tree dump file
pub struct TreeDumpExporter {
    filename: PathBuf,
}

impl Exporter for TreeDumpExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(settings.filename().unwrap()),
        })
    }

    fn export(&mut self, node: Node) -> microcad_core::Result<()> {
        let file = File::create(&self.filename)?;
        let mut writer = std::io::BufWriter::new(&file);
        microcad_render::tree::dump(&mut writer, node)?;
        Ok(())
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["tree.dump"]
    }
}
