// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Specializations for 3D triangles.

use cgmath::{InnerSpace, Vector3};

use crate::*;

impl Triangle<Vector3<f32>> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vector3<f32> {
        (self.2 - self.0).cross(self.1 - self.0)
    }
}

impl Triangle<&Vector3<f32>> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vector3<f32> {
        (self.2 - self.0).cross(self.1 - self.0)
    }

    /// Get area of triangle.
    pub fn area(&self) -> f32 {
        self.normal().magnitude()
    }

    /// Get signed volume of triangle
    ///
    /// <https://stackoverflow.com/questions/1406029/how-to-calculate-the-volume-of-a-3d-mesh-object-the-surface-of-which-is-made-up>
    pub fn signed_volume(&self) -> f32 {
        let v210 = self.2.x * self.1.y * self.0.z;
        let v120 = self.1.x * self.2.y * self.0.z;
        let v201 = self.2.x * self.0.y * self.1.z;
        let v021 = self.0.x * self.2.y * self.1.z;
        let v102 = self.1.x * self.0.y * self.2.z;
        let v012 = self.0.x * self.1.y * self.2.z;

        (1.0 / 6.0) * (-v210 + v120 + v201 - v021 - v102 + v012)
    }
}
