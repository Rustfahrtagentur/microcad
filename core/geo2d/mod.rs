// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

mod bounds;
mod circle;
mod collection;
mod geometry;
mod line;
mod primitives;
mod size;

use crate::*;

pub use bounds::*;
pub use circle::*;
pub use collection::*;
use geo::AffineTransform;
pub use geometry::*;
pub use line::*;
pub use primitives::*;
pub use size::*;

/// Trait to return all points of 2D geometry.
pub trait FetchPoints2D {
    /// Returns all points.
    fn fetch_points_2d(&self) -> Vec<Vec2>;
}

/// Transformed version of a 2D geometry.
pub trait Transformed2D<T = Self> {
    /// Transform from matrix.
    fn transformed_2d(&self, mat: &Mat3) -> T;
}

/// Convert a [`Mat3`]` into an affine transform.
pub(crate) fn mat3_to_affine_transform(mat: &Mat3) -> AffineTransform {
    geo::AffineTransform::new(mat.x.x, mat.y.x, mat.z.x, mat.x.y, mat.y.y, mat.z.y)
}
