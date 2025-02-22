// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::CoordsIter;
use microcad_core::*;

/// Write SVG
pub struct SvgWriter {
    writer: Box<dyn std::io::Write>,
}

#[allow(dead_code)]
impl SvgWriter {
    /// Create new SvgWriter
    /// # Arguments
    /// - `w`: Output writer
    /// - `bounds`: Clipping
    /// - `scale`: Scale of the output
    pub fn new(
        mut w: Box<dyn std::io::Write>,
        bounds: geo2d::Rect,
        scale: f64,
    ) -> std::io::Result<Self> {
        writeln!(&mut w, "<?xml version='1.0' encoding='UTF-8'?>")?;
        writeln!(
            &mut w,
            "<svg version='1.1' xmlns='http://www.w3.org/2000/svg' viewBox='{} {} {} {}'>",
            bounds.min().x * scale,
            bounds.min().y * scale,
            bounds.width() * scale,
            bounds.height() * scale
        )?;
        writeln!(&mut w, "<g transform='scale({scale})'>")?;

        Ok(Self {
            writer: Box::new(w),
        })
    }

    /// Generate rectangle
    pub fn rect(&mut self, rect: &geo2d::Rect, style: &str) -> std::io::Result<()> {
        let x = rect.min().x;
        let y = rect.min().y;
        let width = rect.width();
        let height = rect.height();
        writeln!(
            self.writer,
            "<rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\" style=\"{style}\"/>"
        )
    }

    /// Begin a SVG transformation <g>
    pub fn begin_transform(&mut self, transform: &microcad_core::Mat3) -> std::io::Result<()> {
        let (a, b, c, d, e, f) = (
            transform.x.x,
            transform.x.y,
            transform.y.x,
            transform.y.y,
            transform.z.x,
            transform.z.y,
        );
        writeln!(
            self.writer,
            "<g transform=\"matrix({a} {b} {c} {d} {e} {f})\">"
        )
    }

    /// End a SVG transformation </g>
    pub fn end_transform(&mut self) -> std::io::Result<()> {
        writeln!(self.writer, "</g>")
    }

    /// Generate circle
    pub fn circle(
        &mut self,
        center: &geo2d::Point,
        radius: f64,
        style: &str,
    ) -> std::io::Result<()> {
        let (cx, cy) = center.x_y();
        writeln!(
            self.writer,
            "<circle cx=\"{cx}\" cy=\"{cy}\" r=\"{radius}\" style=\"{style}\"/>"
        )
    }

    /// Generate line
    pub fn line(&mut self, p1: geo2d::Point, p2: geo2d::Point, style: &str) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (p1.x_y(), p2.x_y());
        writeln!(
            self.writer,
            "<line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\" style=\"{style}\"/>"
        )
    }

    /// Generate polygon
    pub fn polygon(&mut self, polygon: &geo2d::Polygon, style: &str) -> std::io::Result<()> {
        write!(self.writer, "<path d=\"")?;
        for (i, point) in polygon.exterior().points().enumerate() {
            let (x, y) = point.x_y();
            match i {
                0 => write!(self.writer, "M")?,
                _ => write!(self.writer, "L")?,
            }

            write!(self.writer, "{x},{y}", x = x, y = y)?;
            if i == polygon.exterior().coords_count() - 1 {
                write!(self.writer, " Z ")?;
            }
        }
        for interior in polygon.interiors() {
            for (i, point) in interior.points().enumerate() {
                let (x, y) = point.x_y();
                match i {
                    0 => write!(self.writer, "M")?,
                    _ => write!(self.writer, "L")?,
                }

                write!(self.writer, "{x},{y}", x = x, y = y)?;
                if i == interior.coords_count() - 1 {
                    write!(self.writer, " Z ")?;
                }
            }
        }

        writeln!(self.writer, "\" style=\"{style}\"/>")
    }

    /// Generate multiple polygons
    pub fn multi_polygon(
        &mut self,
        multi_polygon: &geo2d::MultiPolygon,
        style: &str,
    ) -> std::io::Result<()> {
        for polygon in multi_polygon {
            self.polygon(polygon, style)?;
        }
        Ok(())
    }
}

impl Drop for SvgWriter {
    fn drop(&mut self) {
        writeln!(self.writer, "</g>").unwrap();
        writeln!(self.writer, "</svg>").unwrap();
    }
}
