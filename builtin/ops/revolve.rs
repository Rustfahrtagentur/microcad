// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

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
        context.update_3d(|context, model| {
            let model_ = model.borrow();
            let geometries: Geometries2D = model_.children.render_with_context(context)?;
            let radius = geometries.calc_bounds_2d().max_extent();
            use microcad_core::Extrude;

            let WithBounds3D { inner, bounds } = geometries.revolve_extrude(
                cgmath::Deg(self.revolve_degrees).into(),
                context.current_resolution().circular_segments(radius) as usize,
            );

            Ok(WithBounds3D::new(inner.into(), bounds))
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
