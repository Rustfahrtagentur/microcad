// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Specializations for 3D triangles.

use cgmath::InnerSpace;

use crate::*;

impl Triangle<Vertex> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vec3 {
        (self.2.pos - self.0.pos).cross(self.1.pos - self.0.pos)
    }
}

impl Triangle<&Vertex> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vec3 {
        (self.2.pos - self.0.pos).cross(self.1.pos - self.0.pos)
    }

    /// Get area of triangle.
    pub fn area(&self) -> Scalar {
        self.normal().magnitude()
    }

    /// Get signed volume of triangle
    ///
    /// <https://stackoverflow.com/questions/1406029/how-to-calculate-the-volume-of-a-3d-mesh-object-the-surface-of-which-is-made-up>
    pub fn signed_volume(&self) -> f64 {
        let v210 = self.2.pos.x * self.1.pos.y * self.0.pos.z;
        let v120 = self.1.pos.x * self.2.pos.y * self.0.pos.z;
        let v201 = self.2.pos.x * self.0.pos.y * self.1.pos.z;
        let v021 = self.0.pos.x * self.2.pos.y * self.1.pos.z;
        let v102 = self.1.pos.x * self.0.pos.y * self.2.pos.z;
        let v012 = self.0.pos.x * self.1.pos.y * self.2.pos.z;

        (1.0 / 6.0) * (-v210 + v120 + v201 - v021 - v102 + v012)
    }
}
