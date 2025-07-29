// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! STL Export

mod exporter;
mod primitives;
mod writer;

pub use exporter::*;
pub use writer::*;

/// Trait to write something into an SVG.
pub trait WriteStl {
    /// Write SVG tags.
    fn write_stl(&self, writer: &mut StlWriter) -> std::io::Result<()>;
}
