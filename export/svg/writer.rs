// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::Point;
use microcad_core::*;

use crate::svg::{SvgTagAttributes, canvas::Canvas};

/// SVG writer.
pub struct SvgWriter {
    /// The writer (e.g. a file).
    writer: Box<dyn std::io::Write>,
    /// Indentation level.
    level: usize,
    /// The canvas.
    canvas: Canvas,
}

impl SvgWriter {
    /// Create new SvgWriter
    /// # Arguments
    /// - `w`: Output writer
    /// - `size`: Size of the canvas.
    /// - `scale`: Scale of the output
    pub fn new_canvas(
        mut writer: Box<dyn std::io::Write>,
        size: Option<Size2D>,
        content_rect: Rect,
    ) -> std::io::Result<Self> {
        let size = match size {
            Some(size) => size,
            None => Size2D {
                width: content_rect.width(),
                height: content_rect.height(),
            },
        };
        let x = 0;
        let y = 0;
        let w = size.width as i64;
        let h = size.height as i64;
        let canvas = Canvas::new_centered_content(size, content_rect);

        writeln!(&mut writer, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            &mut writer,
            "<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{x} {y} {w} {h}' width='{w}mm' height='{h}mm'>",
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
            canvas,
        })
    }

    /// Return reference to canvas.
    pub fn canvas(&self) -> &Canvas {
        &self.canvas
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
