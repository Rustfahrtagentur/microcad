// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL Export

use microcad_core::*;

/// Write into STL file
pub struct StlWriter<'a> {
    writer: &'a mut dyn std::io::Write,
}

impl<'a> StlWriter<'a> {
    /// Create new STL writer
    pub fn new(mut w: &'a mut dyn std::io::Write) -> std::io::Result<Self> {
        writeln!(&mut w, "solid")?;

        Ok(Self { writer: w })
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
        writeln!(self.writer, "endsolid").expect("No error");
    }
}
