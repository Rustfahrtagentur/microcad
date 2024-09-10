// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::path::PathBuf;

use microcad_core::{
    geo3d::{Triangle, Vertex},
    Exporter, Scalar,
};

pub struct PlyWriter<'a> {
    writer: &'a mut dyn std::io::Write,
}

impl<'a> PlyWriter<'a> {
    pub fn new(mut w: &'a mut dyn std::io::Write) -> std::io::Result<Self> {
        writeln!(&mut w, "ply")?;
        writeln!(&mut w, "format ascii 1.0")?;
        writeln!(&mut w, "comment written by rust-sdf")?;

        Ok(Self { writer: w })
    }

    pub fn header_element_vertex3d(&mut self, len: usize) -> std::io::Result<()> {
        writeln!(&mut self.writer, "element vertex {len}")?;
        writeln!(&mut self.writer, "property float x")?;
        writeln!(&mut self.writer, "property float y")?;
        writeln!(&mut self.writer, "property float z")?;
        writeln!(&mut self.writer, "property float nx")?;
        writeln!(&mut self.writer, "property float ny")?;
        writeln!(&mut self.writer, "property float nz")?;
        Ok(())
    }

    pub fn header_element_vertex3d_with_colors(&mut self, len: usize) -> std::io::Result<()> {
        self.header_element_vertex3d(len)?;
        writeln!(&mut self.writer, "property uchar red")?;
        writeln!(&mut self.writer, "property uchar green")?;
        writeln!(&mut self.writer, "property uchar blue")?;
        Ok(())
    }

    pub fn header_element_face(&mut self, len: usize) -> std::io::Result<()> {
        writeln!(&mut self.writer, "element face {len}")?;
        writeln!(&mut self.writer, "property list uchar int vertex_index")?;
        Ok(())
    }

    pub fn header_end(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.writer, "end_header")?;
        Ok(())
    }

    pub fn vertex(&mut self, v: &Vertex) -> std::io::Result<()> {
        writeln!(
            &mut self.writer,
            "{} {} {} {} {} {}",
            v.pos.x, v.pos.y, v.pos.z, v.normal.x, v.normal.y, v.normal.z
        )?;
        Ok(())
    }

    pub fn vertices(&mut self, v: &[Vertex]) -> std::io::Result<()> {
        for vertex in v {
            self.vertex(vertex)?;
        }
        Ok(())
    }

    pub fn vertex_color<T: std::fmt::Display>(
        &mut self,
        v: &Vertex,
        color: &(T, T, T),
    ) -> std::io::Result<()> {
        writeln!(
            &mut self.writer,
            "{} {} {} {} {} {} {} {} {}",
            v.pos.x,
            v.pos.y,
            v.pos.z,
            v.normal.x,
            v.normal.y,
            v.normal.z,
            color.0,
            color.1,
            color.2
        )?;
        Ok(())
    }

    pub fn tri_face(&mut self, tri: &Triangle<u32>) -> std::io::Result<()> {
        writeln!(&mut self.writer, "3 {} {} {}", tri.0, tri.1, tri.2)?;
        Ok(())
    }

    pub fn tri_faces(&mut self, tri_faces: &[Triangle<u32>]) -> std::io::Result<()> {
        for face in tri_faces {
            self.tri_face(face)?;
        }
        Ok(())
    }
}

pub struct PlyExporter {
    filename: PathBuf,
    precision: Scalar,
}

impl Exporter for PlyExporter {
    fn from_settings(settings: &microcad_core::ExportSettings) -> microcad_core::Result<Self>
    where
        Self: Sized,
    {
        assert!(settings.filename().is_some());

        Ok(Self {
            filename: PathBuf::from(settings.filename().unwrap()),
            precision: settings.render_precision(),
        })
    }

    fn file_extensions(&self) -> Vec<&str> {
        vec!["ply"]
    }

    fn export(&mut self, node: microcad_render::Node) -> Result<(), microcad_core::Error> {
        let mut renderer = microcad_render::mesh::MeshRenderer::new(self.precision);
        use microcad_render::Renderer3D;
        renderer.render_node(node)?;

        let file = std::fs::File::create(&self.filename)?;
        let mut file = std::io::BufWriter::new(file);
        let mut ply_writer = PlyWriter::new(&mut file)?;

        let mesh = renderer.triangle_mesh();
        ply_writer.header_element_vertex3d(mesh.vertices().len())?;
        ply_writer.header_element_face(mesh.triangle_indices().len())?;
        ply_writer.header_end()?;

        ply_writer.vertices(mesh.vertices())?;
        ply_writer.tri_faces(mesh.triangle_indices())?;

        Ok(())
    }
}

