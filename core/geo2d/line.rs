// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D edge geometry.

use cgmath::{Angle, InnerSpace, Rad};
use geo::AffineOps;

use crate::*;

/// A 2D edge type.
#[derive(Debug, Clone)]
pub struct Line(pub geo2d::Point, pub geo2d::Point);

impl Line {
    /// Shorten edge on both ends by a certain amount.
    pub fn shorter(&self, amount: Scalar) -> Self {
        let d = self.vec();
        let d = 0.5 * d * (1.0 - amount / d.magnitude());
        let c = self.center();
        Self(c - (d.x, d.y).into(), c + (d.x, d.y).into())
    }

    /// Calculate the radius edge from a circle.
    ///
    /// An edge from the circle's center to a point on its periphery.
    pub fn radius_edge(circle: &Circle, angle: &Rad<Scalar>) -> Self {
        let Circle {
            radius,
            offset: Vec2 { x, y },
        } = &circle;
        let x1 = x + radius * angle.cos();
        let y1 = y + radius * angle.sin();
        Self(geo::Point::new(*x, *y), geo::Point::new(x1, y1))
    }

    /// Return vector of this edge.
    pub fn vec(&self) -> Vec2 {
        Vec2::from(self.1.x_y()) - Vec2::from(self.0.x_y())
    }

    /// Return center of this edge.
    pub fn center(&self) -> geo2d::Point {
        (self.0 + self.1) * 0.5
    }

    /// Calculate angle in radians.
    pub fn angle(&self) -> Rad<Scalar> {
        let (p1, p2) = (self.0, self.1);
        let dx = p2.x() - p1.x();
        let dy = p2.y() - p1.y();
        Rad(dy.atan2(dx))
    }

    /// 2D Transformation matrix for this edge (rotation and offset).
    pub fn matrix(&self) -> Mat3 {
        let rot = Mat2::from_angle(self.angle());
        let mut m = Mat3::from_translation(self.0.x_y().into());
        m.x = Vec3::new(rot.x.x, rot.x.y, 0.0);
        m.y = Vec3::new(rot.y.x, rot.y.y, 0.0);
        m
    }
}

impl FetchBounds2D for Line {
    fn fetch_bounds_2d(&self) -> geo2d::Bounds2D {
        geo2d::Bounds2D::new(self.0.x_y().into(), self.1.x_y().into())
    }
}

impl Transformed2D for Line {
    fn transformed_2d(&self, _: &RenderResolution, mat: &Mat3) -> Self {
        let transform = &geo2d::mat3_to_affine_transform(mat);
        Self(
            self.0.affine_transform(transform),
            self.1.affine_transform(transform),
        )
    }
}
