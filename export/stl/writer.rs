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
    pub fn write_triangle(&mut self, tri: &Triangle<&cgmath::Vector3<f32>>) -> std::io::Result<()> {
        let n = tri.normal();
        writeln!(&mut self.writer, "facet normal {} {} {}", n.x, n.y, n.z)?;
        writeln!(&mut self.writer, "\touter loop")?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.0.x, tri.0.y, tri.0.z
        )?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.1.x, tri.1.y, tri.1.z
        )?;
        writeln!(
            &mut self.writer,
            "\t\tvertex {} {} {}",
            tri.2.x, tri.2.y, tri.2.z
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
