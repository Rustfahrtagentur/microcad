// Copyright © 2024 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render resolution

use crate::*;

/// Render resolution when rendering things to polygons or meshes.
#[derive(Debug, Clone)]
pub struct RenderResolution {
    /// Linear resolution in millimeters (Default = 0.1mm)
    pub linear: Scalar,
}

impl RenderResolution {
    /// Create new render resolution.
    pub fn new(linear: Scalar) -> Self {
        Self { linear }
    }
}

impl std::ops::Mul<Mat3> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let scale = Vec2::new(rhs.x.x, rhs.y.y);
        Self {
            linear: self.linear * scale.magnitude(),
        }
    }
}

impl std::ops::Mul<Mat4> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let scale = Vec3::new(rhs.x.x, rhs.y.y, rhs.z.z);
        Self {
            linear: self.linear * scale.magnitude(),
        }
    }
}

impl Default for RenderResolution {
    fn default() -> Self {
        RenderResolution { linear: 0.1 }
    }
}
