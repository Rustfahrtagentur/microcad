// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Pie geometry.

use crate::*;

/// Pie geometry with offset.
#[derive(Debug, Clone)]
pub struct Pie {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Start angle.
    pub start_angle: Angle,

    /// End angle.
    pub end_angle: Angle,

    /// Offset.
    pub offset: Vec2,
}

impl Pie {
    /// Create a new pie.
    pub fn new(radius: Scalar, start_angle: Angle, end_angle: Angle, offset: Vec2) -> Self {
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
            offset,
        }
    }

    /// A pie is a circle when `offset_angle >= 360°`.
    pub fn is_circle(&self) -> bool {
        self.offset_angle() >= cgmath::Rad(360.0)
    }

    /// Calculate offset angle.
    pub fn offset_angle(&self) -> Angle {
        self.end_angle - self.start_angle
    }
}

impl FetchBounds2D for Pie {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        use geo::Coord;

        if self.radius > 0.0 {
            let r = Vec2::new(self.radius, self.radius);
            let min: (Scalar, Scalar) = (self.offset - r).into();
            let max: (Scalar, Scalar) = (self.offset + r).into();

            Some(Rect::new(Coord::from(min), Coord::from(max)))
        } else {
            None
        }
        .into()
    }
}

impl FetchPoints2D for Pie {
    fn fetch_points_2d(&self) -> Vec<Vec2> {
        vec![self.offset]
    }
}

impl RenderToMultiPolygon for Pie {
    fn render_to_polygon(&self, resolution: &RenderResolution) -> Option<Polygon> {
        use std::f64::consts::PI;
        let offset_angle = self.offset_angle();
        let n =
            resolution.circular_segments(self.radius) * (offset_angle / cgmath::Rad(360.0)) as u32;

        let points = if self.is_circle() {
            (0..n).map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.offset.x + self.radius * angle.cos(), y: self.offset.y + self.radius * angle.sin())
            }).collect()
        } else {
            (0..=n).map(|i| {
                let angle = self.start_angle + self.offset_angle() * (i as f64) / (n as f64);
                geo::coord!(x: self.offset.x + self.radius * angle.0.cos(), y: self.offset.y + self.radius * angle.0.sin())
            }).collect()
        };

        Some(Polygon::new(LineString::new(points), vec![]))
    }
}
