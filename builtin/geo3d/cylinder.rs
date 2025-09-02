// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_lang::builtin::*;

pub struct Cylinder;

impl BuiltinWorkbenchDefinition for Cylinder {
    fn id() -> &'static str {
        "Cylinder"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;
        &|args| {
            Ok(BuiltinWorkpieceOutput::Geometry3D(Geometry3D::Cylinder(
                geo3d::Cylinder {
                    radius_bottom: args.get("radius_bottom"),
                    radius_top: args.get("radius_top"),
                    height: args.get("height"),
                },
            )))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(radius_bottom: Scalar),
            parameter!(radius_top: Scalar),
            parameter!(height: Scalar),
        ]
        .into_iter()
        .collect()
    }
}
