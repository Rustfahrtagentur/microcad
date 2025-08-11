// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, model::*, parameter, rc::*, value::*};

pub struct Sphere;

impl BuiltinWorkbenchDefinition for Sphere {
    fn id() -> &'static str {
        "Sphere"
    }

    fn model(args: &Tuple) -> EvalResult<Model> {
        Ok(
            ModelBuilder::new_3d_primitive(Rc::new(geo3d::Geometry3D::Sphere(geo3d::Sphere {
                radius: args.get("radius")?,
            })))
            .build(),
        )
    }

    fn parameters() -> ParameterValueList {
        [parameter!(radius: Scalar)].into_iter().collect()
    }
}
