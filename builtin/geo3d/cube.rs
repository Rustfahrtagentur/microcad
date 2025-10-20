// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// The builtin cube primitive, defined by its size in the x, y, and z dimensions.
#[derive(Debug, Clone)]
pub struct Cube {
    /// Size of the cube in millimeters.
    pub size: Vec3,
}

impl Render<Geometry3D> for Cube {
    fn render(&self, _: &RenderResolution) -> Geometry3D {
        Manifold::cube(self.size.x, self.size.y, self.size.z).into()
    }
}

impl RenderWithContext<Geometry3DOutput> for Cube {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, _| Ok(self.render(&context.current_resolution())))
    }
}

impl BuiltinWorkbenchDefinition for Cube {
    fn id() -> &'static str {
        "Cube"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive3D(Box::new(Cube {
                size: Vec3::new(args.get("size_x"), args.get("size_y"), args.get("size_z")),
            })))
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
