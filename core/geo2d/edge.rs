// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D edge geometry.

use cgmath::InnerSpace;

use crate::{FetchBounds2D, Scalar, Vec2, geo2d};

/// A 2D edge type.
#[derive(Debug, Clone)]
pub struct Edge2D(pub geo2d::Point, pub geo2d::Point);

impl Edge2D {
    /// Return vector of this edge.
    pub fn vec(&self) -> Vec2 {
        Vec2::from(self.1.x_y()) - Vec2::from(self.0.x_y())
    }

    /// Return center of this edge.
    pub fn center(&self) -> geo2d::Point {
        (self.0 + self.1) * 0.5
    }

    /// Shorten edge on both ends by a certain amount.
    pub fn shorter(&self, amount: Scalar) -> Self {
        let d = self.vec();
        let d = 0.5 * d * (1.0 - amount / d.magnitude());
        let c = self.center();
        Self(c - (d.x, d.y).into(), c + (d.x, d.y).into())
    }
}

impl FetchBounds2D for Edge2D {
    fn fetch_bounds_2d(&self) -> geo2d::Bounds2D {
        geo2d::Bounds2D::new(self.0.x_y().into(), self.1.x_y().into())
    }
}
