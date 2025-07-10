// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::CoordsIter;
use microcad_core::*;
use microcad_lang::{
    model_tree::{Element, GetAttribute, ModelNode, ModelNodeOutputType},
    syntax::Identifier,
};

/// SVG writer.
pub struct SvgWriter {
    writer: Box<dyn std::io::Write>,
    level: usize,
}

/// Tag attributes for an SVG tag.
#[derive(Debug, Clone, Default)]
pub struct SvgTagAttributes {
    /// Style attribute.
    pub style: Option<String>,
    /// Fill attribute. Used when color attribute of a node is set.
    pub fill: Option<String>,
}

impl SvgTagAttributes {
    fn is_empty(&self) -> bool {
        self.style.is_none() && self.fill.is_none()
    }
}

impl From<&ModelNode> for SvgTagAttributes {
    fn from(node: &ModelNode) -> Self {
        use microcad_lang::value::ValueAccess;

        match (
            node.get_exporter_attribute(&Identifier::no_ref("svg")),
            node.get_color_attribute(),
        ) {
            (None, None) => SvgTagAttributes::default(),
            (None, Some(color)) => SvgTagAttributes {
                style: None,
                fill: Some(color.to_svg_color()),
            },
            // If boths attributes are present, get style and fill from attributes. Color attribute is ignored.
            (Some(attributes), None) | (Some(attributes), Some(_)) => SvgTagAttributes {
                style: attributes
                    .by_id(&Identifier::no_ref("style"))
                    .map(|value| value.try_string().unwrap_or_default()),
                fill: attributes
                    .by_id(&Identifier::no_ref("fill"))
                    .map(|value| value.try_string().unwrap_or_default()),
            },
        }
    }
}

impl std::fmt::Display for SvgTagAttributes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match (&self.style, &self.fill) {
            (None, None) => Ok(()),
            (None, Some(fill)) => write!(f, "fill=\"{fill}\""),
            (Some(style), None) => write!(f, "style=\"{style}\""),
            (Some(style), Some(fill)) => write!(f, "fill=\"{fill}\" style=\"{style}\""),
        }
    }
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
            level: 1,
        })
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

    fn tag(&mut self, tag: &str, attr: &SvgTagAttributes) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{:indent$}<{tag_inner}/>",
            "",
            indent = 2 * self.level,
            tag_inner = Self::tag_inner(tag, attr)
        )
    }

    fn begin_tag(&mut self, tag: &str, attr: &SvgTagAttributes) -> std::io::Result<()> {
        writeln!(
            self.writer,
            "{:indent$}<{tag_inner}>",
            "",
            indent = 2 * self.level,
            tag_inner = Self::tag_inner(tag, attr)
        )?;
        self.level += 1;
        Ok(())
    }

    fn end_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.level -= 1;
        writeln!(
            self.writer,
            "{:indent$}</{tag}>",
            "",
            indent = 2 * self.level
        )
    }

    /// Begin a new group `<g>`.
    pub fn begin_group(&mut self, attr: &SvgTagAttributes) -> std::io::Result<()> {
        self.begin_tag("g", attr)
    }

    /// End a group `</g>`.
    pub fn end_group(&mut self) -> std::io::Result<()> {
        self.end_tag("g")
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

        self.begin_tag(
            &format!("g transform=\"matrix({a} {b} {c} {d} {e} {f})\""),
            attr,
        )
    }

    /// End a SVG transformation group `</g>`.
    pub fn end_transform(&mut self) -> std::io::Result<()> {
        self.end_group()
    }

    /// Generate rectangle.
    pub fn rect(&mut self, rect: &geo2d::Rect, attr: &SvgTagAttributes) -> std::io::Result<()> {
        let x = rect.min().x;
        let y = rect.min().y;
        let width = rect.width();
        let height = rect.height();

        self.tag(
            &format!("rect x=\"{x}\" y=\"{y}\" width=\"{width}\" height=\"{height}\""),
            attr,
        )
    }

    /// Generate circle.
    pub fn circle(
        &mut self,
        circle: &geo2d::Circle,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        let r = circle.radius;
        let (cx, cy) = (circle.offset.x, circle.offset.y);
        self.tag(&format!("circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\""), attr)
    }

    /// Generate line.
    pub fn line(
        &mut self,
        p1: geo2d::Point,
        p2: geo2d::Point,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (p1.x_y(), p2.x_y());
        self.tag(
            &format!("line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\""),
            attr,
        )
    }

    /// Generate line string.
    pub fn line_string(
        &mut self,
        line_string: &geo2d::LineString,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        let points = line_string.coords().fold(String::new(), |acc, p| {
            acc + &format!("{x},{y} ", x = p.x, y = p.y)
        });
        self.tag(&format!("polyline points=\"{points}\""), attr)
    }

    /// Generate multi line string.
    pub fn multi_line_string(
        &mut self,
        multi_line_string: &geo2d::MultiLineString,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        multi_line_string
            .iter()
            .try_for_each(|line_string| self.line_string(line_string, attr))
    }

    /// Generate polygon.
    pub fn polygon(
        &mut self,
        polygon: &geo2d::Polygon,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
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

        let exterior = line_string_path(polygon.exterior());
        let interior = polygon
            .interiors()
            .iter()
            .map(line_string_path)
            .fold(String::new(), |acc, s| acc + &s);

        self.tag(&format!("path d=\"{exterior} {interior}\""), attr)
    }

    /// Generate multiple polygons
    pub fn multi_polygon(
        &mut self,
        multi_polygon: &geo2d::MultiPolygon,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        for polygon in multi_polygon {
            self.polygon(polygon, attr)?;
        }
        Ok(())
    }

    /// Generate geometry.
    pub fn geometry(
        &mut self,
        geometry: &Geometry2D,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        match geometry {
            Geometry2D::LineString(line_string) => self.line_string(line_string, attr),
            Geometry2D::MultiLineString(multi_line_string) => {
                self.multi_line_string(multi_line_string, attr)
            }
            Geometry2D::Polygon(polygon) => self.polygon(polygon, attr),
            Geometry2D::MultiPolygon(multi_polygon) => self.multi_polygon(multi_polygon, attr),
            Geometry2D::Rect(rect) => self.rect(rect, attr),
            Geometry2D::Circle(circle) => self.circle(circle, attr),
        }
    }

    /// Generate SVG for a node.
    pub fn node(&mut self, node: &ModelNode) -> std::io::Result<()> {
        assert_eq!(node.output_type(), ModelNodeOutputType::Geometry2D);

        let attr: SvgTagAttributes = node.into();

        let b = node.borrow();
        let element = b.element();

        match element {
            Element::Object(_) => {
                if node.has_children() {
                    self.begin_group(&attr)?;
                    node.children().try_for_each(|child| self.node(&child))?;
                    self.end_group()?;
                }
            }
            Element::Transform(affine_transform) => {
                if node.has_children() {
                    self.begin_transform(&affine_transform.mat2d(), &attr)?;
                    node.children().try_for_each(|child| self.node(&child))?;
                    self.end_transform()?;
                }
            }
            Element::Primitive2D(geometry) => {
                self.geometry(geometry, &attr)?;
                if node.has_children() {
                    self.begin_group(&attr)?;
                    node.children().try_for_each(|child| self.node(&child))?;
                    self.end_group()?;
                }
            }
            Element::Operation(operation) => {
                operation
                    .process_2d(node)
                    .iter()
                    .try_for_each(|geometry| self.geometry(geometry, &attr))?;
            }
            _ => {}
        }

        Ok(())
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
