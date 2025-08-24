// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{eval::*, parameter, syntax::WorkbenchKind, value::*};

pub struct Cylinder;

impl BuiltinWorkbenchDefinition for Cylinder {
    fn id() -> &'static str {
        "Cylinder"
    }

    fn kind() -> WorkbenchKind {
        WorkbenchKind::Part
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Geometry3D(Geometry3D::Cylinder(
                geo3d::Cylinder {
                    radius_bottom: args.get("radius_bottom")?,
                    radius_top: args.get("radius_top")?,
                    height: args.get("height")?,
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
