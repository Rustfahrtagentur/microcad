// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) primitives ([`WriteSvg`] trait implementations).

use cgmath::{Deg, InnerSpace};
use geo::{CoordsIter as _, Point, Rect, Translate};
use microcad_core::*;
use microcad_lang::model::{Element, Model, OutputType};

use crate::svg::{attributes::SvgTagAttribute, *};

impl WriteSvg for Edge2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (self.0.x_y(), self.1.x_y());
        writer.tag(
            &format!("line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\"",),
            attr,
        )
    }
}

impl WriteSvgMapped for Edge2D {}

impl WriteSvg for Rect {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let x = self.min().x;
        let y = self.min().y;
        let width = self.width();
        let height = self.height();

        writer.tag(
            &format!("rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\""),
            attr,
        )
    }
}

impl WriteSvgMapped for Rect {}

impl WriteSvg for Bounds2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        if let Some(rect) = self.rect() {
            rect.write_svg(writer, attr)
        } else {
            Ok(())
        }
    }
}

impl WriteSvgMapped for Bounds2D {}

impl WriteSvg for Circle {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let r = self.radius;
        let (cx, cy) = (self.offset.x, self.offset.y);
        writer.tag(&format!("circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\""), attr)
    }
}

impl WriteSvgMapped for Circle {}

impl WriteSvg for LineString {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let points = self.coords().fold(String::new(), |acc, p| {
            acc + &format!("{x},{y} ", x = p.x, y = p.y)
        });
        writer.tag(&format!("polyline points=\"{points}\""), attr)
    }
}

impl WriteSvgMapped for LineString {}

impl WriteSvg for MultiLineString {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.iter()
            .try_for_each(|line_string| line_string.write_svg(writer, attr))
    }
}

impl WriteSvgMapped for MultiLineString {}

impl WriteSvg for Polygon {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        fn line_string_path(l: &geo2d::LineString) -> String {
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

        let exterior = line_string_path(self.exterior());
        let interior = self
            .interiors()
            .iter()
            .map(line_string_path)
            .fold(String::new(), |acc, s| acc + &s);

        writer.tag(&format!("path d=\"{exterior} {interior}\""), attr)
    }
}

impl WriteSvgMapped for Polygon {}

impl WriteSvg for MultiPolygon {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.iter()
            .try_for_each(|polygon| polygon.write_svg(writer, attr))
    }
}

impl WriteSvgMapped for MultiPolygon {}

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

impl WriteSvgMapped for Geometry2D {}

impl WriteSvg for Model {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        assert_eq!(self.final_output_type(), OutputType::Geometry2D);

        let node_attr = attr
            .clone()
            .apply_from_model(self)
            .insert(SvgTagAttribute::class("entity"));

        // Render all output geometries.
        self.fetch_output_geometries_2d()
            .iter()
            .try_for_each(|geometry| {
                geometry.write_svg_mapped(writer, &SvgTagAttributes::default())
            })?;

        let self_ = self.borrow();
        match &self_.element.value {
            Element::Workpiece(_) | Element::Primitive2D(_) => {
                if !self_.is_empty() {
                    writer.begin_group(&node_attr)?;
                    self_.children().try_for_each(|child| {
                        child.write_svg(writer, &SvgTagAttributes::default())
                    })?;
                    writer.end_group()?;
                }
            }
            Element::Transform(affine_transform) => {
                if !self_.is_empty() {
                    let mut mat = affine_transform.mat2d();
                    mat.z.y = -mat.z.y;

                    writer.begin_group(
                        &node_attr
                            .clone()
                            .insert(attributes::SvgTagAttribute::Transform(mat)),
                    )?;
                    self_.children().try_for_each(|child| {
                        child.write_svg(writer, &SvgTagAttributes::default())
                    })?;
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
        let (x, y) = self.rect.center().x_y();
        writer.open_tag(
            format!(r#"text x="{x}" y="{y}" dominant-baseline="middle" text-anchor="middle""#,)
                .as_str(),
            &attr
                .clone()
                .insert(attributes::SvgTagAttribute::FontSizeMM(self.font_size)),
        )?;
        writer.with_indent(&self.text)?;
        writer.close_tag("text")
    }
}

impl WriteSvgMapped for CenteredText {}

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
                width: 10.0,
                height: 10.0,
            },
        }
    }
}

impl WriteSvg for Grid {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let rect = self.bounds.rect().unwrap_or(writer.canvas().rect);
        writer.begin_group(&attr.clone().insert(SvgTagAttribute::class("grid-stroke")))?;

        rect.write_svg(writer, &SvgTagAttributes::default())?;

        let mut left = rect.min().x;
        let right = rect.max().x;
        while left <= right {
            Edge2D(
                geo::Point::new(left, rect.min().y),
                geo::Point::new(left, rect.max().y),
            )
            .write_svg(writer, &SvgTagAttributes::default())?;
            left += self.cell_size.width.map_to_canvas(writer.canvas());
        }

        let mut bottom = rect.min().y;
        let top = rect.max().y;
        while bottom <= top {
            Edge2D(
                geo::Point::new(rect.min().x, bottom),
                geo::Point::new(rect.max().x, bottom),
            )
            .write_svg(writer, &SvgTagAttributes::default())?;
            bottom += self.cell_size.height.map_to_canvas(writer.canvas());
        }

        writer.end_group()?;

        Ok(())
    }
}

/// A struct for drawing a background.
pub struct Background;

impl WriteSvg for Background {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let x = 0;
        let y = 0;
        let width = writer.canvas().size.width;
        let height = writer.canvas().size.height;

        writer.tag(
            &format!("rect class=\"background-fill\" x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\""),
            attr,
        )
    }
}

/// A measure to measure a length of an edge.
pub struct EdgeLengthMeasure {
    // Optional name for this measure.
    name: Option<String>,
    // Original Length
    length: Scalar,
    // Edge.
    edge: Edge2D,
    // Offset (default = 10mm).
    offset: Scalar,
}

impl EdgeLengthMeasure {
    /// Height measure of a rect.
    pub fn height(rect: &Rect, offset: Scalar, name: Option<&str>) -> Self {
        let edge = Edge2D(
            geo::Point::new(rect.min().x, rect.min().y),
            geo::Point::new(rect.min().x, rect.max().y),
        );
        Self {
            name: name.map(|s| s.into()),
            length: edge.vec().magnitude(),
            edge,
            offset: -offset,
        }
    }

    /// Width measure of a rect.
    pub fn width(rect: &Rect, offset: Scalar, name: Option<&str>) -> Self {
        let edge = Edge2D(
            geo::Point::new(rect.min().x, rect.min().y),
            geo::Point::new(rect.max().x, rect.min().y),
        );

        Self {
            name: name.map(|s| s.into()),
            length: edge.vec().magnitude(),
            edge,
            offset,
        }
    }
}

impl MapToCanvas for EdgeLengthMeasure {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self {
            name: self.name.clone(),
            length: self.length,
            edge: self.edge.map_to_canvas(canvas),
            offset: self.offset.map_to_canvas(canvas),
        }
    }
}

impl WriteSvg for EdgeLengthMeasure {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let edge_length = self.edge.vec().magnitude();

        use attributes::SvgTagAttribute::*;

        writer.begin_group(&attr.clone().insert(Transform(self.edge.matrix())))?;

        let center = self.offset / 2.0;
        let bottom_left = Point::new(0.0, 0.0);
        let bottom_right = Point::new(edge_length, 0.0);
        let top_left = Point::new(0.0, center);
        let top_right = Point::new(edge_length, center);

        writer.begin_group(&attr.clone().insert(SvgTagAttribute::class("measure")))?;
        Edge2D(bottom_left, Point::new(0.0, center * 1.5)).write_svg(writer, attr)?;
        Edge2D(bottom_right, Point::new(edge_length, center * 1.5)).write_svg(writer, attr)?;
        Edge2D(top_left, top_right).shorter(1.5).write_svg(
            writer,
            &attr
                .clone()
                .insert(MarkerStart("arrow".into()))
                .insert(MarkerEnd("arrow".into())),
        )?;
        writer.end_group()?;

        CenteredText {
            text: format!(
                "{name}{length:.2}mm",
                name = match &self.name {
                    Some(name) => format!("{name} = "),
                    None => String::new(),
                },
                length = self.length
            ),
            rect: Rect::new(bottom_left, top_right).translate(0.0, center),
            font_size: 2.0,
        }
        .write_svg(writer, &SvgTagAttribute::class("measure-fill").into())?;

        writer.end_group()
    }
}

impl WriteSvgMapped for EdgeLengthMeasure {}

/// A radius measure with an offset.
pub struct RadiusMeasure {
    /// Circle to measure.
    pub circle: Circle,
    /// Original radius to measure.
    pub radius: Scalar,
    /// Name of this measurement.
    pub name: Option<String>,
    /// Angle of the measurement.
    pub angle: Deg<Scalar>,
}

impl RadiusMeasure {
    /// Create new radius measure.
    pub fn new(circle: Circle, name: Option<String>, angle: Option<Deg<Scalar>>) -> Self {
        Self {
            radius: circle.radius,
            circle,
            name,
            angle: angle.unwrap_or(Deg(-45.0)),
        }
    }
}

impl MapToCanvas for RadiusMeasure {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self {
            radius: self.radius,
            circle: self.circle.map_to_canvas(canvas),
            name: self.name.clone(),
            angle: self.angle,
        }
    }
}

impl WriteSvg for RadiusMeasure {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        writer.begin_group(attr)?;

        let edge = Edge2D::radius_edge(&self.circle, &self.angle.into());
        edge.shorter(1.5).write_svg(
            writer,
            &attr
                .clone()
                .insert(SvgTagAttribute::MarkerEnd("arrow".into()))
                .insert(SvgTagAttribute::class("measure")),
        )?;
        let center = edge.center();

        CenteredText {
            text: format!(
                "{name}{radius:.2}mm",
                name = match &self.name {
                    Some(name) => format!("{name} = "),
                    None => String::new(),
                },
                radius = self.radius,
            ),
            rect: Rect::new(center, center),
            font_size: 2.0,
        }
        .write_svg(writer, &SvgTagAttribute::class("measure-fill").into())?;

        writer.end_group()?;

        Ok(())
    }
}

impl WriteSvgMapped for RadiusMeasure {}

/// Size measure of a bounds.
pub struct SizeMeasure {
    bounds: Bounds2D,
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
                bounds: bounds.clone(),
                width: Some(EdgeLengthMeasure::width(rect, 7.0, None)),
                height: Some(EdgeLengthMeasure::height(rect, 7.0, None)),
            }
        } else {
            Self {
                bounds: bounds.clone(),
                width: None,
                height: None,
            }
        }
    }
}

impl MapToCanvas for SizeMeasure {
    fn map_to_canvas(&self, canvas: &Canvas) -> Self {
        Self {
            bounds: self.bounds.map_to_canvas(canvas),
            width: self.width.as_ref().map(|width| width.map_to_canvas(canvas)),
            height: self
                .height
                .as_ref()
                .map(|height| height.map_to_canvas(canvas)),
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

impl WriteSvgMapped for SizeMeasure {}
