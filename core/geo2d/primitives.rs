// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D primitives

use crate::{geo2d::*, *};

/// Line string.
pub type LineString = geo::LineString<Scalar>;
/// Multiple line strings.
pub type MultiLineString = geo::MultiLineString<Scalar>;
/// Polygon.
pub type Polygon = geo::Polygon<Scalar>;
/// Multiple polygons.
pub type MultiPolygon = geo::MultiPolygon<Scalar>;
/// Rectangle.
pub type Rect = geo::Rect<Scalar>;
/// Point.
pub type Point = geo::Point<Scalar>;

/// Circle with offset.
#[derive(Debug, Clone)]
pub struct Circle {
    /// Radius of the circle.
    pub radius: Scalar,

    /// Offset.
    pub offset: Vec2,
}

impl RenderToMultiPolygon for LineString {}
impl RenderToMultiPolygon for MultiLineString {}

impl RenderToMultiPolygon for Polygon {
    fn render_to_polygon(self, _: &RenderResolution) -> Option<Polygon> {
        Some(self)
    }
}

impl RenderToMultiPolygon for MultiPolygon {
    fn render_to_existing_multi_polygon(
        mut self,
        _resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        polygons.0.append(&mut self.0);
    }
}

impl RenderToMultiPolygon for Rect {
    fn render_to_polygon(self, _: &RenderResolution) -> Option<Polygon> {
        Some(self.to_polygon())
    }
}

impl Points2D for Rect {
    fn points_2d(&self) -> Vec<Vec2> {
        let min = self.min();
        let max = self.max();
        vec![
            Vec2::new(min.x, min.y),
            Vec2::new(min.x, max.y),
            Vec2::new(max.x, min.y),
            Vec2::new(max.x, max.y),
        ]
    }
}

impl RenderToMultiPolygon for Circle {
    fn render_to_polygon(self, resolution: &RenderResolution) -> Option<Polygon> {
        use std::f64::consts::PI;

        let n = (self.radius / resolution.linear * PI * 0.5).max(3.0) as u64;

        let range = 0..n;
        let points = range
            .map(|i| {
                let angle = 2.0 * PI * (i as f64) / (n as f64);
                geo::coord!(x: self.offset.x + self.radius * angle.cos(), y: self.offset.y + self.radius * angle.sin())
            })
            .collect();

        Some(Polygon::new(LineString::new(points), vec![]))
    }
}

impl Points2D for Circle {
    fn points_2d(&self) -> Vec<Vec2> {
        vec![self.offset]
    }
}
