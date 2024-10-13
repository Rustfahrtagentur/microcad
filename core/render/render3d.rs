// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D renderable

use super::*;
use crate::*;

/// A `Primitive3D` is a hashable renderable object that can be rendered by a Renderer3D
pub trait Primitive3D: RenderHash + std::fmt::Debug {
    /// Get geometry
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

    /// Render geometry
    fn render_geometry(&self, renderer: &mut dyn Renderer3D) -> Result<geo3d::Geometry>;
}

/// 3D Renderer
pub trait Renderer3D: Renderer {
    /// add mesh
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> Result<()>;

    /// Get geometry
    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo3d::Geometry>> {
        None
    }

    /// Render geometry
    fn render_geometry(&mut self, geometry: &geo3d::Geometry) -> Result<()> {
        match geometry {
            geo3d::Geometry::Mesh(m) => self.mesh(m),
            _ => unimplemented!(),
        }
    }

    /// Render node
    fn render_node(&mut self, node: ModelNode) -> Result<()>;
}
