// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D primitives

use geo::{AffineOps, Coord};

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

impl FetchBounds2D for Circle {
    fn fetch_bounds_2d(&self) -> Bounds2D {
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

impl RenderToMultiPolygon for LineString {}

impl Transformed2D for LineString {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl RenderToMultiPolygon for MultiLineString {}

impl Transformed2D for MultiLineString {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl RenderToMultiPolygon for Polygon {
    fn render_to_polygon(self, _: &RenderResolution) -> Option<Polygon> {
        Some(self)
    }
}

impl Transformed2D for Polygon {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
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

impl Transformed2D for MultiPolygon {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl FetchPoints2D for Rect {
    fn fetch_points_2d(&self) -> Vec<Vec2> {
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

impl RenderToMultiPolygon for Rect {
    fn render_to_polygon(self, _: &RenderResolution) -> Option<Polygon> {
        Some(self.to_polygon())
    }
}

impl Transformed2D for Rect {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl Transformed2D<Polygon> for Rect {
    fn transformed_2d(&self, render_resolution: &RenderResolution, mat: &Mat3) -> Polygon {
        self.render_to_polygon(render_resolution)
            .expect("Polygon")
            .transformed_2d(render_resolution, mat)
    }
}

/// Convert a line string to a vector of [`Scalar`].
pub fn line_string_to_vec(line_string: &LineString) -> Vec<Scalar> {
    line_string
        .points()
        .flat_map(|point| vec![point.x(), point.y()])
        .collect()
}

/// Convert a polygon to a vector of [`Scalar`].
///
/// Exterior polygon has CW winding order, interior polygon have CCW winding order.
pub fn polygon_to_vec(polygon: &Polygon) -> Vec<Scalar> {
    let mut vec = line_string_to_vec(polygon.exterior());
    polygon.interiors().iter().for_each(|interior| {
        vec.append(&mut line_string_to_vec(interior));
    });
    vec
}

/// Convert a multi polygon into a vector of coordinates.
pub fn multi_polygon_to_vec(multi_polygon: &MultiPolygon) -> Vec<Vec<Scalar>> {
    multi_polygon.0.iter().map(polygon_to_vec).collect()
}
