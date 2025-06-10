// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use super::*;
/// Macro crate a 2d coordinate
use strum::IntoStaticStr;

/// Geometry
#[derive(IntoStaticStr)]
pub enum Geometry {
    /// Line string
    LineString(LineString),
    /// Multiple lirc::*ne string
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
    pub fn boolean_op(&self, other: &Self, op: &crate::BooleanOp) -> Option<Self> {
        let a = self.try_convert_to_multi_polygon()?;
        let b = other.try_convert_to_multi_polygon()?;
        use geo::BooleanOps;
        let result = a.boolean_op(&b, op.into());
        Some(Geometry::MultiPolygon(result))
    }

    /// Apply boolean operation to multiple geometries
    pub fn boolean_op_multi(geometries: Vec<Rc<Self>>, op: &crate::BooleanOp) -> Option<Rc<Self>> {
        if geometries.is_empty() {
            return None;
        }

        Some(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(geo.as_ref(), op) {
                        Rc::new(r)
                    } else {
                        acc
                    }
                }),
        )
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

    /// Return a new geometry with the given transform
    pub fn transform(&self, mat: crate::Mat3) -> Self {
        // Extract matrix components
        let a = mat.x.x;
        let b = mat.y.x;
        let x_off = mat.z.x;
        let d = mat.x.y;
        let e = mat.y.y;
        let y_off = mat.z.y;

        use geo::AffineOps;
        let transform = geo::AffineTransform::new(a, b, x_off, d, e, y_off);

        match &self {
            Self::LineString(l) => Self::LineString(l.affine_transform(&transform)),
            Self::MultiLineString(ml) => Self::MultiLineString(ml.affine_transform(&transform)),
            Self::Polygon(p) => Self::Polygon(p.affine_transform(&transform)),
            Self::MultiPolygon(mp) => Self::MultiPolygon(mp.affine_transform(&transform)),
            Self::Rect(r) => Self::Rect(r.affine_transform(&transform)),
        }
    }
}

/// Shortcut to create a MultiPolygon
pub fn line_string_to_multi_polygon(line_string: LineString) -> MultiPolygon {
    MultiPolygon::new(vec![Polygon::new(line_string, vec![])])
}
