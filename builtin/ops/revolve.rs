// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::*;
use microcad_lang::{builtin::*, model::*, render::*};

#[derive(Debug)]
#[allow(dead_code)]
pub struct Revolve {
    revolve_degrees: Scalar,
}

impl Operation for Revolve {
    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }

    fn process_3d(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, model, resolution| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render(context)?;

            use microcad_core::Extrude;
            let radius = geometries
                .fetch_bounds_2d()
                .max()
                .map(|v| v.x.max(v.y))
                .unwrap_or_default();
            let circular_segments = resolution.circular_segments(radius);

            Ok(Some(Rc::new(Geometry3D::Mesh(geometries.revolve_extrude(
                cgmath::Deg(self.revolve_degrees).into(),
                circular_segments as usize,
            )))))
        })
    }
}

impl BuiltinWorkbenchDefinition for Revolve {
    fn id() -> &'static str {
        "revolve"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Operation
    }

    fn output_type() -> OutputType {
        OutputType::Geometry3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Operation(Box::new(Revolve {
                revolve_degrees: args.get("revolve_degrees"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(revolve_degrees: Scalar)].into_iter().collect()
    }
}
