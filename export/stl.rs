// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL Export

use std::path::PathBuf;

use crate::*;
use microcad_core::geo3d::{Triangle, Vertex};

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
    pub fn write_triangle(&mut self, tri: &Triangle<Vertex>) -> std::io::Result<()> {
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

impl<'a> Drop for StlWriter<'a> {
    fn drop(&mut self) {
        writeln!(self.writer, "endsolid").unwrap();
    }
}

/// STL exproter
pub struct StlExporter {
    filename: PathBuf,
}

impl Exporter for StlExporter {
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
        vec!["stl"]
    }

    fn export(&mut self, node: microcad_render::Node) -> microcad_core::Result<()> {
        let mut renderer = microcad_render::mesh::MeshRenderer::default();
        use microcad_render::Renderer3D;
        renderer.render_node(node)?;

        let file = std::fs::File::create(&self.filename)?;
        let mut file = std::io::BufWriter::new(file);
        let mut writer = StlWriter::new(&mut file);

        let triangles = renderer.triangle_mesh.fetch_triangles();
        for triangle in triangles {
            writer.write_triangle(&triangle)?;
        }

        Ok(())
    }
}

#[test]
fn test_stl_export() {
    use microcad_render::NodeInner;

    let settings = ExportSettings::with_filename("../test_output/tests/test.stl".to_string());
    use crate::Exporter;
    let mut exporter = StlExporter::from_settings(&settings).unwrap();

    let node = microcad_render::Node::new(NodeInner::Root);

    use microcad_core::geo3d::*;
    let a = Manifold::cube(1.0, 1.0, 1.0);
    let b = Manifold::sphere(1.0, 32);

    let intersection: Geometry = a.intersection(&b).into();

    node.append(microcad_render::Node::new(NodeInner::Geometry3D(
        std::rc::Rc::new(intersection.fetch_mesh().into()),
    )));

    exporter.export(node).unwrap();
}
