// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mesh renderer

use crate::{
    geo3d::{self},
    Scalar,
};

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
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> crate::Result<()> {
        self.triangle_mesh.append(mesh);
        Ok(())
    }

    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo3d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo3d::Geometry) -> crate::Result<()> {
        match geometry {
            geo3d::Geometry::Mesh(mesh) => self.mesh(mesh),
            geo3d::Geometry::Manifold(manifold) => {
                let mesh = geo3d::TriangleMesh::from(manifold.to_mesh());
                self.mesh(&mesh)
            }
        }
    }

    fn render_node(&mut self, node: crate::geo3d::Node) -> crate::Result<()> {
        let inner = node.borrow();
        use crate::geo3d::NodeInner;

        match &*inner {
            NodeInner::Group => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
                return Ok(());
            }
            NodeInner::Geometry(geometry) => {
                self.render_geometry(geometry)?;
            }
            NodeInner::Transform(transform) => {
                let mut renderer = MeshRenderer::new(self.precision);

                for child in node.children() {
                    renderer.render_node(child)?;
                }

                self.mesh(&renderer.triangle_mesh.transform(transform))?;
            }
        }

        Ok(())
    }
}
