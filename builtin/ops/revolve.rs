// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{
    eval::*,
    model::{render::RenderCache, *},
    parameter,
    value::*,
};

#[derive(Debug)]
pub struct Revolve {
    circular_segments: Integer,
    revolve_degrees: Scalar,
}

impl Operation for Revolve {
    fn output_type(&self) -> OutputType {
        OutputType::Geometry3D
    }

    fn process_3d(&self, cache: &mut RenderCache, model: &Model) -> Geometries3D {
        use std::rc::Rc;
        let geometries = model.render_geometry_2d(cache);

        let multi_polygon_data = geo2d::multi_polygon_to_vec(
            &geometries.render_to_multi_polygon(&model.borrow().output.resolution),
        );
        let multi_polygon_data: Vec<_> = multi_polygon_data
            .iter()
            .map(|ring| ring.as_slice())
            .collect();

        Geometry3D::Manifold(Rc::new(Manifold::revolve(
            &multi_polygon_data,
            self.circular_segments as u32,
            self.revolve_degrees,
        )))
        .into()
    }
}

impl BuiltinWorkbenchDefinition for Revolve {
    fn id() -> &'static str {
        "revolve"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(ModelBuilder::new_operation(Revolve {
            circular_segments: args.get("circular_segments")?,
            revolve_degrees: args.get("revolve_degrees")?,
        })
        .build())
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(circular_segments: Integer),
            parameter!(revolve_degrees: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
