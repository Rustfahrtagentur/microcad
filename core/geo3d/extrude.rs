// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D extrusion algorithm.

use cgmath::{InnerSpace, Point3, SquareMatrix, Transform};

use geo::TriangulateEarcut;

use crate::*;

/// Extrude.
pub trait Extrude {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh;

    fn cap(&self, _m: &Mat4) -> TriangleMesh {
        TriangleMesh::default()
    }

    fn linear_extrude(&self, height: Scalar) -> TriangleMesh {
        let m_a = Mat4::identity();
        let m_b = Mat4::from_translation(Vec3::new(0.0, 0.0, height));
        let mut mesh = self.extrude_slice(&m_a, &m_b);
        //   mesh.append(&self.cap(&m_a));
        //  mesh.append(&self.cap(&m_b));
        mesh
    }
}

impl Extrude for LineString {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();

        let points = self.points();
        let len = points.len();
        if len < 2 {
            return mesh; // Not enough points to extrude
        }

        let transform_point = |p: &Point, m: &Mat4| -> Vec3 {
            let local = Point3::new(p.x(), p.y(), 0.0);
            m.transform_point(local).to_homogeneous().truncate()
        };

        let mut bottom_indices = Vec::with_capacity(len);
        let mut top_indices = Vec::with_capacity(len);

        // Add vertices with position and zeroed normals
        for point in points {
            let bottom_pos = transform_point(&point, m_a);
            let top_pos = transform_point(&point, m_b);

            let bottom_index = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: bottom_pos,
                normal: Vec3::new(0.0, 0.0, 0.0),
            });

            let top_index = mesh.vertices.len() as u32;
            mesh.vertices.push(Vertex {
                pos: top_pos,
                normal: Vec3::new(0.0, 0.0, 0.0),
            });

            bottom_indices.push(bottom_index);
            top_indices.push(top_index);
        }

        let is_closed = self.is_closed();
        let range = if is_closed { 0..len } else { 0..(len - 1) };

        for i in range {
            let next = (i + 1) % len;

            let bl = bottom_indices[i];
            let br = bottom_indices[next];
            let tl = top_indices[i];
            let tr = top_indices[next];

            // Triangle 1: bl, br, tr
            mesh.triangle_indices.push(Triangle(bl, br, tr));
            Vertex::accumulate_normal(&mut mesh.vertices, bl, br, tr);

            // Triangle 2: bl, tr, tl
            mesh.triangle_indices.push(Triangle(bl, tr, tl));
            Vertex::accumulate_normal(&mut mesh.vertices, bl, tr, tl);
        }

        // Normalize vertex normals
        mesh.vertices
            .iter_mut()
            .for_each(|v| v.normal = v.normal.normalize());

        mesh
    }
}

impl Extrude for Polygon {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();
        mesh.append(&self.exterior().extrude_slice(m_a, m_b));
        for interior in self.interiors() {
            mesh.append(&interior.extrude_slice(m_a, m_b));
        }
        mesh
    }

    fn cap(&self, m: &Mat4) -> TriangleMesh {
        let raw_triangulation = self.earcut_triangles_raw();

        TriangleMesh {
            vertices: raw_triangulation
                .vertices
                .as_slice()
                .chunks_exact(3)
                .map(|chunk| {
                    let p = Point3::new(chunk[0], chunk[1], chunk[2]);
                    let p = m.transform_point(p);
                    Vertex {
                        pos: Vec3::new(p.x, p.y, p.z),
                        normal: m.z.truncate().normalize(),
                    }
                })
                .collect(),
            triangle_indices: raw_triangulation
                .triangle_indices
                .as_slice()
                .chunks_exact(3)
                .map(|chunk| Triangle(chunk[0] as u32, chunk[1] as u32, chunk[2] as u32))
                .collect(),
        }
    }
}

impl Extrude for MultiPolygon {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();
        self.iter().for_each(|polygon| {
            mesh.append(&polygon.extrude_slice(m_a, m_b));
        });
        mesh
    }

    fn cap(&self, m: &Mat4) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();
        self.iter().for_each(|polygon| {
            mesh.append(&polygon.cap(m));
        });
        mesh
    }
}

impl Extrude for Geometries2D {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        self.to_multi_polygon().extrude_slice(m_a, m_b)
    }

    fn cap(&self, m: &Mat4) -> TriangleMesh {
        self.to_multi_polygon().cap(m)
    }
}
