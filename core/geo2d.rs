// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Geometry

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

/// Macro crate a 2d coordinate
pub use geo::coord;

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
}

impl Geometry {
    /// Try to convert geometry into multiple polygons
    pub fn try_convert_to_multi_polygon(&self) -> Option<MultiPolygon> {
        match self {
            Geometry::LineString(_) | Geometry::MultiLineString(_) => None,
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

    fn line_string_vertices(l: &LineString) -> Vec<crate::Vec2> {
        l.coords()
            .map(|c| crate::Vec2::new(c.x, c.y))
            .collect::<Vec<_>>()
    }

    fn polygon_vertices(p: &Polygon) -> Vec<crate::Vec2> {
        let mut vertices = Self::line_string_vertices(p.exterior());
        p.interiors()
            .iter()
            .for_each(|interior| vertices.append(&mut Self::line_string_vertices(interior)));
        vertices
    }

    /// Returns the 2d vertices of geometry
    pub fn vertices(&self) -> Vec<crate::Vec2> {
        match &self {
            Self::LineString(l) => Self::line_string_vertices(l),
            Self::MultiLineString(ml) => ml.iter().flat_map(Self::line_string_vertices).collect(),
            Self::Polygon(p) => Self::polygon_vertices(p),
            Self::MultiPolygon(mp) => mp.iter().flat_map(Self::polygon_vertices).collect(),
            Self::Rect(r) => vec![
                crate::Vec2::new(r.min().x, r.min().y),
                crate::Vec2::new(r.max().x, r.min().y),
                crate::Vec2::new(r.min().x, r.max().y),
                crate::Vec2::new(r.max().x, r.max().y),
            ],
        }
    }
}

/// Shortcut to create a MultiPolygon
pub fn line_string_to_multi_polygon(line_string: LineString) -> MultiPolygon {
    MultiPolygon::new(vec![Polygon::new(line_string, vec![])])
}
