// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D extrusion algorithm.

use std::f64::consts::PI;

use cgmath::{InnerSpace, Point3, SquareMatrix, Transform};

use geo::TriangulateEarcut;

use crate::*;

/// Extrude.
pub trait Extrude {
    /// Extrude a single slice of the geometry with top and bottom plane.
    ///
    ///
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh;

    /// Generate the cap geometry.
    fn cap(&self, _m: &Mat4, _normal: &Vec3, _bottom: bool) -> TriangleMesh {
        TriangleMesh::default()
    }

    /// Perform a linear extrusion with a certain height.
    fn linear_extrude(&self, height: Scalar) -> WithBounds3D<TriangleMesh> {
        let m_a = Mat4::identity();
        let m_b = Mat4::from_translation(Vec3::new(0.0, 0.0, height));
        let mut mesh = self.extrude_slice(&m_a, &m_b);
        mesh.append(&self.cap(&m_a, &-m_a.z.truncate(), true));
        mesh.append(&self.cap(&m_b, &m_b.z.truncate(), false));
        let mut mesh = WithBounds3D::new(mesh);
        mesh.repair();
        mesh
    }

    /// Perform a revolve extrusion with a certain angle.
    fn revolve_extrude(&self, angle_rad: Angle, segments: usize) -> WithBounds3D<TriangleMesh> {
        let mut mesh = TriangleMesh::default();
        if segments < 2 {
            return WithBounds3D::default();
        }

        let delta = angle_rad / segments as Scalar;

        // Generate all rotation matrices
        let transforms: Vec<Mat4> = (0..=segments)
            .map(|i| {
                let a = delta * i as Scalar;
                Mat4::from_angle_y(-a)
            })
            .collect();

        // For each segment, extrude between slice i and i+1
        for i in 0..segments {
            let m_a = &transforms[i];
            let m_b = &transforms[i + 1];
            let slice = self.extrude_slice(m_a, m_b);
            mesh.append(&slice);
        }

        // Optionally add caps at start and end
        if angle_rad.0 < PI * 2.0 {
            let m_start = &transforms[0];
            let m_end = transforms.last().expect("Transform");
            let normal_start = m_start.x.truncate(); // Points outward at start
            let normal_end = -m_end.x.truncate(); // Points inward at end

            mesh.append(&self.cap(m_start, &normal_start, true));
            mesh.append(&self.cap(m_end, &normal_end, false));
        }

        let mut mesh = WithBounds3D::new(mesh);
        mesh.repair();
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

    fn cap(&self, m: &Mat4, normal: &Vec3, flip: bool) -> TriangleMesh {
        let raw_triangulation = self.earcut_triangles_raw();

        TriangleMesh {
            vertices: raw_triangulation
                .vertices
                .as_slice()
                .chunks_exact(2)
                .map(|chunk| {
                    let p = Point3::new(chunk[0], chunk[1], 0.0);
                    let p = m.transform_point(p);
                    Vertex {
                        pos: Vec3::new(p.x, p.y, p.z),
                        normal: *normal,
                    }
                })
                .collect(),
            triangle_indices: raw_triangulation
                .triangle_indices
                .as_slice()
                .chunks_exact(3)
                .map(|chunk| match flip {
                    true => Triangle(chunk[2] as u32, chunk[1] as u32, chunk[0] as u32),
                    false => Triangle(chunk[0] as u32, chunk[1] as u32, chunk[2] as u32),
                })
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

    fn cap(&self, m: &Mat4, normal: &Vec3, flip: bool) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();
        self.iter().for_each(|polygon| {
            mesh.append(&polygon.cap(m, normal, flip));
        });
        mesh
    }
}

impl Extrude for Geometries2D {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        self.to_multi_polygon().extrude_slice(m_a, m_b)
    }

    fn cap(&self, m: &Mat4, normal: &Vec3, flip: bool) -> TriangleMesh {
        self.to_multi_polygon().cap(m, normal, flip)
    }
}
