// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mesh renderer

use crate::{
    CoreResult, Scalar,
    geo3d::{self},
};
use std::rc::Rc;

/// Renders a mesh
pub struct MeshRenderer {
    /// Render precision
    precision: Scalar,

    /// Triangle soup
    pub triangle_mesh: geo3d::TriangleMesh,
}

impl MeshRenderer {
    /// Create a MeshRenderer
    pub fn new(precision: Scalar) -> Self {
        Self {
            precision,
            triangle_mesh: geo3d::TriangleMesh::default(),
        }
    }
}

impl crate::Renderer for MeshRenderer {
    fn precision(&self) -> Scalar {
        self.precision
    }
}

impl Default for MeshRenderer {
    fn default() -> Self {
        Self {
            precision: 0.1,
            triangle_mesh: geo3d::TriangleMesh::default(),
        }
    }
}

impl geo3d::Renderer for MeshRenderer {
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> CoreResult<()> {
        self.triangle_mesh.append(mesh);
        Ok(())
    }

    fn fetch_geometry(&mut self, _hash: u64) -> Option<Rc<geo3d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo3d::Geometry) -> CoreResult<()> {
        match geometry {
            geo3d::Geometry::Mesh(mesh) => self.mesh(mesh),
            geo3d::Geometry::Manifold(manifold) => {
                let mesh = geo3d::TriangleMesh::from(manifold.to_mesh());
                self.mesh(&mesh)
            }
        }
    }
}
