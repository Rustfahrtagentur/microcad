// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SVG Export

use std::path::PathBuf;

use microcad_lang::objecttree::ObjectNode;
use microcad_render::svg::SvgRenderer;

use crate::*;

/// SVG exporter
pub struct SvgExporter {
    filename: PathBuf,
}

impl Exporter for SvgExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::CoreResult<Self>
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

    fn file_extensions(&self) -> Vec<&str> {
        vec!["svg"]
    }

    fn export(&mut self, node: ObjectNode) -> microcad_core::CoreResult<()> {
        let file = std::fs::File::create(&self.filename)?;

        use microcad_core::geo2d::Renderer;
        let mut renderer = SvgRenderer::default();
        renderer.set_output(Box::new(file))?;
        let node = microcad_lang::objecttree::bake2d(&mut renderer, node)?;

        renderer.render_node(node)
    }
}
