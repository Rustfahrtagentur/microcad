// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::{Bounds3D, FetchBounds3D, Vec3};
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

impl Triangle<&Vertex> {
    /// Get normal of triangle
    pub fn normal(&self) -> Vec3 {
        (self.2.pos - self.0.pos).cross(self.1.pos - self.0.pos)
    }

    /// Get signed volume of triangle
    ///
    /// <https://stackoverflow.com/questions/1406029/how-to-calculate-the-volume-of-a-3d-mesh-object-the-surface-of-which-is-made-up>
    pub fn signed_volume(&self) -> f64 {
        let v210 = self.2.pos.x * self.1.pos.y * self.0.pos.z;
        let v120 = self.1.pos.x * self.2.pos.y * self.0.pos.z;
        let v201 = self.2.pos.x * self.0.pos.y * self.1.pos.z;
        let v021 = self.0.pos.x * self.2.pos.y * self.1.pos.z;
        let v102 = self.1.pos.x * self.0.pos.y * self.2.pos.z;
        let v012 = self.0.pos.x * self.1.pos.y * self.2.pos.z;

        (1.0 / 6.0) * (-v210 + v120 + v201 - v021 - v102 + v012)
    }
}

/// Triangle mesh
#[derive(Default, Clone)]
pub struct TriangleMesh {
    /// Mesh Vertices
    pub vertices: Vec<Vertex>,
    /// Triangle indices
    pub triangle_indices: Vec<Triangle<u32>>,
}

/// Triangle iterator state.
pub struct Triangles<'a> {
    triangle_mesh: &'a TriangleMesh,
    index: usize,
}

impl<'a> Iterator for Triangles<'a> {
    type Item = Triangle<&'a Vertex>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.triangle_mesh.triangle_indices.len() {
            let t = self.triangle_mesh.triangle_indices[self.index];
            self.index += 1;
            Some(Triangle(
                &self.triangle_mesh.vertices[t.0 as usize],
                &self.triangle_mesh.vertices[t.1 as usize],
                &self.triangle_mesh.vertices[t.2 as usize],
            ))
        } else {
            None
        }
    }
}

impl TriangleMesh {
    /// Clear mesh.
    pub fn clear(&mut self) {
        self.vertices.clear();
        self.triangle_indices.clear();
    }

    /// Fetch triangles.
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

    /// Append a triangle mesh.
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

    /// Triangles iterator.
    pub fn triangles(&'_ self) -> Triangles<'_> {
        Triangles {
            triangle_mesh: self,
            index: 0,
        }
    }

    /// Convert mesh to manifold.
    pub fn to_manifold(&self) -> Manifold {
        let vertices = self
            .vertices
            .iter()
            .flat_map(|v| vec![v.pos.x as f32, v.pos.y as f32, v.pos.z as f32])
            .collect::<Vec<_>>();

        let triangle_indices = self
            .triangle_indices
            .iter()
            .flat_map(|t| vec![t.0, t.1, t.2])
            .collect::<Vec<_>>();

        assert_eq!(vertices.len(), self.vertices.len() * 3);
        assert_eq!(triangle_indices.len(), self.triangle_indices.len() * 3);

        Manifold::from_mesh(Mesh::new(&vertices, &triangle_indices))
    }

    /// Transform the mesh.
    ///
    /// # Arguments
    /// - `transform`: Transformation matrix
    ///
    /// # Returns
    /// Transformed mesh
    pub fn transform(&self, transform: &crate::Mat4) -> Self {
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

    /// Calculate volume of mesh.
    pub fn volume(&self) -> f64 {
        self.triangles()
            .map(|t| t.signed_volume())
            .sum::<f64>()
            .abs()
    }
}

impl FetchBounds3D for TriangleMesh {
    fn fetch_bounds_3d(&self) -> Bounds3D {
        self.vertices.iter().map(|vertex| vertex.pos).collect()
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
                .step_by(3)
                .map(|i| Vertex {
                    pos: Vec3::new(
                        vertices[i] as f64,
                        vertices[i + 1] as f64,
                        vertices[i + 2] as f64,
                    ),
                    normal: Vec3::new(0.0, 0.0, 0.0),
                })
                .collect(),
            triangle_indices: (0..indices.len())
                .step_by(3)
                .map(|i| Triangle(indices[i], indices[i + 1], indices[i + 2]))
                .collect(),
        }
    }
}

impl From<TriangleMesh> for Mesh {
    fn from(mesh: TriangleMesh) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for v in &mesh.vertices {
            vertices.push(v.pos.x as f32);
            vertices.push(v.pos.y as f32);
            vertices.push(v.pos.z as f32);
        }

        for t in &mesh.triangle_indices {
            indices.push(t.0);
            indices.push(t.1);
            indices.push(t.2);
        }

        Mesh::new(vertices.as_slice(), indices.as_slice())
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

    let mesh = mesh.transform(&crate::Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)));

    assert_eq!(mesh.vertices[0].pos, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[0].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[1].pos, Vec3::new(2.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[1].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[2].pos, Vec3::new(1.0, 3.0, 3.0));
    assert_eq!(mesh.vertices[2].normal, Vec3::new(0.0, 0.0, 1.0));
}
