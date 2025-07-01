// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::io::BufWriter;

use geo::coord;
use microcad_lang::{Id, builtin::*, eval::ArgumentMap, model_tree::*, syntax::*, value::*};

use crate::svg::writer::{SvgTagAttributes, SvgWriter};

/// SVG Exporter
pub struct SvgExporter;

impl SvgExporter {
    fn write_node(writer: &mut SvgWriter, node: ModelNode) -> std::io::Result<()> {
        fn fetch_svg_attributes(node: &ModelNode) -> SvgTagAttributes {
            let b = node.borrow();
            let attributes = b.attributes();
            attributes.get_as_tuple(&Identifier::no_ref("svg")).into()
        }

        let b = node.borrow();

        let element = b.element();
        let attributes = fetch_svg_attributes(&node);

        match element {
            Element::Object(_) => {
                writer.begin_group(&attributes)?;
                node.children()
                    .try_for_each(|child| Self::write_node(writer, child))?;
                writer.end_group()?;
            }
            Element::Transform(affine_transform) => {
                writer.begin_transform(&affine_transform.mat2d())?;
                node.children()
                    .try_for_each(|child| Self::write_node(writer, child))?;
                writer.end_transform()?;
            }
            Element::Primitive2D(geometry) => writer.geometry(geometry, &attributes)?,
            Element::Operation(_) => {
                todo!("Output processed operation results")
            }
            _ => {}
        }

        Ok(())
    }
}

impl Exporter for SvgExporter {
    fn export(&mut self, node: ModelNode, args: &ArgumentMap) -> Result<Value, ExportError> {
        assert_eq!(node.output_type(), ModelNodeOutputType::Geometry2D);

        let f = std::fs::File::create(args.get::<String>("filename"))?;

        //node.process();
        let bounds = geo::Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });
        let mut writer = SvgWriter::new(Box::new(BufWriter::new(f)), bounds, 1.0)?;

        Self::write_node(&mut writer, node)?;
        Ok(Value::None)
    }
}

impl FileIoInterface for SvgExporter {
    fn id(&self) -> Id {
        Id::new("svg")
    }
}
