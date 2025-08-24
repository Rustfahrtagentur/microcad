// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, parameter, syntax::WorkbenchKind, value::*};

pub struct Cube;

impl BuiltinWorkbenchDefinition for Cube {
    fn id() -> &'static str {
        "Cube"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Part
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Geometry3D(Geometry3D::Cube(
                geo3d::Cube {
                    size: Vec3::new(
                        args.get("size_x")?,
                        args.get("size_y")?,
                        args.get("size_z")?,
                    ),
                },
            )))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(size_x: Scalar),
            parameter!(size_y: Scalar),
            parameter!(size_z: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
