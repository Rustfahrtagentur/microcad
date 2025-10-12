// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use crate::traits::Align;

use super::*;

use geo::{ConvexHull, MultiPolygon};
use strum::IntoStaticStr;

/// A 2D Geometry which is independent from resolution.
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
    fn transformed_2d(&self, mat: &Mat3) -> Self {
        if self.is_areal() {
            let multi_polygon: MultiPolygon = self.clone().into();
            Self::MultiPolygon(multi_polygon.transformed_2d(mat))
        } else {
            match self {
                Geometry2D::LineString(line_string) => {
                    Self::LineString(line_string.transformed_2d(mat))
                }
                Geometry2D::MultiLineString(multi_line_string) => {
                    Self::MultiLineString(multi_line_string.transformed_2d(mat))
                }
                Geometry2D::Line(line) => Self::Line(line.transformed_2d(mat)),
                Geometry2D::Collection(geometries) => {
                    Self::Collection(geometries.transformed_2d(mat))
                }
                _ => unreachable!("Geometry type not supported"),
            }
        }
    }
}

impl Align for Geometry2D {
    fn align(&self) -> Self {
        if let Some(bounds) = self.fetch_bounds_2d().rect() {
            let d: Vec2 = bounds.center().x_y().into();
            self.transformed_2d(&Mat3::from_translation(-d))
        } else {
            self.clone()
        }
    }
}

impl From<Geometry2D> for MultiPolygon {
    fn from(geo: Geometry2D) -> Self {
        match geo {
            Geometry2D::Polygon(polygon) => polygon.into(),
            Geometry2D::MultiPolygon(multi_polygon) => multi_polygon,
            Geometry2D::Rect(rect) => MultiPolygon(vec![rect.to_polygon()]),
            Geometry2D::Collection(collection) => collection.into(),
            _ => MultiPolygon::empty(),
        }
    }
}

/// Something that can rendered into a 2D geometry with a certain resolution.
pub trait RenderToGeometry2D {
    /// Render self into some Geometry with a certain render resolution
    ///
    /// Note: We might want to have [`RenderCache`] as argument here, hence we return an `Rc`.
    fn render_to_geometry(&self, resolution: &RenderResolution) -> Rc<Geometry2D>;
}

impl RenderToGeometry2D for Rc<Geometry2D> {
    fn render_to_geometry(&self, _: &RenderResolution) -> Rc<Geometry2D> {
        self.clone()
    }
}
