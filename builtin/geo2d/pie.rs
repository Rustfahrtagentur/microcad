// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_core::*;
use microcad_lang::builtin::*;

/// Pie geometry with offset.
#[derive(Debug, Clone)]
pub struct Pie {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Start angle.
    pub start_angle: Angle,

    /// End angle.
    pub end_angle: Angle,
}

impl Pie {
    /// Create a new pie.
    pub fn new(radius: Scalar, start_angle: Angle, end_angle: Angle) -> Self {
        use cgmath::Angle;
        let mut start_angle = start_angle.normalize();
        let mut end_angle = end_angle.normalize();
        if start_angle > end_angle {
            std::mem::swap(&mut start_angle, &mut end_angle);
        }

        Self {
            radius,
            start_angle,
            end_angle,
        }
    }

    /// A pie is a circle when `offset_angle >= 360°`.
    pub fn is_circle(&self) -> bool {
        self.offset_angle() >= cgmath::Deg(360.0).into()
    }

    /// Calculate offset angle.
    pub fn offset_angle(&self) -> Angle {
        self.end_angle - self.start_angle
    }
}

impl RenderToGeometry2D for Pie {
    fn render_to_geometry(&self, resolution: &RenderResolution) -> std::rc::Rc<Geometry2D> {
        use std::f64::consts::PI;
        let offset_angle = self.offset_angle();
        let n =
            (resolution.circular_segments(self.radius) as f64 * (offset_angle.0 / PI / 2.0)) as u32;

        let points = if !self.is_circle() && n > 0 {
            (0..=n)
                .map(|i| {
                    let angle = self.start_angle + offset_angle * (i as f64) / (n as f64);
                    geo::coord!(x: angle.0.cos(), y: angle.0.sin()) * self.radius
                })
                .chain(
                    // Add center point.
                    [geo::coord!(x: 0.0 as Scalar, y: 0.0 as Scalar)]
                        .iter()
                        .cloned(),
                )
                .collect()
        } else {
            (0..n)
                .map(|i| {
                    let angle = 2.0 * PI * (i as f64) / (n as f64);
                    geo::coord!(x: self.radius * angle.cos(), y: self.radius * angle.sin())
                })
                .collect()
        };

        std::rc::Rc::new(Geometry2D::Polygon(Polygon::new(
            LineString::new(points),
            vec![],
        )))
    }
}

impl FetchBounds2D for Pie {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        use geo::Coord;

        if self.radius > 0.0 {
            let r = Vec2::new(self.radius, self.radius);
            let min: (Scalar, Scalar) = (-r).into();
            let max: (Scalar, Scalar) = r.into();

            Some(Rect::new(Coord::from(min), Coord::from(max)))
        } else {
            None
        }
        .into()
    }
}

impl BuiltinWorkbenchDefinition for Pie {
    fn id() -> &'static str {
        "Pie"
    }

    fn kind() -> BuiltinWorkbenchKind {
        BuiltinWorkbenchKind::Primitive2D
    }

    fn workpiece_function() -> &'static BuiltinWorkpieceFn {
        &|args| {
            Ok(BuiltinWorkpieceOutput::Primitive2D(Box::new(Pie::new(
                args.get("radius"),
                args.get("start_angle"),
                args.get("end_angle"),
            ))))
        }
    }

    fn parameters() -> ParameterValueList {
        [
            parameter!(radius: Scalar),
            parameter!(start_angle: Angle = 0.0),
            parameter!(end_angle: Angle = 90.0),
        ]
        .into_iter()
        .collect()
    }
}
