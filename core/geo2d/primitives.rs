// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D primitives

use geo::AffineOps;

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

impl Transformed2D for LineString {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl Transformed2D for MultiLineString {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl Transformed2D for Polygon {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl Transformed2D for MultiPolygon {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
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

impl Transformed2D for Rect {
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        self.affine_transform(&mat3_to_affine_transform(mat))
    }
}

impl Transformed2D<Polygon> for Rect {
    fn transformed_2d(&self, mat: &Mat3) -> Polygon {
        self.to_polygon().transformed_2d(mat)
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
