// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! SVG Export

use std::path::PathBuf;

use microcad_render::svg::SvgRenderer;

use crate::*;

/// SVG exporter
pub struct SvgExporter {
    filename: PathBuf,
}

impl Exporter for SvgExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(settings.filename().unwrap()),
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["svg"]
    }

    fn export(&mut self, node: microcad_render::ModelNode) -> microcad_core::Result<()> {
        let file = std::fs::File::create(&self.filename)?;

        use microcad_core::geo2d::Renderer;
        let mut renderer = SvgRenderer::default();
        renderer.set_output(Box::new(file))?;
        let node = microcad_core::render::tree::bake2d(&mut renderer, node)?;

        renderer.render_node(node)
    }
}
