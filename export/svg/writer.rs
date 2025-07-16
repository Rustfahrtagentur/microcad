// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::coord;
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
    /// - `bounds`: Clipping
    /// - `scale`: Scale of the output
    pub fn new(
        mut w: Box<dyn std::io::Write>,
        bounds: geo2d::Bounds2D,
        scale: f64,
    ) -> std::io::Result<Self> {
        let r = bounds.rect().unwrap_or(Rect::new(
            coord! {x : 0.0, y: 0.0},
            coord! {x : 10.0, y: 10.0},
        ));
        writeln!(&mut w, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            &mut w,
            "<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            r.min().x * scale,
            r.min().y * scale,
            r.width() * scale,
            r.height() * scale
        )?;
        writeln!(
            &mut w,
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

        writeln!(&mut w, "<g transform='scale({scale})'>")?;

        Ok(Self {
            writer: Box::new(w),
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

    /// Begin a SVG transformation <g>
    pub fn begin_transform(
        &mut self,
        transform: &microcad_core::Mat3,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        let (a, b, c, d, e, f) = (
            transform.x.x,
            transform.x.y,
            transform.y.x,
            transform.y.y,
            transform.z.x,
            transform.z.y,
        );

        self.open_tag(
            &format!("g transform=\"matrix({a} {b} {c} {d} {e} {f})\""),
            attr,
        )
    }

    /// End a SVG transformation group `</g>`.
    pub fn end_transform(&mut self) -> std::io::Result<()> {
        self.end_group()
    }

    /// Finish this SVG. This method is also called in the Drop trait implementation.
    pub fn finish(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "</g>")?;
        writeln!(self.writer, "</svg>")
    }
}

impl Drop for SvgWriter {
    fn drop(&mut self) {
        self.finish().expect("No error")
    }
}
