// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::builtin::*;

pub struct Sphere;

impl BuiltinWorkbenchDefinition for Sphere {
    fn id() -> &'static str {
        "Sphere"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive3D(
                geo3d::Geometry3D::Sphere(geo3d::Sphere {
                    radius: args.get("radius"),
                }),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(radius: Scalar)].into_iter().collect()
    }
}
