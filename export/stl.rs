// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL Export

use std::path::PathBuf;

use crate::*;
use microcad_core::{
    geo3d::{Triangle, Vertex},
    Scalar,
};
use microcad_lang::objecttree;
use objecttree::ObjectNode;

/// Write into STL file
pub struct StlWriter<'a> {
    writer: &'a mut dyn std::io::Write,
}

impl<'a> StlWriter<'a> {
    /// Create new STL writer
    pub fn new(mut w: &'a mut dyn std::io::Write) -> Self {
        writeln!(&mut w, "solid").unwrap();

        Self { writer: w }
    }

    /// Write triangle
    pub fn write_triangle(&mut self, tri: &Triangle<&Vertex>) -> std::io::Result<()> {
        let n = tri.normal();
        writeln!(&mut self.writer, "facet normal {} {} {}", n.x, n.y, n.z)?;
        writeln!(&mut self.writer, "\touter loop")?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.0.pos.x, tri.0.pos.y, tri.0.pos.z
        )?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.1.pos.x, tri.1.pos.y, tri.1.pos.z
        )?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.2.pos.x, tri.2.pos.y, tri.2.pos.z
        )?;
        writeln!(&mut self.writer, "\tendloop")?;
        writeln!(&mut self.writer, "endfacet")?;
        Ok(())
    }
}

impl Drop for StlWriter<'_> {
    fn drop(&mut self) {
        writeln!(self.writer, "endsolid").unwrap();
    }
}

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
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(if let Some(filename) = settings.filename() {
                filename
            } else {
                return Err(CoreError::NoFilenameSpecifiedForExport);
            }),
            precision: settings.render_precision()?,
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["stl"]
    }

    fn export(&mut self, node: ObjectNode) -> microcad_core::CoreResult<()> {
        use microcad_core::geo3d::*;
        let mut renderer = MeshRenderer::new(self.precision);
        let node = objecttree::bake3d(&mut renderer, node)?;
        renderer.render_node(node)?;

        let file = std::fs::File::create(&self.filename)?;
        let mut file = std::io::BufWriter::new(file);
        let mut writer = StlWriter::new(&mut file);

        renderer
            .triangle_mesh
            .triangles()
            .try_for_each(|triangle| writer.write_triangle(&triangle))?;

        Ok(())
    }
}
