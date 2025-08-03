// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{
    eval::{BuiltinWorkbenchDefinition, EvalResult},
    model::*,
    parameter,
    value::{Tuple, ValueAccess},
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

    fn process_3d(&self, model: &microcad_lang::model::Model) -> microcad_core::Geometries3D {
        use std::rc::Rc;
        let mut geometries = Geometries2D::default();

        let self_ = model.borrow();
        self_.children.iter().for_each(|model| {
            let b = model.borrow();
            let mat = b.output.local_matrix_2d();
            geometries.append(
                model
                    .process_2d(model)
                    .transformed_2d(&b.output.resolution, &mat),
            );
        });

        let multi_polygon = microcad_core::geo2d::multi_polygon_to_vec(
            &geometries.render_to_multi_polygon(&model.borrow().output.resolution),
        );

        let multi_polygon_data: Vec<_> = multi_polygon.iter().map(|ring| ring.as_slice()).collect();

        Rc::new(Geometry3D::Manifold(Rc::new(Manifold::revolve(
            &multi_polygon_data,
            self.circular_segments as u32,
            self.revolve_degrees,
        ))))
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

    fn parameters() -> microcad_lang::eval::ParameterValueList {
        [
            parameter!(circular_segments: Integer),
            parameter!(revolve_degrees: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
