// Copyright © 2024 The µCAD authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::Vec3;
use manifold_rs::{Manifold, Mesh};

/// Vertex
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// position
    pub pos: Vec3,
    /// normal vector
    pub normal: Vec3,
}

/// Triangle
#[derive(Clone, Copy, Debug)]
pub struct Triangle<T>(pub T, pub T, pub T);

impl Triangle<Vertex> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vec3 {
        (self.2.pos - self.0.pos).cross(self.1.pos - self.0.pos)
    }
}

/// Triangle mesh
#[derive(Default, Clone)]
pub struct TriangleMesh {
    /// Mesh Vertices
    pub vertices: Vec<Vertex>,
    /// Triangle indicies
    pub triangle_indices: Vec<Triangle<u32>>,
}

impl TriangleMesh {
    /// Clear mesh
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangle_indices.clear();
    }

    /// Fetch triangles
    pub fn fetch_triangles(&self) -> Vec<Triangle<Vertex>> {
        self.triangle_indices
            .iter()
            .map(|t| {
                Triangle(
                    self.vertices[t.0 as usize],
                    self.vertices[t.1 as usize],
                    self.vertices[t.2 as usize],
                )
            })
            .collect()
    }

    /// Append a triangle mesh
    pub fn append(&mut self, other: &TriangleMesh) {
        let offset = self.vertices.len() as u32;
        self.vertices.extend_from_slice(&other.vertices);
        self.triangle_indices.extend(
            other
                .triangle_indices
                .iter()
                .map(|t| Triangle(t.0 + offset, t.1 + offset, t.2 + offset)),
        )
    }

    /// convert intro manifold
    pub fn to_manifold(&self) -> Manifold {
        Manifold::from_mesh(self.into())
    }
}

impl From<Mesh> for TriangleMesh {
    fn from(mesh: Mesh) -> Self {
        let vertices = mesh.vertices();
        let indices = mesh.indices();

        // TODO: We could use unsafe std::ptr::copy and cast::transmute to avoid deep copy
        // of vertices and indices

        TriangleMesh {
            vertices: (0..vertices.len())
                .step_by(6)
                .map(|i| Vertex {
                    pos: Vec3::new(
                        vertices[i] as f64,
                        vertices[i + 1] as f64,
                        vertices[i + 2] as f64,
                    ),
                    normal: Vec3::new(
                        vertices[i + 3] as f64,
                        vertices[i + 4] as f64,
                        vertices[i + 5] as f64,
                    ),
                })
                .collect(),
            triangle_indices: (0..indices.len())
                .step_by(3)
                .map(|i| Triangle(indices[i], indices[i + 1], indices[i + 2]))
                .collect(),
        }
    }
}

impl From<&TriangleMesh> for Mesh {
    fn from(mesh: &TriangleMesh) -> Self {
        mesh.to_manifold().to_mesh()
    }
}

impl From<Manifold> for TriangleMesh {
    fn from(manifold: Manifold) -> Self {
        TriangleMesh::from(manifold.to_mesh())
    }
}
