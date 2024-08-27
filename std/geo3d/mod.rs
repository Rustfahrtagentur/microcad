use microcad_core::*;
use microcad_render::{RenderHash, Renderable3D};

pub struct Sphere {
    pub radius: Scalar,
}

impl RenderHash for Sphere {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

impl Renderable3D for Sphere {
    fn render_geometry(&self, renderer: &mut dyn render::Renderer3D) -> Result<geo3d::Geometry> {
        use std::f64::consts::PI;
        let n = (self.radius / renderer.precision() * PI * 0.5).max(3.0) as u32;

        Ok(geo3d::Geometry::Manifold(geo3d::Manifold::sphere(
            self.radius,
            n,
        )))
    }
}
