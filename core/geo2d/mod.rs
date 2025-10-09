// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

mod bounds;
mod circle;
mod collection;
mod geometry;
mod line;
mod primitives;
mod renderable;
mod size;

use crate::*;

pub use bounds::*;
pub use circle::*;
pub use collection::*;
use geo::AffineTransform;
pub use geometry::*;
pub use line::*;
pub use primitives::*;
pub use renderable::*;
pub use size::*;

/// Trait to render a [`Geometry2D`] into a multi polygon.
pub trait RenderToMultiPolygon: Sized {
    /// Render geometry into a [`Polygon`].
    ///
    /// Implement this method if the geometry only returns a single polygon.
    /// Line geometry returns [`None`].
    fn render_to_polygon(&self, _: &RenderResolution) -> Option<Polygon> {
        None
    }

    /// Render a geometry into a new multi polygon.
    ///
    /// This method uses [`RenderToMultiPolygon::render_to_existing_multi_polygon`] and does not need to be reimplemented.  
    fn render_to_multi_polygon(&self, resolution: &RenderResolution) -> MultiPolygon {
        let mut polygons = geo::MultiPolygon(vec![]);
        self.render_to_existing_multi_polygon(resolution, &mut polygons);
        polygons
    }

    /// Render a geometry into a new multi polygon and attaches it to a list of existing polygons.
    ///
    /// Reimplement this function preferably if the geometry returns more than one polygon.
    fn render_to_existing_multi_polygon(
        &self,
        resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        if let Some(polygon) = self.render_to_polygon(resolution) {
            polygons.0.push(polygon);
        }
    }
}

/// Trait to return all points of 2D geometry.
pub trait FetchPoints2D {
    /// Returns all points.
    fn fetch_points_2d(&self) -> Vec<Vec2>;
}

/// Transformed version of a 2D geometry.
pub trait Transformed2D<T = Self> {
    /// Transform from matrix.
    fn transformed_2d(&self, render_resolution: &RenderResolution, mat: &Mat3) -> T;
}

/// Convert a [`Mat3`]` into an affine transform.
pub(crate) fn mat3_to_affine_transform(mat: &Mat3) -> AffineTransform {
    geo::AffineTransform::new(mat.x.x, mat.y.x, mat.z.x, mat.x.y, mat.y.y, mat.z.y)
}
