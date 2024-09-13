// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 2D Renderable

use super::*;
use crate::*;

/// 2D Renderable
pub trait Renderable2D: RenderHash {
    /// Get geometry
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

    /// Render geometry
    fn render_geometry(&self, renderer: &mut dyn Renderer2D) -> Result<geo2d::Geometry>;
}

/// 2D Renderer
pub trait Renderer2D: Renderer {
    /// Render multiple polygons
    fn multi_polygon(&mut self, multi_polygon: &geo2d::MultiPolygon) -> Result<()>;

    /// Get geometry
    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo2d::Geometry>> {
        None
    }

    /// Render geometry
    fn render_geometry(&mut self, geometry: &geo2d::Geometry) -> Result<()> {
        match geometry {
            geo2d::Geometry::MultiPolygon(p) => self.multi_polygon(p),
            _ => unimplemented!(),
        }
    }

    /// Render node
    fn render_node(&mut self, node: Node) -> Result<()>;
}
