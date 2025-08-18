// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

use crate::*;

/// Circle with offset.
#[derive(Debug, Clone)]
pub struct Circle {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Offset.
    pub offset: Vec2,
}

impl FetchBounds2D for Circle {
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

impl FetchPoints2D for Circle {
    fn fetch_points_2d(&self) -> Vec<Vec2> {
        vec![self.offset]
    }
}

impl RenderToMultiPolygon for Circle {
    fn render_to_polygon(self, resolution: &RenderResolution) -> Option<Polygon> {
        use std::f64::consts::PI;

        let n = (self.radius / resolution.linear * PI * 0.5).max(3.0);
        let n = 2_u32.pow(n.log2().ceil() as u32).min(1024);

        let points = (0..n)
            .map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.offset.x + self.radius * angle.cos(), y: self.offset.y + self.radius * angle.sin())
            })
            .collect();

        Some(Polygon::new(LineString::new(points), vec![]))
    }
}
