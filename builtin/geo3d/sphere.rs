// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::builtin::*;

/// The builtin sphere primitive, defined by its radius.
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Radius of the sphere in millimeters.
    pub radius: Scalar,
}

impl Render<Geometry3D> for Sphere {
    fn render(&self, resolution: &RenderResolution) -> Geometry3D {
        Manifold::sphere(self.radius, resolution.circular_segments(self.radius)).into()
    }
}

impl BuiltinWorkbenchDefinition for Sphere {
    fn id() -> &'static str {
        "Sphere"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive3D(Box::new(Sphere {
                radius: args.get("radius"),
            })))
        }
    }

    fn parameters() -> ParameterValueList {
        [parameter!(radius: Scalar)].into_iter().collect()
    }
}
