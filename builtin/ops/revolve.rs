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
            let multi_polygon_data =
                geo2d::multi_polygon_to_vec(&geometries.render_to_multi_polygon(&resolution));
            let multi_polygon_data: Vec<_> = multi_polygon_data
                .iter()
                .map(|ring| ring.as_slice())
                .collect();
            let radius = geometries
                .fetch_bounds_2d()
                .max()
                .map(|v| v.x.max(v.y))
                .unwrap_or_default();
            let circular_segments = resolution.circular_segments(radius);

            Ok(Some(Rc::new(Geometry3D::Manifold(Rc::new(
                Manifold::revolve(&multi_polygon_data, circular_segments, self.revolve_degrees),
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
