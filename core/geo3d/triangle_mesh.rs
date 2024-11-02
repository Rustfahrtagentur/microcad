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

    /// Transform the mesh
    ///
    /// # Arguments
    /// - `transform`: Transformation matrix
    ///
    /// # Returns
    /// Transformed mesh
    pub fn transform(&self, transform: crate::Mat4) -> Self {
        let rot_mat = crate::Mat3::from_cols(
            transform.x.truncate(),
            transform.y.truncate(),
            transform.z.truncate(),
        );
        let vertices = self
            .vertices
            .iter()
            .map(|v| Vertex {
                pos: (transform * v.pos.extend(1.0)).truncate(),
                normal: rot_mat * v.normal,
            })
            .collect();

        TriangleMesh {
            vertices,
            triangle_indices: self.triangle_indices.clone(),
        }
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

#[test]
fn test_triangle_mesh_transform() {
    let mesh = TriangleMesh {
        vertices: vec![
            Vertex {
                pos: Vec3::new(0.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                pos: Vec3::new(1.0, 0.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
            },
            Vertex {
                pos: Vec3::new(0.0, 1.0, 0.0),
                normal: Vec3::new(0.0, 0.0, 1.0),
            },
        ],
        triangle_indices: vec![Triangle(0, 1, 2)],
    };

    let mesh = mesh.transform(crate::Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)));

    assert_eq!(mesh.vertices[0].pos, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[0].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[1].pos, Vec3::new(2.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[1].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[2].pos, Vec3::new(1.0, 3.0, 3.0));
    assert_eq!(mesh.vertices[2].normal, Vec3::new(0.0, 0.0, 1.0));
}
