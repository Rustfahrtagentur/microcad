// Copyright © 2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! 3D extrusion algorithm.

use std::f64::consts::PI;

use cgmath::{Matrix, Point3, SquareMatrix, Transform, Vector3};

use geo::TriangulateEarcut;

use crate::*;

/// Extrude.
pub trait Extrude {
    /// Extrude a single slice of the geometry with top and bottom plane.
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh;

    /// Generate the cap geometry.
    fn cap(&self, _m: &Mat4, _bottom: bool) -> TriangleMesh {
        TriangleMesh::default()
    }

    /// Perform a linear extrusion with a certain height.
    fn linear_extrude(&self, height: Scalar) -> WithBounds3D<TriangleMesh> {
        let m_a = Mat4::identity();
        let m_b = Mat4::from_translation(Vec3::new(0.0, 0.0, height));
        let mut mesh = self.extrude_slice(&m_a, &m_b);
        mesh.append(&self.cap(&m_a, true));
        mesh.append(&self.cap(&m_b, false));
        let bounds = mesh.calc_bounds_3d();
        mesh.repair(&bounds);
        WithBounds3D::new(mesh, bounds)
    }

    /// Perform a revolve extrusion with a certain angle.
    fn revolve_extrude(&self, angle_rad: Angle, segments: usize) -> WithBounds3D<TriangleMesh> {
        let mut mesh = TriangleMesh::default();
        if segments < 2 {
            return WithBounds3D::default();
        }

        let delta = angle_rad / segments as Scalar;

        // Generate all rotation matrices
        let transforms: Vec<_> = (0..=segments)
            .map(|i| {
                let a = delta * i as Scalar;
                let mut mat = Mat4::from_angle_y(a);
                mat.swap_rows(2, 1); // Align to Z plane
                mat
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
            mesh.append(&self.cap(m_start, true));
            mesh.append(&self.cap(m_end, false));
        }

        let bounds = mesh.calc_bounds_3d();
        mesh.repair(&bounds);
        WithBounds3D::new(mesh, bounds)
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
        // Reserve space for positions and indices
        mesh.positions.reserve(len * 2); // each point produces 2 vertices
        mesh.triangle_indices.reserve(len * 2); // each side produces 2 triangles

        let m_a: cgmath::Matrix4<f32> = m_a.cast().expect("Successful cast");
        let m_b: cgmath::Matrix4<f32> = m_b.cast().expect("Successful cast");

        let transform_point =
            |p: &cgmath::Point3<f32>, m: &cgmath::Matrix4<f32>| -> cgmath::Vector3<f32> {
                m.transform_point(*p).to_homogeneous().truncate()
            };

        // Interleave bottom and top vertex positions
        for point in points {
            let point = cgmath::Point3::new(point.x() as f32, point.y() as f32, 0.0_f32);
            mesh.positions.push(transform_point(&point, &m_a)); // bottom
            mesh.positions.push(transform_point(&point, &m_b)); // top
        }

        let range = if self.is_closed() {
            0..len
        } else {
            0..(len - 1)
        };

        for i in range {
            let next = (i + 1) % len;

            let bl = (i * 2) as u32;
            let br = (next * 2) as u32;
            let tl = bl + 1;
            let tr = br + 1;
            mesh.triangle_indices.push(Triangle(bl, br, tr));
            mesh.triangle_indices.push(Triangle(bl, tr, tl));
        }

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

    fn cap(&self, m: &Mat4, flip: bool) -> TriangleMesh {
        let raw_triangulation = self.earcut_triangles_raw();
        let m: cgmath::Matrix4<f32> = m.cast().expect("Successful cast");

        TriangleMesh {
            positions: raw_triangulation
                .vertices
                .as_slice()
                .chunks_exact(2)
                .map(|chunk| {
                    let p = Point3::new(chunk[0] as f32, chunk[1] as f32, 0.0_f32);
                    let p = m.transform_point(p);
                    Vector3::<f32>::new(p.x, p.y, p.z)
                })
                .collect(),
            normals: None,
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

    fn cap(&self, m: &Mat4, flip: bool) -> TriangleMesh {
        let mut mesh = TriangleMesh::default();
        self.iter().for_each(|polygon| {
            mesh.append(&polygon.cap(m, flip));
        });
        mesh
    }
}

impl Extrude for Geometries2D {
    fn extrude_slice(&self, m_a: &Mat4, m_b: &Mat4) -> TriangleMesh {
        self.to_multi_polygon().extrude_slice(m_a, m_b)
    }

    fn cap(&self, m: &Mat4, flip: bool) -> TriangleMesh {
        self.to_multi_polygon().cap(m, flip)
    }
}
