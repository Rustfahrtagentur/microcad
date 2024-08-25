use crate::*;

use super::*;

pub trait Renderable2D: RenderHash {
    fn request_geometry(
        &self,
        renderer: &mut dyn Renderer2D,
    ) -> Result<std::rc::Rc<geo2d::Geometry>> {
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

    fn render_geometry(&self, renderer: &mut dyn Renderer2D) -> Result<geo2d::Geometry>;
}

pub trait Renderer2D {
    /// Precision in mm
    fn precision(&self) -> Scalar;

    fn change_render_state(&mut self, _: &str, _: &str) -> Result<()> {
        Ok(())
    }

    fn multi_polygon(&mut self, multi_polygon: &geo2d::MultiPolygon) -> Result<()>;

    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo2d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo2d::Geometry) -> Result<()> {
        match geometry {
            geo2d::Geometry::MultiPolygon(p) => self.multi_polygon(p),
            _ => unimplemented!(),
        }
    }

    fn render_node(&mut self, node: Node) -> Result<()>;
}
