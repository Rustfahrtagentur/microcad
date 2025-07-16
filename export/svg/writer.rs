// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) file writer

use geo::{CoordsIter, coord};
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
pub struct SvgStyleAttributes {
    /// Style attribute.
    pub style: Option<String>,
    /// Fill attribute. Used when color attribute of a node is set.
    pub fill: Option<String>,
}

impl SvgStyleAttributes {
    fn is_empty(&self) -> bool {
        self.style.is_none() && self.fill.is_none()
    }
}

impl From<&ModelNode> for SvgStyleAttributes {
    fn from(node: &ModelNode) -> Self {
        use microcad_lang::value::ValueAccess;

        match (
            node.get_exporter_attribute(&Identifier::no_ref("svg")),
            node.get_color_attribute(),
        ) {
            (None, None) => SvgStyleAttributes::default(),
            (None, Some(color)) => SvgStyleAttributes {
                style: None,
                fill: Some(color.to_svg_color()),
            },
            // If boths attributes are present, get style and fill from exporter attributes. Color attribute is ignored.
            (Some(attributes), None) | (Some(attributes), Some(_)) => SvgStyleAttributes {
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

impl std::fmt::Display for SvgStyleAttributes {
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
        })
    }

    fn tag_inner(tag: &str, attr: &SvgStyleAttributes) -> String {
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
    fn with_indent(&mut self, s: &str) -> std::io::Result<()> {
        writeln!(self.writer, "{:indent$}{s}", "", indent = 2 * self.level)
    }

    /// Write a single tag `<tag>`.
    fn tag(&mut self, tag: &str, attr: &SvgStyleAttributes) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}/>",
            tag_inner = Self::tag_inner(tag, attr)
        ))
    }

    /// Open a tag `<tag>`
    fn open_tag(&mut self, tag: &str, attr: &SvgStyleAttributes) -> std::io::Result<()> {
        self.with_indent(&format!(
            "<{tag_inner}>",
            tag_inner = Self::tag_inner(tag, attr)
        ))?;

        self.level += 1;
        Ok(())
    }

    /// Close a tag `</tag>`
    fn close_tag(&mut self, tag: &str) -> std::io::Result<()> {
        self.level -= 1;
        self.with_indent(format!("</{tag}>").as_str())
    }

    /// Begin a new group `<g>`.
    pub fn begin_group(&mut self, attr: &SvgStyleAttributes) -> std::io::Result<()> {
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
        attr: &SvgStyleAttributes,
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

    /// Generate rectangle.
    pub fn rect(&mut self, rect: &geo2d::Rect, attr: &SvgStyleAttributes) -> std::io::Result<()> {
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
        attr: &SvgStyleAttributes,
    ) -> std::io::Result<()> {
        let r = circle.radius;
        let (cx, cy) = (circle.offset.x, circle.offset.y);
        self.tag(&format!("circle cx=\"{cx}\" cy=\"{cy}\" r=\"{r}\""), attr)
    }

    /// Generate edge with optional arrow heads.
    pub fn edge(
        &mut self,
        edge: &geo2d::Edge2D,
        attr: &SvgStyleAttributes,
        marker_start: Option<String>,
        marker_end: Option<String>,
    ) -> std::io::Result<()> {
        let ((x1, y1), (x2, y2)) = (edge.0.x_y(), edge.1.x_y());
        self.tag(
            &format!(
                "line x1=\"{x1}\" y1=\"{y1}\" x2=\"{x2}\" y2=\"{y2}\"{marker_start}{marker_end}",
                marker_start = match marker_start {
                    Some(marker_start) => format!(" marker-start=\"url(#{marker_start})\""),
                    None => String::new(),
                },
                marker_end = match marker_end {
                    Some(marker_end) => format!(" marker-end=\"url(#{marker_end})\""),
                    None => String::new(),
                }
            ),
            attr,
        )
    }

    /// Generate line string.
    pub fn line_string(
        &mut self,
        line_string: &geo2d::LineString,
        attr: &SvgStyleAttributes,
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
        attr: &SvgStyleAttributes,
    ) -> std::io::Result<()> {
        multi_line_string
            .iter()
            .try_for_each(|line_string| self.line_string(line_string, attr))
    }

    /// Generate polygon.
    pub fn polygon(
        &mut self,
        polygon: &geo2d::Polygon,
        attr: &SvgStyleAttributes,
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
        attr: &SvgStyleAttributes,
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
        attr: &SvgStyleAttributes,
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
            Geometry2D::Edge(edge) => self.edge(edge, attr, None, None),
        }
    }

    /// Generate SVG for a node.
    pub fn node(&mut self, node: &ModelNode) -> std::io::Result<()> {
        assert_eq!(node.final_output_type(), ModelNodeOutputType::Geometry2D);

        let attr: SvgStyleAttributes = node.into();

        // Render all output geometries.
        node.fetch_output_geometries_2d()
            .iter()
            .try_for_each(|geometry| self.geometry(geometry, &attr))?;

        let node_ = node.borrow();
        match &node_.element.value {
            Element::Object(_) | Element::Primitive2D(_) => {
                if !node_.is_empty() {
                    self.begin_group(&attr)?;
                    node_.children().try_for_each(|child| self.node(child))?;
                    self.end_group()?;
                }
            }
            Element::Transform(affine_transform) => {
                if !node_.is_empty() {
                    self.begin_transform(&affine_transform.mat2d(), &attr)?;
                    node_.children().try_for_each(|child| self.node(child))?;
                    self.end_transform()?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Generate a text centered in a rect angle.
    pub fn centered_text(
        &mut self,
        text: &str,
        rect: &Rect,
        font_size: Option<String>,
        font_family: Option<String>,
        attr: &SvgStyleAttributes,
    ) -> std::io::Result<()> {
        let (x, y) = rect.center().x_y();
        self.open_tag(
            format!(r#"text x="{x}" y="{y}" dominant-baseline="middle" text-anchor="middle" {font_size}{font_family}"#,
                font_size = match font_size {
                    Some(font_size) => format!(" font-size=\"{font_size}\""),
                    None => String::new(),
                },
                font_family = match font_family {
                    Some(font_family) => format!(" font-family=\"{font_family}\""),
                    None => String::new(),
                }
        )
                .as_str(),
            attr,
        )?;
        self.with_indent(text)?;
        self.close_tag("text")
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
