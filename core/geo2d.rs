// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Scalar;

/// Line string
pub type LineString = geo::LineString<Scalar>;
/// Multiple line string
pub type MultiLineString = geo::MultiLineString<Scalar>;
/// Polygon
pub type Polygon = geo::Polygon<Scalar>;
/// Multiple polygons
pub type MultiPolygon = geo::MultiPolygon<Scalar>;
/// Rectangle
pub type Rect = geo::Rect<Scalar>;
/// Point
pub type Point = geo::Point<Scalar>;

/// Geometry
pub enum Geometry {
    /// Line string
    LineString(LineString),
    /// Multiple line string
    MultiLineString(MultiLineString),
    /// Polygon
    Polygon(Polygon),
    /// Multiple polygon
    MultiPolygon(MultiPolygon),
    /// Rectangle
    Rect(Rect),
    /// Point
    Point(Point),
}

impl Geometry {
    /// Try to convert geometry into multiple polygons
    pub fn try_convert_to_multi_polygon(&self) -> Option<MultiPolygon> {
        match self {
            Geometry::LineString(_) | Geometry::Point(_) | Geometry::MultiLineString(_) => None,
            Geometry::Polygon(polygon) => Some(MultiPolygon::new(vec![polygon.clone()])),
            Geometry::MultiPolygon(multi_polygon) => Some(multi_polygon.clone()),
            Geometry::Rect(rect) => Some(MultiPolygon::new(vec![Self::rect_to_polygon(rect)])),
        }
    }

    fn rect_to_polygon(rect: &Rect) -> Polygon {
        use geo::line_string;
        let line_string = line_string![
            (x: rect.min().x, y: rect.min().y),
            (x: rect.max().x, y: rect.min().y),
            (x: rect.max().x, y: rect.max().y),
            (x: rect.min().x, y: rect.max().y),
            (x: rect.min().x, y: rect.min().y),
        ];
        Polygon::new(line_string, vec![])
    }

    /// Apply boolean operation
    pub fn boolean_op(
        &self,
        other: &Self,
        op: &crate::algorithm::boolean_op::BooleanOp,
    ) -> Option<Self> {
        let a = self.try_convert_to_multi_polygon()?;
        let b = other.try_convert_to_multi_polygon()?;
        use geo::BooleanOps;
        let result = a.boolean_op(&b, op.into());
        Some(Geometry::MultiPolygon(result))
    }
}

/// Shortcut to create a MultiPolygon
pub fn line_string_to_multi_polygon(line_string: LineString) -> MultiPolygon {
    MultiPolygon::new(vec![Polygon::new(line_string, vec![])])
}
