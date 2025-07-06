// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Scalable Vector Graphics (SVG) export

use std::io::BufWriter;

use geo::coord;
use microcad_lang::{Id, builtin::*, model_tree::*, parameter, value::*};

use crate::svg::writer::SvgWriter;

/// SVG Exporter
pub struct SvgExporter;

impl Exporter for SvgExporter {
    fn parameters(&self) -> microcad_lang::eval::ParameterValueList {
        vec![parameter!(style: String = String::new())].into()
    }

    fn export(
        &mut self,
        node: ModelNode,
        filename: &std::path::Path,
    ) -> Result<Value, ExportError> {
        assert_eq!(node.output_type(), ModelNodeOutputType::Geometry2D);
        let f = std::fs::File::create(filename)?;

        //node.process();
        // TODO get bounds from a process node:
        // let bounds = node.bounds();
        let bounds = geo::Rect::new(coord! { x: 0., y: 0. }, coord! { x: 100., y: 100. });
        let mut writer = SvgWriter::new(Box::new(BufWriter::new(f)), bounds, 1.0)?;

        writer.node(&node)?;

        Ok(Value::None)
    }
}

impl FileIoInterface for SvgExporter {
    fn id(&self) -> Id {
        Id::new("svg")
    }
}
