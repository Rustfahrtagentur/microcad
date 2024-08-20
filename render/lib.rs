pub mod geo2d;
pub mod svg;
pub mod tree;

use std::hash;

use geo2d::Geometry;
use microcad_core::Scalar;
use tree::Node;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Error {
    NotImplemented,
}

pub trait RenderHash {
    fn render_hash(&self) -> Option<u64> {
        None
    }
}

pub trait Renderable2D: RenderHash {
    fn request_geometry(
        &self,
        renderer: &mut dyn Renderer2D,
    ) -> Result<std::rc::Rc<Geometry>, Error> {
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

    fn render_geometry(&self, renderer: &mut dyn Renderer2D) -> Result<geo2d::Geometry, Error>;
}

pub trait Renderer2D {
    /// Precision in mm
    fn precision(&self) -> Scalar;

    fn change_render_state(&mut self, _: &str, _: &str) -> Result<(), Error> {
        Ok(())
    }

    fn multi_polygon(&mut self, multi_polygon: &geo2d::MultiPolygon) -> Result<(), Error>;

    fn fetch_geometry(&mut self, hash: u64) -> Option<std::rc::Rc<geo2d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo2d::Geometry) -> Result<(), Error> {
        match geometry {
            geo2d::Geometry::MultiPolygon(p) => self.multi_polygon(p),
            _ => unimplemented!(),
        }
    }

    fn render_node(&mut self, node: Node) -> Result<(), Error>;
}
