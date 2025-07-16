// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use microcad_core::*;

use crate::svg::SvgTagAttributes;

/// SVG writer.
pub struct SvgWriter {
    writer: Box<dyn std::io::Write>,
    level: usize,
    bounds: geo2d::Bounds2D,
}

impl SvgWriter {
    /// Create new SvgWriter
    /// # Arguments
    /// - `w`: Output writer
    /// - `size`: Size of the canvas.
    /// - `scale`: Scale of the output
    pub fn new_canvas(
        mut writer: Box<dyn std::io::Write>,
        bounds: Bounds2D,
        _scale: Option<Scalar>,
    ) -> std::io::Result<Self> {
        let x = bounds.rect().map(|r| r.min().x).unwrap_or_default() as i64;
        let y = bounds.rect().map(|r| r.min().y).unwrap_or_default() as i64;
        let w = bounds.width() as i64;
        let h = bounds.height() as i64;

        writeln!(&mut writer, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            &mut writer,
            "<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{x} {y} {w} {h}' width='{w}' height='{h}'>",
        )?;
        writeln!(
            &mut writer,
            r#"
  <defs>
    <!-- A marker to be used as an arrowhead -->
    <marker
      id="arrow"
      viewBox="0 0 10 10"
      refX="5"
      refY="5"
      markerWidth="6"
      markerHeight="6"
      orient="auto-start-reverse">
      <path d="M 0 0 L 10 5 L 0 10 z" />
    </marker>
  </defs>
            "#
        )?;

        Ok(Self {
            writer: Box::new(writer),
            level: 1,
            bounds,
        })
    }

    /// Return reference to bounds.
    pub fn bounds(&self) -> &geo2d::Bounds2D {
        &self.bounds
    }

    fn tag_inner(tag: &str, attr: &SvgTagAttributes) -> String {
        format!(
            "{tag}{attr}",
            attr = if attr.is_empty() {
                String::new()
            } else {
                format!(" {attr}")
            }
        )
    }

    /// Write something into the SVG and consider indentation.
    pub fn with_indent(&mut self, s: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{:indent$}{s}", "", indent = 2 * self.level)
    }

    /// Write a single tag `<tag>`.
    pub fn tag(&mut self, tag: &str, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}/>",
            tag_inner = Self::tag_inner(tag, attr)
        ))
    }

    /// Open a tag `<tag>`
    pub fn open_tag(&mut self, tag: &str, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}>",
            tag_inner = Self::tag_inner(tag, attr)
        ))?;

        self.level += 1;
        Ok(())
    }

    /// Close a tag `</tag>`
    pub fn close_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.level -= 1;
        self.with_indent(format!("</{tag}>").as_str())
    }

    /// Begin a new group `<g>`.
    pub fn begin_group(&mut self, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.open_tag("g", attr)
    }

    /// End a group `</g>`.
    pub fn end_group(&mut self) -> std::io::Result<()> {
        self.close_tag("g")
    }

    /// Finish this SVG. This method is also called in the Drop trait implementation.
    pub fn finish(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "</svg>")
    }
}

impl Drop for SvgWriter {
    fn drop(&mut self) {
        self.finish().expect("No error")
    }
}
