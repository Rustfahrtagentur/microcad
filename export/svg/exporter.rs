// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::rc::Rc;

use microcad_core::Geometry2D;
use microcad_lang::{model_tree::*, syntax::*, value::*};

use crate::{
    Exporter,
    svg::writer::{SvgTagAttributes, SvgWriter},
};

/// SVG Exporter
struct SvgExporter {
    /// The SVG writer
    writer: SvgWriter,

    /// Processed results only
    processed_only: bool,
}

impl SvgExporter {
    fn fetch_svg_attributes(&mut self, node: &ModelNode) -> SvgTagAttributes {
        let b = node.borrow();
        let attributes = b.attributes();
        attributes.get_as_tuple(&Identifier::no_ref("svg")).into()
    }

    fn write_geometry(
        &mut self,
        geometry: &Rc<Geometry2D>,
        attr: &SvgTagAttributes,
    ) -> std::io::Result<()> {
        match geometry.as_ref() {
            Geometry2D::LineString(line_string) => self.writer.line_string(line_string, attr),
            Geometry2D::MultiLineString(multi_line_string) => {
                self.writer.multi_line_string(multi_line_string, attr)
            }
            Geometry2D::Polygon(polygon) => self.writer.polygon(polygon, attr),
            Geometry2D::MultiPolygon(multi_polygon) => {
                self.writer.multi_polygon(multi_polygon, attr)
            }
            Geometry2D::Rect(rect) => self.writer.rect(rect, attr),
            Geometry2D::Circle(circle) => self.writer.circle(circle, attr),
        }
    }

    fn write_node(&mut self, node: ModelNode) -> std::io::Result<()> {
        let b = node.borrow();

        let element = b.element();
        let attributes = self.fetch_svg_attributes(&node);

        match element {
            Element::Object(_) => {
                self.writer.begin_group(&attributes)?;
                node.children()
                    .try_for_each(|child| self.write_node(child))?;
                self.writer.end_group()?;
            }
            Element::Transform(affine_transform) => {
                self.writer.begin_transform(&affine_transform.mat2d())?;
                node.children()
                    .try_for_each(|child| self.write_node(child))?;
                self.writer.end_transform()?;
            }
            Element::Primitive2D(geometry) => self.write_geometry(geometry, &attributes)?,
            Element::Operation(_) => {}
            _ => {}
        }

        Ok(())
    }
}

impl Exporter for SvgExporter {
    fn id() -> &'static str {
        "svg"
    }

    fn export(&mut self, node: ModelNode) -> Result<Value, crate::ExportError> {
        self.write_node(node)?;

        Ok(Value::None)
    }
}
