// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) primitives (SvgWrite trait implementations).

use geo::CoordsIter as _;
use microcad_core::*;
use microcad_lang::model_tree::{Element, ModelNode, ModelNodeOutputType};

use crate::svg::*;

impl WriteSvg for Edge2D {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (self.0.x_y(), self.1.x_y());
        writer.tag(
            &format!("line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\"",),
            attr,
        )
    }
}

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
        let r = self.radius;
        let (cx, cy) = (self.offset.x, self.offset.y);
        writer.tag(&format!("circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\""), attr)
    }
}

impl WriteSvg for LineString {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let points = self.coords().fold(String::new(), |acc, p| {
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

impl WriteSvg for ModelNode {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        assert_eq!(self.final_output_type(), ModelNodeOutputType::Geometry2D);

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
                    writer.begin_transform(&affine_transform.mat2d(), &attr)?;
                    self_
                        .children()
                        .try_for_each(|child| child.write_svg(writer, &attr))?;
                    writer.end_transform()?;
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

impl WriteSvg for Grid {
    fn write_svg(&self, writer: &mut SvgWriter, attr: &SvgTagAttributes) -> std::io::Result<()> {
        if let Some(rect) = self.bounds.rect() {
            self.bounds.write_svg(writer, attr)?;

            let mut left = rect.min().x;
            let right = rect.max().x;
            while left <= right {
                Edge2D(
                    geo::Point::new(left, rect.min().y),
                    geo::Point::new(left, rect.max().y),
                )
                .write_svg(writer, attr)?;
                left += self.cell_size.width;
            }

            let mut bottom = rect.min().y;
            let top = rect.max().y;
            while bottom <= top {
                Edge2D(
                    geo::Point::new(rect.min().x, bottom),
                    geo::Point::new(rect.max().x, top),
                )
                .write_svg(writer, attr)?;
                bottom += self.cell_size.height;
            }
        }
        Ok(())
    }
}
