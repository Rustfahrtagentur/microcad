// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Extrude {
    height: Scalar,
    n_divisions: Integer,
    twist_degrees: Scalar,
    scale_top_x: Scalar,
    scale_top_y: Scalar,
}

impl Operation for Extrude {
    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model, resolution| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render(context)?;

            let multi_polygon_data =
                geo2d::multi_polygon_to_vec(&geometries.render_to_multi_polygon(&resolution));
            let multi_polygon_data: Vec<_> = multi_polygon_data
                .iter()
                .map(|ring| ring.as_slice())
                .collect();

            Ok(Some(Rc::new(Geometry3D::Manifold(Rc::new(
                Manifold::extrude(
                    &multi_polygon_data,
                    self.height,
                    self.n_divisions as u32,
                    self.twist_degrees,
                    self.scale_top_x,
                    self.scale_top_y,
                ),
            )))))
        })
    }
}

impl BuiltinWorkbenchDefinition for Extrude {
    fn id() -> &'static str {
        "extrude"
    }

    fn output_type() -> OutputType {
        OutputType::Geometry3D
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(Extrude {
                height: args.get("height"),
                n_divisions: args.get("n_divisions"),
                twist_degrees: args.get("twist_degrees"),
                scale_top_x: args.get("scale_top_x"),
                scale_top_y: args.get("scale_top_y"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(height: Scalar),
            parameter!(n_divisions: Integer),
            parameter!(twist_degrees: Scalar),
            parameter!(scale_top_x: Scalar),
            parameter!(scale_top_y: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
