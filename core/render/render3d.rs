use super::*;
use crate::*;

pub trait Renderable3D: RenderHash {
    fn request_geometry(
        &self,
        renderer: &mut dyn Renderer3D,
    ) -> Result<std::rc::Rc<geo3d::Geometry>> {
        // Try to fetch the geometry from the render cache
        if let Some(hash) = self.render_hash() {
            if let Some(geometry) = renderer.fetch_geometry(hash) {
                return Ok(geometry);
            }
        }

        // If the geometry is not in the render cache, render it
        let geometry = self.render_geometry(renderer)?;
        Ok(std::rc::Rc::new(geometry))
    }

    fn render_geometry(&self, renderer: &mut dyn Renderer3D) -> Result<geo3d::Geometry>;
}

pub trait Renderer3D: Renderer {
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> Result<()>;

    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo3d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo3d::Geometry) -> Result<()> {
        match geometry {
            geo3d::Geometry::Mesh(m) => self.mesh(m),
            _ => unimplemented!(),
        }
    }

    fn render_node(&mut self, node: Node) -> Result<()>;
}
