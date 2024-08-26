use std::io::Write;

use microcad_core::geo3d::{Triangle, Vertex};

pub struct PlyWriter<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> PlyWriter<'a> {
    pub fn new(mut w: &'a mut dyn Write) -> std::io::Result<Self> {
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
