// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, parameter, syntax::WorkbenchKind, value::*};

pub struct Sphere;

impl BuiltinWorkbenchDefinition for Sphere {
    fn id() -> &'static str {
        "Sphere"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Part
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Geometry3D(
                geo3d::Geometry3D::Sphere(geo3d::Sphere {
                    radius: args.get("radius")?,
                }),
            ))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(radius: Scalar)].into_iter().collect()
    }
}
