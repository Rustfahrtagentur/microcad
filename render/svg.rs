// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::CoordsIter;
use microcad_core::{CoreError, Scalar, *};

/// Write SVG
pub struct SvgWriter {
    writer: Box<dyn std::io::Write>,
}

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

/// SVG renderer state
#[derive(Default)]
pub struct SvgRendererState {
    fill: Option<String>,
    stroke: Option<String>,
    stroke_width: Option<Scalar>,
}

/// SVG renderer
pub struct SvgRenderer {
    writer: Option<SvgWriter>,
    precision: Scalar,
    scale: Scalar,
    bounds: geo2d::Rect,
    state: SvgRendererState,
}

impl SvgRenderer {
    /// Set output
    pub fn set_output(&mut self, file: Box<dyn std::io::Write>) -> std::io::Result<()> {
        self.writer = Some(SvgWriter::new(Box::new(file), self.bounds, self.scale)?);
        Ok(())
    }

    /// Return writer
    fn writer(&mut self) -> &mut SvgWriter {
        self.writer.as_mut().unwrap()
    }

    fn render_state_to_style(&self) -> String {
        let mut style = String::new();
        if let Some(fill) = &self.state.fill {
            style.push_str(&format!("fill:{};", fill));
        }
        if let Some(stroke) = &self.state.stroke {
            style.push_str(&format!("stroke:{};", stroke));
        }
        if let Some(stroke_width) = self.state.stroke_width {
            style.push_str(&format!("stroke-width:{};", stroke_width));
        }
        style
    }
}

impl Default for SvgRenderer {
    fn default() -> Self {
        Self {
            writer: None,
            precision: 0.1,
            bounds: geo2d::Rect::new(geo2d::Point::new(0.0, 0.0), geo2d::Point::new(100.0, 100.0)),
            scale: 1.0,
            state: SvgRendererState::default(),
        }
    }
}

impl microcad_core::Renderer for SvgRenderer {
    fn precision(&self) -> Scalar {
        self.precision
    }

    fn change_render_state(&mut self, key: &str, value: &str) -> microcad_core::CoreResult<()> {
        match key {
            "fill" => self.state.fill = Some(value.to_string()),
            "stroke" => self.state.stroke = Some(value.to_string()),
            "stroke-width" => {
                self.state.stroke_width = Some(value.parse()?);
            }
            _ => return Err(CoreError::NotImplemented),
        }
        Ok(())
    }
}

impl geo2d::Renderer for SvgRenderer {
    fn multi_polygon(
        &mut self,
        multi_polygon: &geo2d::MultiPolygon,
    ) -> microcad_core::CoreResult<()> {
        let style = self.render_state_to_style();
        self.writer().multi_polygon(multi_polygon, &style)?;
        Ok(())
    }

    fn render_node(&mut self, node: microcad_core::geo2d::Node) -> microcad_core::CoreResult<()> {
        let inner = node.borrow();
        use microcad_core::geo2d::NodeInner;

        match &*inner {
            NodeInner::Group => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
            }
            NodeInner::Geometry(geometry) => self.render_geometry(geometry)?,
            NodeInner::Transform(transform) => {
                self.writer().begin_transform(transform)?;
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
                self.writer().end_transform()?;
            }
        };

        Ok(())
    }
}

#[test]
fn svg_write() {
    // Write to file test.svg
    let file = std::fs::File::create("svg_write.svg").unwrap();

    let mut svg = SvgWriter::new(
        Box::new(file),
        geo::Rect::new(geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0)),
        1.0,
    )
    .unwrap();

    let rect = geo::Rect::new(geo::Point::new(10.0, 10.0), geo::Point::new(20.0, 20.0));
    svg.rect(&rect, "fill:blue;").unwrap();

    let circle = geo::Point::new(50.0, 50.0);
    svg.circle(&circle, 10.0, "fill:red;").unwrap();

    let line = (geo::Point::new(0.0, 0.0), geo::Point::new(100.0, 100.0));
    svg.line(line.0, line.1, "stroke:black;").unwrap();
}
