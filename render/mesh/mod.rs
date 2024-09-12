// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Mesh renderer

use crate::*;
use microcad_core::{
    geo3d::{self},
    Error, Scalar,
};

pub struct MeshRenderer {
    precision: Scalar,
    triangle_mesh: geo3d::TriangleMesh,
}

impl MeshRenderer {
    pub fn new(precision: Scalar) -> Self {
        Self {
            precision,
            triangle_mesh: geo3d::TriangleMesh::default(),
        }
    }

    pub fn triangle_mesh(&self) -> &geo3d::TriangleMesh {
        &self.triangle_mesh
    }
}

impl Renderer for MeshRenderer {
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

impl Renderer3D for MeshRenderer {
    fn mesh(&mut self, mesh: &geo3d::TriangleMesh) -> microcad_core::Result<()> {
        self.triangle_mesh.append(mesh);
        Ok(())
    }

    fn fetch_geometry(&mut self, _hash: u64) -> Option<std::rc::Rc<geo3d::Geometry>> {
        None
    }

    fn render_geometry(&mut self, geometry: &geo3d::Geometry) -> microcad_core::Result<()> {
        match geometry {
            geo3d::Geometry::Mesh(mesh) => self.mesh(mesh),
            geo3d::Geometry::Manifold(manifold) => {
                let mesh = geo3d::TriangleMesh::from(manifold.to_mesh());
                self.mesh(&mesh)
            }
        }
    }

    fn render_node(&mut self, node: Node) -> microcad_core::Result<()> {
        let inner = node.borrow();

        match &*inner {
            NodeInner::Export(_) | NodeInner::Group | NodeInner::Root => {
                for child in node.children() {
                    self.render_node(child.clone())?;
                }
                return Ok(());
            }
            NodeInner::Algorithm(algorithm) => {
                let new_node = algorithm.process_3d(self, node.clone())?;
                self.render_node(new_node)?;
            }
            NodeInner::Geometry3D(geometry) => {
                self.render_geometry(geometry)?;
            }
            NodeInner::Renderable3D(renderable) => {
                let geometry = renderable.request_geometry(self)?;
                self.render_geometry(&geometry)?;
            }
            NodeInner::Transform(_) => unimplemented!(),
            NodeInner::Geometry2D(_) | NodeInner::Renderable2D(_) => {
                return Err(Error::NotImplemented);
            }
        }

        Ok(())
    }
}
