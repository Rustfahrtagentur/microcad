// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) primitives (SvgWrite trait implementations).

use cgmath::{InnerSpace, Rad};
use geo::{CoordsIter as _, Point, Rect, Translate};
use microcad_core::*;
use microcad_lang::model::{Element, Model, OutputType};

use crate::svg::*;

impl WriteSvg for Edge2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let mapped = self.map_to_canvas(writer.canvas());
        let ((x1, y1), (x2, y2)) = (mapped.0.x_y(), mapped.1.x_y());
        writer.tag(
            &format!("line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\"",),
            attr,
        )
    }
}

impl WriteSvg for Rect {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let mapped = self.map_to_canvas(writer.canvas());
        let x = mapped.min().x;
        let y = mapped.min().y;
        let width = mapped.width();
        let height = mapped.height();

        writer.tag(
            &format!("rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\""),
            attr,
        )
    }
}

impl WriteSvg for Bounds2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        if let Some(rect) = self.rect() {
            rect.write_svg(writer, attr)
        } else {
            Ok(())
        }
    }
}

impl WriteSvg for Circle {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let mapped = self.map_to_canvas(writer.canvas());
        let r = mapped.radius;
        let (cx, cy) = (mapped.offset.x, mapped.offset.y);
        writer.tag(&format!("circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\""), attr)
    }
}

impl WriteSvg for LineString {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let mapped = self.map_to_canvas(writer.canvas());
        let points = mapped.coords().fold(String::new(), |acc, p| {
            acc + &format!("{x},{y} ", x = p.x, y = p.y)
        });
        writer.tag(&format!("polyline points=\"{points}\""), attr)
    }
}

impl WriteSvg for MultiLineString {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.iter()
            .try_for_each(|line_string| line_string.write_svg(writer, attr))
    }
}

impl WriteSvg for Polygon {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        fn line_string_path(l: geo2d::LineString) -> String {
            l.points()
                .enumerate()
                .fold(String::new(), |acc, (i, point)| {
                    let (x, y) = point.x_y();
                    let mut s = String::new();
                    s += if i == 0 { "M" } else { "L" };
                    s += &format!("{x},{y}");
                    if i == l.coords_count() - 1 {
                        s += " Z ";
                    }
                    acc + &s
                })
        }

        let exterior = line_string_path(self.exterior().map_to_canvas(writer.canvas()));
        let interior = self
            .interiors()
            .iter()
            .map(|l| line_string_path(l.map_to_canvas(writer.canvas())))
            .fold(String::new(), |acc, s| acc + &s);

        writer.tag(&format!("path d=\"{exterior} {interior}\""), attr)
    }
}

impl WriteSvg for MultiPolygon {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.iter()
            .try_for_each(|polygon| polygon.write_svg(writer, attr))
    }
}

impl WriteSvg for Geometry2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        match self {
            Geometry2D::LineString(line_string) => line_string.write_svg(writer, attr),
            Geometry2D::MultiLineString(multi_line_string) => {
                multi_line_string.write_svg(writer, attr)
            }
            Geometry2D::Polygon(polygon) => polygon.write_svg(writer, attr),
            Geometry2D::MultiPolygon(multi_polygon) => multi_polygon.write_svg(writer, attr),
            Geometry2D::Rect(rect) => rect.write_svg(writer, attr),
            Geometry2D::Circle(circle) => circle.write_svg(writer, attr),
            Geometry2D::Edge(edge) => edge.write_svg(writer, attr),
        }
    }
}

impl WriteSvg for Model {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        assert_eq!(self.final_output_type(), OutputType::Geometry2D);

        let node_attr: SvgTagAttributes = self.into();
        let attr = node_attr.merge(attr.clone());

        // Render all output geometries.
        self.fetch_output_geometries_2d()
            .iter()
            .try_for_each(|geometry| geometry.write_svg(writer, &attr))?;

        let self_ = self.borrow();
        match &self_.element.value {
            Element::Object(_) | Element::Primitive2D(_) => {
                if !self_.is_empty() {
                    writer.begin_group(&attr)?;
                    self_
                        .children()
                        .try_for_each(|child| child.write_svg(writer, &attr))?;
                    writer.end_group()?;
                }
            }
            Element::Transform(affine_transform) => {
                if !self_.is_empty() {
                    writer
                        .begin_group(&attr.clone().transform_matrix(&affine_transform.mat2d()))?;
                    self_
                        .children()
                        .try_for_each(|child| child.write_svg(writer, &attr))?;
                    writer.end_group()?;
                }
            }
            _ => {}
        }

        Ok(())
    }
}

/// A struct for drawing a centered text.
pub struct CenteredText {
    /// The actual text.
    pub text: String,
    /// Bounding rectangle
    pub rect: Rect,
    /// Font size in mm.
    pub font_size: Scalar,
}

impl WriteSvg for CenteredText {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let (x, y) = self.rect.center().x_y().map_to_canvas(writer.canvas());
        writer.open_tag(
            format!(r#"text x="{x}" y="{y}" dominant-baseline="middle" text-anchor="middle""#,)
                .as_str(),
            &attr.clone().font_size_mm(self.font_size),
        )?;
        writer.with_indent(&self.text)?;
        writer.close_tag("text")
    }
}

/// A struct for drawing a grid.
pub struct Grid {
    /// Grid bounds.
    pub bounds: Bounds2D,

    /// Grid cell size.
    pub cell_size: Size2D,
}

impl Default for Grid {
    fn default() -> Self {
        Self {
            bounds: Bounds2D::default(),
            cell_size: Size2D {
                width: 1.0,
                height: 1.0,
            },
        }
    }
}

impl WriteSvg for Grid {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let rect = self
            .bounds
            .rect()
            .unwrap_or(writer.canvas().rect)
            .map_to_canvas(writer.canvas());
        writer.begin_group(attr)?;

        rect.write_svg(writer, &SvgTagAttributes::default())?;

        let mut left = rect.min().x;
        let right = rect.max().x;
        while left <= right {
            Edge2D(
                geo::Point::new(left, rect.min().y),
                geo::Point::new(left, rect.max().y),
            )
            .map_to_canvas(writer.canvas())
            .write_svg(writer, &SvgTagAttributes::default())?;
            left += self.cell_size.width;
        }

        let mut bottom = rect.min().y;
        let top = rect.max().y;
        while bottom <= top {
            Edge2D(
                geo::Point::new(rect.min().x, bottom),
                geo::Point::new(rect.max().x, bottom),
            )
            .map_to_canvas(writer.canvas())
            .write_svg(writer, &SvgTagAttributes::default())?;
            bottom += self.cell_size.height;
        }

        writer.end_group()?;

        Ok(())
    }
}

/// A struct for drawing a background.
pub struct Background;

impl WriteSvg for Background {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let bounds = writer.canvas().rect;
        bounds.write_svg(writer, attr)
    }
}

/// A measure to measure a length of an edge.
pub struct EdgeLengthMeasure {
    // Optional name for this measure.
    name: Option<String>,
    // Edge.
    edge: Edge2D,
    // Offset (default = 10mm).
    offset: Scalar,
}

impl EdgeLengthMeasure {
    /// Height measure of a rect.
    pub fn height(rect: &Rect, offset: Scalar, name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.into()),
            edge: Edge2D(
                geo::Point::new(rect.min().x, rect.min().y),
                geo::Point::new(rect.min().x, rect.max().y),
            ),
            offset,
        }
    }

    /// Width measure of a rect.
    pub fn width(rect: &Rect, offset: Scalar, name: Option<&str>) -> Self {
        Self {
            name: name.map(|s| s.into()),
            edge: Edge2D(
                geo::Point::new(rect.min().x, rect.min().y),
                geo::Point::new(rect.max().x, rect.min().y),
            ),
            offset,
        }
    }
}

impl WriteSvg for EdgeLengthMeasure {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let length = self.edge.vec().magnitude();

        writer.begin_group(&attr.clone().transform_matrix(&self.edge.matrix()))?;

        let center = self.offset / 2.0;
        let bottom_left = Point::new(0.0, 0.0);
        let bottom_right = Point::new(length, 0.0);
        let top_left = Point::new(0.0, center);
        let top_right = Point::new(length, center);

        Edge2D(top_left, top_right).write_svg(
            writer,
            &attr.clone().marker_start("arrow").marker_end("arrow"),
        )?;

        Edge2D(bottom_left, Point::new(0.0, center * 1.5)).write_svg(writer, attr)?;
        Edge2D(bottom_right, Point::new(length, center * 1.5)).write_svg(writer, attr)?;

        CenteredText {
            text: format!(
                "{name}{length:.2}mm",
                name = match &self.name {
                    Some(name) => format!("{name} = "),
                    None => String::new(),
                },
            ),
            rect: Rect::new(bottom_left, top_right).translate(0.0, center),
            font_size: 8.0,
        }
        .write_svg(writer, attr)?;

        writer.end_group()
    }
}

/// A radius measure with an offset.
pub struct RadiusMeasure {
    /// Name of this measurement.
    pub name: Option<String>,
    /// Circle to measure.
    pub circle: Circle,
    /// Angle of the measurement.
    pub angle: Rad<Scalar>,
}

impl WriteSvg for RadiusMeasure {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let edge = Edge2D::radius_edge(&self.circle, &self.angle);
        edge.write_svg(writer, &attr.clone().marker_end("arrow"))?;
        let center = edge.center();

        CenteredText {
            text: format!(
                "{name}{radius:.2}mm",
                name = match &self.name {
                    Some(name) => format!("{name} = "),
                    None => String::new(),
                },
                radius = self.circle.radius,
            ),
            rect: Rect::new(center, center),
            font_size: 8.0,
        }
        .write_svg(writer, attr)?;

        Ok(())
    }
}

/// Size measure of a bounds.
pub struct SizeMeasure {
    /// Bounds to measure.
    _bounds: Bounds2D,
    /// Width measure
    width: Option<EdgeLengthMeasure>,
    /// Height measure
    height: Option<EdgeLengthMeasure>,
}

impl SizeMeasure {
    /// Size measure for something that has bounds.
    pub fn bounds<T: FetchBounds2D>(bounds: &T) -> Self {
        let bounds = bounds.fetch_bounds_2d();

        if let Some(rect) = bounds.rect() {
            Self {
                _bounds: bounds.clone(),
                width: Some(EdgeLengthMeasure::width(rect, 10.0, None)),
                height: Some(EdgeLengthMeasure::height(rect, 10.0, None)),
            }
        } else {
            Self {
                _bounds: bounds.clone(),
                width: None,
                height: None,
            }
        }
    }
}

impl WriteSvg for SizeMeasure {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        if let Some(width) = &self.width {
            width.write_svg(writer, attr)?;
        }
        if let Some(height) = &self.height {
            height.write_svg(writer, attr)?;
        }
        Ok(())
    }
}
