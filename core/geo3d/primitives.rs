// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D primitives

use std::rc::Rc;

use manifold_rs::Manifold;

use crate::{geo3d::RenderToMesh, *};

/// The builtin cube primitive, defined by its size in the x, y, and z dimensions.
#[derive(Debug, Clone)]
pub struct Cube {
    /// Size of the cube in millimeters.
    pub size: Vec3,
}

impl RenderToMesh for Cube {
    fn render_to_manifold(self, _: &RenderResolution) -> Rc<Manifold> {
        Rc::new(geo3d::Manifold::cube(self.size.x, self.size.y, self.size.z))
    }
}

/// The builtin sphere primitive, defined by its radius.
#[derive(Debug, Clone)]
pub struct Sphere {
    /// Radius of the sphere in millimeters.
    pub radius: Scalar,
}

impl RenderToMesh for Sphere {
    fn render_to_manifold(self, resolution: &RenderResolution) -> Rc<Manifold> {
        use std::f64::consts::PI;
        let segments = (self.radius / resolution.linear * PI * 0.5).max(3.0) as u32;
        Rc::new(geo3d::Manifold::sphere(self.radius, segments))
    }
}

/// The built-in cylinder primitive, defined by an bottom radius, top radius and height.
/// The cylinder is oriented along the z-axis.
#[derive(Debug, Clone)]
pub struct Cylinder {
    /// Bottom radius of the cylinder in millimeters.
    pub radius_bottom: Scalar,
    /// Top radius of the cylinder in millimeters.
    pub radius_top: Scalar,
    /// Height of the cylinder in millimeters.
    pub height: Scalar,
}

impl RenderToMesh for Cylinder {
    fn render_to_manifold(self, resolution: &RenderResolution) -> Rc<Manifold> {
        use std::f64::consts::PI;
        let n = (self.radius_bottom / resolution.linear * PI * 0.5).max(3.0) as u32;
        Rc::new(geo3d::Manifold::cylinder(
            self.radius_bottom,
            self.radius_top,
            self.height,
            n,
        ))
    }
}
