// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

mod attributes;
mod canvas;
pub mod exporter;
mod primitives;
pub mod writer;

#[cfg(test)]
mod tests;

pub use attributes::SvgTagAttributes;
pub use canvas::*;
pub use exporter::*;
pub use primitives::*;
pub use writer::*;

/// Trait to write something into an SVG.
pub trait WriteSvg {
    /// Write SVG tags.
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()>;
}

/// Trait to write something into an SVG while mapping it to canvas coordinates.
pub trait WriteSvgMapped: WriteSvg + MapToCanvas {
    /// Map and write SVG tags.
    fn write_svg_mapped(
        &self,
        writer: &mut SvgWriter,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        self.map_to_canvas(writer.canvas()).write_svg(writer, attr)
    }
}
