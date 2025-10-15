// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::{builtin::*, render::*};

/// The built-in cylinder primitive, defined by an bottom radius, top radius and height.
/// The cylinder is oriented along the z-axis.
#[derive(Debug, Clone)]
pub struct Cylinder {
    /// Bottom radius of the cylinder in millimeters.
    pub radius_bottom: Scalar,
    /// Top radius of the cylinder in millimeters.
    pub radius_top: Scalar,
    /// Height of the cylinder in millimeters.
    pub height: Scalar,
}

impl Render<Geometry3D> for Cylinder {
    fn render(&self, resolution: &RenderResolution) -> Geometry3D {
        geo3d::Manifold::cylinder(
            self.radius_bottom,
            self.radius_top,
            self.height,
            resolution.circular_segments(self.radius_bottom.max(self.radius_top)),
        )
        .into()
    }
}

impl RenderWithContext<Geometry3DOutput> for Cylinder {
    fn render_with_context(&self, context: &mut RenderContext) -> RenderResult<Geometry3DOutput> {
        context.update_3d(|context, _| {
            Ok(std::rc::Rc::new(
                self.render(&context.current_resolution()).into(),
            ))
        })
    }
}

impl BuiltinWorkbenchDefinition for Cylinder {
    fn id() -> &'static str {
        "Cylinder"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive3D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive3D(Box::new(Cylinder {
                radius_bottom: args.get("radius_bottom"),
                radius_top: args.get("radius_top"),
                height: args.get("height"),
            })))
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
