use std::io::Write;

use microcad_core::geo3d::{Triangle, Vertex};

pub struct StlWriter<'a> {
    writer: &'a mut dyn Write,
}

impl<'a> StlWriter<'a> {
    pub fn new(mut w: &'a mut dyn Write) -> Self {
        writeln!(&mut w, "solid").unwrap();

        Self { writer: w }
    }

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
