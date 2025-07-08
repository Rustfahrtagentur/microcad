// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use super::*;

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

    /// Returns true if this geometry fills an area (e.g. like a polygon or circle).
    pub fn is_areal(&self) -> bool {
        !matches!(
            self,
            Geometry2D::LineString(_) | Geometry2D::MultiLineString(_)
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
                Geometry2D::LineString(line_string) => {
                    Self::LineString(line_string.affine_transform(&transform))
                }
                Geometry2D::MultiLineString(multi_line_string) => {
                    Self::MultiLineString(multi_line_string.affine_transform(&transform))
                }
                _ => unreachable!(),
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
