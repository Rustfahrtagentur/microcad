// Copyright © 2025 The µcad authors <info@ucad.xyz>
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

    /// Coarse render resolution of 1.0mm.
    pub fn coarse() -> Self {
        Self { linear: 1.0 }
    }
}

impl std::ops::Mul<Mat3> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let scale = (rhs.x.x * rhs.y.y).sqrt();
        Self {
            linear: self.linear / scale,
        }
    }
}

impl std::ops::Mul<Mat4> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let scale = (rhs.x.x * rhs.y.y * rhs.z.z).powf(1.0 / 3.0);
        Self {
            linear: self.linear / scale,
        }
    }
}

impl Default for RenderResolution {
    fn default() -> Self {
        RenderResolution { linear: 0.1 }
    }
}
