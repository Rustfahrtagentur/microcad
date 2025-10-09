// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use microcad_core::{Geometry2D, LineString, Polygon, RenderResolution, RenderToGeometry2D};
use microcad_lang::builtin::*;

/// Circle with offset.
#[derive(Debug, Clone)]
pub struct Circle(microcad_core::Circle);

impl RenderToGeometry2D for Circle {
    fn render_to_geometry(&self, resolution: &RenderResolution) -> Rc<Geometry2D> {
        use std::f64::consts::PI;
        let n = resolution.circular_segments(self.0.radius);
        let points = (0..n)
            .map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.0.offset.x + self.0.radius * angle.cos(), y: self.0.offset.y + self.0.radius * angle.sin())
            })
            .collect();

        Rc::new(Geometry2D::Polygon(Polygon::new(
            LineString::new(points),
            vec![],
        )))
    }
}

impl BuiltinWorkbenchDefinition for Circle {
    fn id() -> &'static str {
        "Circle"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        use microcad_core::*;

        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Circle(
                Circle {
                    radius: args.get("radius"),
                    offset: (args.get("cx"), args.get("cy")).into(),
                },
            ))))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(radius: Scalar),
            parameter!(cx: Scalar = 0.0),
            parameter!(cy: Scalar = 0.0),
        ]
        .into_iter()
        .collect()
    }
}
