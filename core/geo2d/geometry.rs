// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::traits::Align;

use super::*;

use geo::{ConvexHull, MultiPolygon};
use strum::IntoStaticStr;

/// A 2D Geometry which independent from resolution.
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
    /// Line.
    Line(Line),
    /// Collection,
    Collection(Geometries2D),
}

impl Geometry2D {
    /// Return name of geometry.
    pub fn name(&self) -> &'static str {
        self.into()
    }

    /// Apply boolean operation.
    pub fn boolean_op(self, other: Self, op: &BooleanOp) -> geo2d::MultiPolygon {
        use geo::BooleanOps;
        self.to_multi_polygon()
            .boolean_op(&other.to_multi_polygon(), op.into())
    }

    /// Convert geometry to a multi_polygon.
    pub fn to_multi_polygon(&self) -> MultiPolygon {
        match self {
            Geometry2D::Line(_) | Geometry2D::LineString(_) | Geometry2D::MultiLineString(_) => {
                MultiPolygon::empty()
            }
            Geometry2D::Polygon(polygon) => MultiPolygon(vec![polygon.clone()]),
            Geometry2D::MultiPolygon(multi_polygon) => multi_polygon.clone(),
            Geometry2D::Rect(rect) => MultiPolygon(vec![rect.to_polygon()]),
            Geometry2D::Collection(collection) => collection.to_multi_polygon(),
        }
    }

    /// Apply hull operation.
    pub fn hull(&self) -> Self {
        match self {
            Geometry2D::LineString(line_string) => Geometry2D::Polygon(line_string.convex_hull()),
            Geometry2D::MultiLineString(multi_line_string) => {
                Geometry2D::Polygon(multi_line_string.convex_hull())
            }
            Geometry2D::Polygon(polygon) => Geometry2D::Polygon(polygon.convex_hull()),
            Geometry2D::MultiPolygon(multi_polygon) => {
                Geometry2D::Polygon(multi_polygon.convex_hull())
            }
            Geometry2D::Rect(rect) => Geometry2D::Rect(*rect),
            Geometry2D::Line(line) => Geometry2D::Polygon(
                LineString::new(vec![line.0.into(), line.1.into()]).convex_hull(),
            ),
            Geometry2D::Collection(collection) => Geometry2D::Polygon(collection.hull()),
        }
    }

    /// Returns true if this geometry fills an area (e.g. like a polygon or circle).
    pub fn is_areal(&self) -> bool {
        !matches!(
            self,
            Geometry2D::LineString(_)
                | Geometry2D::MultiLineString(_)
                | Geometry2D::Line(_)
                | Geometry2D::Collection(_)
        )
    }
}

impl FetchBounds2D for MultiPolygon {
    fn fetch_bounds_2d(&self) -> Bounds2D {
        use geo::BoundingRect;
        self.bounding_rect().into()
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
            Geometry2D::MultiPolygon(multi_polygon) => multi_polygon.fetch_bounds_2d(),
            Geometry2D::Rect(rect) => Some(*rect).into(),
            Geometry2D::Line(line) => line.fetch_bounds_2d(),
            Geometry2D::Collection(collection) => collection.fetch_bounds_2d(),
        }
    }
}

impl Transformed2D for Geometry2D {
    fn transformed_2d(&self, resolution: &RenderResolution, mat: &Mat3) -> Self {
        if self.is_areal() {
            Self::MultiPolygon(
                self.render_to_multi_polygon(resolution)
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
                Geometry2D::Line(line) => Self::Line(line.transformed_2d(resolution, mat)),
                Geometry2D::Collection(geometries) => {
                    Self::Collection(geometries.transformed_2d(resolution, mat))
                }
                _ => unreachable!("Geometry type not supported"),
            }
        }
    }
}

impl Align for Geometry2D {
    fn align(&self, resolution: &RenderResolution) -> Self {
        if let Some(bounds) = self.fetch_bounds_2d().rect() {
            let d: Vec2 = bounds.center().x_y().into();
            self.transformed_2d(resolution, &Mat3::from_translation(-d))
        } else {
            self.clone()
        }
    }
}

impl RenderToMultiPolygon for Geometry2D {
    fn render_to_existing_multi_polygon(
        &self,
        resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        match self {
            Geometry2D::Polygon(polygon) => polygons.0.push(polygon.clone()),
            Geometry2D::MultiPolygon(multi_polygon) => {
                polygons.0.append(&mut multi_polygon.0.clone())
            }
            Geometry2D::Rect(rect) => polygons
                .0
                .push(rect.render_to_polygon(resolution).expect("Polygon")),
            Geometry2D::Collection(geometries) => {
                geometries.render_to_existing_multi_polygon(resolution, polygons);
            }
            _ => {}
        }
    }
}
