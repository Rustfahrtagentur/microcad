// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use microcad_builtin_proc_macro::DefineBuiltinPrimitive3D;
use microcad_core::*;
use microcad_lang::{eval::*, parse::*};

#[derive(DefineBuiltinPrimitive3D, Debug)]
pub struct Sphere {
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

#[derive(DefineBuiltinPrimitive3D, Debug)]
pub struct Cube {
    pub size_x: Scalar,
    pub size_y: Scalar,
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

use crate::NamespaceBuilder;

/// geo3d Builtin module
pub fn builtin_module() -> std::rc::Rc<NamespaceDefinition> {
    NamespaceBuilder::new("geo3d")
        .add(Sphere::builtin_module().into())
        .add(Cube::builtin_module().into())
        .build()
}
