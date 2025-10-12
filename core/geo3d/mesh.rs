// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

use crate::*;
use cgmath::ElementWise;
use manifold_rs::{Manifold, Mesh};

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
    /// Is this mesh empty?
    pub fn is_empty(&self) -> bool {
        self.vertices.is_empty() || self.triangle_indices.is_empty()
    }

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
        self.vertices.append(&mut other.vertices.clone());
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

    /// Calculate volume of mesh.
    pub fn volume(&self) -> f64 {
        self.triangles()
            .map(|t| t.signed_volume())
            .sum::<f64>()
            .abs()
    }

    /// Fetch a vertex triangle from index triangle.
    pub fn fetch_triangle<'a>(&'a self, tri: Triangle<u32>) -> Triangle<&'a Vertex> {
        Triangle(
            &self.vertices[tri.0 as usize],
            &self.vertices[tri.1 as usize],
            &self.vertices[tri.2 as usize],
        )
    }

    /// TriangleMesh.
    pub fn repair(&mut self, bounds: &Bounds3D) {
        // 1. Merge duplicate vertices using a spatial hash map (or hashmap keyed on quantized position)

        let min = bounds.min().unwrap();
        let inv_size = 1.0 / (bounds.max().unwrap() - bounds.min().unwrap());

        // Quantize vertex positions to grid to group duplicates
        let quantize = |pos: &Vec3| {
            let mapped = (pos - min).mul_element_wise(inv_size) * (u32::MAX as Scalar);
            (
                mapped.x.floor() as u32,
                mapped.y.floor() as u32,
                mapped.z.floor() as u32,
            )
        };

        let mut vertex_map: std::collections::HashMap<(u32, u32, u32), u32> =
            std::collections::HashMap::new();
        let mut new_vertices: Vec<Vertex> = Vec::with_capacity(self.vertices.len());
        let mut remap: Vec<u32> = vec![0; self.vertices.len()];

        for (i, vertex) in self.vertices.iter().enumerate() {
            let key = quantize(&vertex.pos);
            if let Some(&existing_idx) = vertex_map.get(&key) {
                // Duplicate vertex found
                remap[i] = existing_idx;
            } else {
                // New unique vertex
                let new_idx = new_vertices.len() as u32;
                new_vertices.push(*vertex);
                vertex_map.insert(key, new_idx);
                remap[i] = new_idx;
            }
        }

        self.vertices = new_vertices;

        // 2. Remap triangle indices and remove degenerate triangles (zero area or repeated vertices)
        let mut new_triangles = Vec::with_capacity(self.triangle_indices.len());

        for tri in &self.triangle_indices {
            let tri_idx = crate::Triangle(
                remap[tri.0 as usize],
                remap[tri.1 as usize],
                remap[tri.2 as usize],
            );

            if tri_idx.is_degenerated() {
                continue;
            }

            // Optional: check zero-area triangle by computing cross product
            let tri = self.fetch_triangle(tri_idx);

            if tri.area() < 1e-8 {
                continue; // Degenerate triangle
            }

            new_triangles.push(tri_idx);
        }

        self.triangle_indices = new_triangles;
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

impl Transformed3D for TriangleMesh {
    fn transformed_3d(&self, mat: &Mat4) -> Self {
        let rot_mat = crate::Mat3::from_cols(mat.x.truncate(), mat.y.truncate(), mat.z.truncate());
        Self {
            vertices: self
                .vertices
                .iter()
                .map(|v| Vertex {
                    pos: (mat * v.pos.extend(1.0)).truncate(),
                    normal: rot_mat * v.normal,
                })
                .collect(),
            triangle_indices: self.triangle_indices.clone(),
        }
    }
}

impl WithBounds3D<TriangleMesh> {
    /// Update bounds and repair mesh.
    pub fn repair(&mut self) {
        self.update_bounds();
        self.inner.repair(&self.bounds);
    }
}

impl From<Geometry3D> for TriangleMesh {
    fn from(geo: Geometry3D) -> Self {
        match geo {
            Geometry3D::Mesh(triangle_mesh) => triangle_mesh,
            Geometry3D::Manifold(manifold) => manifold.to_mesh().into(),
            Geometry3D::Collection(ref collection) => collection.into(),
        }
    }
}

impl From<&Geometries3D> for TriangleMesh {
    fn from(geo: &Geometries3D) -> Self {
        let mut mesh = TriangleMesh::default();
        geo.iter()
            .for_each(|geo| mesh.append(&geo.as_ref().clone().into()));
        mesh
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

    let mesh = mesh.transformed_3d(&crate::Mat4::from_translation(Vec3::new(1.0, 2.0, 3.0)));

    assert_eq!(mesh.vertices[0].pos, Vec3::new(1.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[0].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[1].pos, Vec3::new(2.0, 2.0, 3.0));
    assert_eq!(mesh.vertices[1].normal, Vec3::new(0.0, 0.0, 1.0));
    assert_eq!(mesh.vertices[2].pos, Vec3::new(1.0, 3.0, 3.0));
    assert_eq!(mesh.vertices[2].normal, Vec3::new(0.0, 0.0, 1.0));
}
