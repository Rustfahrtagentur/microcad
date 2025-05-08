// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL Export

use crate::{stl::StlWriter, *};
use microcad_core::Scalar;
use microcad_lang::objects;
use objects::ObjectNode;
use std::path::PathBuf;

/// STL exporter
pub struct StlExporter {
    filename: PathBuf,
    precision: Scalar,
}

impl Exporter for StlExporter {
    fn from_settings(settings: &ExportSettings) -> microcad_core::CoreResult<Self>
    where
        Self: Sized,
    {
        Ok(Self {
            filename: settings.file_path()?,
            precision: settings.render_precision()?,
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["stl"]
    }

    fn export(&mut self, node: ObjectNode) -> microcad_core::CoreResult<()> {
        use microcad_core::geo3d::*;
        let mut renderer = MeshRenderer::new(self.precision);
        let node = objects::bake3d(&mut renderer, node)?;
        renderer.render_node(node)?;

        let file = std::fs::File::create(&self.filename)?;
        let mut file = std::io::BufWriter::new(file);
        let mut writer = StlWriter::new(&mut file)?;

        renderer
            .triangle_mesh
            .triangles()
            .try_for_each(|triangle| writer.write_triangle(&triangle))?;

        Ok(())
    }
}
