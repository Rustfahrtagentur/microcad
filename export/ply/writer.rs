// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! PLY Export

use microcad_core::*;

/// PLY writer
pub struct PlyWriter<'a> {
    writer: &'a mut dyn std::io::Write,
}

impl<'a> PlyWriter<'a> {
    /// Create new PLY writer
    pub fn new(mut w: &'a mut dyn std::io::Write) -> std::io::Result<Self> {
        writeln!(&mut w, "ply")?;
        writeln!(&mut w, "format ascii 1.0")?;
        writeln!(&mut w, "comment written by rust-sdf")?;

        Ok(Self { writer: w })
    }

    /// Generate header for vertex 3D
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

    /// Generate header of vertex 3D with colors
    pub fn header_element_vertex3d_with_colors(&mut self, len: usize) -> std::io::Result<()> {
        self.header_element_vertex3d(len)?;
        writeln!(&mut self.writer, "property uchar red")?;
        writeln!(&mut self.writer, "property uchar green")?;
        writeln!(&mut self.writer, "property uchar blue")?;
        Ok(())
    }

    /// Generate face header
    pub fn header_element_face(&mut self, len: usize) -> std::io::Result<()> {
        writeln!(&mut self.writer, "element face {len}")?;
        writeln!(&mut self.writer, "property list uchar int vertex_index")?;
        Ok(())
    }

    /// End header
    pub fn header_end(&mut self) -> std::io::Result<()> {
        writeln!(&mut self.writer, "end_header")?;
        Ok(())
    }

    /// Generate vertex
    pub fn vertex(&mut self, v: &Vertex) -> std::io::Result<()> {
        writeln!(
            &mut self.writer,
            "{} {} {} {} {} {}",
            v.pos.x, v.pos.y, v.pos.z, v.normal.x, v.normal.y, v.normal.z
        )?;
        Ok(())
    }

    /// Generate multiple vertices
    pub fn vertices(&mut self, v: &[Vertex]) -> std::io::Result<()> {
        v.iter().try_for_each(|v| self.vertex(v))
    }

    /// Generate vertex with color
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

    /// Generate tri-face
    pub fn tri_face(&mut self, tri: &Triangle<u32>) -> std::io::Result<()> {
        writeln!(&mut self.writer, "3 {} {} {}", tri.0, tri.1, tri.2)?;
        Ok(())
    }

    /// Generate multiple tri-faces
    pub fn tri_faces(&mut self, tri_faces: &[Triangle<u32>]) -> std::io::Result<()> {
        tri_faces.iter().try_for_each(|f| self.tri_face(f))
    }
}
