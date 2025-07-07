// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

mod geometry;
mod primitives;

use crate::*;

use geo::BoundingRect;
pub use geometry::*;
pub use primitives::*;

/// Trait to render a [`Geometry2D`] into a multi polygon.
///
/// Implement this trait
pub trait RenderToMultiPolygon: Sized {
    /// Render geometry into a [`Polygon`].
    ///
    /// Implement this method if the geometry only returns a single polygon.
    /// Line geometry returns [`None`].
    fn render_to_polygon(self, _: &RenderResolution) -> Option<Polygon> {
        None
    }

    /// Render a geometry into a new multi polygon.
    ///
    /// This method uses [`RenderToMultiPolygon::render_to_existing_multi_polygon`] and does not need to be reimplemented.  
    fn render_to_multi_polygon(self, resolution: &RenderResolution) -> MultiPolygon {
        let mut polygons = geo::MultiPolygon(vec![]);
        self.render_to_existing_multi_polygon(resolution, &mut polygons);
        polygons
    }

    /// Render a geometry into a new multi polygon and attaches it to a list of existing polygons.
    ///
    /// Reimplement this function preferably if the geometry returns more than one polygon.
    fn render_to_existing_multi_polygon(
        self,
        resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        if let Some(polygon) = self.render_to_polygon(resolution) {
            polygons.0.push(polygon);
        }
    }
}

/// Trait to return all points of 2D geometry.
pub trait Points2D {
    /// Returns all points.
    fn points_2d(&self) -> Vec<Vec2>;
}

/// Trait to return a bounding box of 2D geometry.
pub trait Bounds2D {
    fn bounds_2d(&self) -> Option<Rect>;

    fn bounds_2d_multi(v: &[&Self]) -> Option<Rect> {
        for s in v {}
    }
}

impl Bounds2D for Geometry {
    fn bounds_2d(&self) -> Option<Rect> {
        match &self {
            Geometry::LineString(line_string) => line_string.bounding_rect(),
            Geometry::MultiLineString(multi_line_string) => multi_line_string.bounding_rect(),
            Geometry::Polygon(polygon) => polygon.bounding_rect(),
            Geometry::MultiPolygon(multi_polygon) => multi_polygon.bounding_rect(),
            Geometry::Rect(rect) => Some(*rect),
            Geometry::Circle(circle) => circle.bounds_2d(),
        }
    }
}
