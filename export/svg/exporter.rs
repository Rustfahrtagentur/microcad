// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::{io::BufWriter, rc::Rc};

use geo::coord;
use microcad_core::Geometry2D;
use microcad_lang::{Id, builtin::*, model_tree::*, parameter, value::*};

use crate::svg::writer::SvgWriter;

pub struct EntityList<T> {
    entities: Vec<T>,
}

impl<T> EntityList<T> {
    pub fn new() -> Self {
        Self { entities: vec![] }
    }
}

use thiserror::Error;

#[derive(Error, Debug)]
pub enum GenerateError {}

pub trait Generate<T> {
    fn generate(&self, node: ModelNode) -> Result<EntityList<T>, GenerateError>;
}

impl Generate<Rc<Geometry2D>> for SvgExporter {
    fn generate(&self, node: ModelNode) -> Result<EntityList<Rc<Geometry2D>>, GenerateError> {
        let mut entities = EntityList::new();
        let b = node.borrow_mut();

        match b.element_mut() {
            Element::Object(object) => todo!(),
            Element::ChildrenPlaceholder => todo!(),
            Element::Transform(affine_transform) => todo!(),
            Element::Primitive2D(geometry) => todo!(),
            Element::Primitive3D(geometry) => todo!(),
            Element::Operation(operation) => {
                operation.process(node);
            }
        }

        Ok(entities)
    }
}

/// SVG Exporter
pub struct SvgExporter;

impl Exporter for SvgExporter {
    fn parameters(&self) -> microcad_lang::eval::ParameterValueList {
        vec![parameter!(style: String = String::new())].into()
    }

    fn export(&self, node: &ModelNode, filename: &std::path::Path) -> Result<Value, ExportError> {
        let f = std::fs::File::create(filename)?;

        // TODO The node operations have to be processed at this point.
        // node.process();
        // TODO get bounds from a process node:
        // let bounds = node.bounds();
        let bounds = geo::Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });
        let mut writer = SvgWriter::new(Box::new(BufWriter::new(f)), bounds, 1.0)?;

        writer.node(node)?;

        Ok(Value::None)
    }

    fn node_output_type(&self) -> ModelNodeOutput {
        ModelNodeOutput::Geometry2D
    }
}

impl FileIoInterface for SvgExporter {
    fn id(&self) -> Id {
        Id::new("svg")
    }
}
