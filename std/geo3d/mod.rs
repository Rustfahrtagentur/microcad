// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macro::DefineBuiltinPrimitive3D;
use microcad_core::*;
use microcad_lang::{eval::*, parse::*};

/// The builtin sphere primitive, defined by its radius.
#[derive(DefineBuiltinPrimitive3D, Debug)]
pub struct Sphere {
    /// Radius of the sphere in millimeters
    pub radius: Scalar,
}

impl RenderHash for Sphere {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Sphere {
    fn render_geometry(
        &self,
        renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::Result<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::sphere(
            self.radius,
            n,
        )))
    }
}

/// The builtin cube primitive, defined by its size in the x, y, and z dimensions.
#[derive(DefineBuiltinPrimitive3D, Debug)]
pub struct Cube {
    /// Size of the cube in the x dimension in millimeters
    pub size_x: Scalar,
    /// Size of the cube in the y dimension in millimeters
    pub size_y: Scalar,
    /// Size of the cube in the z dimension in millimeters
    pub size_z: Scalar,
}

impl RenderHash for Cube {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Cube {
    fn render_geometry(
        &self,
        _renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::Result<geo3d::Geometry> {
        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::cube(
            self.size_x,
            self.size_y,
            self.size_z,
        )))
    }
}

/// The built-in cylinder primitive, defined by an bottom radius, top radius and height.
/// The cylinder is oriented along the z-axis.
#[derive(DefineBuiltinPrimitive3D, Debug)]
pub struct Cylinder {
    /// Bottom radius of the cylinder in millimeters
    pub radius_bottom: Scalar,
    /// Top radius of the cylinder in millimeters
    pub radius_top: Scalar,
    /// Height of the cylinder in millimeters
    pub height: Scalar,
}

impl RenderHash for Cylinder {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl geo3d::Primitive for Cylinder {
    fn render_geometry(
        &self,
        renderer: &mut dyn geo3d::Renderer,
    ) -> microcad_core::Result<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius_bottom / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::cylinder(
            self.radius_bottom,
            self.radius_top,
            self.height,
            n,
        )))
    }
}

use crate::NamespaceBuilder;

/// geo3d Builtin module
pub fn builtin_module() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("geo3d")
        .add(Sphere::builtin_module().into())
        .add(Cube::builtin_module().into())
        .add(Cylinder::builtin_module().into())
        .build()
}
