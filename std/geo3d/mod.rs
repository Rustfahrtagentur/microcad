use microcad_builtin_proc_macro::DefineBuiltInRenderable3D;
use microcad_core::*;
use microcad_lang::{eval::*, parse::*};
use microcad_render::{RenderHash, Renderable3D};

#[derive(DefineBuiltInRenderable3D)]
pub struct Sphere {
    pub radius: Scalar,
}

impl RenderHash for Sphere {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl Renderable3D for Sphere {
    fn render_geometry(
        &self,
        renderer: &mut dyn render::Renderer3D,
    ) -> microcad_core::Result<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::sphere(
            self.radius,
            n,
        )))
    }
}

#[derive(DefineBuiltInRenderable3D)]
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

impl Renderable3D for Cube {
    fn render_geometry(
        &self,
        _renderer: &mut dyn render::Renderer3D,
    ) -> microcad_core::Result<geo3d::Geometry> {
        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::cube(
            self.size_x,
            self.size_y,
            self.size_z,
        )))
    }
}

use crate::ModuleBuilder;

pub fn builtin_module() -> std::rc::Rc<ModuleDefinition> {
    ModuleBuilder::new("geo3d")
        .add_builtin_module(Sphere::builtin_module())
        .add_builtin_module(Cube::builtin_module())
        .build()
}
