// Copyright © 2024-2025 The µcad authors <info@ucad.xyz>
// SPDX-License-Identifier: AGPL-3.0-or-later

//! Specializations for 3D triangles.

use crate::Vec3;

/// Vertex
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    /// position
    pub pos: Vec3,
    /// normal vector
    pub normal: Vec3,
}

impl Vertex {
    /// Accumulate normal.
    pub fn accumulate_normal(vertices: &mut [Vertex], i0: u32, i1: u32, i2: u32) {
        let v0 = vertices[i0 as usize].pos;
        let v1 = vertices[i1 as usize].pos;
        let v2 = vertices[i2 as usize].pos;

        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let face_normal = edge1.cross(edge2);

        vertices[i0 as usize].normal += face_normal;
        vertices[i1 as usize].normal += face_normal;
        vertices[i2 as usize].normal += face_normal;
    }
}
