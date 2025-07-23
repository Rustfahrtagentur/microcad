// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

use geo::ConvexHull;
use strum::IntoStaticStr;

/// Geometry
#[derive(IntoStaticStr, Clone, Debug)]
pub enum Geometry2D {
    /// Line string.
    LineString(LineString),
    /// Multiple line strings.
    MultiLineString(MultiLineString),
    /// Polygon.
    Polygon(Polygon),
    /// Multiple polygons.
    MultiPolygon(MultiPolygon),
    /// Rectangle.
    Rect(Rect),
    /// Circle.
    Circle(Circle),
    /// Edge.
    Edge(Edge2D),
}

impl Geometry2D {
    /// Apply boolean operation.
    pub fn boolean_op(
        &self,
        resolution: &RenderResolution,
        other: &Self,
        op: &BooleanOp,
    ) -> Option<Self> {
        let a = self.clone().render_to_multi_polygon(resolution);
        let b = other.clone().render_to_multi_polygon(resolution);
        use geo::BooleanOps;
        let result = a.boolean_op(&b, op.into());
        Some(Geometry2D::MultiPolygon(result))
    }

    /// Apply hull operation.
    pub fn hull(self) -> Self {
        match self {
            Geometry2D::LineString(line_string) => Geometry2D::Polygon(line_string.convex_hull()),
            Geometry2D::MultiLineString(multi_line_string) => {
                Geometry2D::Polygon(multi_line_string.convex_hull())
            }
            Geometry2D::Polygon(polygon) => Geometry2D::Polygon(polygon.convex_hull()),
            Geometry2D::MultiPolygon(multi_polygon) => {
                Geometry2D::Polygon(multi_polygon.convex_hull())
            }
            Geometry2D::Rect(rect) => Geometry2D::Rect(rect),
            Geometry2D::Circle(circle) => Geometry2D::Circle(circle),
            Geometry2D::Edge(edge2_d) => Geometry2D::Edge(edge2_d),
        }
    }

    /// Returns true if this geometry fills an area (e.g. like a polygon or circle).
    pub fn is_areal(&self) -> bool {
        !matches!(
            self,
            Geometry2D::LineString(_) | Geometry2D::MultiLineString(_)
        )
    }
}

impl FetchBounds2D for Geometry2D {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        use geo::BoundingRect;

        match &self {
            Geometry2D::LineString(line_string) => line_string.bounding_rect().into(),
            Geometry2D::MultiLineString(multi_line_string) => {
                multi_line_string.bounding_rect().into()
            }
            Geometry2D::Polygon(polygon) => polygon.bounding_rect().into(),
            Geometry2D::MultiPolygon(multi_polygon) => multi_polygon.bounding_rect().into(),
            Geometry2D::Rect(rect) => Some(*rect).into(),
            Geometry2D::Circle(circle) => circle.fetch_bounds_2d(),
            Geometry2D::Edge(edge) => edge.fetch_bounds_2d(),
        }
    }
}

impl Transformed2D for Geometry2D {
    fn transformed_2d(&self, resolution: &RenderResolution, mat: &Mat3) -> Self {
        if self.is_areal() {
            Self::MultiPolygon(
                self.clone()
                    .render_to_multi_polygon(resolution)
                    .transformed_2d(resolution, mat),
            )
        } else {
            match self {
                Geometry2D::LineString(line_string) => {
                    Self::LineString(line_string.transformed_2d(resolution, mat))
                }
                Geometry2D::MultiLineString(multi_line_string) => {
                    Self::MultiLineString(multi_line_string.transformed_2d(resolution, mat))
                }
                _ => unreachable!("Geometry type not supported"),
            }
        }
    }
}

impl RenderToMultiPolygon for Geometry2D {
    fn render_to_existing_multi_polygon(
        self,
        resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        match self {
            Geometry2D::Polygon(polygon) => polygons.0.push(polygon),
            Geometry2D::MultiPolygon(mut multi_polygon) => polygons.0.append(&mut multi_polygon.0),
            Geometry2D::Rect(rect) => polygons
                .0
                .push(rect.render_to_polygon(resolution).expect("Polygon")),
            Geometry2D::Circle(circle) => polygons
                .0
                .push(circle.render_to_polygon(resolution).expect("Polygon")),
            _ => {}
        }
    }
}
