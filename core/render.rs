// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Render trait.

use cgmath::InnerSpace;

use crate::*;

/// A trait renders something.
pub trait Render<T> {
    /// The render function.
    fn render(&self, resolution: &RenderResolution) -> T;
}

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

    /// Get the number segments for a circle as power of 2.
    ///
    /// The minimal number of segments is 4, the maximum number of segments is 1024.
    pub fn circular_segments(&self, radius: Scalar) -> u32 {
        let n = (radius / self.linear * std::f64::consts::PI * 0.5).max(3.0);
        2_u32.pow(n.log2().ceil() as u32).clamp(4, 1024)
    }
}

impl std::ops::Mul<Mat3> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat3) -> Self::Output {
        let scale = (rhs.x.magnitude() * rhs.y.magnitude()).sqrt();
        Self {
            linear: self.linear / scale,
        }
    }
}

impl std::ops::Mul<Mat4> for RenderResolution {
    type Output = RenderResolution;

    fn mul(self, rhs: Mat4) -> Self::Output {
        let scale = (rhs.x.magnitude() * rhs.y.magnitude() * rhs.z.magnitude()).powf(1.0 / 3.0);
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

impl std::fmt::Display for RenderResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "@{}mm", self.linear)
    }
}
