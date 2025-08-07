// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model::*, parameter, value::*};

#[derive(Debug)]
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

    fn process_3d(&self, model: &Model) -> Geometries3D {
        use std::rc::Rc;
        let geometries = model.render_geometries_2d();

        let multi_polygon_data = geo2d::multi_polygon_to_vec(
            &geometries.render_to_multi_polygon(&model.borrow().output.resolution),
        );
        let multi_polygon_data: Vec<_> = multi_polygon_data
            .iter()
            .map(|ring| ring.as_slice())
            .collect();

        Rc::new(Geometry3D::Manifold(Rc::new(Manifold::extrude(
            &multi_polygon_data,
            self.height,
            self.n_divisions as u32,
            self.twist_degrees,
            self.scale_top_x,
            self.scale_top_y,
        ))))
        .into()
    }
}

impl BuiltinWorkbenchDefinition for Extrude {
    fn id() -> &'static str {
        "extrude"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(ModelBuilder::new_operation(Extrude {
            height: args.get("height")?,
            n_divisions: args.get("n_divisions")?,
            twist_degrees: args.get("twist_degrees")?,
            scale_top_x: args.get("scale_top_x")?,
            scale_top_y: args.get("scale_top_y")?,
        })
        .build())
    }

    fn parameters() -> microcad_lang::eval::ParameterValueList {
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
