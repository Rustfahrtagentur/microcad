// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use std::rc::Rc;

use super::*;
/// Macro crate a 2d coordinate
use strum::IntoStaticStr;

/// Geometry
#[derive(IntoStaticStr, Clone, Debug)]
pub enum Geometry {
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
}

impl Geometry {
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
        Some(Geometry::MultiPolygon(result))
    }

    /// Returns true if this geometry fills an area (e.g. like a polygon or circle).
    pub fn is_areal(&self) -> bool {
        !matches!(self, Geometry::LineString(_) | Geometry::MultiLineString(_))
    }

    /// Apply boolean operation to multiple geometries.
    pub fn boolean_op_multi(
        geometries: Vec<Rc<Self>>,
        resolution: &RenderResolution,
        op: &crate::BooleanOp,
    ) -> Option<Rc<Self>> {
        if geometries.is_empty() {
            return None;
        }

        Some(
            geometries[1..]
                .iter()
                .fold(geometries[0].clone(), |acc, geo| {
                    if let Some(r) = acc.boolean_op(resolution, geo.as_ref(), op) {
                        Rc::new(r)
                    } else {
                        acc
                    }
                }),
        )
    }

    /// Return a new geometry with the given transform
    pub fn transformed(self, resolution: &RenderResolution, mat: crate::Mat3) -> Self {
        // Extract matrix components
        let a = mat.x.x;
        let b = mat.y.x;
        let x_off = mat.z.x;
        let d = mat.x.y;
        let e = mat.y.y;
        let y_off = mat.z.y;

        use geo::AffineOps;
        let transform = geo::AffineTransform::new(a, b, x_off, d, e, y_off);

        if self.is_areal() {
            let polygons = self
                .render_to_multi_polygon(resolution)
                .affine_transform(&transform);

            Self::MultiPolygon(polygons)
        } else {
            match self {
                Geometry::LineString(line_string) => {
                    Self::LineString(line_string.affine_transform(&transform))
                }
                Geometry::MultiLineString(multi_line_string) => {
                    Self::MultiLineString(multi_line_string.affine_transform(&transform))
                }
                _ => unreachable!(),
            }
        }
    }
}

impl RenderToMultiPolygon for Geometry {
    fn render_to_existing_multi_polygon(
        self,
        resolution: &RenderResolution,
        polygons: &mut MultiPolygon,
    ) {
        match self {
            Geometry::Polygon(polygon) => polygons.0.push(polygon),
            Geometry::MultiPolygon(mut multi_polygon) => polygons.0.append(&mut multi_polygon.0),
            Geometry::Rect(rect) => polygons
                .0
                .push(rect.render_to_polygon(resolution).expect("Polygon")),
            Geometry::Circle(circle) => polygons
                .0
                .push(circle.render_to_polygon(resolution).expect("Polygon")),
            _ => {}
        }
    }
}
