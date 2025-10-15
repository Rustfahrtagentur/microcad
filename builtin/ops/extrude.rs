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
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;

            use microcad_core::Extrude;
            let mesh = geometries.linear_extrude(self.height);
            Ok(Rc::new(WithBounds3D::new(mesh.inner.into(), mesh.bounds)))
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
