// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

mod attributes;
pub mod exporter;
mod primitives;
pub mod writer;

#[cfg(test)]
mod tests;

pub use attributes::SvgTagAttributes;
pub use exporter::*;
pub use primitives::*;
pub use writer::*;

/// Trait to write something into an SVG.
pub trait WriteSvg {
    /// Write SVG tags.
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()>;
}
